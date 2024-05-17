//! This crate provides a set of traits for saturating conversion between
//! different numeric types.
//!
//! The trait [`SaturatingFrom`] is implemented by default for all standard
//! numeric types. A blanket implementation of [`SaturatingInto`] is also
//! provided, mirroring the standard library's [`From`] and [`Into`] traits.
//!
//! ## Example
//! ```
//! use saturate::{SaturatingFrom, SaturatingInto};
//!
//! assert_eq!(0, u8::saturating_from(-26));
//! assert_eq!(u32::MAX, i64::MAX.saturating_into());
//! assert!(f32::saturating_from(u128::MAX).is_infinite()); // out of range => infinity
//! assert_eq!(u8::MAX, 300.0.saturating_into());
//! ```

/// Trait to perform a saturating conversion between two numeric types. It is
/// the opposite of [`SaturatingInto`].
///
/// [`SaturatingFrom`] should always be implemented directly; this will also
/// automatically provide an implementation of [`SaturatingInto`] thanks to its
/// blanket implementation.
pub trait SaturatingFrom<T> {
    /// Converts the input type `T` to `Self`
    fn saturating_from(value: T) -> Self;
}

macro_rules! impl_self {
    ($($typ:ty),+) => {
        $(
            impl SaturatingFrom<$typ> for $typ {
                #[inline]
                fn saturating_from(value: $typ) -> $typ {
                    value
                }
            }
        )+
    };
}

impl_self!(bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f64, f32);

macro_rules! impl_from {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    <$dst>::from(value)
                }
            }
        )+
    };
}

impl_from!([bool] => u8);
impl_from!([bool, u8] => u16);
impl_from!([bool, u8, u16] => u32);
impl_from!([bool, u8, u16, u32] => u64);
impl_from!([bool, u8, u16, u32, u64] => u128);

impl_from!([bool] => i8);
impl_from!([bool, i8, u8] => i16);
impl_from!([bool, i8, u8, i16, u16] => i32);
impl_from!([bool, i8, u8, i16, u16, i32, u32] => i64);
impl_from!([bool, i8, u8, i16, u16, i32, u32, i64, u64] => i128);

impl_from!([i8, u8, i16, u16] => f32);
impl_from!([i8, u8, i16, u16, i32, u32, f32] => f64);

macro_rules! impl_clamp {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    use core::convert::TryFrom;
                    // try_from(..).unwrap() is optimised out (tested on 1.78 with opt-level=2)
                    <$dst>::try_from(value.min(<$src>::from(<$dst>::MAX)).max(<$src>::from(<$dst>::MIN))).unwrap()
                }
            }
        )+
    };
}

impl_clamp!([i16, u16, i32, u32, i64, u64, i128, u128] => u8);
impl_clamp!([i32, u32, i64, u64, i128, u128] => u16);
impl_clamp!([i64, u64, i128, u128] => u32);
impl_clamp!([i128, u128] => u64);

impl_clamp!([i16, i32, i64, i128] => i8);
impl_clamp!([i32, i64, i128] => i16);
impl_clamp!([i64, i128] => i32);
impl_clamp!([i128] => i64);

macro_rules! impl_clamp_unsigned {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    use core::convert::TryFrom;
                    // try_from(..).unwrap() is optimised out (tested on 1.78 with opt-level=2)
                    <$dst>::try_from(value.min(<$src>::try_from(<$dst>::MAX).unwrap())).unwrap()
                }
            }
        )+
    };
}

impl_clamp_unsigned!([u8, u16, u32, u64, u128] => i8);
impl_clamp_unsigned!([u16, u32, u64, u128] => i16);
impl_clamp_unsigned!([u32, u64, u128] => i32);
impl_clamp_unsigned!([u64, u128] => i64);
impl_clamp_unsigned!([u128] => i128);
impl_clamp_unsigned!([usize] => isize);

macro_rules! impl_clamp_signed {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    use core::convert::TryFrom;
                    // try_from(..).unwrap() is optimised out (tested on 1.78 with opt-level=2)
                    <$dst>::try_from(value.max(0)).unwrap()
                }
            }
        )+
    };
}

impl_clamp_signed!([i8] => u8);
impl_clamp_signed!([i8, i16] => u16);
impl_clamp_signed!([i8, i16, i32] => u32);
impl_clamp_signed!([i8, i16, i32, i64] => u64);
impl_clamp_signed!([i8, i16, i32, i64, i128] => u128);
impl_clamp_signed!([isize] => usize);

macro_rules! impl_gt_zero {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    value > 0
                }
            }
        )+
    };
}

impl_gt_zero!([i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize] => bool);

macro_rules! impl_gt_zero_float {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    value > 0.0
                }
            }
        )+
    };
}

impl_gt_zero_float!([f32, f64] => bool);

macro_rules! impl_as {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    value as $dst
                }
            }
        )+
    };
}

// `as` will round to nearest (and saturate at f32::INFINITY for `u128` => f32)
impl_as!([i32, u32, i64, u64, i128, u128] => f32);
impl_as!([i64, u64, i128, u128] => f64);
impl_as!([f64] => f32);

// `as` will saturate and convert NaN => 0 since 1.45 (see: rust-lang/rust#10184)
impl_as!([f32, f64] => u8);
impl_as!([f32, f64] => u16);
impl_as!([f32, f64] => u32);
impl_as!([f32, f64] => u64);
impl_as!([f32, f64] => u128);

impl_as!([f32, f64] => i8);
impl_as!([f32, f64] => i16);
impl_as!([f32, f64] => i32);
impl_as!([f32, f64] => i64);
impl_as!([f32, f64] => i128);

macro_rules! impl_bool_float {
    ($($dst:ty),+) => {
        $(
            impl SaturatingFrom<bool> for $dst {
                #[inline]
                fn saturating_from(value: bool) -> $dst {
                    <$dst>::from(u8::from(value))
                }
            }
        )+
    };
}

impl_bool_float!(f32, f64);

macro_rules! impl_equivalent {
    ([$($src:ty as $equ:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    <$dst>::saturating_from(value as $equ)
                }
            }
        )+
    };
    ([$($src:ty),+] => $dst:ty as $equ:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(value: $src) -> $dst {
                    <$equ>::saturating_from(value) as $dst
                }
            }
        )+
    };
}

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
compile_error!("Unsupported target_pointer_width setting");

// Special handling for usize/isize since they vary
#[cfg(target_pointer_width = "16")]
mod size {
    use crate::SaturatingFrom;

    impl_equivalent!([isize as i16, usize as u16] => i8);
    impl_equivalent!([isize as i16, usize as u16] => i16);
    impl_equivalent!([isize as i16, usize as u16] => i32);
    impl_equivalent!([isize as i16, usize as u16] => i64);
    impl_equivalent!([isize as i16, usize as u16] => i128);

    impl_equivalent!([isize as i16, usize as u16] => u8);
    impl_equivalent!([isize as i16, usize as u16] => u16);
    impl_equivalent!([isize as i16, usize as u16] => u32);
    impl_equivalent!([isize as i16, usize as u16] => u64);
    impl_equivalent!([isize as i16, usize as u16] => u128);

    impl_equivalent!([isize as i16, usize as u16] => f32);
    impl_equivalent!([isize as i16, usize as u16] => f64);

    impl_equivalent!([bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64] => usize as u16);
    impl_equivalent!([bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64] => isize as i16);
}

#[cfg(target_pointer_width = "32")]
mod size {
    use crate::SaturatingFrom;

    impl_equivalent!([isize as i32, usize as u32] => i8);
    impl_equivalent!([isize as i32, usize as u32] => i16);
    impl_equivalent!([isize as i32, usize as u32] => i32);
    impl_equivalent!([isize as i32, usize as u32] => i64);
    impl_equivalent!([isize as i32, usize as u32] => i128);

    impl_equivalent!([isize as i32, usize as u32] => u8);
    impl_equivalent!([isize as i32, usize as u32] => u16);
    impl_equivalent!([isize as i32, usize as u32] => u32);
    impl_equivalent!([isize as i32, usize as u32] => u64);
    impl_equivalent!([isize as i32, usize as u32] => u128);

    impl_equivalent!([isize as i32, usize as u32] => f32);
    impl_equivalent!([isize as i32, usize as u32] => f64);

    impl_equivalent!([bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64] => usize as u32);
    impl_equivalent!([bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64] => isize as i32);
}

#[cfg(target_pointer_width = "64")]
mod size {
    use crate::SaturatingFrom;

    impl_equivalent!([isize as i64, usize as u64] => i8);
    impl_equivalent!([isize as i64, usize as u64] => i16);
    impl_equivalent!([isize as i64, usize as u64] => i32);
    impl_equivalent!([isize as i64, usize as u64] => i64);
    impl_equivalent!([isize as i64, usize as u64] => i128);

    impl_equivalent!([isize as i64, usize as u64] => u8);
    impl_equivalent!([isize as i64, usize as u64] => u16);
    impl_equivalent!([isize as i64, usize as u64] => u32);
    impl_equivalent!([isize as i64, usize as u64] => u64);
    impl_equivalent!([isize as i64, usize as u64] => u128);

    impl_equivalent!([isize as i64, usize as u64] => f32);
    impl_equivalent!([isize as i64, usize as u64] => f64);

    impl_equivalent!([bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64] => usize as u64);
    impl_equivalent!([bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, f32, f64] => isize as i64);
}

/// Trait to perform a saturating conversion between two numeric types. It is
/// the opposite of [`SaturatingFrom`].
///
/// [`SaturatingFrom`] should always be implemented directly; this will also
/// automatically provide an implementation of [`SaturatingInto`] thanks to its
/// blanket implementation.
pub trait SaturatingInto<T> {
    /// Converts `self` to the (usually inferred) type `T`
    fn saturating_into(self) -> T;
}

impl<T, U> SaturatingInto<T> for U
where
    T: SaturatingFrom<U>,
{
    #[inline]
    fn saturating_into(self) -> T {
        T::saturating_from(self)
    }
}

#[allow(clippy::bool_assert_comparison)]
#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_is_close {
        ($lhs:expr, $rhs:expr $(,)?) => {
            assert!(($lhs - $rhs).abs() < 1e-6)
        };
    }

    #[test]
    fn has_impl() {
        fn has_impl_inner<T: SaturatingFrom<U>, U: SaturatingInto<T>>() {}

        macro_rules! check_impls {
            (@inner [$src:ty], [$($dst:ty),+]) => {$(
                has_impl_inner::<$dst, $src>();
            )*};
            (@inner [$($src:ty),+], $dst:tt) => {$(
                check_impls!(@inner [$src], $dst);
            )*};
            ($($typ:ty),+) => {
                check_impls!(@inner [$($typ),+], [$($typ),+]);
            };
        }

        // Will fail to compile if any permutation is not implemented
        check_impls!(
            f64, f32, usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool
        );
    }

    #[test]
    fn impl_self() {
        assert_eq!(true, bool::saturating_from(true));
        assert_eq!(19829u32, u32::saturating_from(19829u32));
        assert_eq!(-67516716i64, i64::saturating_from(-67516716i64));
        assert_eq!(6791usize, usize::saturating_from(6791usize));
    }

    #[test]
    fn impl_from() {
        assert_eq!(0u8, u8::saturating_from(false));
        assert_eq!(1u64, u64::saturating_from(true));
        assert_eq!(24635u32, u32::saturating_from(24635u16));
        assert_eq!(204835u128, u128::saturating_from(204835u32));
        assert_eq!(7435637u64, u64::saturating_from(7435637u32));
        assert_eq!(-1617i32, i32::saturating_from(-1617i16));
        assert_eq!(1i128, i128::saturating_from(true));
        assert_eq!(15678i32, i32::saturating_from(15678u16));

        assert_is_close!(16980.0f32, f32::saturating_from(16980u16));
        assert_is_close!(2679.0f64, f64::saturating_from(2679i32));
        assert_is_close!(27696792.0f64, f64::saturating_from(27696792u32));
        assert_is_close!(0.5f64, f64::saturating_from(0.5f32));
        assert!(f64::saturating_from(f32::NAN).is_nan());
        assert!(f64::saturating_from(f32::INFINITY).is_infinite());
    }

    #[test]
    fn impl_clamp() {
        assert_eq!(0u8, u8::saturating_from(-26i16));
        assert_eq!(0xffffu16, u16::saturating_from(1265431463u32));
        assert_eq!(76u8, u8::saturating_from(76i128));
        assert_eq!(-0x80i8, i8::saturating_from(-296078i32));
        assert_eq!(-0x80000000i32, i32::saturating_from(-125431462564574573i64));
        assert_eq!(-12i8, i8::saturating_from(-12i64));
    }

    #[test]
    fn impl_clamp_unsigned() {
        assert_eq!(0x7fi8, i8::saturating_from(60954u16));
        assert_eq!(0x7fffi16, i16::saturating_from(61025u16));
        assert_eq!(62879i32, i32::saturating_from(62879u128));
    }

    #[test]
    fn impl_clamp_signed() {
        assert_eq!(0u8, u8::saturating_from(-12i8));
        assert_eq!(0u16, u16::saturating_from(-294865i32));
        assert_eq!(62879u32, u32::saturating_from(62879i128));
    }

    #[test]
    fn impl_gt_zero() {
        assert_eq!(false, bool::saturating_from(-12i8));
        assert_eq!(false, bool::saturating_from(-294865i32));
        assert_eq!(true, bool::saturating_from(62879i128));

        assert_eq!(false, bool::saturating_from(-12.0f32));
        assert_eq!(true, bool::saturating_from(2.0f64));
        assert_eq!(false, bool::saturating_from(f64::NEG_INFINITY));
        assert_eq!(true, bool::saturating_from(f32::INFINITY));

        // NAN always converts to false. Consistent with integers where NAN becomes 0
        assert_eq!(false, bool::saturating_from(f32::NAN));
        assert_eq!(false, bool::saturating_from(-f64::NAN));
    }

    #[test]
    fn impl_as() {
        assert_is_close!(3.0f32, f32::saturating_from(3i64));
        assert_is_close!(461573.0f64, f64::saturating_from(461573i32));
        assert_eq!(4294967300.0f32, f32::saturating_from(4294967295u32)); // nearest
        assert!(f32::saturating_from(u128::MAX).is_infinite()); // out of range => infinity

        assert_is_close!(15.6f32, f32::saturating_from(15.6f64));
        assert_eq!(0.0f32, f32::saturating_from(1e-60)); // nearest
        assert!(f32::saturating_from(1e40f64).is_infinite()); // out of range => infinity
        assert!(f32::saturating_from(f64::NEG_INFINITY).is_infinite());
        assert!(f32::saturating_from(f64::NAN).is_nan());

        assert_eq!(0xffu8, u8::saturating_from(935.0f32));
        assert_eq!(0u16, u16::saturating_from(-2.0f64));
        assert_eq!(-0x8000i16, i16::saturating_from(-1e20));
        assert_eq!(0i16, i16::saturating_from(-1e-20));
        assert_eq!(u128::MAX, u128::saturating_from(f32::INFINITY));
        assert_eq!(0i32, i32::saturating_from(f64::NAN));
    }

    #[test]
    fn impl_bool_float() {
        assert_eq!(1.0f32, f32::saturating_from(true));
        assert_eq!(0.0f64, f64::saturating_from(false));
    }
}
