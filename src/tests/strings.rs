use crate::ChromaConfig;

#[test]
fn test_strings() {
    let string_simple = "Hello, World!";
    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format(&string_simple)
            .expect("Failed to format simple string")
    );

    // Only single character escape sequences
    let escape_characters_simple = "\n\r\t\\\0\'\"";
    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format(&escape_characters_simple)
            .expect("Failed to format escaped characters")
    );

    // Same as above, but including wide characters
    let escape_characters_wide = "\x41\n\r\t\\\0\'\"\u{1F600}";
    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format(&escape_characters_wide)
            .expect("Failed to format wide escaped characters")
    );
}

#[test]
fn test_chars() {
    let simple = 'a';
    let wide = '\u{1F600}';
    let escaped = '\0';
    let double_quote = '"';

    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format(&simple)
            .expect("Failed to format simple character")
    );

    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format(&wide)
            .expect("Failed to format wide character")
    );

    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format(&escaped)
            .expect("Failed to format escaped character")
    );

    println!(
        "{}",
        ChromaConfig::DEFAULT
            .try_format(&double_quote)
            .expect("Failed to format double quote character")
    );
}
