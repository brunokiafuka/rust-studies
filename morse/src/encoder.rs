pub fn encode(text: &str) -> Result<String, &str> {
    let to_lowercase = text.to_lowercase();
    let chars = to_lowercase.trim().chars();

    let mut result = String::new();
    let mut error_values = vec![];

    for c in chars {
        let code = match c {
            'a' => "._",
            'b' => "_...",
            'c' => "_._.",
            'd' => "_..",
            'e' => ".",
            'f' => ".._.",
            'g' => "__.",
            'h' => "....",
            'i' => "..",
            'j' => ".___",
            'k' => "_._",
            'l' => "._..",
            'm' => "__",
            'n' => "_.",
            'o' => "___",
            'p' => ".__.",
            'q' => "__._",
            'r' => "._.",
            's' => "...",
            't' => "_",
            'u' => ".._",
            'v' => "..._",
            'w' => ".__",
            'x' => "_.._",
            'y' => "_.__",
            'z' => "__..",
            ' ' => "/",
            _ => {
                error_values.push(c.to_string());
                "#"
            }
        };

        result.push_str(code);
        result.push(' ');
    }

    result.pop();

    if error_values.len() == 0 {
        Ok(result)
    } else {
        dbg!(error_values);
        Err("Error")
    }
}
