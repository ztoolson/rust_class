use std::env;

fn main() {
    let num_collatz_steps: i32 = get_input_argument();
    assert!(num_collatz_steps > 0);

    let mut counter = 1;
    while collatz_steps(counter) != num_collatz_steps {
        counter += 1;
    }

    println!("output: {} for the collatz_steps {}", counter, num_collatz_steps);
}

fn collatz_steps(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    match n % 2 {
        0 => { 1 + collatz_steps(n/2) }
        _ => { 1 + collatz_steps(n*3 + 1) }
    }
}

fn get_input_argument() -> i32 {
    let cmd_args: Vec<String> = env::args()
        .map(|x| x.to_string())
        .collect();

    if cmd_args.len() < 2 {
        println!("Error: Please provide an argument");
        return -1;
    }
    let input: Option<i32> = cmd_args[1].trim()
        .parse::<i32>()
        .ok();
    let input_int = match input {
        Some(input_int) => input_int,
        None => {
            println!("Please enter a number");
            return -1;
        }
    };

    return input_int;
}
