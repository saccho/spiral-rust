use std::mem;

pub fn run(n: usize, a: Vec<i32>) -> (Vec<i32>, i32) {
    if n == a.len() {
        bubble_sort(n, a)
    } else {
        panic!("mismatched length (must be `a.len() == n`)")
    }
}

fn bubble_sort(n: usize, mut a: Vec<i32>) -> (Vec<i32>, i32) {
    let mut flag = true;
    let mut count = 0;
    let mut i = 0;
    
    while flag {
        flag = false;
        for j in (i+1..n).rev() {
            if a[j-1] > a[j] {
                let mut x = a[j-1].clone();
                let mut y = a[j].clone();
                mem::swap(&mut x, &mut y);
                a[j-1] = x;
                a[j] = y;
                flag = true;
                count += 1;
            }
        }
        i += 1;
    }
    (a, count)
}
