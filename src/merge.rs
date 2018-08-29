pub fn sort (list: Vec<isize>) -> Vec<isize> {
    let len = list.len();
    let middle = len / 2;

    let left_part: Vec<isize> = list[0..middle].to_vec();
    let right_part: Vec<isize> = list[middle..len].to_vec();

    if left_part.len() < 1 {
        return right_part;
    }

    let mut left_list = sort(left_part);
    let mut right_list = sort(right_part);
    let mut final_list: Vec<isize> = Vec::new();

    while left_list.len() + right_list.len() > 0 {
        if left_list.len() == 0 {
            final_list.append(&mut right_list);
            break;
        }
        if right_list.len() == 0 {
            final_list.append(&mut left_list);
            break;
        }

        if left_list[0] < right_list[0] {
            final_list.push(left_list[0]);
            left_list.remove(0);
        } else {
            final_list.push(right_list[0]);
            right_list.remove(0);
        }
    }

    final_list
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use test::Bencher;
    use tools::random_vector;
    use merge;

    #[test]
    fn merge_sort_should_sort() {
        assert_eq!(
            merge::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[test]
    fn merge_sort_should_sort_ordered() {
        assert_eq!(
            merge::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_merge_sort(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| merge::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0, 5]));
        println!(
            "bench_merge_sort {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_merge_sort_1000(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| merge::sort(random_vector(1000)));
        println!(
            "bench_merge_sort_1000 {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }
}
