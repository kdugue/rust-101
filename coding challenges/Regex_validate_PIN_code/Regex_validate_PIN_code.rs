fn validate_pin(pin: &str) -> bool {
    let mut count = 0;

    for ch in pin.chars() {
        if !ch.is_digit(10) || count > 6 {
            return false;
        }
        count += 1;
    }

    count == 4 || count == 6
}

// TODO: add alternate solution
