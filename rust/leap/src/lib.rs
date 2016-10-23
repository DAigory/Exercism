
pub fn is_leap_year(value: i32) -> bool {
    let value = value as f32;
    if (value % 4.0 == 0.0 && value % 100.0 != 0.0) || value % 400.0 == 0.0 {return true ;}
    false
}




