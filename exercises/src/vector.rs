use std::collections::HashMap;

pub fn median_and_mode(mut vector: Vec<i32>) {
    if vector.is_empty() {
        println!("Vector is empty!");
        return;
    }

    let mut map = HashMap::new();
    let mut mode = vector[0];
    let mut max_count = 0;
    let length = vector.len();

    vector.sort();
    for num in &vector {
        let count = map.entry(*num).or_insert(0);
        *count += 1;

        if *count > max_count {
            mode = *num;
            max_count = *count;
        }
    }

    let median: f64 = if length % 2 == 1 {
        vector[length/2] as f64
    } else {
        let mid1 = vector[length/2] as f64;
        let mid2 = vector[length/2 - 1] as f64;
        (mid1 + mid2)/2.0
    };

    println!("Median: {median}");
    println!("Mode: {mode} (appears {max_count} times)");
}