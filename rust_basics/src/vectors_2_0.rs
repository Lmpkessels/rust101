fn main() {
    // Create vector
    let mut original_vect = vec![];
    // Move ownership 
    let mut new_vect = original_vect;

    // Loop through range 0 to 24
    for vec in 0..24 {
        new_vect.push(vec);
    }


    /* 
    Borrowing vector instead of taking ownership so we can use it through the 
    whole program in the same state
    */
    borrow_vector(&new_vect);

    // Because we've borrowed the vector it's owner will stay the same
    println!("{:?}", new_vect);

    // New owner
    // move_vector(new_vect);

    // Won't work anymore ownership is taken through the function
    // println!("{:?}", new_vect)

}

// Function taking ownership
fn move_vector(v: Vec<i32>) {
    println!("{:?}", v);
}

// Function borrowing vector
fn borrow_vector(v: &Vec<i32>) {
    println!("{:?}", v);
}