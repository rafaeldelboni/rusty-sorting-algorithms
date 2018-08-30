fn max_heapify (mut to_sort: Vec<isize>, length: usize, index: usize) -> Vec<isize> {
    let mut parent = index;
    let left = 2 * index + 1;
    let right = 2 * index + 2;
    if left < length && to_sort[left] > to_sort[parent] {
        parent = left;
    }
    if right < length && to_sort[right] > to_sort[parent] {
        parent = right;
    }
    if parent != index {
        to_sort.swap(index, parent);
        to_sort = max_heapify(to_sort, length, parent);
    }
    to_sort
}

pub fn sort (list: Vec<isize>) -> Vec<isize> {
    let mut to_sort = list.clone();
    let length = to_sort.len();
    for i in (0..length/2).rev() {
        to_sort = max_heapify(to_sort, length, i);
    }
    for i in (0..length).rev() {
        to_sort.swap(0, i);
        to_sort = max_heapify(to_sort, i, 0);
    }
    to_sort
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime};
    use test::Bencher;
    use tools::random_vector;

    use heap;

    #[test]
    fn heap_sort_should_sort() {
        assert_eq!(
            heap::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[test]
    fn heap_sort_should_sort_ordered() {
        assert_eq!(
            heap::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_heap_sort(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| heap::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
        println!(
            "bench_heap_sort {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_heap_sort_ordered(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| heap::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]));
        println!(
            "bench_heap_sort_ordered {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }

    #[bench]
    fn bench_heap_sort_1000(b: &mut Bencher) {
        let before = SystemTime::now();
        b.iter(|| heap::sort(random_vector(1000)));
        println!(
            "bench_heap_sort_1000 {:?}",
            SystemTime::now().duration_since(before).unwrap()
        );
    }
}
