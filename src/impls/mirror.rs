//! A region that copies its inputs.

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    CanPush, Index, IntoOwned, Push, Region, RegionPreference, Reserve, ReserveItems, TryPush,
};

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
/// let r = <MirrorRegion<u8>>::default();
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

impl<T> Region for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    type Owned = T;
    type ReadItem<'a> = T where T: 'a;
    type Index = T;

    #[inline]
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

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, _callback: F) {
        // No storage
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<T> Push<T> for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    #[inline(always)]
    fn push(&mut self, item: T) -> T {
        item
    }
}

impl<T> TryPush<T> for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    #[inline(always)]
    fn try_push(&mut self, item: T) -> Result<Self::Index, T> {
        Ok(item)
    }
}

impl<T> CanPush<T> for MirrorRegion<T> {
    fn can_push<I>(&self, _: I) -> bool
    where
        I: Iterator<Item = T> + Clone,
    {
        true
    }
}

impl<T> Push<&T> for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    #[inline(always)]
    fn push(&mut self, item: &T) -> T {
        *item
    }
}

impl<'a, T> CanPush<&'a T> for MirrorRegion<T> {
    fn can_push<I>(&self, _: I) -> bool
    where
        I: Iterator<Item = &'a T> + Clone,
    {
        true
    }
}

impl<'b, T> TryPush<&'b T> for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    #[inline(always)]
    fn try_push(&mut self, item: &'b T) -> Result<Self::Index, &'b T> {
        Ok(*item)
    }
}

impl<T> Push<&&T> for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    #[inline(always)]
    fn push(&mut self, item: &&T) -> T {
        **item
    }
}

impl<'b, 'c, T> TryPush<&'b &'c T> for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    #[inline(always)]
    fn try_push(&mut self, item: &'b &'c T) -> Result<Self::Index, &'b &'c T> {
        Ok(**item)
    }
}

impl<T> ReserveItems<T> for MirrorRegion<T>
where
    for<'a> T: Index + IntoOwned<'a, Owned = T>,
{
    #[inline(always)]
    fn reserve_items<I>(&mut self, _items: I)
    where
        I: Iterator<Item = T> + Clone,
    {
        // No storage
    }
}

impl<'a, T> ReserveItems<&'a T> for MirrorRegion<T>
where
    for<'b> T: Index + IntoOwned<'b, Owned = T>,
{
    #[inline(always)]
    fn reserve_items<I>(&mut self, _items: I)
    where
        I: Iterator<Item = &'a T> + Clone,
    {
        // No storage
    }
}

impl<T> Reserve for MirrorRegion<T> {
    type Reserve = ();

    fn reserve(&mut self, (): &Self::Reserve) {
        // No storage
    }
}

macro_rules! implement_for {
    ($index_type:ty) => {
        impl RegionPreference for $index_type {
            type Owned = Self;
            type Region = MirrorRegion<Self>;
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

#[cfg(test)]
mod tests {
    use crate::ReserveItems;

    use super::*;

    #[test]
    fn test_reserve_regions() {
        let mut r = MirrorRegion::<u8>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(0));
    }
}
