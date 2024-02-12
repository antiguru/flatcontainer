//! Region delegating to a vector.

use std::ops::{Deref, DerefMut};
use crate::{CopyOnto, Region};

#[derive(Debug)]
pub struct Vector<T>(pub Vec<T>);

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}


impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Clone> Region for Vector<T> {
    type ReadItem<'a> = &'a T where Self: 'a;
    type Index = usize;

    fn merge_regions<'a>(regions: impl Iterator<Item=&'a Self> + Clone) -> Self where Self: 'a {
        Self::with_capacity(regions.map(Vec::len).sum())
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        &self[index]
    }

    fn reserve_regions<'a, I>(&mut self, regions: I) where Self: 'a, I: Iterator<Item=&'a Self> + Clone {
        self.reserve(regions.map(Vec::len).sum());
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T: Clone> CopyOnto<Vector<T>> for T {
    fn copy_onto(self, target: &mut Vector<T>) -> usize {
        target.push(self);
        target.len()  - 1
    }
}

impl<T: Clone> CopyOnto<Vector<T>> for &T {
    fn copy_onto(self, target: &mut Vector<T>) -> usize {
        self.clone().copy_onto(target)
    }
}

#[derive(Debug)]
pub struct CopyVector<T>(pub Vec<T>);

impl<T> Default for CopyVector<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T> Deref for CopyVector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CopyVector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy> Region for CopyVector<T> {
    type ReadItem<'a> = T where Self: 'a;
    type Index = usize;

    fn merge_regions<'a>(regions: impl Iterator<Item=&'a Self> + Clone) -> Self where Self: 'a {
        Self::with_capacity(regions.map(Vec::len).sum())
    }

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        &self[index]
    }

    fn reserve_regions<'a, I>(&mut self, regions: I) where Self: 'a, I: Iterator<Item=&'a Self> + Clone {
        self.reserve(regions.map(Vec::len).sum());
    }

    fn clear(&mut self) {
        self.clear()
    }
}

impl<T: Copy> CopyOnto<CopyVector<T>> for T {
    fn copy_onto(self, target: &mut CopyVector<T>) -> usize {
        target.push(self);
        target.len()  - 1
    }
}

impl<T: Copy> CopyOnto<CopyVector<T>> for &T {
    fn copy_onto(self, target: &mut CopyVector<T>) -> usize {
        self.copied().copy_onto(target)
    }
}
