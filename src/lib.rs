pub fn fibonacci(mut n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    while n > 0 {
        let tem = a;
        a = a + b;
        b = tem;
        n -= 1;
    }
    a
}




#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn test_fib() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(5), 5);
    }
}
