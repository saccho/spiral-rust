pub struct SortedResult {
    pub values: Vec<String>,
    pub is_stable: String,
}

pub struct SortedResults {
    pub bubble: SortedResult,
    pub selection: SortedResult,
}

pub fn run(n: usize, c: Vec<String>) -> SortedResults {
    let bubble_values = bubble_sort(n, &c);
    let bubble_is_stable = "Stable".to_string();
    let selection_values = selection_sort(n, &c);
    let selection_is_stable = is_stable(&bubble_values, &selection_values);

    let bubble = SortedResult {
        values: bubble_values,
        is_stable: bubble_is_stable,
    };
    let selection = SortedResult {
        values: selection_values,
        is_stable: selection_is_stable,
    };

    SortedResults { bubble, selection }
}

fn bubble_sort(n: usize, c: &Vec<String>) -> Vec<String> {
    let mut a = c.clone();
    let mut a_nums = get_num(n, &a);
    let mut flag = true;
    let mut i = 0;

    while flag {
        flag = false;
        for j in (i + 1..n).rev() {
            if a_nums[j - 1] > a_nums[j] {
                let min_num = a_nums[j];
                let min_str = a[j].to_string();

                a_nums[j] = a_nums[j - 1];
                a[j] = a[j - 1].to_string();

                a_nums[j - 1] = min_num;
                a[j - 1] = min_str;

                flag = true;
            }
        }
        i += 1;
    }

    a
}

fn selection_sort(n: usize, c: &Vec<String>) -> Vec<String> {
    let mut a = c.clone();
    let mut a_nums = get_num(n, &a);

    for i in 0..n - 1 {
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

    a
}

fn get_num(n: usize, a: &Vec<String>) -> Vec<i32> {
    let mut a_nums: Vec<i32> = Vec::new();
    for i in 0..n {
        let a_str: String = a[i].chars().skip(1).collect::<String>();
        let a_num: i32 = a_str.parse().unwrap();
        a_nums.push(a_num);
    }
    a_nums
}

fn is_stable(bubble: &Vec<String>, selection: &Vec<String>) -> String {
    let mut result = "Not Stable".to_string();
    if bubble == selection {
        result = "Stable".to_string();
    }
    result
}
