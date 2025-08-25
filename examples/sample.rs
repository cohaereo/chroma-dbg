#![allow(unused)]
use bitflags::bitflags;
use chroma_dbg::{ChromaConfig, ChromaDebug};

bitflags! {
    #[derive(Debug)]
    struct TestFlags: u32 {
        const A = (1 << 0);
        const B = (1 << 1);
        const C = (1 << 2);
        const D = (1 << 3);
    }
}

#[derive(Debug)]
struct Foo {
    t: Baz,
    uu: Vec<NameAndAge>,
    u: Vec<NameAndAge>,
    v: Option<bool>,
    w: Bar,
    x: i32,
    y: f32,
    z: String,
    alpha: Vec<bool>,
    flags: TestFlags,
}

#[derive(Debug)]
struct Bar(bool, i32, String);

#[derive(Debug)]
struct Baz {
    a: i32,
    b: f32,
}

#[derive(Debug)]
struct NameAndAge {
    name: String,
    age: i32,
}

impl NameAndAge {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let foo = Foo {
        t: Baz { a: 42, b: 3.14 },
        uu: vec![NameAndAge::new("Bob", 34)],
        u: vec![
            NameAndAge::new("Bob", 34),
            NameAndAge::new("Alice", 22),
            NameAndAge::new("Eve", 32),
            NameAndAge::new("Mallory", 44),
            NameAndAge::new("Trent", 32),
            NameAndAge::new("Carol", 58),
            // Longer lines will be inlined by default. This can be tweaked with a custom config.
            NameAndAge::new("Foo bar spam bacon eggs and spam, spam spam spam", 67),
        ],
        w: Bar(true, -1337, "Hello, world!".to_string()),
        v: None,
        x: 8193, // Default hex threshold is 8192, so this will be printed as hex
        y: 3.14,
        z: "Hello, world!".to_string(),
        alpha: vec![true, false, true],
        flags: TestFlags::all(),
    };

    if let Err(e) = ChromaConfig::DEFAULT.try_format(&foo) {
        eprintln!("Error parsing debug output: {e:?}");
        return;
    }

    println!("STD (normal)");
    println!("{:?}", foo);
    println!();

    println!("STD (\"pretty\")");
    println!("{:#?}", foo);
    println!();

    println!("Chroma (default)");
    println!("{}", foo.dbg_chroma());
    println!();

    // Custom configs can be used to tweak colors/inlining/integer formatting
    println!("Chroma (custom)");
    println!(
        "{}",
        ChromaConfig {
            identifier_color: chroma_dbg::Color(255, 0, 255),
            ..ChromaConfig::COMPACT
        }
        .format(&foo)
    );
}
