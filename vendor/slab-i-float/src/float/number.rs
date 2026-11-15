use crate::int::number::int::IntNumber;
use crate::int::number::wide_int::WideIntNumber;
use core::fmt::Display;
use core::ops::{Add, Div, Mul, Neg, Sub};

/// Scalar operations used by floating-point geometry.
///
/// The [`crate::float`] coordinate limits apply to geometry inputs, not to this
/// trait's full scalar range or to adapter scales.
pub trait FloatNumber
where
    Self: Copy
        + Mul<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Div<Output = Self>
        + Neg<Output = Self>
        + Display
        + PartialOrd,
{
    const MAX: Self;
    const MIN: Self;
    /// Inclusive maximum absolute input coordinate for floating-point geometry.
    /// This does not limit adapter scales or the scalar's representable range.
    const MAX_COORDINATE: Self;
    /// Smallest positive normal scalar, used as the minimum squared length
    /// supported by floating-point normalization.
    const MIN_POSITIVE: Self;
    const BITS: u32;
    /// One greater than the largest exponent of a finite power of two.
    const MAX_EXP: i32;
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const THREE: Self;
    const FOUR: Self;
    const HALF: Self;

    // Construction.
    fn from_usize(value: usize) -> Self;
    fn from_int<I: IntNumber>(value: I) -> Self;
    fn from_wide_int<I: WideIntNumber>(value: I) -> Self;
    fn from_float<F: FloatNumber>(value: F) -> Self;

    // Math.
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn log2(self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn tan(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn acos(self) -> Self;
    fn asin(self) -> Self;
    /// Returns the signed angle in radians for the vector (x, self).
    fn atan2(self, x: Self) -> Self;
    fn signum(self) -> Self;
    fn is_finite(self) -> bool;

    /// Whether this value is within the inclusive geometry input coordinate
    /// limits defined by [`Self::MAX_COORDINATE`]. NaN and infinity are rejected.
    /// This check does not apply to adapter scales or intermediate results.
    #[inline(always)]
    fn is_in_safe_range(self) -> bool {
        self.abs() <= Self::MAX_COORDINATE
    }

    // Truncating casts.
    fn to_i16(self) -> i16;
    fn to_i32(self) -> i32;
    fn to_i64(self) -> i64;
    fn to_usize(self) -> usize;
    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;

    // Rounding casts.
    fn to_round_i16(self) -> i16;
    fn to_round_i32(self) -> i32;
    fn to_round_i64(self) -> i64;
    fn to_round_i128(self) -> i128;
    fn to_round_usize(self) -> usize;
}

impl FloatNumber for f32 {
    const MAX: Self = f32::MAX;
    const MIN: Self = f32::MIN;
    const MAX_COORDINATE: Self = f32::from_bits((127 + 60) << 23);
    const MIN_POSITIVE: Self = f32::MIN_POSITIVE;
    const BITS: u32 = 32;
    const MAX_EXP: i32 = f32::MAX_EXP;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;
    const FOUR: Self = 4.0;
    const HALF: Self = 0.5;

    // Construction.
    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as f32
    }

    #[inline(always)]
    fn from_int<I: IntNumber>(value: I) -> Self {
        value.to_f64() as f32
    }

    #[inline(always)]
    fn from_wide_int<I: WideIntNumber>(value: I) -> Self {
        value.to_f32()
    }

    #[inline(always)]
    fn from_float<F: FloatNumber>(value: F) -> Self {
        value.to_f32()
    }

    // Math.
    #[inline(always)]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        crate::math::sqrtf(self)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    fn log2(self) -> Self {
        crate::math::log2f(self)
    }

    #[inline(always)]
    fn cos(self) -> Self {
        crate::math::cosf(self)
    }

    #[inline(always)]
    fn sin(self) -> Self {
        crate::math::sinf(self)
    }

    #[inline(always)]
    fn tan(self) -> Self {
        crate::math::tanf(self)
    }

    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        crate::math::sincosf(self)
    }

    #[inline(always)]
    fn acos(self) -> Self {
        crate::math::acosf(self)
    }

    #[inline(always)]
    fn asin(self) -> Self {
        crate::math::asinf(self)
    }

    #[inline(always)]
    fn atan2(self, x: Self) -> Self {
        crate::math::atan2f(self, x)
    }

    #[inline(always)]
    fn signum(self) -> Self {
        self.signum()
    }

    #[inline(always)]
    fn is_finite(self) -> bool {
        self.is_finite()
    }

    // Truncating casts.
    #[inline(always)]
    fn to_i16(self) -> i16 {
        self as i16
    }

    #[inline(always)]
    fn to_i32(self) -> i32 {
        self as i32
    }

    #[inline(always)]
    fn to_i64(self) -> i64 {
        self as i64
    }

    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn to_f32(self) -> f32 {
        self
    }

    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }

    // Rounding casts.
    #[inline(always)]
    fn to_round_i16(self) -> i16 {
        crate::math::roundf(self) as i16
    }

    #[inline(always)]
    fn to_round_i32(self) -> i32 {
        crate::math::roundf(self) as i32
    }

    #[inline(always)]
    fn to_round_i64(self) -> i64 {
        crate::math::roundf(self) as i64
    }

    #[inline(always)]
    fn to_round_i128(self) -> i128 {
        crate::math::roundf(self) as i128
    }

    #[inline(always)]
    fn to_round_usize(self) -> usize {
        crate::math::roundf(self) as usize
    }
}

impl FloatNumber for f64 {
    const MAX: Self = f64::MAX;
    const MIN: Self = f64::MIN;
    const MAX_COORDINATE: Self = f64::from_bits((1023 + 500) << 52);
    const MIN_POSITIVE: Self = f64::MIN_POSITIVE;
    const BITS: u32 = 64;
    const MAX_EXP: i32 = f64::MAX_EXP;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    const THREE: Self = 3.0;
    const FOUR: Self = 4.0;
    const HALF: Self = 0.5;
    // Construction.
    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as f64
    }

    #[inline(always)]
    fn from_int<I: IntNumber>(value: I) -> Self {
        value.to_f64()
    }

    #[inline(always)]
    fn from_wide_int<I: WideIntNumber>(value: I) -> Self {
        value.to_f64()
    }

    #[inline(always)]
    fn from_float<F: FloatNumber>(value: F) -> Self {
        value.to_f64()
    }

    // Math.
    #[inline(always)]
    fn abs(self) -> Self {
        self.abs()
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        crate::math::sqrt(self)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    #[inline(always)]
    fn log2(self) -> Self {
        crate::math::log2(self)
    }

    #[inline(always)]
    fn cos(self) -> Self {
        crate::math::cos(self)
    }

    #[inline(always)]
    fn sin(self) -> Self {
        crate::math::sin(self)
    }

    #[inline(always)]
    fn tan(self) -> Self {
        crate::math::tan(self)
    }

    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        crate::math::sincos(self)
    }

    #[inline(always)]
    fn acos(self) -> Self {
        crate::math::acos(self)
    }

    #[inline(always)]
    fn asin(self) -> Self {
        crate::math::asin(self)
    }

    #[inline(always)]
    fn atan2(self, x: Self) -> Self {
        crate::math::atan2(self, x)
    }

    #[inline(always)]
    fn signum(self) -> Self {
        self.signum()
    }

    #[inline(always)]
    fn is_finite(self) -> bool {
        self.is_finite()
    }

    // Truncating casts.
    #[inline(always)]
    fn to_i16(self) -> i16 {
        self as i16
    }

    #[inline(always)]
    fn to_i32(self) -> i32 {
        self as i32
    }

    #[inline(always)]
    fn to_i64(self) -> i64 {
        self as i64
    }

    #[inline(always)]
    fn to_usize(self) -> usize {
        self as usize
    }

    #[inline(always)]
    fn to_f32(self) -> f32 {
        self as f32
    }

    #[inline(always)]
    fn to_f64(self) -> f64 {
        self
    }

    // Rounding casts.
    #[inline(always)]
    fn to_round_i16(self) -> i16 {
        crate::math::round(self) as i16
    }

    #[inline(always)]
    fn to_round_i32(self) -> i32 {
        crate::math::round(self) as i32
    }

    #[inline(always)]
    fn to_round_i64(self) -> i64 {
        crate::math::round(self) as i64
    }

    #[inline(always)]
    fn to_round_i128(self) -> i128 {
        crate::math::round(self) as i128
    }

    #[inline(always)]
    fn to_round_usize(self) -> usize {
        crate::math::round(self) as usize
    }
}
