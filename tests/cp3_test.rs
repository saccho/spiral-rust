use spiral_rust::cp3::alds3_1_1_a;
use spiral_rust::cp3::alds3_1_2_a;

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

#[test]
fn cp3_1_2_a_1() {
    const N: usize = 5;
    const A: [i32; N] = [5, 3, 2, 4, 1];
    const ANS_V: [i32; N] = [1, 2, 3, 4, 5];
    const ANS_C: i32 = 8;
    let (result, count) = alds3_1_2_a::run(N, A.to_vec());
    assert_eq!(ANS_V.to_vec(), result);
    assert_eq!(ANS_C, count);
}
