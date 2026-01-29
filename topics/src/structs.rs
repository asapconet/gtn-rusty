// PURE STRUCTS

#[derive(Debug)]
struct UserData {
    name: String,
    age: u32,
    active: bool,
    gender: String,
    number_of_followers: u32,
}

struct UserLongevity {
    age: u32,
    number_of_followers: u32,
}

// TUPLE STRUCTS
struct ExchangeRate(i32, String);

//METHODS
impl UserLongevity {
    fn longevity(&self) -> u32 {
        self.age * self.number_of_followers / 5
    }
}

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

    println!("updated user info:  {:#?}", other_user);

    // the operation in other_user takes only the name key in the UserData struct
    // and that renders the new_user invalid as well as it values[can be read however].
    // Now for the rest of the values in the struct they remain unused until a mutate them.

    let todays_rate = ExchangeRate(2342, String::from("USD to AUS"));

    println!(
        "todays {} price is {}",
        todays_rate.1,
        todays_rate.0.to_string()
    );

    let user_longevity = UserLongevity {
        age: new_user.age,
        number_of_followers: new_user.number_of_followers,
    };

    println!(
        "the user is set to be online for {} mins",
        user_longevity.longevity()
    )
}

// Generally struct are like setting types or interfaces in js, but for rust this time
// it remains strict as usual and everything must be allowed either for a read or write.
// If mutable all must be and not just one, it shouts as in ts too when not all the keys and velues are used
