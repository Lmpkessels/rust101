#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
struct NewContact {
    name: MyOption<String>,
    last_name: MyOption<String>,
    email: MyOption<String>,
    phone: MyOption<String>, 
}

fn create_new_contact(name: &str, last_name: &str, email: &str, phone: &str) 
-> NewContact {
    NewContact {
        name: MyOption::Some(name.to_string()),
        last_name: MyOption::Some(last_name.to_string()),
        email: MyOption::Some(email.to_string()),
        phone: MyOption::Some(phone.to_string()),
    }
}

fn main() {
    let luuk = create_new_contact("Luuk", "Kessels", "Luuk@test123.nl", 
    "+31618745600");
    println!("{:?}", luuk);
}