use rand::Rng;

fn main() {
    let size = 100;
    let mut list = get_random_vec(size);

    sort(&mut list);

    println!("Sorted list: {:?}", list)
}

fn sort(list: &mut Vec<i32>) {
    for i in 1..list.len() {
        let mut j = i;

        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
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
    let mut list = vec![3, 95, 34, 23, 12, 4, 54, 65];
    let sorted = vec![3, 4, 12, 23, 34, 54, 65, 95];

    sort(&mut list);

    assert_eq!(list, sorted);
}
