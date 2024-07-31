//! Definitions to use `Vec<T>` as a region.

use crate::{CanPush, Push, Region, Reserve, ReserveItems, TryPush};

impl<T: Clone> Region for Vec<T> {
    type Owned = T;
    type ReadItem<'a> = &'a T where Self: 'a;
    type Index = usize;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self::with_capacity(regions.map(Vec::len).sum())
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        &self[index]
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.reserve(regions.map(Vec::len).sum());
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        callback(self.len() * size_of_t, self.capacity() * size_of_t);
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<T: Clone> Push<T> for Vec<T> {
    fn push(&mut self, item: T) -> Self::Index {
        self.push(item);
        self.len() - 1
    }
}

impl<T: Clone> TryPush<T> for Vec<T> {
    fn try_push(&mut self, item: T) -> Result<Self::Index, T> {
        if self.can_push(std::iter::once(&item)) {
            Ok(Push::push(self, item))
        } else {
            Err(item)
        }
    }
}

impl<T: Clone> Push<&T> for Vec<T> {
    fn push(&mut self, item: &T) -> Self::Index {
        self.push(item.clone());
        self.len() - 1
    }
}

impl<T: Clone> TryPush<&T> for Vec<T> {
    fn try_push<'a>(&mut self, item: &'a T) -> Result<Self::Index, &'a T> {
        if self.can_push(std::iter::once(item)) {
            Ok(Push::push(self, item))
        } else {
            Err(item)
        }
    }
}

impl<'a, T> CanPush<&'a T> for Vec<T>
where
    T: 'a,
{
    fn can_push<I>(&self, items: I) -> bool
    where
        I: Iterator<Item = &'a T> + Clone,
    {
        self.capacity() - self.len() >= items.count()
    }
}

impl<T: Clone> Push<&&T> for Vec<T> {
    fn push(&mut self, item: &&T) -> Self::Index {
        self.push((*item).clone());
        self.len() - 1
    }
}

impl<T: Clone> TryPush<&&T> for Vec<T> {
    fn try_push<'a, 'b>(&mut self, item: &'a &'b T) -> Result<Self::Index, &'a &'b T> {
        if self.can_push(std::iter::once(*item)) {
            Ok(Push::push(self, item))
        } else {
            Err(item)
        }
    }
}

impl<'a, 'b, T> CanPush<&'a &'b T> for Vec<T>
where
    &'b T: 'a,
{
    #[inline]
    fn can_push<I>(&self, items: I) -> bool
    where
        I: Iterator<Item = &'a &'b T> + Clone,
    {
        self.capacity() - self.len() >= items.count()
    }
}

impl<T: Clone, D> ReserveItems<D> for Vec<T> {
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = D> + Clone,
    {
        self.reserve(items.count());
    }
}

impl<T> Reserve for Vec<T> {
    type Reserve = usize;

    fn reserve(&mut self, size: &Self::Reserve) {
        self.reserve(*size);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vec() {
        use crate::{Push, Region, ReserveItems};

        let mut region = Vec::<u32>::new();
        let index = <_ as Push<_>>::push(&mut region, 42);
        assert_eq!(region.index(index), &42);

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
