use std::time::{SystemTime, UNIX_EPOCH};


const LIST_LEN: usize = 100_000;
fn main() {
    // random list:
    let mut rand_array: [u64; LIST_LEN] = rand::random();

    // sorted list:
    //let mut rand_array: [u64; LIST_LEN] = core::array::from_fn(|i| (i * i) as u64);
    //rand_array.reverse(); // comment this for not-reversed list

    println!("{} {}", rand_array[0], rand_array[LIST_LEN -1]);

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let swaps = bubble_sort(&mut rand_array);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let duration = end - start;

    println!("Took {}ms for {} elements. {} swaps", duration.as_millis(), LIST_LEN, swaps);
    assert!(rand_array.windows(2).all(|w| w[0] <= w[1]), "Result is not sorted!");

}

fn bubble_sort(arr: &mut [u64; LIST_LEN]) -> usize {
    let last_check = LIST_LEN;
    let mut swaps = 0;

    for end_index in (1..last_check).rev() {
        for i in 0..end_index {
            if arr[i] > arr[i+1] {
                let temp = arr[i];
                arr[i] = arr[i+1];
                arr[i+1] = temp;
                swaps += 1;
            }
        }
    }

    swaps
    
}