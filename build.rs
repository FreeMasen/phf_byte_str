extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "static KEYWORDS: phf::Map<&'static [u8], Keyword> = ").unwrap();
    phf_codegen::Map::new()
        .entry(b"loop", "Keyword::Loop")
        .entry(b"continue", "Keyword::Continue")
        .entry(b"break", "Keyword::Break")
        .entry(b"fn", "Keyword::Fn")
        .entry(b"extern", "Keyword::Extern")
        .build(&mut file)
        .unwrap();
    write!(&mut file, ";\n").unwrap();
}