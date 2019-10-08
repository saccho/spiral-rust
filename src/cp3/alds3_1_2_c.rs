pub struct SortedResult {
    pub values: Vec<String>,
    pub is_stable: String
}

pub struct SortedResults {
    pub bubble: SortedResult,
    pub selection: SortedResult
}

pub fn run(n: usize, c: Vec<String>) -> SortedResults {
    SortedResults {
        bubble: bubble_sort(n, &c),
        selection: selection_sort(n, &c)
    }
}

fn bubble_sort(n: usize, c: &Vec<String>) -> SortedResult {
    let mut a = c.clone();
    let mut a_nums = get_num(n, &a);
    let svis = get_same_value_indices(n, &a_nums);
    let mut same_values: Vec<Vec<String>> = Vec::new();
    for svi in svis {
        same_values.push(vec![a[svi[0]].clone(), a[svi[1]].clone()]);
    }
    let mut is_stable_str = "Not Stable";
    let mut flag = true;
    let mut i = 0;

    while flag {
        flag = false;
        for j in (i+1..n).rev() {
            if a_nums[j-1] > a_nums[j] {
                let min_num = a_nums[j];
                let min_str = a[j].to_string();

                a_nums[j] = a_nums[j-1];
                a[j] = a[j-1].to_string();

                a_nums[j-1] = min_num;
                a[j-1] = min_str;

                flag = true;
            }
        }
        i += 1;
    }

    let svis = get_same_value_indices(n, &a_nums);
    let mut sorted_same_values: Vec<Vec<String>> = Vec::new();
    for svi in svis {
        sorted_same_values.push(vec![a[svi[0]].clone(), a[svi[1]].clone()]);
    }

    if is_stable(&same_values, &sorted_same_values) {
        is_stable_str = "Stable";
    }

    SortedResult {
        values: a.to_vec(),
        is_stable: is_stable_str.to_string()
    }
}

fn selection_sort(n: usize, c: &Vec<String>) -> SortedResult {
    let mut a = c.clone();
    let mut a_nums = get_num(n, &a);
    let svis = get_same_value_indices(n, &a_nums);
    let mut same_values: Vec<Vec<String>> = Vec::new();
    for svi in svis {
        same_values.push(vec![a[svi[0]].clone(), a[svi[1]].clone()]);
    }
    let mut is_stable_str = "Not Stable";

    for i in 0..n-1 {
        let mut min_i = i;
        for j in i..n {
            if a_nums[j] < a_nums[min_i] {
                min_i = j;
            }
        }
        if min_i != i {
            let min_num = a_nums[min_i];
            let min_str = a[min_i].to_string();
            
            a_nums[min_i] = a_nums[i];
            a[min_i] = a[i].to_string();
            
            a_nums[i] = min_num;
            a[i] = min_str;
        }
    }

    let svis = get_same_value_indices(n, &a_nums);
    let mut sorted_same_values: Vec<Vec<String>> = Vec::new();
    for svi in svis {
        sorted_same_values.push(vec![a[svi[0]].clone(), a[svi[1]].clone()]);
    }

    if is_stable(&same_values, &sorted_same_values) {
        is_stable_str = "Stable";
    }

    SortedResult {
        values: a.to_vec(),
        is_stable: is_stable_str.to_string()
    }
}

fn get_num(n: usize, a: &Vec<String>) -> Vec<i32> {
    let mut a_nums: Vec<i32> = Vec::new();
    for i in 0..n {
        let a_str: char = a[i].chars().nth(1).unwrap();
        let a_num: i32 = a_str as i32 - 48;
        a_nums.push(a_num);
    }
    a_nums
}

fn get_same_value_indices(n: usize, a_num: &Vec<i32>) -> Vec<Vec<usize>> {
    let mut indices: Vec<Vec<usize>> = Vec::new();
    for i in 0..n-1 {
        for j in i+1..n {
            if a_num[i] == a_num[j] {
                indices.push(vec![i, j]);
            }
        }
    }
    indices
}

fn is_stable(same_values: &Vec<Vec<String>>, sorted_same_values: &Vec<Vec<String>>) -> bool {
    let mut b = false;
    for sv in same_values {
        b = false;
        for ssv in sorted_same_values {
            if sv == ssv {
                b = true;
            }
        }
        if !b {
            break;
        }
    }
    b
}
