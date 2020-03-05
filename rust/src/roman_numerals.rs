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
    println!("EXPONENTS: {:?}", exponents);
    let mut roman = Vec::new();

    for (i, exponent) in exponents.iter().enumerate() {
        match (i, exponent) {
            (0, x) => match &x[..] {
                "0" => (),
                "1" => roman.push("I"),
                "2" => roman.push("II"),
                "3" => roman.push("III"),
                "4" => roman.push("IV"),
                "5" => roman.push("V"),
                "6" => roman.push("VI"),
                "7" => roman.push("VII"),
                "8" => roman.push("VIII"),
                "9" => roman.push("IX"),
                _ => panic!("Never"),
            },
            (1, x) => match &x[..] {
                "0" => (),
                "10" => roman.push("X"),
                "20" => roman.push("XX"),
                "30" => roman.push("XXX"),
                "40" => roman.push("XL"),
                "50" => roman.push("L"),
                "60" => roman.push("LX"),
                "70" => roman.push("LXX"),
                "80" => roman.push("LXXX"),
                "90" => roman.push("XC"),
                _ => panic!("Never"),
            },
            _ => (),
        }
    }

    roman.reverse();
    println!("ROMAN: {:?}", roman);
    String::from(roman.join(""))
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
    fn given_two_digit_numbers_return_correct_roman_numerals() {
        let arabics = [13, 90];
        let expected = ["XIII", "XC"];
        for i in 0..arabics.len() {
            let roman = arabic_to_roman(arabics[i]);
            assert_eq!(&roman, expected[i]);
        }
    }

    #[test]
    fn given_13_return_XIII() {
        let arabic = 13;
        let expected = "XIII";
        let roman = arabic_to_roman(arabic);
        assert_eq!(&roman, expected);
    }
}
