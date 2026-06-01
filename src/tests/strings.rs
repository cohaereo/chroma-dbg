use crate::ChromaConfig;

#[test]
fn test_strings() {
    let string_simple = "Hello, World!";
    println!("{}", ChromaConfig::DEFAULT.format(&string_simple));

    // Only single character escape sequences
    let escape_characters_simple = "\n\r\t\\\0\'\"";
    println!(
        "{}",
        ChromaConfig::DEFAULT.format(&escape_characters_simple)
    );

    // Same as above, but including wide characters
    let escape_characters_wide = "\x7F\n\r\t\\\0\'\"\u{3F600}";
    println!("{}", ChromaConfig::DEFAULT.format(&escape_characters_wide));
}

#[test]
fn test_chars() {
    let simple = 'a';
    let wide = '\u{1F600}';
    let escaped = '\0';
    let double_quote = '"';
    let single_quote = '\'';

    println!("{}", ChromaConfig::DEFAULT.format(&simple));

    println!("{}", ChromaConfig::DEFAULT.format(&wide));

    println!("{}", ChromaConfig::DEFAULT.format(&escaped));

    println!("{}", ChromaConfig::DEFAULT.format(&double_quote));

    println!("{}", ChromaConfig::DEFAULT.format(&single_quote));
}
