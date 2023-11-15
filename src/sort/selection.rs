/// For each iteration, we find the min_index between current index and end of list
/// If the value at current index is less than value at min_index, we swap two elements
pub fn selection_sort<T>(inputs: &mut [T])
where
    T: Ord,
{
    for i in 0..inputs.len() {
        let mut min_index = i;

        for j in i + 1..inputs.len() {
            if inputs[j] < inputs[min_index] {
                min_index = j;
            }
        }

        inputs.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![4, 2, 5, 10, 30, 4, 100, 20, 43, 29, 11, 22];
        selection_sort(&mut arr);

        assert_eq!(arr, vec![2, 4, 4, 5, 10, 11, 20, 22, 29, 30, 43, 100]);
    }
}
