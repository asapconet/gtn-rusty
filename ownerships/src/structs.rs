// PURE STRUCTS
struct UserData {
    name: String,
    age: u16,
    active: bool,
    gender: String,
    number_of_followers: u64,
}

// TUPLE STRUCTS
struct Physic(i32, String);
struct ExchangeRate(i32, String);

pub fn run() {
    let new_user = UserData {
        active: true,
        name: String::from("Aaron"),
        gender: String::from("male"),
        age: 55,
        number_of_followers: 9999999,
    };

    println!(
        "old user info: has {} followers",
        new_user.number_of_followers.to_string()
    );

    let other_user = UserData {
        name: String::from("Lola"),
        number_of_followers: 2000,
        ..new_user
    };

    println!("updated user info:  {}", other_user.name);

    // the operation in other_user takes only the name key in the UserData struct
    // and that renders the new_user invalid as well as it values[can be read however].
    // Now for the rest of the values in the struct they remain unused until a mutate them.

    let user_look = Physic(6, String::from("ft"));
    let todays_rate = ExchangeRate(2342, String::from("USD to AUS"));

    println!(
        "todays {} price is {}",
        todays_rate.1,
        todays_rate.0.to_string()
    );
}
