//! Flat representation of regions.

use crate::Region;
use std::marker::PhantomData;
use std::ops::Deref;

/// TODO
pub trait FlatWrite {
    /// TODO
    fn write_lengthened<T: Copy + 'static>(&mut self, data: &[T]) -> std::io::Result<()>;
    /// TODO
    fn write_unit<T: Copy + 'static>(&mut self, unit: &T) -> std::io::Result<()>;
}

/// TODO
pub struct DefaultFlatWrite<W: std::io::Write> {
    inner: W,
    offset: usize,
    alignment: usize,
}

/// TODO
const ALIGNMENT: usize = 64;

impl<W: std::io::Write> DefaultFlatWrite<W> {
    const NULLS: [u8; ALIGNMENT - 1] = [0; ALIGNMENT - 1];

    /// TODO
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            offset: 0,
            alignment: 0,
        }
    }

    fn pad<T>(&mut self) -> std::io::Result<()> {
        let padding = (self.offset as *const u8).align_offset(std::mem::align_of::<T>());
        self.alignment = std::cmp::max(self.alignment, std::mem::align_of::<T>());
        self.inner.write_all(&Self::NULLS[..padding])?;
        self.offset += padding;
        Ok(())
    }

    /// TODO
    pub fn finish(mut self) -> std::io::Result<()> {
        let alignment: u8 = self
            .alignment
            .next_power_of_two()
            .trailing_zeros()
            .try_into()
            .unwrap();
        self.write_unit(&alignment)
    }
}

impl<W: std::io::Write> FlatWrite for DefaultFlatWrite<W> {
    fn write_lengthened<T: Copy + 'static>(&mut self, data: &[T]) -> std::io::Result<()> {
        println!(
            "write_lengthened data len: {}*{}",
            data.len(),
            std::mem::size_of::<T>()
        );
        self.write_unit(&data.len())?;
        self.pad::<T>()?;
        let data: &[u8] = unsafe {
            std::slice::from_raw_parts(data.as_ptr().cast(), std::mem::size_of_val(data))
        };
        println!(
            "write_lengthened data len: {}*{}",
            data.len(),
            std::mem::size_of::<u8>()
        );
        self.inner.write_all(data)?;
        self.offset += data.len();
        Ok(())
    }

    fn write_unit<T: Copy + 'static>(&mut self, unit: &T) -> std::io::Result<()> {
        self.pad::<T>()?;
        let slice = std::slice::from_ref(unit);
        let bytes = unsafe {
            std::slice::from_raw_parts(slice.as_ptr() as *const u8, std::mem::size_of_val(slice))
        };
        self.inner.write_all(bytes)?;
        self.offset += bytes.len();
        Ok(())
    }
}

/// TODO
#[derive(Clone, Copy, Debug, Default)]
pub struct DerefWrapper<S>(S);

impl<S> Deref for DerefWrapper<std::rc::Rc<S>>
where
    S: Deref<Target = [u8]>,
{
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.deref().deref()
    }
}

impl<S> Deref for DerefWrapper<std::sync::Arc<S>>
where
    S: Deref<Target = [u8]>,
{
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.deref().deref()
    }
}

/// TODO
pub struct Bytes<S> {
    buffer: S,
    start: usize,
    end: usize,
}

impl<S> Bytes<S>
where
    S: Deref<Target = [u8]> + Clone,
{
    /// TODO
    pub fn new_aligned(buffer: S, start: usize, end: usize) -> Self {
        if end - start > 1 {
            println!("asdf");
            let alignment = 1 << Bytes::new(&buffer.deref()[end - 1..], 0, 1).read_unit::<u8>();
            println!("alignment: {alignment}");
            let offset = buffer.deref()[start..].as_ptr().align_offset(alignment);
            assert_eq!(
                offset,
                0,
                "Unaliged memory: {:?} off by {} bytes",
                buffer.deref().as_ptr(),
                offset
            );
        }
        Self { buffer, start, end }
    }

    /// TODO
    pub fn new(buffer: S, start: usize, end: usize) -> Self {
        Self { buffer, start, end }
    }

    /// TODO
    pub fn read_lengthened<T: Copy + 'static>(&mut self) -> TypedBytes<S, T> {
        let len = self.read_unit::<usize>();
        let (head, _data, _tail) = unsafe { self.buffer[self.start..].align_to::<T>() };
        let end = self.start + head.len() + len * std::mem::size_of::<T>();
        let bytes = Self::new(self.buffer.clone(), self.start + head.len(), end);
        self.start = end;
        TypedBytes {
            bytes,
            _marker: PhantomData,
        }
    }

    /// TODO
    pub fn read_unit<T: Copy + 'static>(&mut self) -> T {
        let (head, data, _tail) = unsafe { self.buffer[self.start..].align_to::<T>() };
        self.start += head.len() + std::mem::size_of::<T>();
        data[0]
    }

    /// Call `callback` with `size`, `capacity` for each allocation.
    pub fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        callback(self.end - self.start, self.buffer.len());
    }
}

impl<S> Deref for Bytes<S>
where
    S: Deref<Target = [u8]>,
{
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer[self.start..self.end]
    }
}

/// TODO
pub struct TypedBytes<S, T> {
    pub(crate) bytes: Bytes<S>,
    _marker: PhantomData<T>,
}

impl<S, T> Default for TypedBytes<S, T>
where
    S: Default + Deref<Target = [u8]> + Clone,
{
    fn default() -> Self {
        Self {
            bytes: Bytes::new(S::default(), 0, 0),
            _marker: PhantomData,
        }
    }
}

impl<S, T> Deref for TypedBytes<S, T>
where
    S: Deref<Target = [u8]>,
    T: Copy + 'static,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let (head, data, _tail) = unsafe { self.bytes.deref().align_to::<T>() };
        assert_eq!(head.len(), 0, "Unaligned memory");
        data
    }
}

/// TODO
pub trait Flatten {
    /// TODO
    type Flat<S>;

    /// TODO
    fn entomb<W: FlatWrite>(&self, write: &mut W) -> std::io::Result<()>;

    /// TODO
    fn exhume<S>(buffer: &mut Bytes<S>) -> std::io::Result<Self::Flat<S>>
    where
        S: Deref<Target = [u8]> + Clone;
}

/// A type that can write its contents into a region.
pub trait CopyOnto<C: Region> {
    /// Copy self into the target container, returning an index that allows to
    /// look up the corresponding read item.
    fn copy_onto(self, target: &mut C) -> C::Index;
}

#[cfg(test)]
mod tests {
    use crate::flatten::{Bytes, DefaultFlatWrite, DerefWrapper, Flatten, ALIGNMENT};
    use crate::{OwnedRegion, Push, Region, StringRegion};
    use std::io::Write;
    use std::rc::Rc;

    #[test]
    fn test_flatten_slice() {
        let mut buffer = Vec::new();
        let mut write = DefaultFlatWrite::new(&mut buffer);

        let mut region = OwnedRegion::default();
        let index = region.push("abc".as_bytes());

        region.entomb(&mut write).unwrap();
        write.finish().unwrap();

        println!("{:?}", buffer);
        let end = buffer.len();

        let mut read = Bytes::new_aligned(&buffer[..], 0, end);

        let flat = OwnedRegion::<u8>::exhume(&mut read).unwrap();
        assert_eq!("abc".as_bytes(), flat.index(index));
    }

    #[test]
    fn test_flatten_string() {
        let mut buffer = Vec::new();
        let mut write = DefaultFlatWrite::new(&mut buffer);

        let mut region = <StringRegion>::default();
        let index = region.push("abc");
        let index2 = region.push("defghij");

        let mut other_region = OwnedRegion::default();
        let other_index = other_region.push([0x11223344566778899u128; 16]);

        region.entomb(&mut write).unwrap();
        other_region.entomb(&mut write).unwrap();
        write.finish().unwrap();

        let mut aligned_buffer = vec![0u8; buffer.len() + ALIGNMENT];
        let offset = aligned_buffer.as_ptr().align_offset(ALIGNMENT);
        println!("aligning to offset {offset}");
        (&mut aligned_buffer[offset..])
            .write_all(&buffer[..])
            .unwrap();

        println!("{:?}", buffer);

        let end = buffer.len();
        let mut read =
            Bytes::new_aligned(DerefWrapper(Rc::new(aligned_buffer)), offset, end + offset);

        let flat = <StringRegion>::exhume(&mut read).unwrap();
        assert_eq!("abc", flat.index(index));
        assert_eq!("defghij", flat.index(index2));
        let other_flat = OwnedRegion::<u128>::exhume(&mut read).unwrap();
        assert_eq!(other_flat.index(other_index), [0x11223344566778899u128; 16]);
    }
}
