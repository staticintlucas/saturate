pub trait SaturatingFrom<T> {
    fn saturating_from(src: T) -> Self;
}

macro_rules! impl_self {
    ($($typ:ty),+) => {
        $(
            impl SaturatingFrom<$typ> for $typ {
                #[inline]
                fn saturating_from(src: $typ) -> $typ {
                    src
                }
            }
        )+
    };
}

impl_self!(f64, f32, usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool);

macro_rules! impl_from {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(src: $src) -> $dst {
                    <$dst>::from(src)
                }
            }
        )+
    };
}

impl_from!([bool] => u8);
impl_from!([u8, bool] => u16);
impl_from!([u16, u8, bool] => u32);
impl_from!([u32, u16, u8, bool] => u64);
impl_from!([u64, u32, u16, u8, bool] => u128);

impl_from!([bool] => i8);
impl_from!([u8, i8, bool] => i16);
impl_from!([u16, i16, u8, i8, bool] => i32);
impl_from!([u32, i32, u16, i16, u8, i8, bool] => i64);
impl_from!([u64, i64, u32, i32, u16, i16, u8, i8, bool] => i128);

impl_from!([u16, i16, u8, i8] => f32);
impl_from!([f32, u32, i32, u16, i16, u8, i8] => f64);

macro_rules! impl_clamp {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(src: $src) -> $dst {
                    use core::convert::TryFrom;
                    <$dst>::try_from(src.min(<$src>::from(<$dst>::MAX)).max(<$src>::from(<$dst>::MIN))).unwrap()
                }
            }
        )+
    };
}

impl_clamp!([u128, i128, u64, i64, u32, i32, u16, i16] => u8);
impl_clamp!([u128, i128, u64, i64, u32, i32] => u16);
impl_clamp!([u128, i128, u64, i64] => u32);
impl_clamp!([u128, i128] => u64);

impl_clamp!([i128, i64, i32, i16] => i8);
impl_clamp!([i128, i64, i32] => i16);
impl_clamp!([i128, i64] => i32);
impl_clamp!([i128] => i64);

macro_rules! impl_clamp_unsigned {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(src: $src) -> $dst {
                    use core::convert::TryFrom;
                    <$dst>::try_from(src.min(<$dst>::MAX as $src)).unwrap()
                }
            }
        )+
    };
}

impl_clamp_unsigned!([u128, u64, u32, u16, u8] => i8);
impl_clamp_unsigned!([u128, u64, u32, u16] => i16);
impl_clamp_unsigned!([u128, u64, u32] => i32);
impl_clamp_unsigned!([u128, u64] => i64);
impl_clamp_unsigned!([u128] => i128);
impl_clamp_unsigned!([usize] => isize);

macro_rules! impl_clamp_signed {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(src: $src) -> $dst {
                    use core::convert::TryFrom;
                    <$dst>::try_from(src.max(0)).unwrap()
                }
            }
        )+
    };
}

impl_clamp_signed!([i8] => u8);
impl_clamp_signed!([i16, i8] => u16);
impl_clamp_signed!([i32, i16, i8] => u32);
impl_clamp_signed!([i64, i32, i16, i8] => u64);
impl_clamp_signed!([i128, i64, i32, i16, i8] => u128);
impl_clamp_signed!([isize] => usize);

macro_rules! impl_gt_zero {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(src: $src) -> $dst {
                    src > 0
                }
            }
        )+
    };
}

impl_gt_zero!([usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8] => bool);

macro_rules! impl_gt_zero_float {
    ([$($src:ty),+] => $dst:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(src: $src) -> $dst {
                    src > 0.0
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
                fn saturating_from(src: $src) -> $dst {
                    src as $dst
                }
            }
        )+
    };
}

// `as` will round to nearest (and saturate at f32::INFINITY for `u128` => f32)
impl_as!([u128, i128, u64, i64, u32, i32] => f32);
impl_as!([u128, i128, u64, i64] => f64);
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
                fn saturating_from(src: bool) -> $dst {
                    <$dst>::from(u8::from(src))
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
                fn saturating_from(src: $src) -> $dst {
                    <$dst>::saturating_from(src as $equ)
                }
            }
        )+
    };
    ([$($src:ty),+] => $dst:ty as $equ:ty) => {
        $(
            impl SaturatingFrom<$src> for $dst {
                #[inline]
                fn saturating_from(src: $src) -> $dst {
                    <$equ>::saturating_from(src) as $dst
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

    impl_equivalent!([usize as u16, isize as i16] => i8);
    impl_equivalent!([usize as u16, isize as i16] => i16);
    impl_equivalent!([usize as u16, isize as i16] => i32);
    impl_equivalent!([usize as u16, isize as i16] => i64);
    impl_equivalent!([usize as u16, isize as i16] => i128);

    impl_equivalent!([usize as u16, isize as i16] => u8);
    impl_equivalent!([usize as u16, isize as i16] => u16);
    impl_equivalent!([usize as u16, isize as i16] => u32);
    impl_equivalent!([usize as u16, isize as i16] => u64);
    impl_equivalent!([usize as u16, isize as i16] => u128);

    impl_equivalent!([usize as u16, isize as i16] => f32);
    impl_equivalent!([usize as u16, isize as i16] => f64);

    impl_equivalent!([f64, f32, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool] => usize as u16);
    impl_equivalent!([f64, f32, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool] => isize as i16);
}

#[cfg(target_pointer_width = "32")]
mod size {
    use crate::SaturatingFrom;

    impl_equivalent!([usize as u32, isize as i32] => i8);
    impl_equivalent!([usize as u32, isize as i32] => i16);
    impl_equivalent!([usize as u32, isize as i32] => i32);
    impl_equivalent!([usize as u32, isize as i32] => i64);
    impl_equivalent!([usize as u32, isize as i32] => i128);

    impl_equivalent!([usize as u32, isize as i32] => u8);
    impl_equivalent!([usize as u32, isize as i32] => u16);
    impl_equivalent!([usize as u32, isize as i32] => u32);
    impl_equivalent!([usize as u32, isize as i32] => u64);
    impl_equivalent!([usize as u32, isize as i32] => u128);

    impl_equivalent!([usize as u32, isize as i32] => f32);
    impl_equivalent!([usize as u32, isize as i32] => f64);

    impl_equivalent!([f64, f32, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool] => usize as u32);
    impl_equivalent!([f64, f32, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool] => isize as i32);
}

#[cfg(target_pointer_width = "64")]
mod size {
    use crate::SaturatingFrom;

    impl_equivalent!([usize as u64, isize as i64] => i8);
    impl_equivalent!([usize as u64, isize as i64] => i16);
    impl_equivalent!([usize as u64, isize as i64] => i32);
    impl_equivalent!([usize as u64, isize as i64] => i64);
    impl_equivalent!([usize as u64, isize as i64] => i128);

    impl_equivalent!([usize as u64, isize as i64] => u8);
    impl_equivalent!([usize as u64, isize as i64] => u16);
    impl_equivalent!([usize as u64, isize as i64] => u32);
    impl_equivalent!([usize as u64, isize as i64] => u64);
    impl_equivalent!([usize as u64, isize as i64] => u128);

    impl_equivalent!([usize as u64, isize as i64] => f32);
    impl_equivalent!([usize as u64, isize as i64] => f64);

    impl_equivalent!([f64, f32, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool] => usize as u64);
    impl_equivalent!([f64, f32, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool] => isize as i64);
}

pub trait SaturatingInto<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

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

        check_impls!(
            f64, f32, usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8, bool
        );
    }

    #[test]
    fn impl_self() {
        let bool = true;
        let u8 = 5u8;
        let i8 = -12i8;
        let u16 = 65u16;
        let i16 = -700i16;
        let u32 = 19829u32;
        let i32 = 680157i32;
        let u64 = 687243576u64;
        let i64 = -67516716i64;
        let u128 = 6879266981669u128;
        let i128 = 79826986429864i128;
        let usize = 6791usize;
        let isize = 5687isize;

        assert_eq!(bool, bool::saturating_from(bool));
        assert_eq!(u8, u8::saturating_from(u8));
        assert_eq!(i8, i8::saturating_from(i8));
        assert_eq!(u16, u16::saturating_from(u16));
        assert_eq!(i16, i16::saturating_from(i16));
        assert_eq!(u32, u32::saturating_from(u32));
        assert_eq!(i32, i32::saturating_from(i32));
        assert_eq!(u64, u64::saturating_from(u64));
        assert_eq!(i64, i64::saturating_from(i64));
        assert_eq!(u128, u128::saturating_from(u128));
        assert_eq!(i128, i128::saturating_from(i128));
        assert_eq!(usize, usize::saturating_from(usize));
        assert_eq!(isize, isize::saturating_from(isize));
    }
}
