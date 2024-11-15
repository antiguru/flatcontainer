//! A region that stores results.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::rank_select::RankSelect;
use crate::{
    Clear, HeapSize, Index, IndexAs, IntoOwned, Len, Push, Region, RegionPreference, ReserveItems,
};

impl<T: RegionPreference, E: RegionPreference> RegionPreference for Result<T, E> {
    type Owned = Result<T::Owned, E::Owned>;
    type Region = ResultRegion<T::Region, E::Region>;
}

/// A region to hold [`Result`]s.
///
/// # Examples
///
/// Add results to a result region:
/// ```
/// use flatcontainer::{RegionPreference, Push, Region, ResultRegion, Index};
/// let mut r = <Result<(), String> as RegionPreference>::Region::default();
///
/// r.push(Result::<(), String>::Ok(()));
/// r.push(Result::<(), String>::Err("Error".to_string()));
///
/// assert_eq!(Ok(&()), r.index(0));
/// assert_eq!(Err("Error"), r.index(1));
/// ```
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResultRegion<T, E, RC = Vec<u64>, RV = Vec<u64>> {
    ranks: RankSelect<RC, RV>,
    oks: T,
    errs: E,
}

impl<T: Clone, E: Clone, RC: Clone, RV: Clone> Clone for ResultRegion<T, E, RC, RV> {
    fn clone(&self) -> Self {
        Self {
            ranks: self.ranks.clone(),
            oks: self.oks.clone(),
            errs: self.errs.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.ranks.clone_from(&source.ranks);
        self.oks.clone_from(&source.oks);
        self.errs.clone_from(&source.errs);
    }
}

impl<T, E, RC, RV> Region for ResultRegion<T, E, RC, RV>
where
    T: Region,
    E: Region,
    RC: Region,
    RV: Region,
{
    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            ranks: RankSelect::merge_regions(regions.clone().map(|r| &r.ranks)),
            oks: T::merge_regions(regions.clone().map(|r| &r.oks)),
            errs: E::merge_regions(regions.map(|r| &r.errs)),
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
        self.oks.reserve_regions(regions.clone().map(|r| &r.oks));
        self.errs.reserve_regions(regions.map(|r| &r.errs));
    }
}

impl<T, E, RC, RV> Index for ResultRegion<T, E, RC, RV>
where
    T: Index,
    E: Index,
    RC: IndexAs<u64> + Len,
    RV: IndexAs<u64> + Len,
{
    type Owned = Result<T::Owned, E::Owned>;
    type ReadItem<'a> = Result<T::ReadItem<'a>, E::ReadItem<'a>> where Self: 'a;

    #[inline]
    fn index(&self, index: usize) -> Self::ReadItem<'_> {
        if self.ranks.index_as(index) {
            Ok(self.oks.index(self.ranks.rank(index)))
        } else {
            Err(self.errs.index(index - self.ranks.rank(index)))
        }
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item.map(T::reborrow).map_err(E::reborrow)
    }
}

impl<T, E, RC, RV> Clear for ResultRegion<T, E, RC, RV>
where
    T: Clear,
    E: Clear,
    RC: Clear,
    RV: Clear,
{
    #[inline]
    fn clear(&mut self) {
        self.ranks.clear();
        self.oks.clear();
        self.errs.clear();
    }
}

impl<T, E, RC, RV> HeapSize for ResultRegion<T, E, RC, RV>
where
    T: HeapSize,
    E: HeapSize,
{
    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.oks.heap_size(&mut callback);
        self.errs.heap_size(callback);
    }
}

impl<T, E, RC, RV> Len for ResultRegion<T, E, RC, RV>
where
    RV: Len,
{
    #[inline]
    fn len(&self) -> usize {
        self.ranks.len()
    }
}

impl<'a, T, E> IntoOwned<'a> for Result<T, E>
where
    T: IntoOwned<'a>,
    E: IntoOwned<'a>,
{
    type Owned = Result<T::Owned, E::Owned>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.map(T::into_owned).map_err(E::into_owned)
    }

    #[inline]
    fn clone_onto(self, other: &mut Self::Owned) {
        match (self, other) {
            (Ok(item), Ok(target)) => T::clone_onto(item, target),
            (Err(item), Err(target)) => E::clone_onto(item, target),
            (Ok(item), target) => *target = Ok(T::into_owned(item)),
            (Err(item), target) => *target = Err(E::into_owned(item)),
        }
    }

    #[inline]
    fn borrow_as(owned: &'a Self::Owned) -> Self {
        owned.as_ref().map(T::borrow_as).map_err(E::borrow_as)
    }
}

impl<T, TC, E, EC, RC, RV> Push<Result<T, E>> for ResultRegion<TC, EC, RC, RV>
where
    TC: Push<T>,
    EC: Push<E>,
    RC: IndexAs<u64> + Len + Push<u64>,
    RV: IndexAs<u64> + Len + Push<u64>,
{
    #[inline]
    fn push(&mut self, item: Result<T, E>) {
        match item {
            Ok(t) => {
                self.ranks.push(true);
                self.oks.push(t);
            }
            Err(r) => {
                self.ranks.push(false);
                self.errs.push(r);
            }
        }
    }
}

impl<'a, T: 'a, TC, E: 'a, EC, RC, RV> Push<&'a Result<T, E>> for ResultRegion<TC, EC, RC, RV>
where
    TC: Push<&'a T>,
    EC: Push<&'a E>,
    RC: IndexAs<u64> + Len + Push<u64>,
    RV: IndexAs<u64> + Len + Push<u64>,
{
    #[inline]
    fn push(&mut self, item: &'a Result<T, E>) {
        match item {
            Ok(t) => {
                self.ranks.push(true);
                self.oks.push(t);
            }
            Err(r) => {
                self.ranks.push(false);
                self.errs.push(r);
            }
        }
    }
}

impl<T, TC, E, EC, RC, RV> ReserveItems<Result<T, E>> for ResultRegion<TC, EC, RC, RV>
where
    TC: Region + ReserveItems<T>,
    EC: Region + ReserveItems<E>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = Result<T, E>> + Clone,
    {
        self.oks.reserve_items(items.clone().filter_map(|r| r.ok()));
        self.errs.reserve_items(items.filter_map(|r| r.err()));
    }
}

impl<'a, T: 'a, TC, E: 'a, EC, RC, RV> ReserveItems<&'a Result<T, E>>
    for ResultRegion<TC, EC, RC, RV>
where
    TC: Region + ReserveItems<&'a T>,
    EC: Region + ReserveItems<&'a E>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Result<T, E>> + Clone,
    {
        self.oks
            .reserve_items(items.clone().filter_map(|r| r.as_ref().ok()));
        self.errs
            .reserve_items(items.filter_map(|r| r.as_ref().err()));
    }
}

#[cfg(test)]
mod tests {
    use crate::{OwnedRegion, ReserveItems};

    use super::*;

    #[test]
    fn test_reserve() {
        let mut r = <ResultRegion<Vec<u8>, Vec<u8>>>::default();
        ReserveItems::reserve_items(&mut r, [Ok(0), Err(1)].iter());

        ReserveItems::reserve_items(&mut r, [Ok(0), Err(1)].into_iter());
    }

    #[test]
    fn test_heap_size() {
        let mut r = <ResultRegion<OwnedRegion<u8>, OwnedRegion<u8>>>::default();
        ReserveItems::reserve_items(&mut r, [Ok([1; 0]), Err([1; 1])].iter());
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }
}
