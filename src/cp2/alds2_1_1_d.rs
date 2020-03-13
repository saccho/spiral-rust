pub fn run(n: usize, r: Vec<i32>) -> i32 {
    if n == r.len() {
        maximum_profit(n, r)
    } else {
        panic!("mismatched length (must be `r.len() == n`)")
    }
}

fn maximum_profit(n: usize, r: Vec<i32>) -> i32 {
    let mut minv = 1_000_000_000;
    let mut maxv = 1;
    let mut p = maxv - minv;

    for i in 0..n - 1 {
        if r[i] < minv {
            minv = r[i];
            maxv = r[i + 1];
        } else if r[i + 1] > maxv {
            maxv = r[i + 1];
        }
        p = maxv - minv;
    }

    if p < 0 {
        -1
    } else {
        p
    }
}
