use std::marker::PhantomData;

use crate::{Containerized, CopyOnto, Region, ReserveItems};

/// A region for types where the read item type is equal to the index type.
#[derive(Debug)]
pub struct MirrorRegion<T>(PhantomData<*const T>);

impl<T> Default for MirrorRegion<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Copy> Region for MirrorRegion<T> {
    type ReadItem<'a> = T where T: 'a;
    type Index = T;

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        index
    }

    #[inline(always)]
    fn reserve_regions<'a, I>(&mut self, _regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        // No storage
    }

    #[inline(always)]
    fn clear(&mut self) {
        // No storage
    }
}

macro_rules! implement_for {
    ($index_type:ty) => {
        impl Containerized for $index_type {
            type Region = MirrorRegion<Self>;
        }

        impl CopyOnto<MirrorRegion<Self>> for $index_type {
            #[inline(always)]
            fn copy_onto(self, _target: &mut MirrorRegion<Self>) -> $index_type {
                self
            }

            #[inline(always)]
            fn reserve_items<I>(_target: &mut MirrorRegion<Self>, _items: I)
            where
                I: Iterator<Item = Self> + Clone,
            {
            }
        }

        impl<'a> CopyOnto<MirrorRegion<$index_type>> for &'a $index_type {
            #[inline(always)]
            fn copy_onto(self, _target: &mut MirrorRegion<$index_type>) -> $index_type {
                *self
            }

            #[inline(always)]
            fn reserve_items<I>(_target: &mut MirrorRegion<$index_type>, _items: I)
            where
                I: Iterator<Item = Self> + Clone,
            {
            }
        }

        impl<'a> ReserveItems<MirrorRegion<$index_type>> for $index_type {
            #[inline(always)]
            fn reserve_items<I>(_target: &mut MirrorRegion<$index_type>, _items: I)
            where
                I: Iterator<Item = Self> + Clone,
            {
                // No storage
            }
        }

        impl<'a> ReserveItems<MirrorRegion<$index_type>> for &'a $index_type {
            #[inline(always)]
            fn reserve_items<I>(_target: &mut MirrorRegion<$index_type>, _items: I)
            where
                I: Iterator<Item = Self> + Clone,
            {
                // No storage
            }
        }
    };
}

implement_for!(());
implement_for!(bool);
implement_for!(char);

implement_for!(u8);
implement_for!(u16);
implement_for!(u32);
implement_for!(u64);
implement_for!(u128);
implement_for!(usize);

implement_for!(i8);
implement_for!(i16);
implement_for!(i32);
implement_for!(i64);
implement_for!(i128);
implement_for!(isize);

implement_for!(f32);
implement_for!(f64);

implement_for!(std::num::Wrapping<i8>);
implement_for!(std::num::Wrapping<i16>);
implement_for!(std::num::Wrapping<i32>);
implement_for!(std::num::Wrapping<i64>);
implement_for!(std::num::Wrapping<i128>);
implement_for!(std::num::Wrapping<isize>);

implement_for!(std::time::Duration);
