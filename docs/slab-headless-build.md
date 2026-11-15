# Slab headless evaluation build

This local patch retains DeckCraft's real CLI, engine and stdio MCP server. It adds
explicit Cargo gates; it does not implement a replacement format adapter.

The ordinary CLI and UI builds retain the existing defaults. A CLI built with
`--no-default-features` excludes these optional inputs:

| Feature | Default input | Behavior when disabled |
|---|---|---|
| `symphonia-audio` | MPL-2.0 audio fallback | Native Opus/video paths remain; unsupported audio returns an explicit error and is not reported decodable |
| `legacy-fallback-font` | egui's Ubuntu Light font | Build requires Inter Regular from `CRAFT_FONTS_DIR`; no egui default font package |
| `system-fonts` | Installed-font directory scan | Platform font directories are empty; only supplied/embedded document fonts remain |
| `pdf-export` | craft-pdf/Krilla exporter | PDF requests return an explicit error; PPTX/native/PNG operations remain |

The PDF gate is needed because the exact Krilla upstream NOTICE identifies
MPL-derived `content_draw_path` code, still present in the pinned vendor snapshot.
That exporter also introduces a core_maths/libm path. The independent i_float
path to libm is replaced on Windows by native std math methods in the retained
MIT-licensed i_float source. All its numeric/geometric operations remain;
the upstream no_std backend is kept only for other targets, outside this audit.

ZIP Deflate uses the existing `flate2` + `miniz_oxide` backend explicitly instead
of zip's umbrella feature, which also selects zopfli/zlib-rs. This preserves the
ZIP/PPTX formats; compressed byte streams can differ. Save/reopen is checked.

Use a reviewed font input directory with a `fonts/manifest.txt` containing only
the approved font rows and files. This evaluation uses byte-identical Inter 4.1
Regular, SHA-256 `40d692fce188e4471e2b3cba937be967878f631ad3ebbbdcd587687c7ebe0c82`,
from `storytold/craft-fonts` commit `c1db6d1a8ed772191241c3d185e2104be84d8691`,
originally `rsms/inter` release v4.1. Its complete OFL-1.1 text must travel with it.
Do not sell the font by itself. More scripts/styles need additional reviewed faces;
this input is deliberately limited, not a complete presentation font catalog.

Set `CRAFT_FONTS_DIR` to that directory and a session-specific `CARGO_TARGET_DIR`.
After reviewing the selected closure and inputs:

```text
cargo build --locked --offline -p deckcraft-cli --no-default-features --target x86_64-pc-windows-msvc
deckcraft-cli capabilities
deckcraft-cli mcp --automation-read-root PATH --automation-write-root PATH
```

`capabilities` and the MCP `build_capabilities` tool report the actual compiled
font/audio/PDF options. The report describes this server's engine; a remote
desktop process can have different capabilities. Capability errors are retained
instead of silently omitting a requested export or generating a fake file.

Scope is this Windows headless build. GUI, web, other platforms, full multimedia,
PDF, arbitrary document fonts/media, every imported file and later plugins require
their own audit. Upstream NOTICE/ATTRIBUTION files remain intact.
