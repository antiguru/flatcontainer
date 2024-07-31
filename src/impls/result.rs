//! A region that stores results.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{IntoOwned, Push, Region, RegionPreference, Reserve, ReserveItems};

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
/// use flatcontainer::{RegionPreference, Push, Region, ResultRegion};
/// let mut r =
///     <ResultRegion<<() as RegionPreference>::Region, <String as RegionPreference>::Region>>::default();
///
/// let ok_index = r.push(Result::<(), String>::Ok(()));
/// let err_index = r.push(Result::<(), String>::Err("Error".to_string()));
///
/// assert_eq!(Ok(()), r.index(ok_index));
/// assert_eq!(Err("Error"), r.index(err_index));
/// ```
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResultRegion<T, E> {
    oks: T,
    errs: E,
}

impl<T: Clone, E: Clone> Clone for ResultRegion<T, E> {
    fn clone(&self) -> Self {
        Self {
            oks: self.oks.clone(),
            errs: self.errs.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.oks.clone_from(&source.oks);
        self.errs.clone_from(&source.errs);
    }
}

impl<T, E> Region for ResultRegion<T, E>
where
    T: Region,
    E: Region,
{
    type Owned = Result<T::Owned, E::Owned>;
    type ReadItem<'a> = Result<T::ReadItem<'a>, E::ReadItem<'a>> where Self: 'a;
    type Index = Result<T::Index, E::Index>;

    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            oks: T::merge_regions(regions.clone().map(|r| &r.oks)),
            errs: E::merge_regions(regions.map(|r| &r.errs)),
        }
    }

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        match index {
            Ok(index) => Ok(self.oks.index(index)),
            Err(index) => Err(self.errs.index(index)),
        }
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.oks.reserve_regions(regions.clone().map(|r| &r.oks));
        self.errs.reserve_regions(regions.map(|r| &r.errs));
    }

    #[inline]
    fn clear(&mut self) {
        self.oks.clear();
        self.errs.clear();
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.oks.heap_size(&mut callback);
        self.errs.heap_size(callback);
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item.map(T::reborrow).map_err(E::reborrow)
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

impl<T, TC, E, EC> Push<Result<T, E>> for ResultRegion<TC, EC>
where
    TC: Region + Push<T>,
    EC: Region + Push<E>,
{
    #[inline]
    fn push(&mut self, item: Result<T, E>) -> <ResultRegion<TC, EC> as Region>::Index {
        match item {
            Ok(t) => Ok(self.oks.push(t)),
            Err(e) => Err(self.errs.push(e)),
        }
    }
}

impl<'a, T: 'a, TC, E: 'a, EC> Push<&'a Result<T, E>> for ResultRegion<TC, EC>
where
    TC: Region + Push<&'a T>,
    EC: Region + Push<&'a E>,
{
    #[inline]
    fn push(&mut self, item: &'a Result<T, E>) -> <ResultRegion<TC, EC> as Region>::Index {
        match item {
            Ok(t) => Ok(self.oks.push(t)),
            Err(e) => Err(self.errs.push(e)),
        }
    }
}

impl<T, TC, E, EC> ReserveItems<Result<T, E>> for ResultRegion<TC, EC>
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

impl<'a, T: 'a, TC, E: 'a, EC> ReserveItems<&'a Result<T, E>> for ResultRegion<TC, EC>
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

impl<T, E> Reserve for ResultRegion<T, E>
where
    T: Reserve,
    E: Reserve,
{
    type Reserve = (T::Reserve, E::Reserve);

    fn reserve(&mut self, (t, e): &Self::Reserve) {
        self.oks.reserve(t);
        self.errs.reserve(e);
    }
}

#[cfg(test)]
mod tests {
    use crate::{MirrorRegion, OwnedRegion, Region, ReserveItems};

    use super::*;

    #[test]
    fn test_reserve() {
        let mut r = <ResultRegion<MirrorRegion<u8>, MirrorRegion<u8>>>::default();
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
