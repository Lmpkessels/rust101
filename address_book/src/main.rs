use std::collections::HashMap;
fn main() {
    /* 
    Create mutable Hashmap, ID = u32 growable till a range of ~4.2 Billion. 
    
    contacts is the owner of the HasMap and allocated on the heap
    */
    let mut contacts: HashMap<u32, Contact> = HashMap::new();

    // Create two contacts to check if it works
    let anja: Contact = add_new_contact("anja", "kessels", "+31600000000",
    "anja@test321.nl", None, None);
    // We use: Some() here to indicate there's a value assigned to the key
    let koen: Contact = add_new_contact("Koen", "Kessels", "+31612345678",
    "koen@test123.nl", None, Some("Family"));

    /* 
    Access HashMap, use .insert to insert both variable that contain user data 
    + allocate a ID so it can be found fast.

    Both variables are now part of the hashmap and stored on the heap
    */
    contacts.insert(1, anja);
    contacts.insert(2, koen);

    println!("{:?}", contacts);
}

#[derive(Debug)]
// We allow dead code because we haven't used it yet (still testing)
#[allow(dead_code)]
// Create the structure of the HashMap
struct Contact {
    name: String,
    lastname: String, 
    phone: String,
    email: String, 
    // Leave these optionall
    /*
    NOTE: About Option<type>, it would be better to use option for each value 
    of the key because sometimes only one or 2 get filled in

    Action step:
    - Write logic and change every type of the key to optional, 
    then when a field is filled accept the contact to be true
    */
    address: Option<String>,
    tags: Option<String>,
}

/*
Create function to add contacts to Contact HashMap

Use references as values for parameters, in this way we can use normal string 
while creating the new user
*/
fn add_new_contact(name: &str, lastname: &str, phone: &str, email: &str, 
    address: Option<&str>, tags: Option<&str>) -> Contact {
    Contact {
        /* 
        Use: .to_string() to transform values from key to a heap-allocated 
        string 
        */
        name: name.to_string(),
        lastname: lastname.to_string(),
        phone: phone.to_string(),
        email: email.to_string(),
        /*
        Use: .map as method to take a closure, so we can take in Option<type>
        it's rust anonymous use of a lambda function

        It's a closure wrapped value and a in acts as parameter so it takes a 
        value
        */
        address: address.map(|a| a.to_string()),
        tags: tags.map(|t| t.to_string()),
    }
}    

