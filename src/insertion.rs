pub fn sort (list: Vec<isize>) -> Vec<isize> {
    let mut to_sort = list.clone();
    for i in 0..to_sort.len() {
        for j in (0..i).rev() {
            if to_sort[j] >= to_sort[j + 1] {
                to_sort.swap(j, j + 1);
            } else {
                break
            }
        }
    }
    to_sort
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime};
    use test::Bencher;
    use tools::random_vector;

    use insertion;

    #[test]
    fn insertion_sort_should_sort() {
        assert_eq!(
            insertion::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[test]
    fn insertion_sort_should_sort_ordered() {
        assert_eq!(
            insertion::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_insertion_sort(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| insertion::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
        println!(
            "bench_insertion_sort {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_insertion_sort_ordered(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| insertion::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]));
        println!(
            "bench_insertion_sort_ordered {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_insertion_sort_1000(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| insertion::sort(random_vector(1000)));
        println!(
            "bench_insertion_sort_1000 {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }
}
