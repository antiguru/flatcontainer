//! A slice container that Huffman encodes its contents.

use std::collections::BTreeMap;

use crate::{Push, Region};

use self::encoded::Encoded;
use self::huffman::Huffman;
use self::wrapper::Wrapped;

/// A container that contains slices `[B]` as items.
pub struct HuffmanContainer<B: Ord + Clone> {
    /// Either encoded data or raw data.
    /// Encoded data is a map, a list of bytes, and a number of valid *bits*.
    inner: Result<(Huffman<B>, Vec<u8>, usize), Vec<B>>,
    /// Counts of the number of each pattern we've seen.
    stats: BTreeMap<B, i64>,
}

impl<B> HuffmanContainer<B>
where
    B: Ord + Clone,
{
    /// Prints statistics about encoded containers.
    pub fn print(&self) {
        if let Ok((_huff, _bytes, bits)) = &self.inner {
            println!(
                "Bits: {:?}, Symbols: {:?}",
                bits,
                self.stats.values().sum::<i64>()
            );
        }
    }
}

impl<B: Ord + Clone> Clone for HuffmanContainer<B> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            stats: self.stats.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
        self.stats.clone_from(&source.stats);
    }
}

impl<B> Region for HuffmanContainer<B>
where
    B: Ord + Clone + Sized + 'static,
{
    type Owned = Vec<B>;
    type ReadItem<'a> = Wrapped<'a, B>;

    type Index = (usize, usize);

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        for region in regions.clone().filter(|r| r.inner.is_ok()) {
            region.print();
        }

        let mut counts = BTreeMap::default();
        for (symbol, count) in regions.flat_map(|r| r.stats.iter()) {
            *counts.entry(symbol.clone()).or_insert(0) += count;
        }

        let bytes = Vec::with_capacity(counts.values().cloned().sum::<i64>() as usize);
        let huffman = Huffman::create_from(counts);
        let inner = Ok((huffman, bytes, 0));

        Self {
            inner,
            stats: Default::default(),
        }
    }

    fn index(&self, (lower, upper): Self::Index) -> Self::ReadItem<'_> {
        match &self.inner {
            Ok((huffman, bytes, _bits)) => {
                Wrapped::encoded(Encoded::new(huffman, bytes, (lower, upper)))
            }
            Err(raw) => Wrapped::decoded(&raw[lower..upper]),
        }
    }

    fn reserve_regions<'a, I>(&mut self, _regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        todo!()
    }

    fn clear(&mut self) {
        match &mut self.inner {
            Ok(_) => self.inner = Err(Vec::default()),
            Err(vec) => vec.clear(),
        }
        self.stats.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, _callback: F) {
        todo!()
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

/// Re-used function to push encoded symbols into a byte vector.
///
/// This function encodes the symbols of `iter` into a sequence of bits,
/// which are bundled as bytes and pushed into `bytes`. The total number
/// of encoded bits is updated at the same time.
///
/// The first three arguments correspond to the `Ok` variant of the
/// `HuffmanContainer` type, and this function would be a method of the
/// hypothetical type that this variant represents.
fn push_symbols<'a, I, B>(
    huffman: &'a Huffman<B>,
    bytes: &mut Vec<u8>,
    bits: &mut usize,
    iter: I,
) -> (usize, usize)
where
    B: Ord + 'a,
    I: Iterator<Item = &'a B>,
{
    // We'll only append bits, and start at the number of bits we have already.
    let start = *bits;
    // Any incomplete bytes are peeled off and re-presented as by the encoder,
    // so we should shear them off from the count here to avoid double counting
    // when we get encoder outputs.
    *bits = *bits - (*bits % 8);
    // We may end with a partial byte, in which case we should
    // determine and start with those bits, to write the newly
    // encoded bits into the same byte.
    let initially = if start % 8 == 0 {
        (0, 0)
    } else {
        let bits = start % 8;
        let byte = bytes.pop().unwrap() >> (8 - bits);
        (byte, bits)
    };
    // Each encoded by should be pushed, and the number of bits maintained.
    // The `Ok` and `Err` variants describe whole and partial bytes, respectively.
    for byte in huffman.encode(initially, iter) {
        match byte {
            Ok(byte) => {
                bytes.push(byte);
                *bits += 8;
            }
            Err((byte, bs)) => {
                bytes.push(byte);
                *bits += bs;
            }
        }
    }
    (start, *bits)
}

impl<B> Push<&[B]> for HuffmanContainer<B>
where
    B: Ord + Clone + Sized + 'static,
{
    fn push(&mut self, item: &[B]) -> (usize, usize) {
        for x in item.iter() {
            *self.stats.entry(x.clone()).or_insert(0) += 1;
        }
        match &mut self.inner {
            Ok((huffman, bytes, bits)) => push_symbols(huffman, bytes, bits, item.iter()),
            Err(raw) => {
                let start = raw.len();
                raw.extend_from_slice(item);
                (start, raw.len())
            }
        }
    }
}

impl<B, const N: usize> Push<[B; N]> for HuffmanContainer<B>
where
    B: Ord + Clone + Sized + 'static,
{
    fn push(&mut self, item: [B; N]) -> (usize, usize) {
        self.push(item.as_slice())
    }
}

impl<B, const N: usize> Push<&[B; N]> for HuffmanContainer<B>
where
    B: Ord + Clone + Sized + 'static,
{
    fn push(&mut self, item: &[B; N]) -> (usize, usize) {
        self.push(item.as_slice())
    }
}

impl<B> Push<Vec<B>> for HuffmanContainer<B>
where
    B: Ord + Clone + Sized + 'static,
{
    fn push(&mut self, item: Vec<B>) -> (usize, usize) {
        self.push(item.as_slice())
    }
}

impl<B> Push<&Vec<B>> for HuffmanContainer<B>
where
    B: Ord + Clone + Sized + 'static,
{
    fn push(&mut self, item: &Vec<B>) -> (usize, usize) {
        self.push(item.as_slice())
    }
}

impl<'a, B> Push<Wrapped<'a, B>> for HuffmanContainer<B>
where
    B: Ord + Clone + Sized + 'static,
{
    fn push(&mut self, item: Wrapped<'a, B>) -> (usize, usize) {
        match item.decode() {
            Ok(decoded) => {
                for x in decoded {
                    *self.stats.entry(x.clone()).or_insert(0) += 1;
                }
            }
            Err(symbols) => {
                for x in symbols.iter() {
                    *self.stats.entry(x.clone()).or_insert(0) += 1;
                }
            }
        }
        match (item.decode(), &mut self.inner) {
            (Ok(decoded), Ok((huffman, bytes, bits))) => {
                push_symbols(huffman, bytes, bits, decoded)
            }
            (Ok(decoded), Err(raw)) => {
                let start = raw.len();
                raw.extend(decoded.cloned());
                (start, raw.len())
            }
            (Err(symbols), Ok((huffman, bytes, bits))) => {
                push_symbols(huffman, bytes, bits, symbols.iter())
            }
            (Err(symbols), Err(raw)) => {
                let start = raw.len();
                raw.extend(symbols.iter().cloned());
                (start, raw.len())
            }
        }
    }
}

impl<B: Ord + Clone> Default for HuffmanContainer<B> {
    fn default() -> Self {
        Self {
            inner: Err(Vec::new()),
            stats: Default::default(),
        }
    }
}

mod wrapper {
    use std::fmt::Debug;

    use super::Encoded;

    pub struct Wrapped<'a, B: Ord> {
        inner: Result<Encoded<'a, B>, &'a [B]>,
    }

    impl<B: Ord + Debug> std::fmt::Debug for Wrapped<'_, B> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            let mut list = fmt.debug_list();
            match &self.inner {
                Ok(encoded) => list.entries(encoded.decode()).finish(),
                Err(symbols) => list.entries(*symbols).finish(),
            }
        }
    }

    impl<'a, B: Ord> Wrapped<'a, B> {
        /// Returns either a decoding iterator, or just the bytes themselves.
        pub fn decode(&'a self) -> Result<impl Iterator<Item = &'a B> + 'a, &'a [B]> {
            match &self.inner {
                Ok(encoded) => Ok(encoded.decode()),
                Err(symbols) => Err(symbols),
            }
        }
        /// A wrapper around an encoded sequence.
        pub fn encoded(e: Encoded<'a, B>) -> Self {
            Self { inner: Ok(e) }
        }
        /// A wrapper around a decoded sequence.
        pub fn decoded(d: &'a [B]) -> Self {
            Self { inner: Err(d) }
        }
    }

    impl<'a, B: Ord> Copy for Wrapped<'a, B> {}
    impl<'a, B: Ord> Clone for Wrapped<'a, B> {
        fn clone(&self) -> Self {
            *self
        }
    }

    use crate::IntoOwned;
    use std::cmp::Ordering;

    impl<'a, 'b, B: Ord> PartialEq<Wrapped<'a, B>> for Wrapped<'b, B> {
        fn eq(&self, other: &Wrapped<'a, B>) -> bool {
            match (self.decode(), other.decode()) {
                (Ok(decode1), Ok(decode2)) => decode1.eq(decode2),
                (Ok(decode1), Err(bytes2)) => decode1.eq(bytes2.iter()),
                (Err(bytes1), Ok(decode2)) => bytes1.iter().eq(decode2),
                (Err(bytes1), Err(bytes2)) => bytes1.eq(bytes2),
            }
        }
    }
    impl<'a, B: Ord> Eq for Wrapped<'a, B> {}
    impl<'a, 'b, B: Ord> PartialOrd<Wrapped<'a, B>> for Wrapped<'b, B> {
        fn partial_cmp(&self, other: &Wrapped<'a, B>) -> Option<Ordering> {
            match (self.decode(), other.decode()) {
                (Ok(decode1), Ok(decode2)) => decode1.partial_cmp(decode2),
                (Ok(decode1), Err(bytes2)) => decode1.partial_cmp(bytes2.iter()),
                (Err(bytes1), Ok(decode2)) => bytes1.iter().partial_cmp(decode2),
                (Err(bytes1), Err(bytes2)) => bytes1.partial_cmp(bytes2),
            }
        }
    }
    impl<'a, B: Ord> Ord for Wrapped<'a, B> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    impl<'a, B: Ord + Clone> IntoOwned<'a> for Wrapped<'a, B> {
        type Owned = Vec<B>;

        fn into_owned(self) -> Self::Owned {
            match self.decode() {
                Ok(iter) => iter.cloned().collect(),
                Err(slice) => slice.to_vec(),
            }
        }

        fn clone_onto(self, other: &mut Self::Owned) {
            match self.decode() {
                Ok(iter) => {
                    other.clear();
                    other.extend(iter.cloned());
                }
                Err(slice) => {
                    other.clear();
                    other.extend_from_slice(slice);
                }
            }
        }

        fn borrow_as(owned: &'a Self::Owned) -> Self {
            Self {
                inner: Err(owned.as_slice()),
            }
        }
    }
}

/// Wrapper around a Huffman decoder and byte slices, decodeable to a byte sequence.
mod encoded {

    use super::Huffman;

    /// Welcome to GATs!
    pub struct Encoded<'a, B: Ord> {
        /// Text that decorates the data.
        huffman: &'a Huffman<B>,
        /// The data itself.
        bytes: &'a [u8],
        /// Bit addressed range, start and end, of valid bits.
        ///
        /// This has the potential to include a partial byte at the start, at the end,
        /// and potentially be less than a byte in total for that matter.
        bit_range: (usize, usize),
    }

    impl<'a, B: Ord> Encoded<'a, B> {
        /// Returns either a decoding iterator, or just the bytes themselves.
        pub fn decode(&'a self) -> impl Iterator<Item = &'a B> + 'a {
            let iter = BitIterator {
                bytes: self.bytes,
                bit_range: self.bit_range,
            };
            self.huffman.decode(iter)
        }
        pub fn new(huffman: &'a Huffman<B>, bytes: &'a [u8], bit_range: (usize, usize)) -> Self {
            Self {
                huffman,
                bytes,
                bit_range,
            }
        }
    }

    impl<'a, B: Ord> Copy for Encoded<'a, B> {}
    impl<'a, B: Ord> Clone for Encoded<'a, B> {
        fn clone(&self) -> Self {
            *self
        }
    }

    /// An iterator over bits in a byte slice.
    ///
    /// The iterator returns a byte at a time and the number of bits in that byte.
    /// This can often be a whole valid byte at a time, but the first and last bytes
    /// may only contain partial information.
    struct BitIterator<'a> {
        /// Byte storage within which the addressed bits live.
        bytes: &'a [u8],
        /// Bit addressed range, start and end, of valid bits.
        ///
        /// This has the potential to include a partial byte at the start, at the end,
        /// and potentially be less than a byte in total for that matter.
        bit_range: (usize, usize),
    }

    impl<'a> Iterator for BitIterator<'a> {
        type Item = (u8, usize);
        fn next(&mut self) -> Option<Self::Item> {
            // If bits remain to consume ...
            if self.bit_range.0 < self.bit_range.1 {
                // We will certainly pull the byte from `self.bytes[self.bit_range.0 / 8]`.
                let byte = self.bytes[self.bit_range.0 / 8];
                // The number of bits we will pull depends on the start and end of the range.
                // We can't pull more bits than our range allows, nor more bits than are in the byte.
                let bits = std::cmp::min(
                    self.bit_range.1 - self.bit_range.0,
                    8 - self.bit_range.0 % 8,
                );
                // Now we need to clean up the byte, shifting and masking it.
                // This shift depends on the start of the range and the valid bits.
                let byte = (byte >> (8 - self.bit_range.0 % 8 - bits)) & ((1 << bits) - 1);
                // Advance our cursor to reflect the bits we have consumed.
                self.bit_range.0 += bits;
                Some((byte, bits))
            } else {
                None
            }
        }
    }
}

mod huffman {

    use std::collections::BTreeMap;
    use std::convert::TryInto;

    use self::decoder::Decoder;
    use self::encoder::Encoder;

    /// Encoding and decoding state for Huffman codes.
    pub struct Huffman<T: Ord> {
        /// byte indexed description of what to blat down for encoding.
        /// An entry `(bits, code)` indicates that the low `bits` of `code` should be blatted down.
        /// Probably every `code` fits in a `u64`, unless there are crazy frequencies?
        encode: BTreeMap<T, (usize, u64)>,
        /// Byte-by-byte decoder.
        decode: [Decode<T>; 256],
    }

    impl<T: Ord + Clone> Clone for Huffman<T> {
        fn clone(&self) -> Self {
            Self {
                encode: self.encode.clone(),
                decode: self.decode.clone(),
            }
        }

        fn clone_from(&mut self, source: &Self) {
            self.encode.clone_from(&source.encode);
            self.decode.clone_from(&source.decode);
        }
    }

    impl<T: Ord> Huffman<T> {
        /// Encodes the provided symbols as a sequence of bytes.
        ///
        /// The last byte may only contain partial information, but it should be recorded as presented,
        /// as we haven't a way to distinguish (e.g. a `Result` return type).
        pub fn encode<'a, I>(
            &'a self,
            initially: (u8, usize),
            symbols: I,
        ) -> Encoder<'a, T, I::IntoIter>
        where
            I: IntoIterator<Item = &'a T>,
        {
            Encoder::new(&self.encode, initially, symbols.into_iter())
        }

        /// Decodes the provided bytes as a sequence of symbols.
        pub fn decode<I>(&self, bytes: I) -> Decoder<'_, T, I::IntoIter>
        where
            I: IntoIterator<Item = (u8, usize)>,
        {
            Decoder::new(&self.decode, bytes.into_iter())
        }

        pub fn create_from(counts: BTreeMap<T, i64>) -> Self
        where
            T: Clone,
        {
            if counts.is_empty() {
                return Self {
                    encode: Default::default(),
                    decode: Decode::map(),
                };
            }

            let mut heap = std::collections::BinaryHeap::new();
            for (item, count) in counts {
                heap.push((-count, Node::Leaf(item)));
            }
            let mut tree = Vec::with_capacity(2 * heap.len() - 1);
            while heap.len() > 1 {
                let (count1, least1) = heap.pop().unwrap();
                let (count2, least2) = heap.pop().unwrap();
                let fork = Node::Fork(tree.len(), tree.len() + 1);
                tree.push(least1);
                tree.push(least2);
                heap.push((count1 + count2, fork));
            }
            tree.push(heap.pop().unwrap().1);

            let mut levels = Vec::with_capacity(1 + tree.len() / 2);
            let mut todo = vec![(tree.last().unwrap(), 0)];
            while let Some((node, level)) = todo.pop() {
                match node {
                    Node::Leaf(sym) => {
                        levels.push((level, sym));
                    }
                    Node::Fork(l, r) => {
                        todo.push((&tree[*l], level + 1));
                        todo.push((&tree[*r], level + 1));
                    }
                }
            }
            levels.sort_by(|x, y| x.0.cmp(&y.0));
            let mut code: u64 = 0;
            let mut prev_level = 0;
            let mut encode = BTreeMap::new();
            let mut decode = Decode::map();
            for (level, sym) in levels {
                if prev_level != level {
                    code <<= level - prev_level;
                    prev_level = level;
                }
                encode.insert(sym.clone(), (level, code));
                Self::insert_decode(&mut decode, sym, level, code << (64 - level));

                code += 1;
            }

            for (index, entry) in decode.iter().enumerate() {
                if entry.any_void() {
                    panic!("VOID FOUND: {:?}", index);
                }
            }

            Huffman { encode, decode }
        }

        /// Inserts a symbol, and
        fn insert_decode(map: &mut [Decode<T>; 256], symbol: &T, bits: usize, code: u64)
        where
            T: Clone,
        {
            let byte: u8 = (code >> 56).try_into().unwrap();
            if bits <= 8 {
                for off in 0..(1 << (8 - bits)) {
                    map[(byte as usize) + off] = Decode::Symbol(symbol.clone(), bits);
                }
            } else {
                if let Decode::Void = &map[byte as usize] {
                    map[byte as usize] = Decode::Further(Box::new(Decode::map()));
                }
                if let Decode::Further(next_map) = &mut map[byte as usize] {
                    Self::insert_decode(next_map, symbol, bits - 8, code << 8);
                }
            }
        }
    }
    /// Tree structure for Huffman bit length determination.
    #[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
    enum Node<T> {
        Leaf(T),
        Fork(usize, usize),
    }

    /// Decoder
    #[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Default, Clone)]
    pub enum Decode<T> {
        /// An as-yet unfilled slot.
        #[default]
        Void,
        /// The symbol, and the number of bits consumed.
        Symbol(T, usize),
        /// An additional map to push subsequent bytes at.
        Further(Box<[Decode<T>; 256]>),
    }

    impl<T> Decode<T> {
        /// Tests to see if the map contains any invalid values.
        ///
        /// A correctly initialized map will have no invalid values.
        /// A map with invalid values will be unable to decode some
        /// input byte sequences.
        fn any_void(&self) -> bool {
            match self {
                Decode::Void => true,
                Decode::Symbol(_, _) => false,
                Decode::Further(map) => map.iter().any(|m| m.any_void()),
            }
        }
        /// Creates a new map containing invalid values.
        fn map() -> [Decode<T>; 256] {
            let mut vec = Vec::with_capacity(256);
            for _ in 0..256 {
                vec.push(Decode::Void);
            }
            vec.try_into().ok().unwrap()
        }
    }

    /// A tabled Huffman decoder, written as an iterator.
    mod decoder {

        use super::Decode;

        #[derive(Copy, Clone)]
        pub struct Decoder<'a, T, I> {
            decode: &'a [Decode<T>; 256],
            bytes: I,
            pending_byte: u16,
            pending_bits: usize,
        }

        impl<'a, T, I> Decoder<'a, T, I>
        where
            I: Iterator<Item = (u8, usize)>,
        {
            pub fn new(decode: &'a [Decode<T>; 256], mut bytes: I) -> Self {
                // Read an initial potentially partial byte to start the process.
                let (pending_byte, pending_bits) = bytes.next().unwrap_or((0, 0));
                Self {
                    decode,
                    bytes,
                    pending_byte: pending_byte.into(),
                    pending_bits,
                }
            }
        }

        impl<'a, T, I> Iterator for Decoder<'a, T, I>
        where
            I: Iterator<Item = (u8, usize)>,
        {
            type Item = &'a T;
            fn next(&mut self) -> Option<&'a T> {
                // We must navigate `self.decode`, restocking bits whenever possible.
                // We stop if ever there are not enough bits remaining.
                let mut map = self.decode;
                loop {
                    if self.pending_bits < 8 {
                        // We only attempt to read from `self.bytes` once, which should work fine as long
                        // as we only have one partial byte at the end, as we are done anyhow in that case.
                        // It means that we *must* read the initial byte when constructing the iterator, to
                        // avoid a partial byte in the first read.
                        if let Some((next_byte, next_bits)) = self.bytes.next() {
                            self.pending_byte = (self.pending_byte << next_bits) + next_byte as u16;
                            self.pending_bits += next_bits;
                        }
                    }

                    if self.pending_bits < 8 {
                        // We have run out of bytes. We may yet be able to decode the remaining bits.
                        // Promote the valid bits and consult the map; if it only consumes valid bits,
                        // we are able to ship the result and advance. If it consumes more bits than
                        // we have, the data are mysteriously invalid.
                        let byte = (self.pending_byte << (8 - self.pending_bits)) as usize;
                        match &map[byte] {
                            Decode::Void => {
                                panic!("invalid decoding map");
                            }
                            Decode::Further(_) => {
                                panic!("malformed data: decode incomplete (Further)");
                            }
                            Decode::Symbol(s, bits) => {
                                if bits <= &self.pending_bits {
                                    self.pending_bits -= bits;
                                    self.pending_byte &= (1 << self.pending_bits) - 1;
                                    return Some(s);
                                } else if self.pending_bits == 0 {
                                    return None;
                                } else {
                                    panic!("malformed data: decode incomplete (Symbol)");
                                }
                            }
                        }
                    }
                    let byte = (self.pending_byte >> (self.pending_bits - 8)) as usize;
                    match &map[byte] {
                        Decode::Void => {
                            panic!("invalid decoding map");
                        }
                        Decode::Symbol(s, bits) => {
                            self.pending_bits -= bits;
                            self.pending_byte &= (1 << self.pending_bits) - 1;
                            return Some(s);
                        }
                        Decode::Further(next_map) => {
                            self.pending_bits -= 8;
                            self.pending_byte &= (1 << self.pending_bits) - 1;
                            map = next_map;
                        }
                    }
                }
            }
        }
    }

    /// A tabled Huffman encoder, written as an iterator.
    mod encoder {

        use std::collections::BTreeMap;

        #[derive(Copy, Clone)]
        pub struct Encoder<'a, T, I> {
            encode: &'a BTreeMap<T, (usize, u64)>,
            symbols: I,
            pending_byte: u64,
            pending_bits: usize,
        }

        impl<'a, T, I> Encoder<'a, T, I> {
            pub fn new(
                encode: &'a BTreeMap<T, (usize, u64)>,
                initially: (u8, usize),
                symbols: I,
            ) -> Self {
                Self {
                    encode,
                    symbols,
                    pending_byte: initially.0 as u64,
                    pending_bits: initially.1,
                }
            }
        }

        impl<'a, T: Ord, I> Iterator for Encoder<'a, T, I>
        where
            I: Iterator<Item = &'a T>,
        {
            type Item = Result<u8, (u8, usize)>;
            fn next(&mut self) -> Option<Result<u8, (u8, usize)>> {
                // We repeatedly ship bytes out of `self.pending_byte`, restocking from `self.symbols`.
                while self.pending_bits < 8 {
                    if let Some(symbol) = self.symbols.next() {
                        let (bits, code) = self.encode.get(symbol).unwrap();
                        self.pending_byte <<= bits;
                        self.pending_byte += code;
                        self.pending_bits += bits;
                    } else {
                        // We have run out of symbols. Perhaps there is a final fractional byte to ship?
                        if self.pending_bits > 0 {
                            let bits = self.pending_bits;
                            let byte = self.pending_byte << (8 - self.pending_bits);
                            self.pending_bits = 0;
                            self.pending_byte = 0;
                            return Some(Err((byte as u8, bits)));
                        } else {
                            return None;
                        }
                    }
                }

                let byte = self.pending_byte >> (self.pending_bits - 8);
                self.pending_bits -= 8;
                self.pending_byte &= (1 << self.pending_bits) - 1;
                Some(Ok(byte as u8))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntoOwned, Push, Region};

    use super::*;

    #[test]
    fn test_huffman() {
        let copy = |r: &mut HuffmanContainer<u8>, item: [u8; 3]| {
            let index = r.push(item);
            println!("{:?}", r.index(index));
            assert_eq!(item.as_slice(), r.index(index).into_owned().as_slice());
        };

        let mut c = HuffmanContainer::<u8>::default();
        copy(&mut c, [1, 2, 3]);
        copy(&mut c, [1, 2, 3]);
        copy(&mut c, [1, 2, 3]);
        copy(&mut c, [1, 2, 3]);
        copy(&mut c, [2, 3, 4]);
        copy(&mut c, [2, 3, 4]);

        let mut c2 = HuffmanContainer::merge_regions([&c].into_iter());
        copy(&mut c2, [1, 2, 3]);
        copy(&mut c2, [1, 2, 3]);
        copy(&mut c2, [1, 2, 3]);
        copy(&mut c2, [1, 2, 3]);
        copy(&mut c2, [2, 3, 4]);
        copy(&mut c2, [2, 3, 4]);

        let mut c3 = HuffmanContainer::merge_regions([&c2].into_iter());
        copy(&mut c3, [1, 2, 3]);
        copy(&mut c3, [1, 2, 3]);
        copy(&mut c3, [1, 2, 3]);
        copy(&mut c3, [1, 2, 3]);
        copy(&mut c3, [2, 3, 4]);
        copy(&mut c3, [2, 3, 4]);
    }
}
