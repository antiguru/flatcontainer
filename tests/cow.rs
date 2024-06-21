//! What follows is an example of a Cow-like type that can be used to switch between a GAT
//! and an owned type at runtime.

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use flatcontainer::{FlatStack, IntoOwned, Push, Region, StringRegion};

pub struct GatCow<'a, B>
where
    B: IntoOwned<'a>,
{
    inner: GatCowInner<B, B::Owned>,
    _marker: PhantomData<()>,
}

enum GatCowInner<B, T> {
    Borrowed(B),
    Owned(T),
}

impl<'a, B> From<GatCowInner<B, B::Owned>> for GatCow<'a, B>
where
    B: IntoOwned<'a>,
{
    fn from(inner: GatCowInner<B, B::Owned>) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<'a, B> GatCow<'a, B>
where
    B: IntoOwned<'a> + Copy,
{
    pub const fn is_borrowed(&self) -> bool {
        use GatCowInner::*;
        match &self.inner {
            Borrowed(_) => true,
            Owned(_) => false,
        }
    }

    pub const fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn to_mut(&mut self) -> &mut B::Owned {
        match self.inner {
            GatCowInner::Borrowed(borrowed) => {
                self.inner = GatCowInner::Owned(borrowed.into_owned());
                match &mut self.inner {
                    GatCowInner::Borrowed(..) => unreachable!(),
                    GatCowInner::Owned(owned) => owned,
                }
            }
            GatCowInner::Owned(ref mut owned) => owned,
        }
    }
}

impl<'a, B> IntoOwned<'a> for GatCow<'a, B>
where
    B: IntoOwned<'a> + Copy,
{
    type Owned = B::Owned;

    fn into_owned(self) -> B::Owned {
        match self.inner {
            GatCowInner::Borrowed(b) => b.into_owned(),
            GatCowInner::Owned(o) => o,
        }
    }

    fn clone_onto(self, other: &mut B::Owned) {
        match self.inner {
            GatCowInner::Borrowed(b) => b.clone_onto(other),
            GatCowInner::Owned(o) => *other = o,
        }
    }

    fn borrow_as(owned: &'a B::Owned) -> Self {
        GatCowInner::Borrowed(IntoOwned::borrow_as(owned)).into()
    }
}

impl<'a, B> Debug for GatCow<'a, B>
where
    B: IntoOwned<'a> + Debug,
    B::Owned: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            GatCowInner::Borrowed(b) => b.fmt(f),
            GatCowInner::Owned(o) => o.fmt(f),
        }
    }
}

#[derive(Default, Debug, Clone)]
struct CowRegion<R>(R);

impl<R> Region for CowRegion<R>
where
    R: Region,
    for<'a> R::ReadItem<'a>: Copy,
{
    type Owned = <R as Region>::Owned;
    type ReadItem<'a> = GatCow<'a, R::ReadItem<'a>> where Self: 'a;
    type Index = R::Index;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self(R::merge_regions(regions.map(|r| &r.0)))
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        GatCowInner::Borrowed(self.0.index(index)).into()
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.0.reserve_regions(regions.map(|r| &r.0))
    }

    fn clear(&mut self) {
        self.0.clear()
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.0.heap_size(callback)
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        match item.inner {
            GatCowInner::Borrowed(b) => GatCowInner::Borrowed(R::reborrow(b)),
            GatCowInner::Owned(o) => GatCowInner::Owned(o),
        }
        .into()
    }
}

impl<R, D> Push<D> for CowRegion<R>
where
    R: Region + Push<D>,
    for<'a> R::ReadItem<'a>: Copy,
{
    fn push(&mut self, item: D) -> Self::Index {
        self.0.push(item)
    }
}

#[test]
fn test_gat_cow() {
    let mut c = <FlatStack<CowRegion<StringRegion>>>::default();
    c.copy("abc");

    assert_eq!("abc", c.get(0).into_owned());
    let mut item = c.get(0);
    item.to_mut().push_str("def");
    assert_eq!("abcdef", item.into_owned());
    assert_eq!("abc", c.get(0).into_owned());
}
