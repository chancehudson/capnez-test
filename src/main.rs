use capnez_macros::capnp;
use capnez_codegen::capnp_include;
use serde::{Serialize, Deserialize};

#[capnp]
#[derive(Serialize, Deserialize, PartialEq)]
struct Things {
    gloves: u32,
    name: String,
    is_something: bool,
}

fn main() {
    let things = Things {
        gloves: 2,
        name: "something".to_string(),
        is_something: true,
    };

    let encoded = bincode::serialize(&things).unwrap();
    let decoded = bincode::deserialize(&encoded[..]).unwrap();
    assert!(things == decoded);
    println!("Encoded and decoded from {} bytes!", encoded.len());
}
