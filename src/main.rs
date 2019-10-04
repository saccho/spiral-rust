use spiral_rust::input;
use spiral_rust::cp2::alds2_1_1_d;

fn main() {
    input!{
        n: usize,
        r: [i32; n]
    }
    println!("{}", alds2_1_1_d::run(n, r));
}
