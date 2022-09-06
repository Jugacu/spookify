pub(crate) fn ask(question: &str) -> String {
    println!("> {}", question);

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer).expect("Failed to parse stdin");

    buffer.trim().to_string()
}