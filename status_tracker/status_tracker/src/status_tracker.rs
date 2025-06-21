#[derive (Debug)]
// Status enum with variants containing data-type, Active and Banned from 'Stack',
// Paused from 'Heap'.
enum Status {
    Active(bool),
    Paused(String),
    Banned(u32),
}

#[derive (Debug)]
// Stuct for User, 2 named keys, data type.
struct User {
    // Get String literal from 'Heap'.
    name: String,
    // Use enum for status
    status: Status,
}

// User log in, use String slice so function doesn't take ownership.
// Return User structure.
fn log_in(name: &str, status: Status) -> User {
    User {
        // Change value .to_string() so it receives the right type.
        name: name.to_string(),
        // Status will match key + variant.
        status,
    }
}

// Refer to Status else ownership is taken.
fn check_status(status: &Status) {
    // Match parameter, so enum Status.
    match status {
        // Left side = match, right is what returned.
        Status::Active(b) => {
            // Use dereference, because a match tree automatically refers to.
            if *b {
                // Return if Active status is True.
                println!("Is active.");
            } else {
                // Return if Active status is False.
                println!("Is inactive.")
            }
        },
        Status::Paused(msg) => println!("Is paused: {msg:?}"),
        Status::Banned(code) => println!("Is banned with code: {code:?}"), 
    }
}

// Implement user to access everything by &self.
impl User {
    // Fn returning a warn message when user banned.
    fn warn_if_banned(&self) {
        // If pattern matches self.status,
        if let Status::Banned(code) = &self.status {
            // Return message with username.
            println!("{:?} is banned. Access declined.", self.name)
        }
    }
}

// Implement status to access everything by &self.
impl Status {
    // Check if user is active, return true OR false.
    fn active_msg(&self) -> bool {
        // Check wich data-type status contains, then make decision what to return.
        if let Status::Active(b) = &self {
            // Return if true.
            true
        } else {
            // Return if false.
            false
        }
    }
}

fn main() {
    // Create users, to match every status, and everything works.
    let user1 = log_in("test2", Status::Paused("Having lunch".to_string()));
    let user2 = log_in("test3", Status::Banned(1223));
    let user3 = log_in("test1", Status::Active(true));
    
    // Check full match tree and display every user.
    println!("{user1:?}");
    println!("{user2:?}");
    println!("{user3:?}");
    
    // Display warn message.
    user2.warn_if_banned();

    // Display active message.
    let check = user3.status.active_msg();
    println!("{check:?}");
}