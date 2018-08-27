pub fn sort (list: Vec<isize>) -> Vec<isize> {
    let mut to_sort = list.clone();
    let len = to_sort.len() - 1;
    for _ in 0..len {
        for current in 0..len {
            let next = current + 1 as usize;
            if to_sort[current] > to_sort[next] {
                to_sort.swap(current, next);
            }
        }
    }
    to_sort.clone()
}

pub fn sort_improved (list: Vec<isize>) -> Vec<isize> {
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
    to_sort.clone()
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use bubble;

    #[test]
    fn bubble_sort_should_sort() {
        assert_eq!(
            bubble::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[test]
    fn bubble_sort_improved_should_sort() {
        assert_eq!(
            bubble::sort_improved(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        b.iter(|| bubble::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
    }

    #[bench]
    fn bench_bubble_sort_improved(b: &mut Bencher) {
        b.iter(|| bubble::sort_improved(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
    }
}
