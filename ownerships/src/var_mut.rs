pub fn run() {
    // tuples
    tuple_fn();
    //functions with arguments and return values
    let calculate_average_age = calculate_average_age(10, 30, 40);
    println!("Their average age is {} years old.", calculate_average_age);
    print!("");

    //control flow -- loops
    loop_control();
    while_control(5);

    for_control();
}

// deep
fn tuple_fn() {
    let tuple = ("This man here", 42u32);
    let (name, _age_u32): (&str, u32) = tuple;
    let show_tuple: u32 = tuple.1;

    println!("{} is {} years old.", name, show_tuple);
}

// deeper
fn calculate_average_age(child_age: u32, mothers_age: u32, fathers_age: u32) -> u32 {
    println!("The child is {} years old.", child_age);
    println!("The mother is {} years old.", mothers_age);
    println!("The father is {} years old.", fathers_age);
    child_age + mothers_age + fathers_age / 3
}

// control flow
// loop
fn loop_control() {
    let mut count = 0;
    let result = loop {
        count += 2;
        println!("Count is now {}", count);
        if count == 8 {
            print!("Yes we've gotten to the spot now");
            break count;
        } else {
            println!("We are still counting");
        }
    };
    print!(" The result is {}", result)
}

// while
fn while_control(x: u32) {
    let mut initial_count: u32 = x;
    while initial_count != 0 {
        println!("We are coming until we get to the end by {}", initial_count);
        initial_count -= 1;
    }
    println!("We have reached the end now");
}

// for
fn for_control() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for element in array.iter() {
        println!("The value is {}", element);
    }

    for number in 1..=5 {
        println!("The number is {}", number);
    }
}
