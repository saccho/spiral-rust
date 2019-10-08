use std::env;
use spiral_rust::input;
use spiral_rust::cp2::alds2_1_1_d;
use spiral_rust::cp3::alds3_1_1_a;
use spiral_rust::cp3::alds3_1_2_a;
use spiral_rust::cp3::alds3_1_2_b;
use spiral_rust::cp3::alds3_1_2_c;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("A problem number is required as an argument. (e.g.: $ cargo run cp2_1_1_d)")
    }
    let q = args[1].to_string();

    match &*q {
        "cp2_1_1_d" => {
            input!{
                n: usize,
                r: [i32; n]
            }
            println!("{}", alds2_1_1_d::run(n, r));
        }
        "cp3_1_1_a" => {
            input!{
                n: usize,
                a: [i32; n]
            }
            let results = alds3_1_1_a::run(n, a);
            for i in 0..n {
                println!("{:?}", results[i]);
            }
        }
        "cp3_1_2_a" => {
            input!{
                n: usize,
                a: [i32; n]
            }
            let (result, count) = alds3_1_2_a::run(n, a);
            println!("{:?}", result);
            println!("{}", count);
        }
        "cp3_1_2_b" => {
            input!{
                n: usize,
                a: [i32; n]
            }
            let (result, count) = alds3_1_2_b::run(n, a);
            println!("{:?}", result);
            println!("{}", count);
        }
        "cp3_1_2_c" => {
            input!{
                n: usize,
                c: [String; n]
            }
            let results = alds3_1_2_c::run(n, c);
            println!("{:?}", results.bubble.values);
            println!("{}", results.bubble.is_stable);
            println!("{:?}", results.selection.values);
            println!("{}", results.selection.is_stable);
        }
        _ => println!("Unimplemented.")
    }
}
