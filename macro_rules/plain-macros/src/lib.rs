#[macro_export]
macro_rules! my_vec {
    // handle the empty case.
    () => {
        {
            Vec::new()
        }
    };

    // handle an optional trailing comma
    ( $( $x:expr ),* $(,)*) => {
        {
        let mut v = Vec::new();

        $(
            v.push($x);
        )*
        v

        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_vec() {
        let empty_left: Vec<i32> = my_vec![];
        let empty_right: Vec<i32> = vec![];

        assert_eq!(empty_left, empty_right);
        assert_eq!(my_vec![1], vec![1]);
        assert_eq!(my_vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]);
        assert_eq!(my_vec![1, 2, 3, 4, 5,], vec![1, 2, 3, 4, 5,]);
    }
}
