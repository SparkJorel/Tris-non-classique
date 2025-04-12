use crate::utils::insertion_sort;

pub fn bionic_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    if n <= 10 {
        insertion_sort(arr);
        return;
    }

    let mid = n / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..n].to_vec();

    bionic_sort(&mut left);
    bionic_sort(&mut right);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}