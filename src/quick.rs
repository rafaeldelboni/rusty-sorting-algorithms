pub fn sort (list: Vec<isize>) -> Vec<isize> {
    let len = list.len();
    if len <= 1 {
        return list;
    }

    let pivot = list[len-1];
    let mut left_list: Vec<isize> = Vec::new();
    let mut pivot_list: Vec<isize> = Vec::new();
    let mut right_list: Vec<isize> = Vec::new();

    for i in 0..len {
        if list[i] < pivot {
            left_list.push(list[i])
        } else if list[i] > pivot {
            right_list.push(list[i])
        } else {
            pivot_list.push(list[i])
        }
    }

    let mut final_list = sort(left_list);

    final_list
        .extend(pivot_list);

    final_list
        .extend(sort(right_list));

    final_list
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use test::Bencher;
    use tools::random_vector;
    use quick;

    #[test]
    fn quick_sort_should_sort() {
        assert_eq!(
            quick::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[test]
    fn quick_sort_should_sort_ordered() {
        assert_eq!(
            quick::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_quick_sort_good_pivot(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| quick::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0, 5]));
        println!(
            "bench_quick_sort_good_pivot {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_quick_sort(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| quick::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
        println!(
            "bench_quick_sort {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_quick_sort_ordered(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| quick::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]));
        println!(
            "bench_quick_sort_ordered {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_quick_sort_1000(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| quick::sort(random_vector(1000)));
        println!(
            "bench_quick_sort_1000 {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }
}
