use crate::ChromaConfig;

#[test]
fn vdf_vmt() {
    let raw_debug = include_str!("vdf_vmt.txt");
    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format_string(raw_debug)
            .expect("Failed to format string")
    );
}
