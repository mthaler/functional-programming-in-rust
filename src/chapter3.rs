use std::fmt;
use num;

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List::Nil
    }

    fn append(self, item: T) -> Self {
        List::Cons(item, Box::new(self))
    }
}

impl<T> fmt::Display for List<T> where T: fmt::Display {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");
        let mut node = self;
        while let List::Cons(ref head, ref  tail) = *node {
            write!(f, "{} ", head);
            node = tail;
        }
        write!(f, "]")
    }
}

impl<T> List<T> where T: std::ops::Mul + num::One + Copy {
    fn product(&self) -> T {
        match self {
            List::Nil => num::one(),
            List::Cons(head, tail) => *head * tail.product(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::List;

    #[test]
    fn test_append() {
        let l = List::new();
        l.append(42);
    }

    #[test]
    fn test_display() {
        let mut l = List::new();
        l = l.append(42);
        l = l.append(100);
        l = l.append(7);
        assert_eq!(format!("{}", l), "[7 100 42 ]");
    }

    #[test]
    fn test_product() {
        let mut l = List::new();
        l = l.append(2);
        l = l.append(3);
        l = l.append(4);   
        assert_eq!(l.product(), 24);
    }
}