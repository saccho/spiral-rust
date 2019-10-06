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
    
    while flag {
        flag = false;
        for i in (1..n).rev() {
            if a[i-1] > a[i] {
                let mut x = a[i-1].clone();
                let mut y = a[i].clone();
                mem::swap(&mut x, &mut y);
                a[i-1] = x;
                a[i] = y;
                flag = true;
                count += 1;
            }
        }
    }
    (a, count)
}
