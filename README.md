## Lexicon Fractional Index

> A Rust version for https://www.npmjs.com/package/fractional-index .

> Keys are based on string, and fit with the string order.

### Usage

```rs
use lexicon_fractional_index::fractional_index;

fractional_index("a", "b")
// Result("aP")
```

### Todo Next

- deal with fragile behavior of `fractional_index("", "")`
- use `Option<T>` to reduce chaos brought by `null` and `""` from JavaScript.

### License

_TODO_
