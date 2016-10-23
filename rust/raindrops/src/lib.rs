pub fn raindrops(value: i32) -> String {
    let mut result = String::new();
    if value % 3 == 0 {
        result.push_str("Pling");
    }
    if value % 5 ==0 {
        result.push_str("Plang")
    }
    if value % 7 ==0 {
        result.push_str("Plong")
    }
    if result == "".to_string() {return value.to_string();}
    else
    {return result;}
}
