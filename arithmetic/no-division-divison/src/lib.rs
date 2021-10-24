fn div(divisor: i32, quotient: i32) -> i32 {
    let mut result = 0;
    let mut working_total = divisor;
    while working_total >= quotient {
        working_total = working_total - quotient;
        result = result + 1;
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(div(4, 2), 2);
        assert_eq!(div(8, 2), 4);
        assert_eq!(div(4, 1), 4);
        assert_eq!(div(7, 3), 2);
    }
}
