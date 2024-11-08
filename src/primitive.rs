//! Support for primitive types.

use crate::{IntoOwned, RegionPreference};

macro_rules! impl_region_preference {
    ($($index_type:ty),*) => {
        $(
            impl RegionPreference for $index_type {
                type Owned = Self;
                type Region = Vec<Self>;
            }

            impl<'a> IntoOwned<'a> for $index_type {
                type Owned = $index_type;

                #[inline]
                fn into_owned(self) -> Self::Owned {
                    self
                }

                #[inline]
                fn clone_onto(self, other: &mut Self::Owned) {
                    *other = self;
                }

                #[inline]
                fn borrow_as(owned: &'a Self::Owned) -> Self {
                    *owned
                }
            }
        )*
    };
}

impl_region_preference!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    char,
    bool,
    (),
    std::num::Wrapping<i8>,
    std::num::Wrapping<i16>,
    std::num::Wrapping<i32>,
    std::num::Wrapping<i64>,
    std::num::Wrapping<i128>,
    std::num::Wrapping<isize>,
    std::time::Duration
);
