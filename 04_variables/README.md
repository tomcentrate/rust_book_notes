## Rust Variables
We cover the differences between mutable, immutable and shadowed variables.

### Mutability
variables are immutable by default.

```rust
let x = 10;
//This will cause a compilation error.
x = 11;
```

To Make it mutable, use the `mut` keyword.

```rust
let mut x = 10;
x = 11;
```

### Shadowing

Shadowing creates a brand new variable, using the same name.

You can shadow a variable by redefining it with the `let` keyword.

```rust
let x = 10;

let x = 'Ten';
```
