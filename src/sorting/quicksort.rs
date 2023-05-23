pub fn quicksort<T: Ord + Clone>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    
    let pivot_index = partition(array);
    
    let (left, right) = array.split_at_mut(pivot_index);
    
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord + Clone>(array: &mut [T]) -> usize {
    let len = array.len();
    let pivot_index = len / 2; // Choosing the middle element as the pivot
    let pivot_value = array[pivot_index].clone();
    
    array.swap(pivot_index, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if array[j] <= pivot_value {
            array.swap(i, j);
            i += 1;
        }
    }
    
    array.swap(i, len - 1);
    
    i
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut arr = [5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
