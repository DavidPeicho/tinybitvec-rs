# TinyBitVec

Tiny vector of bits using a `u32` slice as storage.

## Example

Basic example:

```rust
use tinybitvec::BitVec;

// 10 bits initialized to false
let mut bits = BitVec::new(10, false);
bits.push(false);
bits.push(true);
bits.push(false);

println!("{:?}", bits[1]); // false
println!("{:?}", bits.get(1)); // Some(false)

bits.set(1);
println!("{:?}", bits[1]); // true
```

Immutable and mutable slice types:

```rust
use tinybitvec::BitVec;

let bits = BitVec::from(&[false, true, false, false][..]);

let slice = bits.as_slice();
println!("{:?}", slice.len()); // 4

let slice = slice.slice(0..3);
println!("{:?}", slice.iter().collect::<Vec<bool>>()); // [false, true, false]
```
