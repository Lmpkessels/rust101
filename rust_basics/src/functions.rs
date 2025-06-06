fn main() {
    greet_user("Luuk", 25);

    let numbers = vec![1, 2, 3, 4, 5, 6];
    return_average(numbers);
} // Function greet user gets out of scope here so it get's dropped

/* 
Function greet user, parameter name is data type reference to string literal, 
Parameter age is data type unassigned integer

Both data types are gotten from the stack
*/
fn greet_user(name: &str, age: u8) {
    println!("Hello, {name} you're {age} years old!");
}


fn return_average(vector: Vec<i32>) {
    /* 
    Create mutable variable, 
    default value 0 stored on the heap as assigned integer of 32 bits  
    */
    let mut sum: i32 = 0;
    let mut total: i32 = 0;

    // Get item out of vector, we borrow so the list stays the same
    for item in &vector {
        /*
        We take the default number of sum, then we increment it with the value
        of assigned to the item
        */
        sum += item;
        println!("{:?}", sum);
        
        total + item;
        println!("{}", total);
    }
}

// Create data storage that can be recalled
fn store(name: String, number: i64, storage: &mut Vec<(String, i64)>) {
    storage.push((name, number));
}

fn call_store() {
    let mut contact = Vec::new();

    store(String::from("Luuk"), 31618745600, &mut contact);
    store(String::from("Koen"), 31617188810, &mut contact);

    println!("{:?}", contact);
}