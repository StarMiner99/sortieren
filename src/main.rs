use std::time::{SystemTime, UNIX_EPOCH};


const LIST_LEN: usize = 100_000;
fn main() {
    //let mut rand_array: [u64; LIST_LEN] = rand::random();

    let mut rand_array: [u64; LIST_LEN] = core::array::from_fn(|i| i as u64);
    rand_array.reverse();

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    bubble_sort(&mut rand_array);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let duration = end - start;

    println!("Took {}us for {} elements.", duration.as_millis(), LIST_LEN);
    assert!(rand_array.windows(2).all(|w| w[0] <= w[1]), "Result is not sorted!");

}

fn bubble_sort(arr: &mut [u64]) {
    let last_check = arr.len();

    for end_index in (1..last_check).rev() {
        for i in 0..end_index {
            if arr[i] > arr[i+1] {
                let temp = arr[i];
                arr[i] = arr[i+1];
                arr[i+1] = temp;
            }
        }
    }
    
}