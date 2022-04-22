# tournament

An implementation of a `k`-way merge iterator using a binary heap.
The `k`-way merge iterator is very useful when given `k` sets
of sorted data, you want to find the `n` top elements in between
the sets in an efficient way, without sorting the entire data set.

Imagine having dozens of slices with hundreds of elements, where
you only care about the top 10.

```rust
use tournament::Tournament;

let t = Tournament::from_iters_min([(1..2000), (1..20000), (1..5000000)]);
assert_eq!(t.take(5).collect::<Vec<_>>(), [1, 1, 1, 2, 2]);

```

License: MIT OR Apache-2.0
