use std::env::args;

mod functions;

fn main() {
    let args: Vec<String> = args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let numbers = [1, 2, 3, 4, 5];
    let mut count = 0;
    let result = loop {
        count += 1;
        if count + 1 >= numbers.len() {
            break numbers[count];
        }
    }; // This is an assignment (a statement!) so we need to put semicolon here.
}
