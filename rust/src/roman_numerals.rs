fn arabic_to_roman(number: usize) -> String {
    let arabic = format!("{}", number);

    // split into exponents
    // e.g. 249 = [200,40,9]
    let mut exponents = Vec::<String>::new();
    for (i, digit) in arabic.chars().enumerate() {
        let exp = format!(
            "{}{}",
            digit,
            vec![0; arabic.len() - 1 - i]
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
        exponents.push(exp);
    }

    exponents.reverse();

    let mut roman = String::from("");

    for (i, exponent) in exponents.iter().enumerate() {
        roman += match (i, exponent) {
            (0, x) => match &x[..] {
                "1" => "I",
                "2" => "II",
                "3" => "III",
                "4" => "IV",
                "5" => "V",
                "6" => "VI",
                "7" => "VII",
                "8" => "VIII",
                "9" => "IX",
                _ => panic!("Never"),
            },
            _ => "",
        }
    }

    roman
    // String::from("FU")

    // match &arabic[..] {
    //     "1" => String::from("I"),
    //     "2" => String::from("II"),
    //     "3" => String::from("III"),
    //     "4" => String::from("IV"),
    //     "5" => String::from("V"),
    //     "6" => String::from("VI"),
    //     "7" => String::from("VII"),
    //     "8" => String::from("VIII"),
    //     "9" => String::from("IX"),
    //     "10" => String::from("X"),
    //     _ => panic!("Not implemented!"),
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_single_digit_return_correct_symbol() {
        let arabics = 1..10;
        let expected = ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"];
        for arabic in arabics {
            let roman = arabic_to_roman(arabic);
            assert_eq!(&roman, expected[arabic - 1]);
        }
    }

    #[test]
    fn given_13_return_XIII() {
        let arabic = 13456;
        let expected = "XIII";
        let roman = arabic_to_roman(arabic);
        assert_eq!(&roman, expected);
    }
}
