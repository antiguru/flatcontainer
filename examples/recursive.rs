use flatcontainer::impls::deduplicate::ConsecutiveOffsetPairs;
use flatcontainer::impls::offsets::OffsetContainer;
use flatcontainer::{CopyOnto, ReadRegion, Region, StringRegion};

struct R<T>(T, Option<Box<R<T>>>);

struct RRef<'a, C: ReadRegion>(&'a RRegion<C>, <C as ReadRegion>::Index, Option<usize>);

impl<'a, C: ReadRegion> RRef<'a, C> {
    fn inner(&self) -> C::ReadItem<'_> {
        self.0.inner.index(self.1)
    }

    fn next(&self) -> Option<Self> {
        self.2.map(|index| self.0.index(index))
    }
}

#[derive(Debug)]
struct RRegion<C: ReadRegion> {
    indexes: Vec<(C::Index, Option<usize>)>,
    inner: C,
}

impl<C: ReadRegion> ReadRegion for RRegion<C> {
    type ReadItem<'a> = RRef<'a, C> where C: 'a;
    type Index = usize;

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        let (inner_index, continuation) = self.indexes[index];
        RRef(self, inner_index, continuation)
    }
}

impl<C: Region> Default for RRegion<C> {
    fn default() -> Self {
        Self {
            indexes: Vec::default(),
            inner: C::default(),
        }
    }
}

impl<C: Region> Region for RRegion<C> {
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            indexes: Vec::with_capacity(regions.clone().map(|r| r.indexes.len()).sum()),
            inner: C::merge_regions(regions.map(|r| &r.inner)),
        }
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
        self.indexes.heap_size(&mut callback);
        self.inner.heap_size(callback);
    }
}

impl<C, T> CopyOnto<RRegion<C>> for &R<T>
where
    C: Region,
    for<'a> &'a T: CopyOnto<C>,
{
    fn copy_onto(self, target: &mut RRegion<C>) -> <RRegion<C> as ReadRegion>::Index {
        let inner_index = (&self.0).copy_onto(&mut target.inner);
        let continuation = self.1.as_deref().map(|next| next.copy_onto(target));
        target.indexes.push((inner_index, continuation));
        target.indexes.len() - 1
    }
}

fn main() {
    let mut region = <RRegion<ConsecutiveOffsetPairs<StringRegion>>>::default();
    let r = R("abc", Some(Box::new(R("def", None))));
    let index = (&r).copy_onto(&mut region);

    let rref = region.index(index);
    assert_eq!(rref.inner(), "abc");
    let next = rref.next();
    assert!(next.is_some());
    let next = next.unwrap();
    assert_eq!(next.inner(), "def");
    assert!(next.next().is_none());

    println!("{region:?}");
}
