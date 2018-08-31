fn largest_in_vector (list: &Vec<isize>) -> isize {
    let mut largest = list[0];
    for index in 0..list.len() {
        if list[index] > largest {
            largest = list[index];
        }
    }
    largest
}

pub fn sort (list: Vec<isize>) -> Vec<isize> {
    let len = list.len();
    let radix = largest_in_vector(&list) as usize + 1;
    let mut count_list: Vec<isize> = vec![0; radix];
    let mut result_list: Vec<isize> = vec![0; len];

    for index in 0..len {
        count_list[list[index] as usize] += 1;
    }

    for index in 1..radix {
        count_list[index] = count_list[index - 1] + count_list[index];
    }

    for index in (0..len).rev() {
        let mut current_position = count_list[list[index] as usize] as usize;
        result_list[current_position - 1] = list[index];
        count_list[list[index] as usize] -= 1;
    }

    result_list
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime};
    use test::Bencher;
    use tools::random_vector;

    use counting;

    #[test]
    fn counting_sort_should_sort() {
        assert_eq!(
            counting::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[test]
    fn counting_sort_should_sort_ordered() {
        assert_eq!(
            counting::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_counting_sort(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| counting::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
        println!(
            "bench_counting_sort {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_counting_sort_ordered(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| counting::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]));
        println!(
            "bench_counting_sort_ordered {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_counting_sort_1000(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| counting::sort(random_vector(1000)));
        println!(
            "bench_counting_sort_1000 {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }
}
