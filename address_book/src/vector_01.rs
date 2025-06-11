/*
PROBLEM:
- When I create a new contact, what happens is only the last gets created not
a string of contacts gets stored in the hashmap variable (Solve this)
*/

use std::collections::HashMap;

fn main() {
    // Create new contact with ID
    let create_id_and_contact: &mut HashMap<u32, Contact> = &mut HashMap::new();

    create_contact(create_id_and_contact, "koen", "kessels", "koen@test123.com", true);
    create_contact(create_id_and_contact, "anja", "kessels", "anja@test123.com", true);
    create_contact(create_id_and_contact, "johan", "kessels", "johan@test123.com", true);

    println!("{:?}", create_id_and_contact);
}

#[derive(Debug)]
// Create structure for User
struct Contact {
    // Key + Value, with Data-type
    name: String,
    lastname: String,
    email: String,
}

// Use lifetime so Rust knows everything asked for outlives
fn create_contact<'a>(contact: &'a mut HashMap<u32, Contact>, name: &'a str, lastname: &'a str, 
    email: &'a str, new: bool) -> &'a mut HashMap<u32, Contact> {
        let mut id = 0;
        if new == true {
            id += 1
        }

        // Create logic so the HashMap expands everytime a new Contact is created

        // Insert id + Contact to HashMap
        contact.insert(id, Contact {
            name: name.to_string(),
            lastname: lastname.to_string(),
            email: email.to_string(),
        });

        // Return HashMap
        contact
}

// fn store_contact(store: Vec<HashMap<i32, Contact>>, to_store: HashMap<i32, Contact>) {
    
// }