# readall

A simple CLI application for reading all files in a dir using a glob and having then pretty outputed.

## Example

```bash
# Read all Rust source files (use quotes to prevent shell expansion)
readall 'src/**/*.rs'

# Read multiple patterns
readall 'src/**/*.rs' 'README.md' 'Cargo.toml'
```

Output format:
```
src/main.rs
────────────────────────────────────────
use std::fs;

fn main() {
    println!("Hello, world!");
}
════════════════════════════════════════
```
