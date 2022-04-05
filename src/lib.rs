fn swap<T: Copy + PartialOrd>(arr: &mut Vec<T>, i: usize, j: usize) {
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

fn partition<T: Copy + PartialOrd>(arr: &mut Vec<T>, low: usize, high: usize) -> usize {
    let pivot = arr[high];

    let mut i: i32 = low as i32 - 1;

    for j in low..high {
        if arr[j] < pivot {
            i += 1;
            swap(arr, i as usize, j);
        }
    }
    swap(arr, (i + 1) as usize, high);

    (i + 1) as usize
}

fn sort<T: Copy + PartialOrd>(arr: &mut Vec<T>, low: usize, high: usize) {
    if low < high {
        let index = partition(arr, low, high);

        sort(arr, low, index - 1);
        sort(arr, index + 1, high);
    }
}

pub fn quick_sort<T: Copy + PartialOrd>(arr: &mut Vec<T>) {
    let len = arr.len();
    sort(arr, 0, len - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_sort_test() {
        let mut arr = vec![8, 4, 1, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 4, 8]);
    }

    #[test]
    fn swap_test() {
        let mut result = vec![1, 2];
        swap(&mut result, 0, 1);
        assert_eq!(result, vec![2, 1]);
    }
}
