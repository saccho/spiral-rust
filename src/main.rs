// use std::env;
// use spiral_rust::input;
// use spiral_rust::cp2::alds2_1_1_d;
// use spiral_rust::cp3::alds3_1_1_a;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     if args.len() < 2 {
//         panic!("A problem number is required as an argument. (e.g.: $ cargo run cp2_1_1_d)")
//     }
//     let q = args[1].to_string();

//     match &*q {
//         "cp2_1_1_d" => {
//             input!{
//                 n: usize,
//                 r: [i32; n]
//             }
//             println!("{}", alds2_1_1_d::run(n, r));
//         }
//         "cp3_1_1_a" => {
//             input!{
//                 n: usize,
//                 a: [i32; n]
//             }
//             let results = alds3_1_1_a::run(n, a);
//             for i in 0..n {
//                 println!("{:?}", results[i]);
//             }
//         }
//         _ => println!("Unimplemented.")
//     }
// }




#[macro_export]
// #[snippet(name = "input")]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        $crate::input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        $crate::input_inner!{next, $($r)*}
    };
}

#[macro_export(local_inner_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = $crate::read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[macro_export(local_inner_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!{
        n: usize,
        a: [i32; n]
    }
    let results = run(n, a);
    for i in 0..n {
        println!("{:?}", results[i]);
    }
}

pub fn run(n: usize, a: Vec<i32>) -> Vec<Vec<i32>> {
    if n == a.len() {
        insert_sort(n, a)
    } else {
        panic!("mismatched length (must be `r.len() == n`)")
    }
}

fn insert_sort(n: usize, mut a: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results: Vec<Vec<i32>> = Vec::new();
    results.push(a.to_vec());

    for i in 0..n-1 {
        let mut j = i;
        let mut done = false;
        let v = a[i+1];

        while !done {
            a[j+1] = a[j];
            if a[j] <= v {
                a[j+1] = v;
                done = true;
            } else if j == 0 {
                a[0] = v;
                done = true;
            } else {
                j -= 1;
            }
        }
        results.push(a.to_vec())
    }
    results
}
