pub fn fibonacci(mut n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    while n > 0 {
        a = a + b;
        b = a - b;
        n -= 1;
    }
    a
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fibonacci(2), 0);
    }
}
