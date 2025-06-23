#[derive(Debug)]
// Enum to modify address book using structured actions.
enum ModifyAddressBook {
    // Each variant holds a full contact struct as data.
    Add(Contact),
    Delete(Contact),
}

#[derive(Debug, PartialEq)]
struct Contact {
    // Key + Value, each value is given the righ Data-type.
    // Data-types 'heap' allocated, owned by the struct.
    first_name: String,
    last_name: String,
    phone: String,
    email: String,
}

// Creates and returns a new Contact struct.
// Handle ownership, using string slices.
fn create_contact(first_name: &str, last_name: &str, phone: &str, email: &str) 
// Return structure of new Contact.
-> Contact {    
    // Create contact.
    Contact {
        // Change value .to_string() and assign it to key.
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        phone: phone.to_string(),
        email: email.to_string(),
    }
}

// Modify the contact storage (Vec<Contact>) by either adding or deleting.
fn add_contact(contact: ModifyAddressBook, storage: &mut Vec<Contact>) {
    match contact {
        // Match the right contact and move it into the Vector.
        ModifyAddressBook::Add(c) => {
            storage.push(c)
        }
        // Match the right contact if matched remove it from the vector.
        ModifyAddressBook::Delete(c) => {
            storage.retain(|o| *o != c)
        }
    }
}

fn main() {
    // Storage for contacts, stored on 'heap'.
    let mut contacts = Vec::new();

    // Create 2 new contacts, 'heap' allocated data.
    let cont1 = create_contact("luuk", "kessels", "+31618000000", 
    "test@test123.nl");
    let cont2 = create_contact("koen", "kessels", "+31656000000", 
    "test@test123.com");

    // Add cont1, delete cont2 (which was never added — so nothing happens).
    add_contact(ModifyAddressBook::Add(cont1), &mut contacts);
    add_contact(ModifyAddressBook::Delete(cont2), &mut contacts);    

    // Print the current state of the contacts vector.
    println!("{contacts:?}");
}