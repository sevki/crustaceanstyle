# `CrustaceanStyle`

A safety-, performance-, and developer-experience-oriented coding style for Rust, inspired by
`TigerStyle`.

The priorities are:

1. **Safety**
2. **Performance**
3. **Developer experience**

In that order.

`CrustaceanStyle` is not a mechanical port of `TigerStyle`. Rust and Zig have different semantics,
so the rules are adapted where ownership, borrowing, `Result`, `usize`, allocation, iterators,
and unsafe code change the trade-offs.

See [STYLE.md](STYLE.md) for the style guide.

## Crate

The crate starts with small types and compile-time tools that make common rules enforceable:

```rust
use crustaceanstyle::{invariant, Count, Index, SizeBytes};

const MESSAGE_SIZE_MAX: u32 = 64 * 1024;
const FRAME_SIZE_MAX: u32 = 128 * 1024;

invariant!(MESSAGE_SIZE_MAX <= FRAME_SIZE_MAX);

let index = Index::new(0);
assert_eq!(index.to_count(), Some(Count::new(1)));

let payload_size = SizeBytes::new(4096);
assert_eq!(payload_size.get(), 4096);
```

The library is intentionally dependency-free.
