# Notes
Safe wrapper for initialising fixed size arrays available from the following crate: https://lib.rs/crates/array-init

The HashMaps also could have been individually initialised via the following syntax:

```rust
let mut quotes: [HashMap<String, String>; 3] = [HashMap::new(), HashMap::new(), HashMap::new()];
// Populate each HashMap with data
```
