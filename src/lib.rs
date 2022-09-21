pub fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fibonacci(2), 0);
    }
}
