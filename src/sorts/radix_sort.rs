pub fn radix_sort(arr: &mut Vec<i32>) {
    let max = *arr.iter().max().unwrap_or(&0);
    let mut exp = 1;

    while max / exp > 0 {
        counting_sort_for_radix(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_for_radix(arr: &mut Vec<i32>, exp: i32) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    for &item in arr.iter() {
        count[((item / exp) % 10) as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for i in (0..n).rev() {
        let index = ((arr[i] / exp) % 10) as usize;
        output[count[index] - 1] = arr[i];
        count[index] -= 1;
    }

    for i in 0..n {
        arr[i] = output[i];
    }
}