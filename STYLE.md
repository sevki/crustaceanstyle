# CrustaceanStyle

CrustaceanStyle adapts the ideas behind TigerStyle to Rust rather than translating Zig rules
literally.

The priorities are:

1. Safety.
2. Performance.
3. Developer experience.

In that order.

## Safety

### Bound everything

Queues, buffers, retries, work per tick, and externally controlled inputs should have explicit
limits. A limit is part of the design, not an implementation detail.

### Distinguish programmer errors from operating errors

Programmer errors violate invariants and should fail loudly with assertions.

Operating errors are expected outcomes from the environment and should be represented with
`Result` or another explicit type.

### Use domain integers

Use explicitly sized integers for domain data.

Use `usize` only where Rust requires an address-space-sized value, such as indexing a slice.
Convert at that boundary.

Do not use the same primitive integer type to mean index, count, and byte size when those values can
be confused.

### Avoid unchecked arithmetic

Use checked arithmetic where overflow would violate an invariant. Make rounding behavior explicit.

### Assert invariants

Assert preconditions, postconditions, and internal invariants where doing so increases confidence.

### Minimize unsafe Rust

Safe Rust is the default. Unsafe code must sit behind a safe abstraction, have a precise safety
contract, and be small enough to audit in isolation.

## Performance

Think about network, disk, memory, and CPU costs during design. Prefer batching over reacting to every
external event individually.

Avoid hidden allocation, unnecessary copies, and abstractions that obscure work in hot paths.

## Developer experience

Names are design. Prefer domain names and units such as `latency_ms_max` and `size_bytes`.

Keep control flow explicit, scopes small, warnings at zero, and comments focused on why.

## Formatting

Run `cargo fmt`. The repository uses a 100-column limit.

## Dependencies

Dependencies have supply-chain, compile-time, binary-size, correctness, and maintenance costs. A
dependency must earn its place.
