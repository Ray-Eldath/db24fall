use std::fs;

mod de;
mod ser;

fn main() {
    let content = fs::read_to_string("pubmed24n0001.xml").unwrap();
    de::serde(&content);
}
