//! Slab Windows std math backend. No operations are disabled.
#![allow(dead_code)]
#[inline]
pub fn exp2(x:f64) -> f64 { x.exp2() }
#[inline]
pub fn floor(x:f64) -> f64 { x.floor() }
#[inline]
pub fn ceil(x:f64) -> f64 { x.ceil() }
#[inline]
pub fn hypot(x:f64,y:f64) -> f64 { x.hypot(y) }
#[inline]
pub fn atan2(y:f64,x:f64) -> f64 { y.atan2(x) }
#[inline]
pub fn sin(x:f64) -> f64 { x.sin() }
#[inline]
pub fn cos(x:f64) -> f64 { x.cos() }
#[inline]
pub fn tan(x:f64) -> f64 { x.tan() }
#[inline]
pub fn round(x:f64) -> f64 { x.round() }
#[inline]
pub fn sqrt(x:f64) -> f64 { x.sqrt() }
#[inline]
pub fn log2(x:f64) -> f64 { x.log2() }
#[inline]
pub fn sincos(x:f64) -> (f64,f64) { x.sin_cos() }
#[inline]
pub fn acos(x:f64) -> f64 { x.acos() }
#[inline]
pub fn asin(x:f64) -> f64 { x.asin() }
#[inline]
pub fn sqrtf(x:f32) -> f32 { x.sqrt() }
#[inline]
pub fn log2f(x:f32) -> f32 { x.log2() }
#[inline]
pub fn cosf(x:f32) -> f32 { x.cos() }
#[inline]
pub fn sinf(x:f32) -> f32 { x.sin() }
#[inline]
pub fn tanf(x:f32) -> f32 { x.tan() }
#[inline]
pub fn sincosf(x:f32) -> (f32,f32) { x.sin_cos() }
#[inline]
pub fn acosf(x:f32) -> f32 { x.acos() }
#[inline]
pub fn asinf(x:f32) -> f32 { x.asin() }
#[inline]
pub fn atan2f(y:f32,x:f32) -> f32 { y.atan2(x) }
#[inline]
pub fn roundf(x:f32) -> f32 { x.round() }
