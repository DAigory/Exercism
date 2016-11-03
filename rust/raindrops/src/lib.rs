pub fn raindrops(value: i32) -> String {
    f(value, &[(3, "Pling"), (5, "Plang"), (7, "Plong")]).unwrap_or(value.to_string())

}

fn f(value : i32, dividers : &[(i32, &str)]) -> Option<String> {
    dividers.split_first().and_then(|(&(divider, name), tail)| 
        if value % divider == 0 { 
            Some(name.to_string() + &f(value, tail).unwrap_or("".to_string())) 
        } else { 
            None 
        })
}
