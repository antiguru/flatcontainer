//! A region that stores options.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Containerized, CopyOnto, ReadRegion, Region, ReserveItems};

impl<T: Containerized> Containerized for Option<T> {
    type Region = OptionRegion<T::Region>;
}

/// A region to hold [`Option`]s.
///
/// # Examples
///
/// The region can hold options:
/// ```
/// # use flatcontainer::{Containerized, CopyOnto, OptionRegion, Region};
/// let mut r = <OptionRegion<<u8 as Containerized>::Region>>::default();
///
/// let some_index = Some(123).copy_onto(&mut r);
/// // Type annotations required for `None`:
/// let none_index = Option::<u8>::None.copy_onto(&mut r);
///
/// assert_eq!(Some(123), r.index(some_index));
/// assert_eq!(None, r.index(none_index));
/// ```
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OptionRegion<R> {
    inner: R,
}
impl<R: ReadRegion> ReadRegion for OptionRegion<R> {
    type ReadItem<'a> = Option<R::ReadItem<'a>> where Self: 'a;
    type Index = Option<R::Index>;

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        index.map(|t| self.inner.index(t))
    }
}

impl<R: Region> Region for OptionRegion<R> {
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            inner: R::merge_regions(regions.map(|r| &r.inner)),
        }
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
}

impl<T, TR> CopyOnto<OptionRegion<TR>> for Option<T>
where
    TR: Region,
    T: CopyOnto<TR>,
{
    #[inline]
    fn copy_onto(self, target: &mut OptionRegion<TR>) -> <OptionRegion<TR> as ReadRegion>::Index {
        self.map(|t| t.copy_onto(&mut target.inner))
    }
}

impl<'a, T: 'a, TR> CopyOnto<OptionRegion<TR>> for &'a Option<T>
where
    TR: Region,
    &'a T: CopyOnto<TR>,
{
    #[inline]
    fn copy_onto(self, target: &mut OptionRegion<TR>) -> <OptionRegion<TR> as ReadRegion>::Index {
        self.as_ref().map(|t| t.copy_onto(&mut target.inner))
    }
}

impl<'a, T: 'a, TR> ReserveItems<OptionRegion<TR>> for &'a Option<T>
where
    TR: Region,
    &'a T: ReserveItems<TR>,
{
    fn reserve_items<I>(target: &mut OptionRegion<TR>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(&mut target.inner, items.filter_map(|r| r.as_ref()));
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
