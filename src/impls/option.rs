//! A region that stores options.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::rank_select::RankSelect;
use crate::{
    Clear, HeapSize, Index, IndexAs, IntoOwned, Len, Push, Region, RegionPreference, ReserveItems,
};

impl<T: RegionPreference> RegionPreference for Option<T> {
    type Owned = Option<T::Owned>;
    type Region = OptionRegion<T::Region, Vec<u64>, Vec<u64>>;
}

/// A region to hold [`Option`]s.
///
/// # Examples
///
/// The region can hold options:
/// ```
/// # use flatcontainer::{RegionPreference, Push, OptionRegion, Index};
/// let mut r = <OptionRegion<<u8 as RegionPreference>::Region>>::default();
///
/// r.push(Some(123));
/// // Type annotations required for `None`:
/// r.push(Option::<u8>::None);
///
/// assert_eq!(Some(&123), r.index(0));
/// assert_eq!(None, r.index(1));
/// ```
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OptionRegion<R, RC = Vec<u64>, RV = Vec<u64>> {
    ranks: RankSelect<RC, RV>,
    inner: R,
}

impl<R: Clone, RC: Clone, RV: Clone> Clone for OptionRegion<R, RC, RV> {
    fn clone(&self) -> Self {
        Self {
            ranks: self.ranks.clone(),
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
    }
}

impl<R: Region, RC: Region, RV: Region> Region for OptionRegion<R, RC, RV> {
    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            ranks: RankSelect::merge_regions(regions.clone().map(|r| &r.ranks)),
            inner: R::merge_regions(regions.map(|r| &r.inner)),
        }
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.ranks
            .reserve_regions(regions.clone().map(|r| &r.ranks));
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }
}

impl<R, RC, RV> HeapSize for OptionRegion<R, RC, RV>
where
    R: HeapSize,
    RC: HeapSize,
    RV: HeapSize,
{
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.ranks.heap_size(&mut callback);
        self.inner.heap_size(callback);
    }
}

impl<R, RC, RV> Clear for OptionRegion<R, RC, RV>
where
    R: Clear,
    RC: Clear,
    RV: Clear,
{
    fn clear(&mut self) {
        self.ranks.clear();
        self.inner.clear();
    }
}

impl<R, RC, RV: Len> Len for OptionRegion<R, RC, RV> {
    fn len(&self) -> usize {
        self.ranks.len()
    }
}

impl<R, RC, RV> Index for OptionRegion<R, RC, RV>
where
    R: Index,
    RC: IndexAs<u64> + Len,
    RV: IndexAs<u64> + Len,
{
    type Owned = Option<R::Owned>;
    type ReadItem<'a> = Option<<R as Index>::ReadItem<'a>> where Self: 'a;

    #[inline]
    fn index(&self, index: usize) -> Self::ReadItem<'_> {
        if self.ranks.index_as(index) {
            Some(self.inner.index(self.ranks.rank(index)))
        } else {
            None
        }
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item.map(R::reborrow)
    }
}

impl<'a, T> IntoOwned<'a> for Option<T>
where
    T: IntoOwned<'a>,
{
    type Owned = Option<T::Owned>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.map(IntoOwned::into_owned)
    }

    #[inline]
    fn clone_onto(self, other: &mut Self::Owned) {
        match (self, other) {
            (Some(item), Some(target)) => T::clone_onto(item, target),
            (Some(item), target) => *target = Some(T::into_owned(item)),
            (None, target) => *target = None,
        }
    }

    #[inline]
    fn borrow_as(owned: &'a Self::Owned) -> Self {
        owned.as_ref().map(T::borrow_as)
    }
}

impl<T, TR, RC, RV> Push<Option<T>> for OptionRegion<TR, RC, RV>
where
    TR: Push<T>,
    RC: IndexAs<u64> + Len + Push<u64>,
    RV: IndexAs<u64> + Len + Push<u64>,
{
    #[inline]
    fn push(&mut self, item: Option<T>) {
        match item {
            Some(t) => {
                self.ranks.push(true);
                self.inner.push(t);
            }
            None => {
                self.ranks.push(false);
            }
        }
    }
}

impl<'a, T: 'a, TR, RC, RV> Push<&'a Option<T>> for OptionRegion<TR, RC, RV>
where
    TR: Push<&'a T>,
    RC: IndexAs<u64> + Len + Push<u64>,
    RV: IndexAs<u64> + Len + Push<u64>,
{
    #[inline]
    fn push(&mut self, item: &'a Option<T>) {
        match item {
            Some(t) => {
                self.ranks.push(true);
                self.inner.push(t);
            }
            None => {
                self.ranks.push(false);
            }
        }
    }
}

impl<T, TR, RC, RV> ReserveItems<Option<T>> for OptionRegion<TR, RC, RV>
where
    TR: Region + ReserveItems<T>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = Option<T>> + Clone,
    {
        // Clippy is confused about using `flatten` here, which we cannot use because
        // the iterator isn't `Clone`.
        #[allow(clippy::filter_map_identity)]
        self.inner.reserve_items(items.filter_map(|r| r));
    }
}

impl<'a, T: 'a, TR, RC, RV> ReserveItems<&'a Option<T>> for OptionRegion<TR, RC, RV>
where
    TR: Region + ReserveItems<&'a T>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Option<T>> + Clone,
    {
        self.inner.reserve_items(items.filter_map(|r| r.as_ref()));
    }
}

#[cfg(test)]
mod tests {
    use crate::{OwnedRegion, ReserveItems};

    use super::*;

    #[test]
    fn test_reserve() {
        let mut r = <OptionRegion<Vec<u8>>>::default();
        ReserveItems::reserve_items(&mut r, [Some(0), None].iter());

        ReserveItems::reserve_items(&mut r, [Some(0), None].into_iter());
    }

    #[test]
    fn test_heap_size() {
        let mut r = <OptionRegion<OwnedRegion<u8>>>::default();
        ReserveItems::reserve_items(
            &mut r,
            std::iter::once(Some(&[1; 1])).chain(std::iter::repeat_n(None, 1000)),
        );
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
        for item in std::iter::once(Some(&[1; 1])).chain(std::iter::repeat_n(None, 10000)) {
            r.push(item);
        }
        let mut siz = 0;
        r.heap_size(|s, _| {
            siz += s;
        });
        assert!(siz > 0);
        println!("{siz}");
        println!("{r:?}")
    }
}
