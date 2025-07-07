# Rust Demo for Hashes

---

## Preamble

Run this ASAP to get our deps built before you need them.

```rust
:dep sp-core = { version = "6.0.0", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.25" }
:dep rand = { version = "0.8.5", features = ["small_rng"] }
use sp_core::*;

/// Helper function to compare a subset of bytes from an array.
fn sized_compare<H:std::cmp::PartialEq+?Sized>(a: &H, b: &H) -> bool {
	a == b
}
```

---

## Fixed-Size Outputs

```rust
let short_input_hash = blake2_256(&b"abcd"[..]);
let long_input_hash = blake2_256(&[0u8; 1024][..]);
assert_eq!(short_input_hash.len(), long_input_hash.len());
```

```rust
HexDisplay::from(&short_input_hash)
```

```rust
HexDisplay::from(&long_input_hash)
```

---

## Computation Speed Properties

```rust
use std::time::{Instant};
// Let's give the function 1kb of data to hash
let value_to_hash = [0u8; 1024];

```

```rust
// Blake2
let blake2_start = Instant::now();
for _ in 0..1000 {
	let _ = blake2_256(&value_to_hash[..]);
}
let blake2_elapsed_time = blake2_start.elapsed().as_micros();
```

```rust
// TwoX
let twox_start = Instant::now();
for _ in 0..1000 {
	let _ = twox_256(&value_to_hash[..]);
}
let twox_elapsed_time = twox_start.elapsed().as_micros();
```

```rust
// Compare times for the 1000 rounds we did for each.
// We expect TwoX to be about 10x faster.
// Note this is native hardware, Wasm may be different.
blake2_elapsed_time
```

```rust
twox_elapsed_time
```

---

## Pre-Image Attacks

```rust
use rand::prelude::*;

// We want to find some other pre-image that will give the same hash as "blockchain".
let attack_target: [u8; 32] = blake2_256(b"blockchain");

// Hopefully we could not actually succeed in this! So let's cheat, and truncate the hash to just
// its first few bytes. We'll look for a collision on this truncated hash.
let difficulty = 2;
let mut count = 0u32;
loop {
	count += 1;
	// Generate some random array.
	let x: [u8; 16] = random();
	let x_hash: [u8; 32] = blake2_256(&x[..]);
	if sized_compare(&attack_target[0..difficulty], &x_hash[0..difficulty]) {
		println!("\nSecond pre-image found in {:?} attempts! {:?}", count, x);
		println!("{:?}", &x_hash[0..difficulty]);
		println!("{:?}", &attack_target[0..difficulty]);
		break;
	}

	// Let's not let our CPU run forever and bail. Usually takes about 200k.
	if count == 500_000 {
		println!("\nGiving up on pre-image attack");
		break;
	}
}
```

---

## Collisions

```rust
use rand::prelude::*;

// Here there is no "attack target". The attacker controls both inputs. This should find a collision
// much faster than a second pre-image, and demonstrate the birthday paradox.

// Store the previous hashes to check new ones against.
//                     hash,     value
let mut previous: Vec<([u8; 32], [u8; 16])> = Vec::new();

let mut count = 0u32;
let mut break_loop = false;
let difficulty = 2; // number of bytes to call it a "collision"
loop {
	count += 1;

	// Generate a new value and hash it.
	let x: [u8; 16] = random();
	let x_hash: [u8; 32] = blake2_256(&x[..]);

	// for all previous...
	/*
	Looping through this vec every time and checking each value is obviously really
	inefficient. After the hash-based data structure lesson, perhaps students could code up a
	better solution, like a sorted list with binary search.
	*/
	for hh in &previous {
		// check if the new one matches
		if sized_compare(&x_hash[0..difficulty], &hh.0[0..difficulty]) {
			println!("\nCollision found in {:?} attempts!", count);
			println!("pre-image 1: {:?}", &hh.1);
			println!("pre-image 2: {:?}", x);
			println!("{:?}", &hh.0[0..difficulty]);
			println!("{:?}", &x_hash[0..difficulty]);
			break_loop = true;
			break;
		}
	}
	if break_loop {
		break;
	}

	// if not, add the new one to previous
	previous.push((x_hash, x));

	// bail if it's taking too long
	if count == 100_000 {
		println!("\nGiving up on collision");
		break;
	}
}
```
