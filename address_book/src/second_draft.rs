/*
TODO:
- Create function that loops to find user based on key + the ability to modify key 

TODO:
- Create a loop that only asks the user for data that's needed not where it's stored
*/

use std::collections::HashMap;

fn main() {
    // HashMap to store data, received from the 'heap'
    let mut contacts = HashMap::new();

    // Create contacts, give arguments asked for
    create_contact(&mut contacts, "koen", "kessels", "koen@test123.com", 
    "+31600000000");
    create_contact(&mut contacts, "johan", "kessels", "johan@test123.com", 
    "+31600000000");
    create_contact(&mut contacts, "anja", "kessels", "anja@test123.com", 
    "+31600000000");

    // Check if finding contacts is possible
    find_contact(&mut contacts, "koen");
    find_contact(&mut contacts, "johan");    
}

// Structure for Contact, with key + value type string
#[derive(Debug)]
struct ContactInfo {
    name: String,
    lastname: String,
    phone: String,
    email: String,
}

// Function to create new contact, use: reference to mut so var contacts keeps
// ownership
fn create_contact(store: &mut HashMap<String, ContactInfo>, name: &str, 
    lastname: &str, phone: &str, email: &str) {

        // Use name as ID, .to_string() to receive the right data-type,
        // Contact_info for structure
        store.insert(name.to_string(), ContactInfo {
            name: name.to_string(),
            lastname: lastname.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
        });
}

fn find_contact(store: &mut HashMap<String, ContactInfo>, name: &str) {
    // Access key and value in HashMap
    for (key, value) in store {
        // Check if Key is equal to name
        // Use .to_string() to match the datatype of the key in HashMap
        // Use dereferencing (*) to access value the pointer points to
        if name.to_string() == *key {
            println!("{:?}", value)
        } 
    } 
} 

