/// Binding to CRlibm, a library of proved correctly-rounded
/// mathematical functions.


mod crlibm {
    use std::os::raw::{c_double, c_ulonglong};

    #[link(name = "crlibm")]
    extern "C" {
        pub fn crlibm_init() -> c_ulonglong;
        pub fn crlibm_exit(oldcw: c_ulonglong);

        // Order of the definitions is the same as in crlibm/crlibm.h
        pub fn exp_rn(x: c_double) -> c_double;
        pub fn exp_ru(x: c_double) -> c_double;
        pub fn exp_rd(x: c_double) -> c_double;

        pub fn log_rn(x: c_double) -> c_double;
        pub fn log_ru(x: c_double) -> c_double;
        pub fn log_rd(x: c_double) -> c_double;
        pub fn log_rz(x: c_double) -> c_double;

        pub fn cos_rn(x: c_double) -> c_double;
        pub fn cos_ru(x: c_double) -> c_double;
        pub fn cos_rd(x: c_double) -> c_double;
        pub fn cos_rz(x: c_double) -> c_double;

        pub fn sin_rn(x: c_double) -> c_double;
        pub fn sin_ru(x: c_double) -> c_double;
        pub fn sin_rd(x: c_double) -> c_double;
        pub fn sin_rz(x: c_double) -> c_double;

        pub fn tan_rn(x: c_double) -> c_double;
        pub fn tan_ru(x: c_double) -> c_double;
        pub fn tan_rd(x: c_double) -> c_double;
        pub fn tan_rz(x: c_double) -> c_double;

        pub fn cospi_rn(x: c_double) -> c_double;
        pub fn cospi_ru(x: c_double) -> c_double;
        pub fn cospi_rd(x: c_double) -> c_double;
        pub fn cospi_rz(x: c_double) -> c_double;

        pub fn sinpi_rn(x: c_double) -> c_double;
        pub fn sinpi_ru(x: c_double) -> c_double;
        pub fn sinpi_rd(x: c_double) -> c_double;
        pub fn sinpi_rz(x: c_double) -> c_double;

        pub fn tanpi_rn(x: c_double) -> c_double;
        pub fn tanpi_ru(x: c_double) -> c_double;
        pub fn tanpi_rd(x: c_double) -> c_double;
        pub fn tanpi_rz(x: c_double) -> c_double;

        pub fn atan_rn(x: c_double) -> c_double;
        pub fn atan_ru(x: c_double) -> c_double;
        pub fn atan_rd(x: c_double) -> c_double;
        pub fn atan_rz(x: c_double) -> c_double;

        pub fn atanpi_rn(x: c_double) -> c_double;
        pub fn atanpi_ru(x: c_double) -> c_double;
        pub fn atanpi_rd(x: c_double) -> c_double;
        pub fn atanpi_rz(x: c_double) -> c_double;

        pub fn cosh_rn(x: c_double) -> c_double;
        pub fn cosh_ru(x: c_double) -> c_double;
        pub fn cosh_rd(x: c_double) -> c_double;
        pub fn cosh_rz(x: c_double) -> c_double;

        pub fn sinh_rn(x: c_double) -> c_double;
        pub fn sinh_ru(x: c_double) -> c_double;
        pub fn sinh_rd(x: c_double) -> c_double;
        pub fn sinh_rz(x: c_double) -> c_double;

        pub fn log2_rn(x: c_double) -> c_double;
        pub fn log2_ru(x: c_double) -> c_double;
        pub fn log2_rd(x: c_double) -> c_double;
        pub fn log2_rz(x: c_double) -> c_double;

        pub fn log10_rn(x: c_double) -> c_double;
        pub fn log10_ru(x: c_double) -> c_double;
        pub fn log10_rd(x: c_double) -> c_double;
        pub fn log10_rz(x: c_double) -> c_double;

        pub fn asin_rn(x: c_double) -> c_double;
        pub fn asin_ru(x: c_double) -> c_double;
        pub fn asin_rd(x: c_double) -> c_double;
        pub fn asin_rz(x: c_double) -> c_double;

        pub fn acos_rn(x: c_double) -> c_double;
        pub fn acos_ru(x: c_double) -> c_double;
        pub fn acos_rd(x: c_double) -> c_double;

        pub fn asinpi_rn(x: c_double) -> c_double;
        pub fn asinpi_ru(x: c_double) -> c_double;
        pub fn asinpi_rd(x: c_double) -> c_double;
        pub fn asinpi_rz(x: c_double) -> c_double;

        pub fn acospi_rn(x: c_double) -> c_double;
        pub fn acospi_ru(x: c_double) -> c_double;
        pub fn acospi_rd(x: c_double) -> c_double;

        pub fn expm1_rn(x: c_double) -> c_double;
        pub fn expm1_ru(x: c_double) -> c_double;
        pub fn expm1_rd(x: c_double) -> c_double;
        pub fn expm1_rz(x: c_double) -> c_double;

        pub fn log1p_rn(x: c_double) -> c_double;
        pub fn log1p_ru(x: c_double) -> c_double;
        pub fn log1p_rd(x: c_double) -> c_double;
        pub fn log1p_rz(x: c_double) -> c_double;
    }
}

use std::sync::Once;

static C_LIB_INITIALIZED: Once = Once::new();
static mut CW : std::os::raw::c_ulonglong = 0;

#[ctor::ctor]
fn init() {
    C_LIB_INITIALIZED.call_once(|| unsafe {
        CW = crlibm::crlibm_init();
    });
}

#[ctor::dtor]
fn exit() { unsafe { crlibm::crlibm_exit(CW); }}

/// `exp(x)` rounded to the nearset floating point.
pub fn exp_rn(x: f64) -> f64 { unsafe { crlibm::exp_rn(x) }}
/// `exp(x)` rounded toward +∞.
pub fn exp_ru(x: f64) -> f64 { unsafe { crlibm::exp_ru(x) }}
/// `exp(x)` rounded toward -∞.
pub fn exp_rd(x: f64) -> f64 { unsafe { crlibm::exp_rd(x) }}
/// `exp(x)` rounded toward zero.
pub fn exp_rz(x: f64) -> f64 {
    // See crlibm.h, exp_rz is just an alias for exp_rd.
    unsafe { crlibm::exp_rd(x) }}

/// `expm1(x)` returns `exp(x) - 1` (rounded to the nearest) in a way
/// that is accurate even for values of `x` near zero.
pub fn expm1_rn(x: f64) -> f64 { unsafe { crlibm::expm1_rn(x) }}

/// `expm1(x)` returns `exp(x) - 1` (rounded toward + ∞) in a way
/// that is accurate even for values of `x` near zero.
pub fn expm1_ru(x: f64) -> f64 { unsafe { crlibm::expm1_ru(x) }}

/// `expm1(x)` returns `exp(x) - 1` (rounded toward -∞) in a way
/// that is accurate even for values of `x` near zero.
pub fn expm1_rd(x: f64) -> f64 { unsafe { crlibm::expm1_rd(x) }}

/// `expm1(x)` returns `exp(x) - 1` (rounded toward zero) in a way
/// that is accurate even for values of `x` near zero.
pub fn expm1_rz(x: f64) -> f64 { unsafe { crlibm::expm1_rz(x) }}

/// `log(x)` rounded to the nearset floating point.
pub fn log_rn(x: f64) -> f64 { unsafe { crlibm::log_rn(x) }}
/// `log(x)` rounded toward +∞.
pub fn log_ru(x: f64) -> f64 { unsafe { crlibm::log_ru(x) }}
/// `log(x)` rounded toward -∞.
pub fn log_rd(x: f64) -> f64 { unsafe { crlibm::log_rd(x) }}
/// `log(x)` rounded toward zero.
pub fn log_rz(x: f64) -> f64 { unsafe { crlibm::log_rz(x) }}

/// `log1p(x)` returns `log(1 + x)` (rounded to the nearest) in a way
/// that is accurate even for values of `x` near zero.
pub fn log1p_rn(x: f64) -> f64 { unsafe { crlibm::log1p_rn(x) }}

/// `log1p(x)` returns `log(1 + x)` (rounded toward +∞) in a way
/// that is accurate even for values of `x` near zero.
pub fn log1p_ru(x: f64) -> f64 { unsafe { crlibm::log1p_ru(x) }}

/// `log1p(x)` returns `log(1 + x)` (rounded toward -∞) in a way
/// that is accurate even for values of `x` near zero.
pub fn log1p_rd(x: f64) -> f64 { unsafe { crlibm::log1p_rd(x) }}

/// `log1p(x)` returns `log(1 + x)` (rounded toward zero) in a way
/// that is accurate even for values of `x` near zero.
pub fn log1p_rz(x: f64) -> f64 { unsafe { crlibm::log1p_rz(x) }}

/// `cos(x)` rounded to the nearset floating point.
pub fn cos_rn(x: f64) -> f64 { unsafe { crlibm::cos_rn(x) }}
/// `cos(x)` rounded toward +∞.
pub fn cos_ru(x: f64) -> f64 { unsafe { crlibm::cos_ru(x) }}
/// `cos(x)` rounded toward -∞.
pub fn cos_rd(x: f64) -> f64 { unsafe { crlibm::cos_rd(x) }}
/// `cos(x)` rounded toward zero.
pub fn cos_rz(x: f64) -> f64 { unsafe { crlibm::cos_rz(x) }}

/// `sin(x)` rounded to the nearset.
pub fn sin_rn(x: f64) -> f64 { unsafe { crlibm::sin_rn(x) }}
/// `sin(x)` rounded toward +∞.
pub fn sin_ru(x: f64) -> f64 { unsafe { crlibm::sin_ru(x) }}
/// `sin(x)` rounded toward -∞.
pub fn sin_rd(x: f64) -> f64 { unsafe { crlibm::sin_rd(x) }}
/// `sin(x)` rounded toward zero.
pub fn sin_rz(x: f64) -> f64 { unsafe { crlibm::sin_rz(x) }}

/// `tan(x)` rounded to the nearset.
pub fn tan_rn(x: f64) -> f64 { unsafe { crlibm::tan_rn(x) }}
/// `tan(x)` rounded toward +∞.
pub fn tan_ru(x: f64) -> f64 { unsafe { crlibm::tan_ru(x) }}
/// `tan(x)` rounded toward -∞.
pub fn tan_rd(x: f64) -> f64 { unsafe { crlibm::tan_rd(x) }}
/// `tan(x)` rounded toward zero.
pub fn tan_rz(x: f64) -> f64 { unsafe { crlibm::tan_rz(x) }}

/// `cospi(x)` returns `cos(π·x)` rounded to the nearset.
pub fn cospi_rn(x: f64) -> f64 { unsafe { crlibm::cospi_rn(x) }}
/// `cospi(x)` returns `cos(π·x)` rounded toward +∞.
pub fn cospi_ru(x: f64) -> f64 { unsafe { crlibm::cospi_ru(x) }}
/// `cospi(x)` returns `cos(π·x)` rounded toward -∞.
pub fn cospi_rd(x: f64) -> f64 { unsafe { crlibm::cospi_rd(x) }}
/// `cospi(x)` returns `cos(π·x)` rounded toward zero.
pub fn cospi_rz(x: f64) -> f64 { unsafe { crlibm::cospi_rz(x) }}

/// `sinpi(x)` returns `sin(π·x)` rounded to the nearset.
pub fn sinpi_rn(x: f64) -> f64 { unsafe { crlibm::sinpi_rn(x) }}
/// `sinpi(x)` returns `sin(π·x)` rounded toward +∞.
pub fn sinpi_ru(x: f64) -> f64 { unsafe { crlibm::sinpi_ru(x) }}
/// `sinpi(x)` returns `sin(π·x)` rounded toward -∞.
pub fn sinpi_rd(x: f64) -> f64 { unsafe { crlibm::sinpi_rd(x) }}
/// `sinpi(x)` returns `sin(π·x)` rounded toward zero.
pub fn sinpi_rz(x: f64) -> f64 { unsafe { crlibm::sinpi_rz(x) }}

/// `tanpi(x)` returns `tan(π·x)` rounded to the nearset.
pub fn tanpi_rn(x: f64) -> f64 { unsafe { crlibm::tanpi_rn(x) }}
/// `tanpi(x)` returns `tan(π·x)` rounded toward +∞.
pub fn tanpi_ru(x: f64) -> f64 { unsafe { crlibm::tanpi_ru(x) }}
/// `tanpi(x)` returns `tan(π·x)` rounded toward -∞.
pub fn tanpi_rd(x: f64) -> f64 { unsafe { crlibm::tanpi_rd(x) }}
/// `tanpi(x)` returns `tan(π·x)` rounded toward zero.
pub fn tanpi_rz(x: f64) -> f64 { unsafe { crlibm::tanpi_rz(x) }}

/// `asin(x)` rounded to the nearset.
pub fn asin_rn(x: f64) -> f64 { unsafe { crlibm::asin_rn(x) }}
/// `asin(x)` rounded toward +∞.
pub fn asin_ru(x: f64) -> f64 { unsafe { crlibm::asin_ru(x) }}
/// `asin(x)` rounded toward -∞.
pub fn asin_rd(x: f64) -> f64 { unsafe { crlibm::asin_rd(x) }}
/// `asin(x)` rounded toward zero.
pub fn asin_rz(x: f64) -> f64 { unsafe { crlibm::asin_rz(x) }}

/// `acos(x)` rounded to the nearset.
pub fn acos_rn(x: f64) -> f64 { unsafe { crlibm::acos_rn(x) }}
/// `acos(x)` rounded toward +∞.
pub fn acos_ru(x: f64) -> f64 { unsafe { crlibm::acos_ru(x) }}
/// `acos(x)` rounded toward -∞.
pub fn acos_rd(x: f64) -> f64 { unsafe { crlibm::acos_rd(x) }}
/// `acos(x)` rounded toward zero.
pub fn acos_rz(x: f64) -> f64 {
    // See crlibm.h, acos_rz is just an alias for alias_rd
    unsafe { crlibm::acos_rd(x) }}

/// `atan(x)` rounded to the nearset.
pub fn atan_rn(x: f64) -> f64 { unsafe { crlibm::atan_rn(x) }}
/// `atan(x)` rounded toward +∞.
pub fn atan_ru(x: f64) -> f64 { unsafe { crlibm::atan_ru(x) }}
/// `atan(x)` rounded toward -∞.
pub fn atan_rd(x: f64) -> f64 { unsafe { crlibm::atan_rd(x) }}
/// `atan(x)` rounded toward zero.
pub fn atan_rz(x: f64) -> f64 { unsafe { crlibm::atan_rz(x) }}

/// `asinpi(x)` returns `asin(x)`/π ∈ \[-0.5, 0.5\] rounded to the nearset.
pub fn asinpi_rn(x: f64) -> f64 { unsafe { crlibm::asinpi_rn(x) }}
/// `asinpi(x)` returns `asin(x)`/π ∈ \[-0.5, 0.5\] rounded toward +∞.
pub fn asinpi_ru(x: f64) -> f64 { unsafe { crlibm::asinpi_ru(x) }}
/// `asinpi(x)` returns `asin(x)`/π ∈ \[-0.5, 0.5\] rounded toward -∞.
pub fn asinpi_rd(x: f64) -> f64 { unsafe { crlibm::asinpi_rd(x) }}
/// `asinpi(x)` returns `asin(x)`/π ∈ \[-0.5, 0.5\] rounded toward zero.
pub fn asinpi_rz(x: f64) -> f64 { unsafe { crlibm::asinpi_rz(x) }}

/// `acospi(x)` returns `acos(x)`/π ∈ \[0., 1.\] rounded to the nearset.
pub fn acospi_rn(x: f64) -> f64 { unsafe { crlibm::acospi_rn(x) }}
/// `acospi(x)` returns `acos(x)`/π ∈ \[0., 1.\] rounded toward +∞.
pub fn acospi_ru(x: f64) -> f64 { unsafe { crlibm::acospi_ru(x) }}
/// `acospi(x)` returns `acos(x)`/π ∈ \[0., 1.\] rounded toward -∞.
pub fn acospi_rd(x: f64) -> f64 { unsafe { crlibm::acospi_rd(x) }}
/// `acospi(x)` returns `acos(x)`/π ∈ \[0., 1.\] rounded toward zero.
pub fn acospi_rz(x: f64) -> f64 {
    // See crlibm.h, acospi_rz is an alias for acospi_rd
    unsafe { crlibm::acospi_rd(x) }}

/// `atanpi(x)` returns `atan(x)`/π ∈ \[-0.5, 0.5\] rounded to the nearset.
pub fn atanpi_rn(x: f64) -> f64 { unsafe { crlibm::atanpi_rn(x) }}
/// `atanpi(x)` returns `atan(x)`/π ∈ \[-0.5, 0.5\] rounded toward +∞.
pub fn atanpi_ru(x: f64) -> f64 { unsafe { crlibm::atanpi_ru(x) }}
/// `atanpi(x)` returns `atan(x)`/π ∈ \[-0.5, 0.5\] rounded toward -∞.
pub fn atanpi_rd(x: f64) -> f64 { unsafe { crlibm::atanpi_rd(x) }}
/// `atanpi(x)` returns `atan(x)`/π ∈ \[-0.5, 0.5\] rounded toward zero.
pub fn atanpi_rz(x: f64) -> f64 { unsafe { crlibm::atanpi_rz(x) }}

/// `sinh(x)` rounded to the nearset.
pub fn sinh_rn(x: f64) -> f64 { unsafe { crlibm::sinh_rn(x) }}
/// `sinh(x)` rounded toward +∞.
pub fn sinh_ru(x: f64) -> f64 { unsafe { crlibm::sinh_ru(x) }}
/// `sinh(x)` rounded toward -∞.
pub fn sinh_rd(x: f64) -> f64 { unsafe { crlibm::sinh_rd(x) }}
/// `sinh(x)` rounded toward zero.
pub fn sinh_rz(x: f64) -> f64 { unsafe { crlibm::sinh_rz(x) }}

/// `cosh(x)` rounded to the nearset.
pub fn cosh_rn(x: f64) -> f64 { unsafe { crlibm::cosh_rn(x) }}
/// `cosh(x)` rounded toward +∞.
pub fn cosh_ru(x: f64) -> f64 { unsafe { crlibm::cosh_ru(x) }}
/// `cosh(x)` rounded toward -∞.
pub fn cosh_rd(x: f64) -> f64 { unsafe { crlibm::cosh_rd(x) }}
/// `cosh(x)` rounded toward zero.
pub fn cosh_rz(x: f64) -> f64 { unsafe { crlibm::cosh_rz(x) }}

/// `log2(x)` rounded to the nearset.
pub fn log2_rn(x: f64) -> f64 { unsafe { crlibm::log2_rn(x) }}
/// `log2(x)` rounded toward +∞.
pub fn log2_ru(x: f64) -> f64 { unsafe { crlibm::log2_ru(x) }}
/// `log2(x)` rounded toward -∞.
pub fn log2_rd(x: f64) -> f64 { unsafe { crlibm::log2_rd(x) }}
/// `log2(x)` rounded toward zero.
pub fn log2_rz(x: f64) -> f64 { unsafe { crlibm::log2_rz(x) }}

/// `log10(x)` rounded to the nearset.
pub fn log10_rn(x: f64) -> f64 { unsafe { crlibm::log10_rn(x) }}
/// `log10(x)` rounded toward +∞.
pub fn log10_ru(x: f64) -> f64 { unsafe { crlibm::log10_ru(x) }}
/// `log10(x)` rounded toward -∞.
pub fn log10_rd(x: f64) -> f64 { unsafe { crlibm::log10_rd(x) }}
/// `log10(x)` rounded toward zero.
pub fn log10_rz(x: f64) -> f64 { unsafe { crlibm::log10_rz(x) }}




#[cfg(test)]
mod tests {
    use crate::*;
    use std::f64::consts::PI;

    #[test]
    pub fn test_cos_pi_ru() {
        assert_ne!(cos_rn(PI), cos_ru(PI));
    }
    #[test]
    pub fn test_cos_pi_rd() {
        assert_eq!(cos_rn(PI), cos_rd(PI));
    }
    #[test]
    pub fn test_cospi_rd() {
        assert_eq!(cospi_rn(1.), cospi_rd(1.));
    }
    #[test]
    pub fn test_cospi_ru() {
        assert_eq!(cospi_rn(1.), cospi_ru(1.));
    }
}
