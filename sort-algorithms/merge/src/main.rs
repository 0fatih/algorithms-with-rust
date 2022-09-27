use rand::Rng;

fn main() {
    let size = 100;
    let list = get_random_vec(size);

    let sorted_list = merge_sort(&list);

    println!("Sorted list: {:?}", sorted_list);
}

fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size = vec.len() / 2;
        let left = merge_sort(&vec[0..size].to_vec());
        let right = merge_sort(&vec[size..].to_vec());
        let merged = merge(&left, &right);

        merged
    }
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i = i + 1;
        } else {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i = i + 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j = j + 1;
        }
    }

    merged
}

fn get_random_vec(size: i32) -> Vec<i32> {
    let mut list = Vec::with_capacity(size as usize);

    for _ in 0..size {
        list.push(rand::thread_rng().gen_range(0..1_000_000));
    }

    list
}

#[test]
fn test_sort() {
    let list = vec![34, 3423, 324, 454, 64, 65, 4];
    let sorted_list = vec![4, 34, 64, 65, 324, 454, 3423];

    let res = merge_sort(&list);

    assert_eq!(res, sorted_list);
}
