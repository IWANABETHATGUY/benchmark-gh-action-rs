pub fn fibonacci(n: i32) -> i32 {
    for i in 10..1000 {}
    n
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fibonacci(2), 0);
    }
}
