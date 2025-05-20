use capnez_macros::capnp;
use capnez_codegen::capnp_include;
use serde::{Serialize, Deserialize};

capnp_include!();

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

    // bincode serialization
    let encoded = bincode::serialize(&things).unwrap();
    let decoded = bincode::deserialize(&encoded[..]).unwrap();
    assert!(things == decoded);
    println!("bincode encoded and decoded from {} bytes!", encoded.len());

    // capnp serialization
    let mut message = capnp::message::Builder::new_default();
    let mut builder = message.init_root::<schema_capnp::things::Builder>();
    builder.set_gloves(things.gloves);
    builder.set_name(things.name.clone());
    builder.set_is_something(things.is_something);

    let encoded = capnp::serialize::write_message_to_words(&message);

    let reader = capnp::serialize::read_message_from_flat_slice(&mut &encoded[..], capnp::message::ReaderOptions::default()).unwrap();
    let root = reader.get_root::<schema_capnp::things::Reader>().unwrap();
    let decoded = Things {
        gloves: root.get_gloves(),
        name: root.get_name().unwrap().to_string().unwrap(),
        is_something: root.get_is_something(),
    };
    assert!(things == decoded);
    println!("capnp encoded and decoded from {} bytes!", encoded.len());
}
