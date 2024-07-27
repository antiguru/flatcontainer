//! Tests that [`Reserve`](crate::Reserve) works as expected.

use flatcontainer::impls::tuple::TupleABCRegion;
use flatcontainer::{MirrorRegion, OwnedRegion, Reserve, SliceRegion, StringRegion, TryPush};

#[test]
fn string_reserve() {
    let mut r = <StringRegion>::default();
    assert_eq!(r.try_push("abc"), Err("abc"));
    r.reserve(&64);
    assert_eq!(r.try_push("abc"), Ok((0, 3)));
}

#[test]
fn tuple_reserve() {
    let mut r = <TupleABCRegion<MirrorRegion<u8>, OwnedRegion<usize>, StringRegion>>::default();
    let item = (8, [1, 2, 3], "abc");
    assert_eq!(TryPush::try_push(&mut MirrorRegion::default(), &1u8), Ok(1));
    assert_eq!(
        TryPush::try_push(&mut <OwnedRegion<_>>::default(), &[1, 2, 3]),
        Err(&[1, 2, 3])
    );
    assert_eq!(
        TryPush::try_push(&mut <StringRegion>::default(), &"abc"),
        Err(&"abc")
    );
    <TupleABCRegion<MirrorRegion<u8>, OwnedRegion<usize>, StringRegion> as TryPush<&(
        u8,
        [usize; 3],
        &str,
    )>>::try_push(&mut r, &item);
    assert_eq!(TryPush::try_push(&mut r, &item), Err(&item));
    assert_eq!(r.try_push(&item), Err(&item));
    r.reserve(&((), 3, 3));
    assert!(r.try_push(&item).is_ok());
}

#[test]
fn slice_reserve() {
    let mut r = <SliceRegion<StringRegion>>::default();
    let item = ["abc", "def", "ghi"];
    assert_eq!(r.try_push(&item), Err(&item));
    r.reserve(&(3, 9));
    assert!(r.try_push(&item).is_ok());
}
