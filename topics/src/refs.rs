pub fn run() {
    let mut value = String::from("stop suggesting this shit");

    let value_length = length_calculator(&value);
    println!("the string length of '{value}' is {value_length}");

    // mutating references
    update_value(&mut value);

    // multiple mutable references
    {
        let k1 = &mut value;
        println!("the first mutation is valid and hence we have '{k1}'");
    }
    let k2 = &mut value;
    println!("the second mutation is valid and hence we have '{k2}'");

    // mixing mutable and immutable refs
    let rw1 = &value;
    let rw2 = &value;
    println!("the immutable references are valid and hence we have '{rw1}' and '{rw2}'");

    //the mix
    let rw3 = &mut value;
    println!("the mutable reference is valid and hence we have '{rw3}'");

    // DANGLING REFERENCES
    dangle();
}

fn length_calculator(value: &String) -> usize {
    value.len()
}

fn update_value(value: &mut String) {
    value.push_str(" anymore");
    println!("the new value is '{value}'");
}

fn dangle() -> String {
    let s = String::from("danglover");

    s
}

// * Generally referencing is more like giving permissions to values, where the &=read, and the &mut = read and write
// Also this permissions is not possible when the original value is not mutable, you cannot have one value
// with giving refs of both read and write same time.

// * Also, when you have a mutable reference, you cannot have any other references to the same value, as it would lead to data races.
// Thats to say until the usage of that mutable reference is dropped you cannot have any other ref to same value

// * For dangles, you cannot return a reference to a local variable, as it would be dropped at the end of the function scope.
// Thats just like not returning anything when you declear a function with operations and not returning anything
