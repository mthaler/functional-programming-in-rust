fn factorial(n: i64) -> i64 {
    fn go(n: i64, acc: i64) -> i64 {
        if n <= 0 { acc } else { go (n-1, n*acc) }
    }

    go(n, 1)
}

fn format_result<F>(name: &str, n: i64, f: F) -> String where F: Fn(i64) -> i64 {
    format!("The {} of {} is {}", name, n, f(n))
}

fn find_first<T, P>(v: &Vec<T>, p: P) -> i32 where P: Fn(&T) -> bool {
    for (i, x) in v.iter().enumerate() {
        if p(x) {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::{factorial, format_result, find_first};

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_format_result() {
        assert_eq!(format_result("factorial", 5, factorial), "The factorial of 5 is 120");
        assert_eq!(format_result("absolute value", -5, |n: i64| if n >= 0 { n } else { -n }), "The absolute value of -5 is 5");
    }

    #[test]
    fn test_find_first() {
    
        assert_eq!(find_first(&vec![1, 10, 100], |x| *x == 10), 1);
    }
}
