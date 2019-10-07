pub fn run(n: usize, a: Vec<i32>) -> (Vec<i32>, i32) {
    if n == a.len() {
        selection_sort(n, a)
    } else {
        panic!("mismatched length (must be `a.len() == n`)")
    }
}

fn selection_sort(n: usize, mut a: Vec<i32>) -> (Vec<i32>, i32) {
    let mut count = 0;

    for i in 0..n {
        let mut min_i = i;
        for j in i..n {
            if a[j] < a[min_i] {
                min_i = j;
            }
        }
        if min_i != i {
            let min = a[min_i];
            a[min_i] = a[i];
            a[i] = min;
            count += 1;
        }
    }
    (a, count)
}
