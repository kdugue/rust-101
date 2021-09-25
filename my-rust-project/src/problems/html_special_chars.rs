// https://www.codewars.com/kata/56bcaedfcf6b7f2125001118/rust

/*

Safen User Input Part I - htmlspecialchars
You are a(n) novice/average/experienced/professional/world-famous Web Developer (choose one) who owns a(n) simple/clean/slick/beautiful/complicated/professional/business website (choose one or more) which contains form fields so visitors can send emails or leave a comment on your website with ease. However, with ease comes danger. Every now and then, a hacker visits your website and attempts to compromise it through the use of XSS (Cross Site Scripting). This is done by injecting script tags into the website through form fields which may contain malicious code (e.g. a redirection to a malicious website that steals personal information).

Mission
Your mission is to implement a function htmlspecialchars() that converts the following potentially harmful characters:

< --> &lt;
> --> &gt;
" --> &quot;
& --> &amp;
Good luck :D

*/

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
