use spiral_rust::cp4::alds4_1_3_a;

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
