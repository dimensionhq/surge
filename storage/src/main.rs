use std::io::{Cursor, Write};

use bson::Document;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: i32,
    email: String,
}

fn main() {
    let point = User {
        name: String::from("Varun Potti"),
        age: 14,
        email: String::from("me@example.com"),
    };
    let now = std::time::Instant::now();

    let bson = bson::to_bson(&point).unwrap();
    let doc = bson.as_document().unwrap();

    let mut buf = Vec::new();
    doc.to_writer(&mut buf).unwrap();

    println!("Serialized: {:?}", buf);
    println!(
        "Serialized in {:#?} micro seconds\n",
        now.elapsed().as_micros()
    );

    let now = std::time::Instant::now();
    // write buf to file
    let mut file = std::fs::File::create("foo.bson").unwrap();
    file.write_all(&buf).unwrap();

    println!(
        "Writing to file took {:#?} micro seconds\n",
        now.elapsed().as_micros()
    );

    let now = std::time::Instant::now();
    let doc = Document::from_reader(&mut Cursor::new(&buf[..])).unwrap();
    println!("Deserialized: {:?}", doc);

    println!(
        "Deserializing took {:#?} micro seconds",
        now.elapsed().as_micros()
    );
}
