use crate::{CopyOnto, Region, ReserveItems};
use std::ops::Deref;

/// A container representing slices of data.
#[derive(Debug)]
pub struct SliceRegion<C: Region> {
    // offsets: Vec<usize>,
    slices: Vec<C::Index>,
    inner: C,
}

impl<C: Region> Region for SliceRegion<C> {
    type ReadItem<'a> = (&'a C, &'a [C::Index]) where Self: 'a;
    type Index = (usize, usize);

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        let slice = &self.slices[start..end];
        (&self.inner, slice)
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.slices
            .reserve(regions.clone().map(|r| r.slices.len()).sum());
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    #[inline]
    fn clear(&mut self) {
        // self.offsets.clear();
        self.slices.clear();
        self.inner.clear();
    }
}

impl<C: Region + Default> Default for SliceRegion<C> {
    fn default() -> Self {
        Self {
            slices: Vec::default(),
            inner: C::default(),
        }
    }
}

impl<'a, C, T: 'a> CopyOnto<SliceRegion<C>> for &'a [T]
where
    C: Region,
    &'a T: CopyOnto<C>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        let start = target.slices.len();
        target
            .slices
            .extend(self.iter().map(|t| t.copy_onto(&mut target.inner)));
        (start, target.slices.len())
    }

    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.clone().map(|i| i.len()).sum());
        CopyOnto::reserve_items(&mut target.inner, items.flat_map(|i| i.iter()));
    }
}

impl<'a, T, R: Region> ReserveItems<SliceRegion<R>> for &'a [T]
where
    &'a T: ReserveItems<R> + 'a,
{
    fn reserve_items<I>(target: &mut SliceRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.clone().map(|i| i.len()).sum());
        ReserveItems::reserve_items(&mut target.inner, items.flat_map(|i| i.iter()));
    }
}

impl<'a, C, T> CopyOnto<SliceRegion<C>> for &'a Vec<T>
where
    C: Region,
    &'a [T]: CopyOnto<SliceRegion<C>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        self.as_slice().copy_onto(target)
    }

    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        CopyOnto::reserve_items(target, items.map(Deref::deref))
    }
}

impl<'a, T: 'a, R: Region> ReserveItems<SliceRegion<R>> for &'a Vec<T>
where
    &'a T: ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut SliceRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(Deref::deref))
    }
}

impl<C, T> CopyOnto<SliceRegion<C>> for Vec<T>
where
    C: Region,
    for<'a> &'a [T]: CopyOnto<SliceRegion<C>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        self.as_slice().copy_onto(target)
    }

    fn reserve_items<I>(_target: &mut SliceRegion<C>, _items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        //CopyOnto::reserve_items(target, items.map(Deref::deref))
    }
}

impl<'a, C: Region + 'a> CopyOnto<SliceRegion<C>> for &'a (&'a C, &'a [C::Index])
where
    C::ReadItem<'a>: CopyOnto<C>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        let (container, indexes) = self;
        let start = target.slices.len();
        target.slices.extend(
            indexes
                .iter()
                .map(|&index| container.index(index).copy_onto(&mut target.inner)),
        );
        (start, target.slices.len())
    }

    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target
            .slices
            .reserve(items.clone().map(|(_c, is)| is.len()).sum());
        CopyOnto::reserve_items(
            &mut target.inner,
            items.flat_map(|(c, is)| is.iter().map(|i| c.index(*i))),
        )
    }
}

impl<'a, C: Region + 'a> ReserveItems<SliceRegion<C>> for &'a (C, &'a [C::Index])
where
    C::ReadItem<'a>: ReserveItems<C>,
{
    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target
            .slices
            .reserve(items.clone().map(|(_c, is)| is.len()).sum());
        ReserveItems::reserve_items(
            &mut target.inner,
            items.flat_map(|(c, is)| is.iter().map(|i| c.index(*i))),
        )
    }
}
