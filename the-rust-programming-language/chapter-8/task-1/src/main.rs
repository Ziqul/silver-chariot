use std::collections::HashMap;

fn main() {
    let integers = [5, 6, 6, 9, 12, 14, 15];

    let mut integers: Vec<i32> = integers.to_vec();

    integers.sort();

    let mut sum: u32 = 0;
    let mut elems_counted = HashMap::new();

    for elem in &integers {
        //  Counting sum of element to find
        // avarage.
        sum += *elem as u32;

        //  Counting how many times each element is
        // encountered to find mode later.
        let count = elems_counted.entry(elem).or_insert(0);
        *count += 1;
    }

    let mut middle_elem: i32 =
        *(integers
            //  Method .len() returns integer
            // and integer devided by integer will
            // give us "floored" value.
            // Example: 5 / 2 = 2
            // [1, 2, 3, 4, 5].get(2) = 3
            .get(integers.len() / 2)
            .unwrap());

    //  If array has even number of elements
    // we take avarage of two middle elements.
    if integers.len() % 2 == 0 {
        middle_elem = (
            middle_elem +
            integers
                .get(integers.len() / 2 - 1)
                .unwrap()
        ) / 2;
    }

    //  Average value.
    println!(
        "Mean: {}",
        sum / integers.len() as u32);

    //  When sorted, the value in the middle
    // position.
    println!(
        "Median: {}", middle_elem);

    //  Value that occurs most often.
    println!(
        "Mode: {}",
        elems_counted
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| k)
            .unwrap());
}
