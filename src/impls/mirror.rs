//! A region that copies its inputs.

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Containerized, CopyOnto, Index, Region, ReserveItems};

/// A region for types where the read item type is equal to the index type.
///
/// This region is useful where the type is not larger than roughly two `usize`s (or 1.5x with
/// some optimizations), or looking up the value is too costly. For larger copy types, the memory
/// required to store the copy type and an index is only marginally bigger, with the benefit
/// that the index remains compact.
///
/// # Examples
///
/// For [`MirrorRegion`]s, we can index with a copy type:
/// ```
/// # use flatcontainer::{MirrorRegion, Region};
/// let r = MirrorRegion::<u8>::default();
/// let output: u8 = r.index(42);
/// assert_eq!(output, 42);
/// ```
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MirrorRegion<T>(PhantomData<T>);

impl<T> Default for MirrorRegion<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Debug for MirrorRegion<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MirrorRegion<{}>", std::any::type_name::<T>())
    }
}

impl<T: Index + CopyOnto<Self>> Region for MirrorRegion<T> {
    type ReadItem<'a> = T where T: 'a;
    type Index = T;

    fn merge_regions<'a>(_regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self::default()
    }

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

impl<T: Index> CopyOnto<MirrorRegion<Self>> for T {
    #[inline(always)]
    fn copy_onto(self, _target: &mut MirrorRegion<Self>) -> T {
        self
    }
}

impl<'a, T: Index> CopyOnto<MirrorRegion<T>> for &'a T {
    #[inline(always)]
    fn copy_onto(self, _target: &mut MirrorRegion<T>) -> T {
        *self
    }
}

impl<T: Index> ReserveItems<MirrorRegion<T>> for T {
    #[inline(always)]
    fn reserve_items<I>(_target: &mut MirrorRegion<T>, _items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        // No storage
    }
}

impl<'a, T: Index> ReserveItems<MirrorRegion<T>> for &'a T {
    #[inline(always)]
    fn reserve_items<I>(_target: &mut MirrorRegion<T>, _items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        // No storage
    }
}

macro_rules! implement_for {
    ($index_type:ty) => {
        impl Containerized for $index_type {
            type Region = MirrorRegion<Self>;
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
