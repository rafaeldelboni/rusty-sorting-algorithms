pub fn sort (list: Vec<isize>) -> Vec<isize> {
    let mut to_sort = list.clone();
    let mut len = to_sort.len() - 1;
    for _ in 0..len {
        for current in 0..len {
            let next = current + 1 as usize;
            if to_sort[current] > to_sort[next] {
                to_sort.swap(current, next);
            }
        }
        len -= 1;
    }
    to_sort
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime};
    use test::Bencher;
    use tools::random_vector;

    use bubble;

    #[test]
    fn bubble_sort_should_sort() {
        assert_eq!(
            bubble::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[test]
    fn bubble_sort_should_sort_ordered() {
        assert_eq!(
            bubble::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| bubble::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
        println!(
            "bench_bubble_sort {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_bubble_sort_ordered(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| bubble::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]));
        println!(
            "bench_bubble_sort_ordered {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_bubble_sort_1000(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| bubble::sort(random_vector(1000)));
        println!(
            "bench_bubble_sort_1000 {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }
}
