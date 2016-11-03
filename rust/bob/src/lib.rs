pub fn reply(value: &str) -> String {

    if value.is_empty() {
        "Fine. Be that way!".to_string()
    }
    else if test_upper_alphabetic(value) {
        "Whoa, chill out!".to_string()
    }
    else if test_question(value) {
        "Sure.".to_string()
    }
    else {
        "Whatever.".to_string()
    }
}

fn test_upper_alphabetic(x : &str) -> bool {
    x.chars().all(|x| !x.is_alphabetic() || x.is_uppercase())
}

fn test_question(x: &str) -> bool {
    x.chars().last().map_or(false, |y| y == '?')
}
