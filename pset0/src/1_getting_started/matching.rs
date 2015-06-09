fn main() {
    // An exercise in Matching
    let x = (41, true);

    match x {
        (20...26, true) => { println!("a: true boolean with number between 20...26"); }
        (40...49, _)    => { println!("c: number is between 40...49 and bool is anything"); }
        (_, true)       => { println!("b: true boolean with a non 20...26 number"); }
        _               => { println!("d: default"); }
    }
}
