pub fn run(n: usize, a: Vec<i32>) -> Vec<Vec<i32>> {
    if n == a.len() {
        insert_sort(n, a)
    } else {
        panic!("mismatched length (must be `r.len() == n`)")
    }
}

fn insert_sort(n: usize, mut a: Vec<i32>) -> Vec<Vec<i32>> {
    let mut results: Vec<Vec<i32>> = Vec::new();
    results.push(a.to_vec());

    for i in 0..n-1 {
        let mut j = i;
        let mut done = false;
        let v = a[i+1];

        while !done {
            a[j+1] = a[j];
            if a[j] <= v {
                a[j+1] = v;
                done = true;
            } else if j == 0 {
                a[0] = v;
                done = true;
            } else {
                j -= 1;
            }
        }
        results.push(a.to_vec())
    }
    results
}
