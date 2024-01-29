flatcontainer
=======

A flat container for Rust.

```toml
[dependencies]
flatcontainer = "0.1"
```

## Example

```rust
use flatcontainer::{ResultContainer, CopyOnto};
fn main() {
  let r: Result<_, u16> = Ok("abc");
  let mut c = ResultContainer::default();
  let idx = r.copy_onto(&mut c);
  assert_eq!(r, c.index(idx));
}
```

## Details

TODO

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
