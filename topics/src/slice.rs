pub fn run() {
    let s = String::from("The boy wanna learn so badly");
    let the_word = get_first_word(&s);
    let proper_word = get_proper_sliced_word(&s);

    let len = s.len();
    let other_words = &s[7..len];
    let native_slice = get_first_sliced_word(&s);

    println!("the first word takes {} bytes", the_word);
    println!("the properly sliced word is {}", proper_word);
    println!("the other words are {}", other_words);

    println!("the natively sliced word is {}", native_slice);
}

// this method returns the size of the first word in bytes
fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

// this method returns the actual first word as string
fn get_first_sliced_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

// BONUS: the method returns the first word in a string using [..] parameters
fn get_proper_sliced_word(s: &String) -> String {
    let first_word = &s[..3];

    return first_word.to_string();
}
