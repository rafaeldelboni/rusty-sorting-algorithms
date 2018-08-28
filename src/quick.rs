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
    use test::Bencher;
    use quick;

    use std::time::SystemTime;
    #[test]
    fn quick_sort_should_sort() {
        assert_eq!(
            quick::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]),
            vec![0, 1, 1, 3, 4, 4, 6, 8, 9]
        );
    }

    #[bench]
    fn bench_quick_sort_good_pivot(b: &mut Bencher) {
        b.iter(|| quick::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0, 5]));
    }

    #[bench]
    fn bench_quick_sort(b: &mut Bencher) {
        b.iter(|| quick::sort(vec![4, 1, 3, 6, 8, 9, 4, 1, 0]));
    }

    #[bench]
    fn bench_quick_sort_ordered(b: &mut Bencher) {
        b.iter(|| quick::sort(vec![0, 1, 1, 3, 4, 4, 6, 8, 9]));
    }
}