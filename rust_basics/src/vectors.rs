fn main() {
    /* 
    Create vector that takes in element types as Sting, stored on the heap, 
    has no prefixed elements in it
    */
    let mut names: Vec<String> = Vec::new();

    /* 
    Here i refer to names, will give me an empty list, 
    is stored on stack based as owner is at given time 
    */
    let reference = &names;
    /* 
    (I think after questioning the why) that :? is there to create at compile 
    time 
    */
    println!("{:?}", reference);
    
    /* 
    All strings are stored on the heap, to calculate specific amount of bits i 
    need ASKII

    They're stored on the heap because they can change in size

    We push each string to the Vector where it will take place
    */
    names.push(String::from("Koen")); // Index location 0
    names.push(String::from("Anja")); // Index location 1
    names.push(String::from("Johan")); // Index location 2
    names.push(String::from("Luuk")); // Index location 3
    names.push(String::from("Denny")); // Index location 4
    names.push(String::from("Nick")); // Index location 5

    println!("{:?}", names);

    /* 
    Here I greet every user, by looping through the Vector and getting 
    each stored element individually 
    */
    for name in &names {
        println!("Hello, {}!", name)
    }

    /*
    NOTE: Each function has owners, and borrowers, so remember if stored on the 
    stack then they have a location from top to bottom || lifo, aka last in first 
    out

    So if we have a table, then each owner has a address, name, value, and each
    borrower has a addres, name, value (in this case the address of wich it borrows)

    So for example: The first Variable within a_vec = a mutable Vector called
    number_v and has the elements 1, 2, 3, and is stored on the heap taking in a 
    amount of 96 bits

    This function has a scope and the functions called within this function are
    now part of that scope so if you visualise, then this scope will go till
    end of function c_vec
    */

    // Call function A
    a_vec();

    // Call function B
    b_vec();

    // Call function C
    c_vec();
}

fn a_vec() {
    /* 
    Mutable prefixed vector with 3 elements, using a macro call so it can create
    at compile time, the total amount of bits recerved on the heap is 96 bits
    */
    let mut number_v: Vec<i32> = vec![1, 2, 3];
    
    // Push integer 4, total amount of bits recerved on heap now is 128
    number_v.push(4);
    println!("{:?}", number_v);
    
    /* 
    We pop of elements from the vector meaning we take of elements, 
    it will start at -1 wich is the last element of the list in this case index 
    3 
    */
    number_v.pop();
    number_v.pop();
    number_v.pop();
    number_v.pop();
    
    println!("{:?}", number_v);
}

fn b_vec() {
    /*
    NOTE: The elements are probably stored on the stack in prefixed size

    Create vector with a prefixed size of 3 elements, Vector itself stored on heap

    Total bits reserved on heap are 96
    */
    let number_vec: Vec<i32> = vec![100, 200, 300];

    /* 
    We get the element on index 1 out wich lives on the heap

    .get() returns an Option<&i32>, and that reference lives on the stack, but 
    points to the heap

    received number 200 
    */
    let get_1 = number_vec.get(1);
    
    // We get the element on index 1 out wich is stored nowhere, received None
    let get_2 = number_vec.get(99);

    println!("{:?}", get_1);
    println!("{:?}", get_2);
}

fn c_vec() {
    /* 
    Create vector wich refers to a String item, no prefixed size, 
    stored on the heap 
    */
    let mut number_vect: Vec<&str> = Vec::new();

    /* 
    First we push some items towards the heap, on the Vector, hmmm if they are
    references do they then go on the heap, no. So the Vector will be on the heap
    and the &str will be on the stack
    */
    number_vect.push("Koen");
    number_vect.push("Anja");
    number_vect.push("Johan");
    number_vect.push("Luuk");
 
    // But if we check the length then we get for so the Vector definitely has become
    // 4 it's getting complicated
    // But i think it refers to the items on the stack
    let length = number_vect.len();

    println!("{}", length);
}