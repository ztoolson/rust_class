use std::io;

fn main() {
    // 1. Take n:
    //      if n is odd, set n = 3n+1
    //      else set n = n/2
    // 2. Repeat until 1 is reached

    println!("Welcome! please enter a number:");

    // Read number from user
    let mut input_text = String::new();

    let mut reader = io::stdin();
    reader.read_line(&mut input_text)
        .ok()
        .expect("failed to read line");
    let input_opt: Option<i32> = input_text.trim().parse::<i32>().ok();
    let input_int = match input_opt {
        Some(input_int) => input_int,
        None => {
            println!("please enter a number");
            return;
        }
    };

    // Calculate collatz steps
    println!("{} has {} Collatz staps", input_int, collatz(input_int));


}

fn collatz(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3+1) }
    }
}
