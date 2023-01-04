# Non Blank String type

Non blank string type for approach suggested by Alexis King - ["Parse, don't validate"](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/).

## How to use:

**Add dependency:**

```toml
non-blank-string-rs = { git = "https://github.com/lebe-dev/non-blank-string-rs", version = "1.0.1" }
```

**Create type:**

```rust
let username = NonBlankString::from_str("Hellow")?;

let username: NonBlankString = "Hellow".parse()?;
```

**Explicit usage:**

```rust
let username: NonBlankString = "Hellow".parse()?;

login(username.as_str())
```

**Implicit usage:**

```rust
let username: NonBlankString = "Hellow".parse()?;

login(&username)
```

## Util functions

Add to `Cargo.toml`:

```toml
[dev-dependencies]
non-blank-string-rs = { git = "https://github.com/lebe-dev/non-blank-string-rs", version = "1.0.1", features = ["utils"] }
```

Functions:

- `get_random_nonblank_string()` - return random `NonBlankString`. Useful for tests.


## Thanks

- Alexis King, article - [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- Justin Wernick, article - [The Newtype Pattern In Rust](https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)
