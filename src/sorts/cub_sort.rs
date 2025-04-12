use crate::utils::insertion_sort;

pub fn cub_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    let max = arr.iter().max().unwrap_or(&0);
    let min = arr.iter().min().unwrap_or(&0);
    if max == min {
        return;
    }

    let bucket_count = (n as f64).cbrt() as usize;
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; bucket_count];

    for &item in arr.iter() {
        let index = ((item - min) as f64 / (*max - min + 1) as f64 * bucket_count as f64) as usize;
        buckets[index.min(bucket_count - 1)].push(item);
    }

    let mut index = 0;
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
        for &item in bucket.iter() {
            arr[index] = item;
            index += 1;
        }
    }
}