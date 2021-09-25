fn html_special_chars(html: &str) -> String {
    let mut result: String = "".to_owned();

    for c in html.chars() {
        if c == '<' {
            result.push_str("&lt;")
        } else if c == '>' {
            result.push_str("&gt;")
        } else if c == '"' {
            result.push_str("&quot;")
        } else if c == '&' {
            result.push_str("&amp;")
        } else {
            result.push(c)
        };
    }

    return result;
}
