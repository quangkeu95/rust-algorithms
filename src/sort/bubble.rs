pub fn bubble_sort<T>(unsorted: &mut [T])
where
    T: Ord,
{
    let mut swapped = true;
    while swapped {
        swapped = false;

        for i in 0..unsorted.len() - 1 {
            if unsorted[i + 1] < unsorted[i] {
                unsorted.swap(i, i + 1);

                swapped = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![4, 2, 5, 10, 30, 4, 100, 20, 43, 29, 11, 22];
        bubble_sort(&mut arr);

        assert_eq!(arr, vec![2, 4, 4, 5, 10, 11, 20, 22, 29, 30, 43, 100]);
    }
}
