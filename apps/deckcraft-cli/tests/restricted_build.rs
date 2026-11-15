//! Checks the explicit feature contract of the Slab Windows CLI build.
#![cfg(not(any(feature = "legacy-fallback-font", feature = "system-fonts", feature = "symphonia-audio", feature = "pdf-export")))]

use std::process::Command;

#[test]
fn reduced_build_reports_its_capabilities() {
    let result = Command::new(env!("CARGO_BIN_EXE_deckcraft-cli")).arg("capabilities").output().expect("run CLI");
    assert!(result.status.success());
    let capabilities: serde_json::Value = serde_json::from_slice(&result.stdout).expect("JSON capabilities");
    for key in ["legacyFallbackFont", "systemFonts", "symphoniaAudio", "pdfExport"] {
        assert_eq!(capabilities[key], false, "{key}");
    }
    assert_eq!(capabilities["fallbackFont"], "Inter");
}

#[test]
fn unavailable_pdf_returns_an_error_without_writing_fake_pdf() {
    let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/restricted-test");
    std::fs::create_dir_all(&folder).expect("test folder");
    let path = folder.join(format!("unavailable-{}.pdf", std::process::id()));
    let result = Command::new(env!("CARGO_BIN_EXE_deckcraft-cli")).args(["run", "--save"]).arg(&path).output().expect("run CLI");
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("pdf-export disabled"));
    assert!(!path.exists());
}
