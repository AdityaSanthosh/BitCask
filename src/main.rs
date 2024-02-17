use std::io;

fn main() {
    println!("Welcome to BitCask, a key-value Data Store!\nChoose any of the following options");
    println!("1. Insert a key\n2. Get Key Value\n3. Update Key\n4. Delete key");
    let mut raw_choice = String::new();
    io::stdin().read_line(&mut raw_choice).expect("Failed to read choice");
    let choice: i32 = raw_choice.trim().parse().expect("Not an Integer");
    print!("You entered {choice}");
    // Todo: Create functions/interfaces of the CRUD operations
    // Todo: Run the function based on the user choice
}

/*
This is only going to be a key-value store
What are the operations?
1. Insert
2. Get
3. Update
4. Delete
5. Automatic Internal Compaction

Steps
1. Console Program
2. Server with API's
3. [Optional] Create a client library
*/