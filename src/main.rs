use spiral_rust::cp2::alds2_1_1_d;
use spiral_rust::cp3::alds3_1_1_a;
use spiral_rust::cp3::alds3_1_2_a;
use spiral_rust::cp3::alds3_1_2_b;
use spiral_rust::cp3::alds3_1_2_c;
use spiral_rust::cp3::alds3_1_2_d;
use spiral_rust::cp4::alds4_1_3_a;
use spiral_rust::cp4::alds4_1_3_b;
use spiral_rust::input;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("A problem number is required as an argument. (e.g.: $ cargo run cp2_1_1_d)")
    }
    let q = args[1].to_string();

    match &*q {
        "cp2_1_1_d" => {
            input! {
                n: usize,
                r: [i32; n]
            }
            println!("{}", alds2_1_1_d::run(n, r));
        }
        "cp3_1_1_a" => {
            input! {
                n: usize,
                a: [i32; n]
            }
            let results = alds3_1_1_a::run(n, a);
            for i in 0..n {
                println!("{:?}", results[i]);
            }
        }
        "cp3_1_2_a" => {
            input! {
                n: usize,
                a: [i32; n]
            }
            let (result, count) = alds3_1_2_a::run(n, a);
            println!("{:?}", result);
            println!("{}", count);
        }
        "cp3_1_2_b" => {
            input! {
                n: usize,
                a: [i32; n]
            }
            let (result, count) = alds3_1_2_b::run(n, a);
            println!("{:?}", result);
            println!("{}", count);
        }
        "cp3_1_2_c" => {
            input! {
                n: usize,
                c: [String; n]
            }
            let results = alds3_1_2_c::run(n, c);
            println!("{:?}", results.bubble.values);
            println!("{}", results.bubble.is_stable);
            println!("{:?}", results.selection.values);
            println!("{}", results.selection.is_stable);
        }
        "cp3_1_2_d" => {
            input! {
                n: usize,
                a: [i32; n]
            }
            let result = alds3_1_2_d::run(n, a);
            println!("{:?}", result.m);
            for i in 0..result.m {
                print!("{}", result.intervals[i]);
                if i == result.m - 1 {
                    print!("\n");
                } else {
                    print!(" ");
                }
            }
            println!("{}", result.cnt);
            for i in 0..n {
                println!("{}", result.values[i]);
            }
        }
        "cp4_1_3_a" => {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            println!("{}", alds4_1_3_a::run(s));
        }
        "cp4_1_3_b" => {
            input! {
                n: usize,
                q: i32,
                processes: [(String, i32); n]
            }
            println!("{:?}", alds4_1_3_b::run(n, q, processes));
        }
        _ => println!("Unimplemented."),
    }
}
