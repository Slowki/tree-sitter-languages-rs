# `tree-sitter-languages`
A library to download and link [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) [grammars](https://tree-sitter.github.io/tree-sitter/using-parsers).

## Usage
### Example Crate
For an example, see the example crate in [`./example`](./example)

### `Cargo.toml`
Add this crate to your dependencies
```toml
tree-sitter = "<SOME_VERSION_HERE>"

# Replace the features array with an array of whichever grammars you want to enable
tree-sitter-languages = { git = "https://github.com/Slowki/tree-sitter-languages-rs.git", features = ["rust", "python"] }
```

### Feature Flags
This crate has 3 features flags for each supported language:
* `<name>-latest` (example: `rust-latest`): Use the latest tagged version for a given grammar
* `<name>-master` (example: `rust-master`): Use the master branch for a given grammar
* `<name>` (example: `rust`): An alias for `<name>-latest`

You can also enable every language by using the `all` feature, which enables the `<name>-latest` feature for each grammar.

### Using the Grammars
```rust
use tree_sitter_languages::{python_is_available};

fn main() {
    //...

    // A <name>_is_available macro is generated for each grammar
    if python_is_available!() {
        // A get_<name> function is generated for each grammar, it returns a normal Tree-sitter
        // Language object which can be used by tree_sitter::Parser
        let language: tree_sitter::Language = tree_sitter_languages::get_python();
        // ...
    }

    //...
}
```

