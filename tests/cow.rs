//! What follows is an example of a Cow-like type that can be used to switch between a GAT
//! and an owned type at runtime.

use flatcontainer::{FlatStack, IntoOwned, Push, Region, StringRegion};
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};

#[allow(dead_code)]
enum GatCow<'a, B, T> {
    Borrowed(B),
    Owned(T),
    Never(&'a Infallible),
}

impl<'a, B, T> GatCow<'a, B, T>
where
    B: IntoOwned<'a, Owned = T> + Copy,
{
    pub fn to_mut(&mut self) -> &mut T {
        match self {
            Self::Borrowed(borrowed) => {
                *self = Self::Owned(borrowed.into_owned());
                match *self {
                    Self::Borrowed(..) => unreachable!(),
                    Self::Owned(ref mut owned) => owned,
                    Self::Never(_) => unreachable!(),
                }
            }
            Self::Owned(ref mut owned) => owned,
            Self::Never(_) => unreachable!(),
        }
    }
}

impl<'a, B, T> IntoOwned<'a> for GatCow<'a, B, T>
where
    B: IntoOwned<'a, Owned = T> + Copy,
{
    type Owned = T;

    fn into_owned(self) -> T {
        match self {
            GatCow::Borrowed(b) => b.into_owned(),
            GatCow::Owned(o) => o,
            Self::Never(_) => unreachable!(),
        }
    }

    fn clone_onto(self, other: &mut T) {
        match self {
            GatCow::Borrowed(b) => b.clone_onto(other),
            GatCow::Owned(o) => *other = o,
            Self::Never(_) => unreachable!(),
        }
    }

    fn borrow_as(owned: &'a T) -> Self {
        GatCow::Borrowed(IntoOwned::borrow_as(owned))
    }
}

impl<B, T> Debug for GatCow<'_, B, T>
where
    B: Debug,
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GatCow::Borrowed(b) => b.fmt(f),
            GatCow::Owned(o) => o.fmt(f),
            Self::Never(_) => unreachable!(),
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
    type ReadItem<'a> = GatCow<'a, R::ReadItem<'a>, R::Owned> where Self: 'a;
    type Index = R::Index;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self(R::merge_regions(regions.map(|r| &r.0)))
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        GatCow::Borrowed(self.0.index(index))
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
        match item {
            GatCow::Borrowed(b) => GatCow::Borrowed(R::reborrow(b)),
            GatCow::Owned(o) => GatCow::Owned(o),
            GatCow::Never(_) => unreachable!(),
        }
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
