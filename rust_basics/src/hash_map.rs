use std::collections::HashMap;

fn main() {
    let mut contacts: HashMap<u64, Contact> = HashMap::new();

    let koen = create_new_contact("Koen", "Kessels", "+31618745555", 
    "koenkessels@live.nl", None, None);
    contacts.insert(1, koen);

    println!("{:?}", contacts);
}

#[derive(Debug)]
struct Contact {
    name: String,
    lastname: String,
    phone: String,
    email: String,
    // Use Option<String> to indicate value of this Key is optional
    address: Option<String>,
    post: Option<String>,
}

fn create_new_contact(name: &str, lastname: &str, phone: &str, email: &str, 
    address: Option<&str>, post: Option<&str>) -> Contact {
        Contact {
            name: name.to_string(),
            lastname: lastname.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            /* 
            |a| = closure in rust, like a mini function, 
            it tells rust; take input b, return input b if and only if it contains 
            a value
            */
            address: address.map(|a| a.to_string()),
            /*
            |b| = closure in rust, a closure is like a mini-function,
            it tells rust; take input b, return input b if and only if it contains 
            a value
            */
            post: post.map(|p| p.to_string()),
        }
    }