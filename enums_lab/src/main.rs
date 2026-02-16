enum OptionValue {
    Some(i32),
    None,
}

// Returns an OptionValue so both Some and None code paths are exercised.
fn get_value(should_have_value: bool) -> OptionValue {
    if should_have_value {
        OptionValue::Some(10)
    } else {
        OptionValue::None
    }
}

fn main() {
    let value = get_value(true);

    let OptionValue::Some(v) = value else {
        panic!("No value");
    };

    println!("Value is {}", v);
}
