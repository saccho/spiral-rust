use spiral_rust::cp3::alds3_1_1_a;

#[test]
fn cp3_1_1_a_1() {
    const N: usize = 6;
    const A: [i32; N] = [5, 2, 4, 6, 1, 3];
    const ANS: [[i32; N]; N] = [
        [5, 2, 4, 6, 1, 3],
        [2, 5, 4, 6, 1, 3],
        [2, 4, 5, 6, 1, 3],
        [2, 4, 5, 6, 1, 3],
        [1, 2, 4, 5, 6, 3],
        [1, 2, 3, 4, 5, 6],
    ];
    let results = alds3_1_1_a::run(N, A.to_vec());
    for i in 0..N {
        assert_eq!(ANS[i].to_vec(), results[i]);
    }
}
