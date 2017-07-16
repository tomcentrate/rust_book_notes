Function declarations look like:

```rust

fn function_name(param1: type, param2: type) {
}
```

You can also have a function have a return type via this arrow notation.
To return a variable, you can either use the `return` keyword or the last line in the function returns a value, like Ruby.

```rust
fn is_numeric(number: i32) -> bool {
  return true;
}

// also acceptable
fn is_numeric(number: i32) -> bool {
  true
}
```


Statements vs Expressions:

Statements - Do stuff and does not return value.

Expressions - Evaluate to a certain value.