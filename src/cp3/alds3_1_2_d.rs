pub struct SortedResult {
    pub m: usize,
    pub intervals: Vec<usize>,
    pub cnt: i32,
    pub values: Vec<i32>
}

pub fn run(n: usize, a: Vec<i32>) -> SortedResult {
    let (shell_values, m, intervals, cnt) = shell_sort(n, &a);
    SortedResult {
        m,
        intervals,
        cnt,
        values: shell_values
    }
}

fn shell_sort(n: usize, a: &Vec<i32>) -> (Vec<i32>, usize, Vec<usize>, i32) {
    let mut result = a.clone();
    let mut cnt = 0;
    let mut intervals: Vec<usize> = vec![1];
    loop {
        let x = 3 * intervals.first().unwrap() + 1;
        if x > a.len() {
            break;
        } else {
            intervals.insert(0, x);
        }
    }
    let m: usize = intervals.len();

    for i in 0..m {
        result = insert_sort(n, &result, intervals[i], &mut cnt);
    }

    (result, m, intervals, cnt)
}

fn insert_sort(n: usize, a: &Vec<i32>, g: usize, cnt: &mut i32) -> Vec<i32> {
    let mut result = a.clone();

    for i in g..n {
        let mut j = i - g;
        let mut done = false;
        let v = result[i];

        while !done {
            result[j+g] = result[j];
            if result[j] <= v {
                result[j+g] = v;
                *cnt += 1;
                done = true;
            } else if j == 0 {
                result[0] = v;
                done = true;
            } else {
                j -= g;
            }
        }
    }

    result
}
