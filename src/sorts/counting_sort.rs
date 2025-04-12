pub fn counting_sort(arr: &mut Vec<i32>) {
    let max = arr.iter().max().unwrap_or(&0);
    let min = arr.iter().min().unwrap_or(&0);
    let range = (max - min + 1) as usize;
    let mut count = vec![0; range];
    let mut output = vec![0; arr.len()];

    for &item in arr.iter() {
        count[(item - min) as usize] += 1;
    }

    for i in 1..range {
        count[i] += count[i - 1];
    }

    for i in (0..arr.len()).rev() {
        let item = arr[i];
        count[(item - min) as usize] -= 1;
        output[count[(item - min) as usize]] = item;
    }

    for i in 0..arr.len() {
        arr[i] = output[i];
    }
}