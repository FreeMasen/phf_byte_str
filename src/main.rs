
extern crate phf;

#[derive(Clone, Debug)]
enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn parse_keyword(keyword: &[u8]) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}

fn main() {
    let keys = vec![
        b"loop",
        b"continue",
        b"break",
        b"fn",
        b"extern",
    ];
    for key in keys {
        println!("{:?}", parse_keyword(key))
    }
}
