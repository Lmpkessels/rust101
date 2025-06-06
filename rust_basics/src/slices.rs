fn main() {
    let string = String::from("Hello, Luuk");

    // The pointer in this case, uses the index we set as range, and points
    // to that index in memory

    // Here we fix the total range, and get the range from index 0 till
    // index 5
    let range_both_index_fixed = &string[0..5];
    // Here we get the range starting from the first index till the index we've
    // assigned so index 4
    let range_first_index_fixed = &string[..4];
    // Here we get the 3 index because we tell so till the end of the string
    let range_last_index_fixed = &string[3..];

    println!("{:?}", range_both_index_fixed);
    println!("{:?}", range_first_index_fixed);
    println!("{:?}", range_last_index_fixed);
}


