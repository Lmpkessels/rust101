fn main() {
    // List a of 8 bits
    let a = [0, 1, 1, 0, 0, 1, 1, 1];
    // List b of 8 bits
    let b = [1, 0, 0, 1, 1, 1, 1, 0];

    // Operator of choice
    let operator = "-";
    // Default borrow in
    let mut borrow_in = 0;
    // Default carry in
    let mut carry_in = 0;

    // Total list after calculation
    let mut total = [0; 8];

    // Check operator, if '-' follow commands
    if operator == "-"{
        // Reverse list then get each bit in range 0..8 individually
        for bit in (0..8).rev() {
            /* 
            Access subtract function, assign operator a and b to bit for list, 
            do: calculation bit-by-bit
            */
            let (difference, borrow_out) = subtract_bit(a[bit], b[bit], borrow_in);
            // Calculation done? Give back total difference in 8-bit list
            total[bit] = difference;
            // Communicate underflow previous bit
            borrow_in = borrow_out;
        }
        // Check out if it works as planned
        println!("Total: {:?}", total);
        println!("Borrow in: {}", borrow_in);
    }

    // Check operator, if '+' follow commands
    if operator == "+" {
        // Reverse list then get each bit in range 0..8 individually
        for bit in (0..8).rev() {
            /* 
            Access addition function, assign operator a and b to bit for list, 
            do: calculation bit-by-bit
            */
            let (whole, carry_out) = add_bit(a[bit], b[bit], carry_in);
            // Calculation done? Give back total difference in 8-bit list
            total[bit] = whole;
            // Communicate overflow previous bit
            carry_in = carry_out;
        }
        // Check out if it works as planned
        println!("Total: {:?}", total);
        println!("Carry in: {}", carry_in);
    }

}

// Indicate data types used to parameter, and within function
fn subtract_bit(a: i8, b: i8, borrow_in: i8) -> (i8, i8) {
    // Check if a is greater than or equal to if false
    if a >= b + borrow_in {
        // Don't borrow and follow calculation
        (a - b - borrow_in, 0)
    } else {
        // Borrow out and follow calculation
        (a + 2 - b - borrow_in, 1)
    }
}

// Indicate data types used to parameter, and within function
fn add_bit(a: i8, b: i8, carry_in: i8) -> (i8, i8) {

    // Add whole together
    let whole = a + b + carry_in;
    // Calculate sum, calculate carry out 
    (whole % 2, whole / 2)
}