#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! ret_3 {
    () => {
        3
    };
}

fn show_output() {
    println!("I should appear as the output");
}

macro_rules! show_output {
    () => {
        show_output()
    };
}

macro_rules! torf {
    (t) => {
        true
    };

    (f) => {
        false
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ret_3() {
        assert_eq!(3, ret_3!());
    }

    #[test]
    fn test_torf() {
        assert!(torf!(t));
        assert!(!torf!(f));
    }
}
