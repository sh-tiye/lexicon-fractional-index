## Lexicon Fractional Index

> A Rust version for fractional-indexing . Keys in string and satisfy lexicographic order.

Rewritten from existed codebase:

- Go https://github.com/rocicorp/fracdex
- JavaScript https://github.com/rocicorp/fractional-indexing

### Usage

[Docs](https://docs.rs/lexicon_fractional_index/).

```rs
use lexicon_fractional_index::key_between;

// create an initial key
let k0 = key_between(&None, &None)?;

// two keys, notice that some strings are not valid keys
let left = "Xb0M".to_owned();
let right = "Xb0M0V".to_owned();

// new key at beginning
let next = key_between(&None, &Some(right.to_owned()))?;

// new key at end
let next = key_between(&Some(left.to_owned()), &None)?;

// new key between 2 keys
let next = key_between(&Some(left.to_owned()), &Some(right.to_owned()))?;
```

### License

_TODO_
