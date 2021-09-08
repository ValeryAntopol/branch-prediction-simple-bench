fn check_array(arr: &Vec<i32>, threshold: i32) -> i32 {
    let mut counter = 0;
    for value in arr {
        if *value < threshold {
            counter += 1;
        }
    }

    counter
}

use rand::Rng;

const ARRAY_SIZE: i32 = 1_000_000_000;
const RNG_MIN: i32 = 0;
const RNG_MAX: i32 = 100;
const THRESHOLD: i32 = 50;

fn gen_array() -> Vec<i32> {
    let mut rng = rand::thread_rng();

    (0..ARRAY_SIZE)
        .map(|_| rng.gen_range(RNG_MIN, RNG_MAX))
        .collect()
}
fn main() {
    let arr_unsorted = gen_array();
    let mut arr_sorted = arr_unsorted.clone();
    arr_sorted.sort();

    use std::time::Instant;

    let start = Instant::now();
    let val_sorted = check_array(&arr_sorted, THRESHOLD);
    let duration_sorted = start.elapsed();

    let start = Instant::now();
    let val_unsorted = check_array(&arr_unsorted, THRESHOLD);
    let duration_unsorted = start.elapsed();

    println!(
        "unsorted: {}, elapsed {}ms\
        \nsorted: {}, elapsed {}ms",
        val_unsorted,
        duration_unsorted.as_millis(),
        val_sorted,
        duration_sorted.as_millis()
    );
}
