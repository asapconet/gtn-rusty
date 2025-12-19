fn main() {
    let value = String::from("stop suggesting this shit");

    let value_length = length_calculator(&value);
    println!("the string length of '{value}' is {value_length}");
}

fn length_calculator(value: &String) -> usize {
    value.len()
}
