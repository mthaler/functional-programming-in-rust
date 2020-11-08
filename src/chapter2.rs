fn factorial(n: i64) -> i64 {
    fn go(n: i64, acc: i64) -> i64 {
        if n <= 0 { acc } else { go (n-1, n*acc) }
    }

    go(n, 1)
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }
}
