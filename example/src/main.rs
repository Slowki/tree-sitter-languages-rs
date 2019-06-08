use tree_sitter_languages::{python_is_available, rust_is_available};

static PYTHON_SOURCE: &'static str = "from foo import bar\nprint([x for x in y])";
static RUST_SOURCE: &'static str = include_str!("main.rs");

fn main() {
    let mut parser = tree_sitter::Parser::new();

    if rust_is_available!() {
        println!("Hooray, Rust is available!");

        parser
            .set_language(tree_sitter_languages::get_rust())
            .unwrap();
        let tree = parser.parse(RUST_SOURCE, None).unwrap();
        let root_node = tree.root_node();

        println!("{}", root_node.to_sexp());
    } else if python_is_available!() {
        println!("Hooray, Python is available!");
        parser
            .set_language(tree_sitter_languages::get_python())
            .unwrap();
        let tree = parser.parse(PYTHON_SOURCE, None).unwrap();
        let root_node = tree.root_node();

        println!("{}", root_node.to_sexp());
    }
}
