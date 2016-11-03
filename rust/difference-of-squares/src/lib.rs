
pub fn square_of_sum(value: i32) -> i32 {
    square_of_sum_re(value, 0)
}

pub fn sum_of_squares(value: i32) -> i32 {
   sum_of_squares_re(value, 0)
}

pub fn difference(value: i32) -> i32 {
    square_of_sum(value) - sum_of_squares(value)
}

fn sum_of_squares_re(value: i32, result: i32) -> i32 {
    if value > 0 {
        return sum_of_squares_re(value - 1, result + (value) * (value))
    }
    result
}

fn square_of_sum_re(value: i32, result: i32) -> i32 {
    if value > 0 {
        return  square_of_sum_re(value - 1, result + value)
    }
    result * result
}




