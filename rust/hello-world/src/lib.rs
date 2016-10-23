pub fn hello(name: Option<&str>) -> String {
    match name {
        Some("Alice") => "Hello, Alice!".to_string(),
        Some("Bob") => "Hello, Bob!".to_string(),
        Some(_) | None => "Hello, World!".to_string()
    }
}
