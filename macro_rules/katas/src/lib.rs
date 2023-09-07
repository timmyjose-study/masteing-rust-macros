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

macro_rules! num {
    (one) => {
        1
    };

    (two) => {
        2
    };

    (three) => {
        3
    };
}

macro_rules! do_thing {
    (print $metavar:literal) => {
        println!("{}", $metavar);
    };
}

macro_rules! math {
    ($x:literal plus $y:literal) => {
        $x + $y
    };

    (square $x:literal) => {
        $x * $x
    };
}

macro_rules! emath {
    ($x:expr, plus, $y:expr) => {
        $x + $y
    };

    (square $x:expr) => {
        $x * $x
    };
}

macro_rules! for_2d {
    ($row:ident <$row_ty:ty> in $row_range:expr, $col:ident <$col_ty:ty> in $col_range:expr, $code:block) => {
        for $row in $row_range {
            let $row: $row_ty = $row;
            for $col in $col_range {
                let $col: $col_ty = $col;
                $code
            }
        }
    };
}

macro_rules! listing_literals {
    (the $x:literal and the $y:literal and the $z:literal) => {{
        let mut v = Vec::new();
        v.push($x);
        v.push($y);
        v.push($z);
        v
    }};

    (the $x:literal and the $y:literal) => {{
        let mut v = Vec::new();
        v.push($x);
        v.push($y);
        v
    }};

    (the $x:literal) => {{
        let mut v = Vec::new();
        v.push($x);
        v
    }};
}

macro_rules! listing_literals_rep {
    (the $x:literal $(and the $y:literal)*) => {{
        let mut v = Vec::new();
        v.push($x);

        $(
            v.push($y);
        )*
        v
    }};
}

macro_rules! two_vecs {
    ($($e:expr),* ; $($f:expr),*) => {{
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();

        $(
            v1.push($e);
        )*

            $(
                v2.push($f);
            )*

            (v1, v2)
    }}
}

macro_rules! hashmap {
    ($($k:expr => $v:expr),*$(,)?) => {{
        let mut m = HashMap::new();
        $(
            m.insert($k, $v);
         )*
        m
    }};
}

macro_rules! graph {
    ($($from:literal -> ($($to:literal),*));*$(;)?) => {{
        let mut v = Vec::new();

        $(
            $(
                v.push(($from, $to));
            )*
        )*
        v
    }};
}

macro_rules! sum {
    ($first:expr , $($next:expr),+$(,)?) => {{
        let mut s = $first;

        $(
            s += $next;
        )*
        s
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_ret_3() {
        assert_eq!(3, ret_3!());
    }

    #[test]
    fn test_torf() {
        assert!(torf!(t));
        assert!(!torf!(f));
    }

    #[test]
    fn test_num() {
        assert_eq!(1, num!(one));
        assert_eq!(2, num!(two));
        assert_eq!(3, num!(three));
    }

    #[test]
    fn test_do_thing() {
        do_thing!(print "hello");
        do_thing!(print 1);
        do_thing!(print 1.2345);
        do_thing!(print true);
        do_thing!(print 0b1010);
    }

    #[test]
    fn test_math() {
        assert_eq!(2 + 3, math!(2 plus 3));
        assert_eq!(2 * 2, math!(square 2));
        assert_eq!(2 + 3, math!(2    plus 3));
        assert_eq!(2 * 2, math!( square    2));
    }

    #[test]
    fn test_emath() {
        let var = 5;
        assert_eq!((2 * 3) + 5, emath!((2 * 3), plus, var));
        assert_eq!(5 * 5, emath!(square var));
    }

    #[test]
    fn test_for_2d() {
        #[derive(Debug)]
        struct Coord {
            x: i32,
            y: i32,
        }

        impl Coord {
            fn show(&self) {
                println!("({}, {})", self.x, self.y);
            }
        }

        for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
            (Coord { x: row.into(), y: col.into() }).show();
        })
    }

    #[test]
    fn test_listing_literals() {
        assert_eq!(vec![1.2345], listing_literals!(the 1.2345));
        assert_eq!(
            vec!["lion", "witch", "wardrobe"],
            listing_literals!(the "lion" and the "witch" and the "wardrobe")
        );
        assert_eq!(vec![9, 5], listing_literals!(the 9 and the 5));
    }

    #[test]
    fn test_listing_literals_rep() {
        assert_eq!(vec![1.2345], listing_literals_rep!(the 1.2345));
        assert_eq!(
            vec!["lion", "witch", "wardrobe"],
            listing_literals_rep!(the "lion" and the "witch" and the "wardrobe")
        );
        assert_eq!(vec![9, 5], listing_literals_rep!(the 9 and the 5));
    }

    #[test]
    fn test_two_vecs() {
        assert_eq!(
            (vec![1, 2, 3], vec!["one", "two", "three"]),
            two_vecs!(1, 2, 3; "one", "two", "three")
        );
        assert_eq!(
            (vec![1, 2, 3], vec!["one", "two"]),
            two_vecs!(1, 2, 3; "one", "two")
        );
    }

    #[test]
    fn test_hashmap() {
        fn print_hashmap(m: &HashMap<&str, &str>) {
            println!("{m:#?}");
        }

        let value = "my_string";
        let my_hashmap = hashmap!(
            "hash" => "map",
            "Key" => value
        );
        print_hashmap(&my_hashmap);

        let another_hashmap = hashmap! {
            "mash" => "cash",
        };
        print_hashmap(&another_hashmap);
    }

    #[test]
    fn test_graph() {
        assert_eq!(
            vec![
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 1),
                (2, 3),
                (3, 2),
                (5, 1),
                (5, 2),
                (5, 3)
            ],
            graph!(
                1 -> (2, 3, 4, 5);
                2 -> (1, 3);
                3 -> (2);
                4 -> ();
                5 -> (1, 2, 3);
            )
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(3, sum!(1, 2));
        assert_eq!(3, sum!(1, 2,));
        assert_eq!(15, sum!(1, 2, 3, 4, 5));
    }
}
