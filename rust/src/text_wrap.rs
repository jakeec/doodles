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

    String::from(wrapped_text.trim_end_matches('\n'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_two_words_of_combined_length_20_and_max_line_length_of_15_wrap_over_two_lines() {
        let result = wrap_text(String::from("1234567890 1234567890"), 15);
        assert_eq!(result.split("\n").collect::<Vec<&str>>().len(), 2);
    }

    #[test]
    fn given_block_of_text_wrap_lines() {
        let result = wrap_text(
            String::from("The quick brown fox jumped over the lazy dog"),
            15,
        );
        assert_eq!(result.split("\n").collect::<Vec<&str>>().len(), 3);
    }

    #[test]
    fn given_block_of_text_wrap_lines_width_3() {
        let result = wrap_text(
            String::from("The quick brown fox jumped over the lazy dog"),
            3,
        );
        assert_eq!(result.split("\n").collect::<Vec<&str>>().len(), 9);
    }
}
