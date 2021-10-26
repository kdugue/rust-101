fn xo(string: &'static str) -> bool {
    let mut x_count = 0;
    let mut o_count = 0;

    for ch in string.chars() {
        if ch == 'x' || ch == 'X' {
            x_count += 1;
        }

        if ch == 'o' || ch == 'O' {
            o_count += 1;
        }
    }

    x_count == o_count
}
