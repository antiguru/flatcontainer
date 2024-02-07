#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Containerized, CopyOnto, Region, ReserveItems};

impl<T: Containerized, E: Containerized> Containerized for Result<T, E> {
    type Region = ResultRegion<T::Region, E::Region>;
}

/// A region to hold [`Result`]s.
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
        self.errs.reserve_regions(regions.clone().map(|r| &r.errs));
    }

    #[inline]
    fn clear(&mut self) {
        self.oks.clear();
        self.errs.clear();
    }
}

impl<T, TC, E, EC> CopyOnto<ResultRegion<TC, EC>> for Result<T, E>
where
    TC: Region,
    EC: Region,
    T: CopyOnto<TC>,
    E: CopyOnto<EC>,
{
    #[inline]
    fn copy_onto(
        self,
        target: &mut ResultRegion<TC, EC>,
    ) -> <ResultRegion<TC, EC> as Region>::Index {
        match self {
            Ok(t) => Ok(t.copy_onto(&mut target.oks)),
            Err(e) => Err(e.copy_onto(&mut target.errs)),
        }
    }
}

impl<'a, T: 'a, TC, E: 'a, EC> CopyOnto<ResultRegion<TC, EC>> for &'a Result<T, E>
where
    TC: Region,
    EC: Region,
    &'a T: CopyOnto<TC>,
    &'a E: CopyOnto<EC>,
{
    #[inline]
    fn copy_onto(
        self,
        target: &mut ResultRegion<TC, EC>,
    ) -> <ResultRegion<TC, EC> as Region>::Index {
        match self {
            Ok(t) => Ok(t.copy_onto(&mut target.oks)),
            Err(e) => Err(e.copy_onto(&mut target.errs)),
        }
    }
}

impl<'a, T: 'a, TC, E: 'a, EC> ReserveItems<ResultRegion<TC, EC>> for &'a Result<T, E>
where
    TC: Region,
    EC: Region,
    &'a T: ReserveItems<TC>,
    &'a E: ReserveItems<EC>,
{
    fn reserve_items<I>(target: &mut ResultRegion<TC, EC>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(&mut target.oks, items.clone().flat_map(|r| r.as_ref().ok()));
        ReserveItems::reserve_items(
            &mut target.errs,
            items.clone().flat_map(|r| r.as_ref().err()),
        );
    }
}
