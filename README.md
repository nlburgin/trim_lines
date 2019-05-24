# trim_lines

**NOTE**: This crate is **deprecated**. You should be using `map(str::trim)` instead:

```rust
fn main() {
    let text = "    foo    \r\n    bar    \n   \n    baz    \n";
    let mut lines = text.lines().map(str::trim);
    assert_eq!(Some("foo"), lines.next());
    assert_eq!(Some("bar"), lines.next());
    assert_eq!(Some(""), lines.next());
    assert_eq!(Some("baz"), lines.next());
    assert_eq!(None, lines.next());
}
```

An extremely simple and tiny library which provides an iterator over the lines of a string, trimmed of whitespace. It is a simple wrapper around the Lines iterator in std::str which trims the whitespace from each line.
