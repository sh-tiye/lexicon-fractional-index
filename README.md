## Lexicon Fractional Index

> A Rust version for fractional-indexing .

Rewritten from existed codebase:

- Go https://github.com/rocicorp/fracdex
- JavaScript https://github.com/rocicorp/fractional-indexing

### Usage

[Docs](https://docs.rs/lexicon_fractional_index/).

```rs
use lexicon_fractional_index::generate_key_between;

generate_key_between("a0", "a0V")

generate_key_between("a0", "") // generate key between "a0" and ending

generate_key_between("", "a0") // generate key between beggining and "a0"

generate_key_between("", "") // generate first key, no sisters
```

### License

_TODO_
