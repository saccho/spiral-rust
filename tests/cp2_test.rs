use spiral_rust::cp2::alds2_1_1_d;

#[test]
fn cp2_1_1_d_1() {
    const N: usize = 6;
    const R: [i32; N] = [5, 3, 1, 3, 4, 3];
    assert_eq!(3, alds2_1_1_d::run(N, R.to_vec()));
}

#[test]
fn cp2_1_1_d_2() {
    const N: usize = 3;
    const R: [i32; N] = [4, 3, 2];
    assert_eq!(-1, alds2_1_1_d::run(N, R.to_vec()));
}
