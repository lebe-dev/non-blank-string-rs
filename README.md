# Non Blank String type

Non blank string type for approach suggested by Alexis King - ["Parse, don't validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/).

## How to use:

Add dependency:

```toml
non-blank-string-rs = { git = "https://github.com/lebe-dev/non-blank-string-rs", version = "1.0.0", features = ["utils"] }
```

Use:

```rust
let username = NonBlankString::parse("Hellow")?;
```

## Util functions

Add to `Cargo.toml`:

```toml
[dev-dependencies]
non-blank-string-rs = { git = "https://github.com/lebe-dev/non-blank-string-rs", version = "1.0.0", features = ["utils"] }
```

Functions:

- `get_random_nonblank_string()` - return random `NonBlankString`. Useful for tests.

## Limitations

Do not use `NonBlankString` for incoming structs, i.e.:

```rust
pub struct LoginRequest {
    pub username: NonBlankString,
    pub password: NonBlankString
}
```