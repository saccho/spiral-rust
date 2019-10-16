use spiral_rust::cp3::alds3_1_1_a;
use spiral_rust::cp3::alds3_1_2_a;
use spiral_rust::cp3::alds3_1_2_b;
use spiral_rust::cp3::alds3_1_2_c;
use spiral_rust::cp3::alds3_1_2_d;

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
fn cp3_1_1_a_2() {
    const N: usize = 6;
    const A: [i32; N] = [8, 3, 1, 5, 2, 1];
    const ANS: [[i32; N]; N] = [
        [8, 3, 1, 5, 2, 1],
        [3, 8, 1, 5, 2, 1],
        [1, 3, 8, 5, 2, 1],
        [1, 3, 5, 8, 2, 1],
        [1, 2, 3, 5, 8, 1],
        [1, 1, 2, 3, 5, 8],
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

#[test]
fn cp3_1_2_a_2() {
    const N: usize = 8;
    const A: [i32; N] = [8, 4, 3, 7, 6, 5, 2, 1];
    const ANS_V: [i32; N] = [1, 2, 3, 4, 5, 6, 7, 8];
    const ANS_C: i32 = 22;
    let (result, count) = alds3_1_2_a::run(N, A.to_vec());
    assert_eq!(ANS_V.to_vec(), result);
    assert_eq!(ANS_C, count);
}

#[test]
fn cp3_1_2_b_1() {
    const N: usize = 6;
    const A: [i32; N] = [5, 6, 4, 2, 1, 3];
    const ANS_V: [i32; N] = [1, 2, 3, 4, 5, 6];
    const ANS_C: i32 = 4;
    let (result, count) = alds3_1_2_b::run(N, A.to_vec());
    assert_eq!(ANS_V.to_vec(), result);
    assert_eq!(ANS_C, count);
}

#[test]
fn cp3_1_2_b_2() {
    const N: usize = 7;
    const A: [i32; N] = [5, 4, 8, 7, 9, 3, 1];
    const ANS_V: [i32; N] = [1, 3, 4, 5, 7, 8, 9];
    const ANS_C: i32 = 5;
    let (result, count) = alds3_1_2_b::run(N, A.to_vec());
    assert_eq!(ANS_V.to_vec(), result);
    assert_eq!(ANS_C, count);
}

#[test]
fn cp3_1_2_c_1() {
    const N: usize = 5;
    let c: [String; N] = [
        "H4".to_string(), 
        "C9".to_string(), 
        "S4".to_string(), 
        "D2".to_string(), 
        "C3".to_string()
    ];
    let ans_bv: [String; N] = [
        "D2".to_string(), 
        "C3".to_string(), 
        "H4".to_string(), 
        "S4".to_string(), 
        "C9".to_string()
    ];
    let ans_bs: String = "Stable".to_string();
    let ans_sv: [String; N] = [
        "D2".to_string(), 
        "C3".to_string(), 
        "S4".to_string(), 
        "H4".to_string(), 
        "C9".to_string()
    ];
    let ans_ss: String = "Not Stable".to_string();
    let results = alds3_1_2_c::run(N, c.to_vec());
    assert_eq!(ans_bv.to_vec(), results.bubble.values);
    assert_eq!(ans_bs, results.bubble.is_stable);
    assert_eq!(ans_sv.to_vec(), results.selection.values);
    assert_eq!(ans_ss, results.selection.is_stable);
}

#[test]
fn cp3_1_2_c_2() {
    const N: usize = 2;
    let c: [String; N] = ["S1".to_string(), "H1".to_string()];
    let ans_bv: [String; N] = ["S1".to_string(), "H1".to_string()];
    let ans_bs: String = "Stable".to_string();
    let ans_sv: [String; N] = ["S1".to_string(), "H1".to_string()];
    let ans_ss: String = "Stable".to_string();
    let results = alds3_1_2_c::run(N, c.to_vec());
    assert_eq!(ans_bv.to_vec(), results.bubble.values);
    assert_eq!(ans_bs, results.bubble.is_stable);
    assert_eq!(ans_sv.to_vec(), results.selection.values);
    assert_eq!(ans_ss, results.selection.is_stable);
}

#[test]
fn cp3_1_2_d_1() {
    const N: usize = 5;
    const A: [i32; N] = [5, 1, 4, 3, 2];
    const ANS_M: usize = 2;
    const ANS_INTERVALS: [usize; ANS_M] = [4, 1];
    const ANS_CNT: i32 = 3;
    const ANS_V: [i32; N] = [1, 2, 3, 4, 5];
    let result = alds3_1_2_d::run(N, A.to_vec());
    assert_eq!(ANS_M, result.m);
    assert_eq!(ANS_INTERVALS.to_vec(), result.intervals);
    assert_eq!(ANS_CNT, result.cnt);
    assert_eq!(ANS_V.to_vec(), result.values);
}

#[test]
fn cp3_1_2_d_2() {
    const N: usize = 3;
    const A: [i32; N] = [3, 2, 1];
    const ANS_M: usize = 1;
    const ANS_INTERVALS: [usize; ANS_M] = [3];
    const ANS_CNT: i32 = 3;
    const ANS_V: [i32; N] = [1, 2, 3];
    let result = alds3_1_2_d::run(N, A.to_vec());
    assert_eq!(ANS_M, result.m);
    assert_eq!(ANS_INTERVALS.to_vec(), result.intervals);
    assert_eq!(ANS_CNT, result.cnt);
    assert_eq!(ANS_V.to_vec(), result.values);
}
