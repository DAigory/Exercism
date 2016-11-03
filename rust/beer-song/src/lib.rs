pub fn verse(count: i32) -> String {
    get_phrase(count)
}

pub fn sing(a: i32, b: i32) -> String {

    let mut sing = "".to_string();
    for x in get_sequence(b, a) {
        if x != b {
            sing+= "\n";
        }
        sing+= &get_phrase(b-x+a);
    }
    sing
}

fn get_sequence(a: i32, b: i32) -> Vec<i32> {
    if a < b {
        (a .. b + 1).collect()
    } else {
        (b .. a + 1).rev().collect()
    }
}

fn get_phrase(count: i32) -> String {
    get_first(count) +
    " of beer on the wall, " +
    &get_second(count) +
    " of beer.\n" +
    &get_third(count) +
    ", " +
    &get_fourh(count) +
    " of beer on the wall.\n"
}


fn get_first(count: i32) -> String {
   some_kind_of_uppercase_first_letter(&get_second(count))
}

fn get_second(count: i32) -> String {
    match count {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        x =>  x.to_string()+(" bottles")
    }
}

fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_third(count: i32) -> String {
    match count {
        0 => "Go to the store and buy some more".to_string(),
        1 => "Take it down and pass it around".to_string(),
        _ => "Take one down and pass it around".to_string()
    }
}

fn get_fourh(count: i32) -> String {
     match count {
        0 => "99 bottles".to_string(),
        1 => "no more bottles".to_string(),
        2 => "1 bottle".to_string(),
        x => (x - 1).to_string()+(" bottles")
    }
}

