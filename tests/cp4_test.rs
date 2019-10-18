use spiral_rust::cp4::alds4_1_3_a;
use spiral_rust::cp4::alds4_1_3_b;

#[test]
fn cp4_1_3_a_1() {
    let s: String = "1 2 +".to_string();
    const ANS: i32 = 3;
    assert_eq!(ANS, alds4_1_3_a::run(s));
}

#[test]
fn cp4_1_3_a_2() {
    let s: String = "1 2 + 3 4 - *".to_string();
    const ANS: i32 = -3;
    assert_eq!(ANS, alds4_1_3_a::run(s));
}

#[test]
fn cp4_1_3_b_1() {
    const N: usize = 5;
    const Q: i32 = 100;
    let p = [
        ("p1".to_string(), 150),
        ("p2".to_string(), 80),
        ("p3".to_string(), 200),
        ("p4".to_string(), 350),
        ("p5".to_string(), 20),        
    ];
    let ans_p = [
        ("p2".to_string(), 180),
        ("p5".to_string(), 400),
        ("p1".to_string(), 450),
        ("p3".to_string(), 550),
        ("p4".to_string(), 800),        
    ];
    assert_eq!(ans_p.to_vec(), alds4_1_3_b::run(N, Q, p.to_vec()));
}
