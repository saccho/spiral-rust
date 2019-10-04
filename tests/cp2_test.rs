use spiral_rust::cp2::alds2_1_1_d;

#[test]
fn cp2_1_1_d_1() {
    let n: usize = 6;
    let r = &[5, 3, 1, 3, 4, 3];
    assert_eq!(3, alds2_1_1_d::run(n, r.to_vec()));
}

#[test]
fn cp2_1_1_d_2() {
    let n: usize = 3;
    let r = &[4, 3, 2];
    assert_eq!(-1, alds2_1_1_d::run(n, r.to_vec()));
}
