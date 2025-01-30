#![allow(unused)]

fn main() {
    use chroma_dbg::ChromaDebug;

    #[derive(Debug)] // Regular std Debug derive macro, no tricks
    struct MyStruct {
        field1: i32,
        field2: String,
        field3: Vec<i32>,
    }

    let my_struct = MyStruct {
        field1: 42,
        field2: "Hello, World! How are you on this fine day?".to_string(),
        field3: vec![1, 2, 3],
    };

    // Print the struct like normal
    println!("{:#?}", my_struct);

    // Print the struct with syntax highlighting
    println!("{}", my_struct.dbg_chroma());
}
