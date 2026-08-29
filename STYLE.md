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

Prefer bounded data structures when the maximum is knowable.

### Distinguish programmer errors from operating errors

Programmer errors violate invariants and should fail loudly with assertions.

Operating errors are expected outcomes from the environment and should be represented with
`Result` or another explicit type.

Do not turn invariant violations into recoverable errors merely to keep a process alive.

### Use domain integers

Use explicitly sized integers for domain data.

Use `usize` only where Rust requires an address-space-sized value, such as indexing a slice or
interacting with an API expressed in `usize`. Convert at that boundary.

Do not use the same primitive integer type to mean index, count, and byte size when those values can
be confused.

### Avoid unchecked arithmetic

Use checked arithmetic where overflow would violate an invariant.

Make rounding behavior explicit. Prefer operations whose names make exact, floor, or ceiling
semantics clear.

### Assert invariants

Assert preconditions, postconditions, and internal invariants where doing so increases confidence in
the code.

Prefer simple assertions over compound assertions when separate failures provide better diagnostics.

### Minimize unsafe Rust

Safe Rust is the default.

Unsafe code must exist behind a safe abstraction, have a precise safety contract, and be small enough
to audit in isolation.

## Performance

Think about network, disk, memory, and CPU costs during design.

Prefer batching over reacting to every external event individually.

Keep hot loops simple. Avoid hidden allocation, unnecessary copies, and abstractions that obscure
work in performance-sensitive paths.

Measure after implementation, but do not postpone basic mechanical-sympathy decisions until then.

## Developer experience

### Names are design

Use names that describe the domain rather than the implementation.

Prefer `latency_ms_max` over `max_latency_ms`, and `size_bytes` over `size`.

Avoid abbreviations unless they are established domain terminology.

### Keep control flow obvious

Prefer explicit control flow over clever control flow.

Keep branching near the function that owns the decision and move branch-free computation into small
helpers.

Avoid recursion in code where a bound on work matters.

### Keep scopes small

Introduce values close to where they are checked and used.

Do not create aliases to mutable state merely for convenience.

### Treat warnings as errors

Code should compile without warnings under the configured Rust and Clippy lint sets.

### Explain why

Comments should capture rationale, invariants, and surprising constraints. Do not restate code in
prose.

## Formatting

Run `cargo fmt`.

The repository uses a 100-column limit.

## Dependencies

Dependencies have supply-chain, compile-time, binary-size, correctness, and maintenance costs.

A dependency must earn its place. Prefer the standard library when it provides a clear and adequate
solution.
