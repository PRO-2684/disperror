# disperror

`Display`ing errors instead of `Debug`ging them when returned from `main`.

## Usage

Simply wrap your error type `MyError` in a `DispError`:

```diff
- fn main() -> Result<(), MyError> {
+ use disperror::DispError;
+ fn main() -> Result<(), DispError<MyError>> {
```

Note that `MyError` must implement `std::error::Error`.

## Example

```rust should_panic
use disperror::DispError;

fn main() -> Result<(), DispError<std::io::Error>> {
    let contents = std::fs::read_to_string("nonexistent_file.txt")?;
    println!("{}", contents);
    Ok(())
}
```

Should `Display` the following error message if that file does not exist:

```text
Error: No such file or directory (os error 2)
```

Instead of the usual `Debug` output:

```text
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

## Implementation

The `DispError` type is a simple wrapper around an error type `E` that implements `std::error::Error`:

```rust
use std::error::Error;

pub struct DispError<E: Error> {
    error: E,
}
```

The `Debug` implementation of `DispError` forwards to the `Display` implementation of the inner error:

```rust
use std::{error::Error, fmt::Debug};
#
# pub struct DispError<E: Error> {
#     error: E,
# }

impl<E: Error> Debug for DispError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
```

In addition, `DispError` implements `From<E>` for implicit conversion:

```rust
# use std::{error::Error, fmt::Debug};
#
# pub struct DispError<E: Error> {
#     error: E,
# }

impl<E: Error> From<E> for DispError<E> {
    fn from(error: E) -> Self {
        Self { error }
    }
}
```

In this way, when an error of type `E` is returned from `main`, it is automatically converted to a `DispError<E>`. When the `Err` variant of a `Result` is returned from `main`, the `Debug` implementation is used to print the error message, thus forwarding to the `Display` implementation of the inner error.

## Notes

This project is heavily inspired by [`main_error`](https://docs.rs/main_error). If you're working with `Box<dyn std::error::Error>` in your `main` function, use [`main_error`](https://docs.rs/main_error) instead. Here's a quick comparison:

|                    | `disperror`    | `main_error` |
| ------------------ | -------------- | ------------ |
| Library size       | Tiny           | Small        |
| Overhead           | None           | Negligible   |
| Dynamic dispatch   | ✘              | ✔           |
| Usage              | `DispError<E>` | `MainError`  |
