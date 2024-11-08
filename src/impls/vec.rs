//! Definitions to use `Vec<T>` as a region.

use crate::{Clear, HeapSize, Index, IndexAs, Len, Push, Region, Reserve, ReserveItems};

impl<T> Region for Vec<T> {
    #[inline(always)]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self::with_capacity(regions.map(Vec::len).sum())
    }

    #[inline(always)]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.reserve(regions.map(Vec::len).sum());
    }
}

impl<T: Clone> Index for Vec<T> {
    type Owned = T;
    type ReadItem<'a> = &'a T where Self: 'a;
    #[inline(always)]
    fn index(&self, index: usize) -> Self::ReadItem<'_> {
        &self[index]
    }
    #[inline(always)]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<T: Copy> IndexAs<T> for Vec<T> {
    #[inline(always)]
    fn index_as(&self, index: usize) -> T {
        self[index]
    }
}

impl<T: Clone> Push<T> for Vec<T> {
    #[inline(always)]
    fn push(&mut self, item: T) {
        self.push(item);
    }
}

impl<T: Clone> Push<&T> for Vec<T> {
    #[inline(always)]
    fn push(&mut self, item: &T) {
        self.push(item.clone());
    }
}

impl<T: Clone> Push<&&T> for Vec<T> {
    #[inline(always)]
    fn push(&mut self, item: &&T) {
        self.push((*item).clone());
    }
}

impl<T> Clear for Vec<T> {
    #[inline(always)]
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> Len for Vec<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HeapSize for Vec<T> {
    #[inline(always)]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        callback(self.len() * size_of_t, self.capacity() * size_of_t);
    }
}

impl<T> Reserve for Vec<T> {
    #[inline(always)]
    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }
}

impl<T: Clone, D> ReserveItems<D> for Vec<T> {
    #[inline(always)]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = D> + Clone,
    {
        self.reserve(items.count());
    }
}

#[cfg(test)]
mod tests {
    use crate::Index;

    #[test]
    fn vec() {
        use crate::{Push, ReserveItems};

        let mut region = Vec::<u32>::new();
        <_ as Push<_>>::push(&mut region, 42);
        assert_eq!(region.index(0), &42);

        let mut region = Vec::<u32>::new();
        region.push(42);
        region.push(43);
        region.push(44);
        region.reserve_items([1, 2, 3].iter());
        assert_eq!(region.index(0), &42);
        assert_eq!(region.index(1), &43);
        assert_eq!(region.index(2), &44);
    }
}
