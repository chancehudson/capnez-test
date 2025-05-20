fn main() {
    println!("{}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    capnez_codegen::generate_schema().expect("Failed to generate schema");
} 

