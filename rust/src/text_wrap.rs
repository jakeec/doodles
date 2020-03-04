pub fn wrap_text(text_to_wrap: String, max_line_length: i32) -> String {
    let words: Vec<&str> = text_to_wrap.split(" ").collect();

    let mut i = 0;
    let mut wrapped_text = String::from("");
    while i < words.len() {
        let mut chars_this_line: i32 = 0;
        wrapped_text += words[i];
        wrapped_text += " ";
        chars_this_line += (words[i].len() + 1) as i32;
        let mut j = i + 1;
        if j < words.len() {
            while max_line_length - chars_this_line >= words[j].len() as i32 {
                chars_this_line += words[j].len() as i32;
                wrapped_text += words[j];
                if max_line_length - chars_this_line > 0 {
                    chars_this_line += 1;
                    wrapped_text += " ";
                }
                i += 1;
                if j < words.len() - 1 {
                    j += 1;
                }
            }
            wrapped_text += "\n";
        }
        i += 1;
    }

    wrapped_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_words_of_combined_length_20_and_max_line_length_of_15_wrap_over_two_lines() {
        let result = wrap_text(String::from("1234567890 1234567890"), 15);
        println!("{}", result);
        println!("{:?}", result.split("\n").collect::<Vec<&str>>());
        assert_eq!(result.split("\n").collect::<Vec<&str>>().len(), 2);
    }
}
