//! Demonstration of how to encode recursive data structures.

use flatcontainer::impls::deduplicate::ConsecutiveOffsetPairs;
use flatcontainer::{IntoOwned, Push, Region, Storage, StringRegion};

#[derive(Clone)]
struct List<T>(T, Option<Box<List<T>>>);

struct ListRef<'a, C: Region>(
    Result<(&'a ListRegion<C>, <C as Region>::Index, Option<usize>), &'a List<C::Owned>>,
);

impl<'a, C: Region> ListRef<'a, C>
where
    C::Owned: Clone,
{
    fn inner(&self) -> C::ReadItem<'_> {
        match &self.0 {
            Ok((region, index, _continuation)) => region.inner.index(*index),
            Err(list) => IntoOwned::borrow_as(&list.0),
        }
    }

    fn next(&self) -> Option<Self> {
        match &self.0 {
            Ok((region, _index, continuation)) => continuation.map(|index| region.index(index)),
            Err(list) => list
                .1
                .as_ref()
                .map(|next| Self(Err(IntoOwned::borrow_as(next.as_ref())))),
        }
    }
}

impl<'a, C: Region> IntoOwned<'a> for ListRef<'a, C>
where
    C::Owned: Clone,
{
    type Owned = List<C::Owned>;

    fn into_owned(self) -> Self::Owned {
        List(
            self.inner().into_owned(),
            self.next().map(|next| Box::new(next.into_owned())),
        )
    }

    fn clone_onto(self, other: &mut Self::Owned) {
        *other = self.into_owned();
    }

    fn borrow_as(owned: &'a Self::Owned) -> Self {
        Self(Err(owned))
    }
}

#[derive(Debug)]
struct ListRegion<C: Region> {
    indexes: Vec<(C::Index, Option<usize>)>,
    inner: C,
}

impl<C: Region> Default for ListRegion<C> {
    fn default() -> Self {
        Self {
            indexes: Vec::default(),
            inner: C::default(),
        }
    }
}

impl<C: Region> Region for ListRegion<C>
where
    C::Owned: Clone,
{
    type Owned = List<C::Owned>;
    type ReadItem<'a> = ListRef<'a, C> where C: 'a;
    type Index = usize;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            indexes: Vec::with_capacity(regions.clone().map(|r| r.indexes.len()).sum()),
            inner: C::merge_regions(regions.map(|r| &r.inner)),
        }
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        let (inner_index, continuation) = self.indexes[index];
        ListRef(Ok((self, inner_index, continuation)))
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.indexes
            .reserve(regions.clone().map(|r| r.indexes.len()).sum());
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    fn clear(&mut self) {
        self.indexes.clear();
        self.inner.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        Storage::heap_size(&self.indexes, &mut callback);
        self.inner.heap_size(callback);
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<C, T> Push<&List<T>> for ListRegion<C>
where
    for<'a> C: Region + Push<&'a T>,
    C::Owned: Clone,
{
    fn push(&mut self, item: &List<T>) -> <ListRegion<C> as Region>::Index {
        let inner_index = self.inner.push(&item.0);
        let continuation = item.1.as_deref().map(|next| self.push(next));
        self.indexes.push((inner_index, continuation));
        self.indexes.len() - 1
    }
}

#[test]
fn recursive() {
    let mut region = <ListRegion<ConsecutiveOffsetPairs<StringRegion>>>::default();
    let r = List("abc", Some(Box::new(List("def", None))));
    let index = region.push(&r);

    let rref = region.index(index);
    assert_eq!(rref.inner(), "abc");
    let next = rref.next();
    assert!(next.is_some());
    let next = next.unwrap();
    assert_eq!(next.inner(), "def");
    assert!(next.next().is_none());

    println!("{region:?}");
}
