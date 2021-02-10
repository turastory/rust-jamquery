use std::fmt::Display;

pub fn test() {
    let a = [1, 2, 3, 4, 5];
    let a: Vec<i32> = vec![1, 2, 3];
    let a = [3; 5];

    print_vector(a.to_vec())
}

fn print_vector<T: Display>(arr: Vec<T>) {
    let mut cnt = 0;
    loop {
        match arr.get(cnt) {
            Some(v) => print!("{}", v),
            None => println!("End"),
        }
        cnt += 1;
        if cnt >= arr.len() {
            break;
        }
    }
    println!()
}
