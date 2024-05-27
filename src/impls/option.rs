//! A region that stores options.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Containerized, Push, Region, ReserveItems};

impl<T: Containerized> Containerized for Option<T> {
    type Region = OptionRegion<T::Region>;
}

/// A region to hold [`Option`]s.
///
/// # Examples
///
/// The region can hold options:
/// ```
/// # use flatcontainer::{Containerized, Push, OptionRegion, Region};
/// let mut r = <OptionRegion<<u8 as Containerized>::Region>>::default();
///
/// let some_index = r.push(Some(123));
/// // Type annotations required for `None`:
/// let none_index = r.push(Option::<u8>::None);
///
/// assert_eq!(Some(123), r.index(some_index));
/// assert_eq!(None, r.index(none_index));
/// ```
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OptionRegion<R> {
    inner: R,
}

impl<R: Region> Region for OptionRegion<R> {
    type ReadItem<'a> = Option<R::ReadItem<'a>> where Self: 'a;
    type Index = Option<R::Index>;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            inner: R::merge_regions(regions.map(|r| &r.inner)),
        }
    }

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        index.map(|t| self.inner.index(t))
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    #[inline]
    fn clear(&mut self) {
        self.inner.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.inner.heap_size(callback);
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item.map(R::reborrow)
    }
}

impl<T, TR> Push<Option<T>> for OptionRegion<TR>
where
    TR: Region + Push<T>,
{
    #[inline]
    fn push(&mut self, item: Option<T>) -> <OptionRegion<TR> as Region>::Index {
        item.map(|t| self.inner.push(t))
    }
}

impl<'a, T: 'a, TR> Push<&'a Option<T>> for OptionRegion<TR>
where
    TR: Region + Push<&'a T>,
{
    #[inline]
    fn push(&mut self, item: &'a Option<T>) -> <OptionRegion<TR> as Region>::Index {
        item.as_ref().map(|t| self.inner.push(t))
    }
}

impl<'a, T: 'a, TR> ReserveItems<&'a Option<T>> for OptionRegion<TR>
where
    TR: Region + ReserveItems<&'a T>,
{
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Option<T>> + Clone,
    {
        self.inner.reserve_items(items.filter_map(|r| r.as_ref()));
    }
}

#[cfg(test)]
mod tests {
    use crate::{MirrorRegion, OwnedRegion, Region, ReserveItems};

    use super::*;

    #[test]
    fn test_reserve() {
        let mut r = <OptionRegion<MirrorRegion<u8>>>::default();
        ReserveItems::reserve_items(&mut r, [Some(0), None].iter());
    }

    #[test]
    fn test_heap_size() {
        let mut r = <OptionRegion<OwnedRegion<u8>>>::default();
        ReserveItems::reserve_items(&mut r, [Some([1; 1]), None].iter());
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }
}
