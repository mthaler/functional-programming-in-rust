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

    fn from(elems: Vec<T>) -> List<T> {
        let mut l = List::new();
        for elem in elems {
            l = l.append(elem);
        }
        l
    }

    fn append(self, item: T) -> Self {
        List::Cons(item, Box::new(self))
    }

    fn tail(self) -> Self {
        match self {
            List::Nil => List::Nil,
            List::Cons(_, tail) => *tail,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            List::Nil => true,
            List::Cons(_, _) => false
        }
    }

    fn drop(self, n: i32) -> Self {
        if n <= 0 {
            self
        } else {
            let mut i = 1;
            let mut t = self.tail();
            while !t.is_empty() && i < n {
                t = t.tail();
                i += 1;
            }
            t
        }
    }
}

impl<T: Copy> List<T> {
    fn drop_while<P>(self, p: P) -> Self where P: Fn(T) -> bool {
        match self {
            List::Nil => self,
            List::Cons(h, t) => if p(h) {
                t.drop_while(p)
            } else {
                List::Cons(h, t)
            }
        }
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
        let l = List::from(vec![1, 2, 3, 4]);
        assert_eq!(l.product(), 24);
    }

    #[test]
    fn test_tail() {
        let l = List::from(vec![1, 2, 3, 4]);
        assert_eq!(format!("{}", l), "[4 3 2 1 ]");
        let tail = l.tail();
        assert_eq!(format!("{}", tail), "[3 2 1 ]");
    }

    #[test]
    fn test_is_empty() {
        let l0: List<i32> = List::new();
        assert!(l0.is_empty());
        let l1 = List::from(vec![1, 2, 3, 4]);
        assert!(!l1.is_empty());
    }

    #[test]
    fn test_is_drop() {
        let l = List::from(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(format!("{}", l), "[6 5 4 3 2 1 ]");
        let l2 = l.drop(2);
        assert_eq!(format!("{}", l2), "[4 3 2 1 ]");
    }

    #[test]
    fn test_is_drop_while() {
        let l = List::from(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(format!("{}", l), "[6 5 4 3 2 1 ]");
        let l2 = l.drop_while(|x| x > 4);
        assert_eq!(format!("{}", l2), "[4 3 2 1 ]");
    }
}