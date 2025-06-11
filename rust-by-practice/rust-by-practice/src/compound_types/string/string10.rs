pub fn main() {
    // Corrected: This is a normal string (not raw) because we want escapes to work
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // Correct raw string with quotes inside
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // Correct raw string with "# inside
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Correct raw string with "##" inside
    let long_delimiter = r####"Hello, "##""####;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
