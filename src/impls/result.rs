//! A region that stores results.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Containerized, Push, Region, ReserveItems};

impl<T: Containerized, E: Containerized> Containerized for Result<T, E> {
    type Region = ResultRegion<T::Region, E::Region>;
}

/// A region to hold [`Result`]s.
///
/// # Examples
///
/// Add results to a result region:
/// ```
/// use flatcontainer::{Containerized, Push, Region, ResultRegion};
/// let mut r =
///     <ResultRegion<<() as Containerized>::Region, <String as Containerized>::Region>>::default();
///
/// let ok_index = r.push(Result::<(), String>::Ok(()));
/// let err_index = r.push(Result::<(), String>::Err("Error".to_string()));
///
/// assert_eq!(Ok(()), r.index(ok_index));
/// assert_eq!(Err("Error"), r.index(err_index));
/// ```
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResultRegion<T, E> {
    oks: T,
    errs: E,
}

impl<T, E> Region for ResultRegion<T, E>
where
    T: Region,
    E: Region,
{
    type ReadItem<'a> = Result<T::ReadItem<'a>, E::ReadItem<'a>> where Self: 'a;
    type Index = Result<T::Index, E::Index>;

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

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.oks.heap_size(&mut callback);
        self.errs.heap_size(callback);
    }
}

impl<T, TC, E, EC> Push<Result<T, E>> for ResultRegion<TC, EC>
where
    TC: Region,
    EC: Region,
    TC: Push<T>,
    EC: Push<E>,
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
    TC: Region,
    EC: Region,
    TC: Push<&'a T>,
    EC: Push<&'a E>,
{
    #[inline]
    fn push(&mut self, item: &'a Result<T, E>) -> <ResultRegion<TC, EC> as Region>::Index {
        match item {
            Ok(t) => Ok(self.oks.push(t)),
            Err(e) => Err(self.errs.push(e)),
        }
    }
}

impl<'a, T: 'a, TC, E: 'a, EC> ReserveItems<&'a Result<T, E>> for ResultRegion<TC, EC>
where
    TC: Region,
    EC: Region,
    TC: ReserveItems<&'a T>,
    EC: ReserveItems<&'a E>,
{
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
    use crate::{MirrorRegion, OwnedRegion, Region, ReserveItems};

    use super::*;

    #[test]
    fn test_reserve() {
        let mut r = <ResultRegion<MirrorRegion<u8>, MirrorRegion<u8>>>::default();
        ReserveItems::reserve_items(&mut r, [Ok(0), Err(1)].iter());
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
