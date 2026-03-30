#[derive(Debug)]
pub enum CustomRange {
    Random,
    Orderly,
}

#[derive(Debug)]
pub enum CheckRange {
    Monthly,
    Weekly,
    Daily,
    Hourly(CustomRange),
}

pub fn match_partners(range: CheckRange) -> u8 {
    match range {
        CheckRange::Monthly => {
            println!("Your maximum checks is 30, upgrade your subscription to increase your limit");
            30
        }
        CheckRange::Weekly => 7,
        CheckRange::Daily => 1,
        CheckRange::Hourly(_range) => {
            println!("Your list will be randomly ordered");
            100
        }
    }
}

// USING OPTIONS<T> IN MATCHING

pub fn options_match() {
    let five = Some(5);
    let _six = add_to_list(five);
    let _seven = add_to_list(None);

    fn add_to_list(list_count: Option<i32>) -> Option<i32> {
        match list_count {
            None => None,
            Some(count) => Some(count + 1),
        }
    }

    let balance = 1000;
    match balance {
        500 => {
            println!("I can double your balance")
        }
        999 => println!("You need to grind some more"),
        1000 => println!("You have a full balance"),
        _ => println!("Your final balance is {}", balance),
    }
}
