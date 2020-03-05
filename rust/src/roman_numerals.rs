fn arabic_to_roman(number: usize) -> String {
    let arabic = format!("{}", number);
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
    let mut roman = Vec::new();

    for (i, exponent) in exponents.iter().enumerate() {
        match (i, &exponent[..1]) {
            (_, "0") => (),
            (0, x) => roman.push(convert_symbol(x, "I", "V", "X")),
            (1, x) => roman.push(convert_symbol(x, "X", "L", "C")),
            (2, x) => roman.push(convert_symbol(x, "C", "D", "M")),
            (3, x) => roman.push(convert_symbol(x, "M", "V", "X")),
            _ => (),
        }
    }

    roman.reverse();
    String::from(roman.join(""))
}

fn convert_symbol(digit: &str, l: &str, m: &str, u: &str) -> String {
    match digit {
        "0" => panic!("Evil number!"),
        "1" => format!("{l}", l = l),
        "2" => format!("{l}{l}", l = l),
        "3" => format!("{l}{l}{l}", l = l),
        "4" => format!("{l}{m}", l = l, m = m),
        "5" => format!("{m}", m = m),
        "6" => format!("{m}{l}", m = m, l = l),
        "7" => format!("{m}{l}{l}", m = m, l = l),
        "8" => format!("{m}{l}{l}{l}", m = m, l = l),
        "9" => format!("{l}{u}", l = l, u = u),
        _ => panic!("Never"),
    }
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
        let arabics = [13, 90, 76, 35];
        let expected = ["XIII", "XC", "LXXVI", "XXXV"];
        for i in 0..arabics.len() {
            let roman = arabic_to_roman(arabics[i]);
            assert_eq!(&roman, expected[i]);
        }
    }

    #[test]
    fn given_three_digit_numbers_return_correct_roman_numerals() {
        let arabics = [135, 907, 763, 354];
        let expected = ["CXXXV", "CMVII", "DCCLXIII", "CCCLIV"];
        for i in 0..arabics.len() {
            let roman = arabic_to_roman(arabics[i]);
            assert_eq!(&roman, expected[i]);
        }
    }

    #[test]
    fn given_four_digit_numbers_return_correct_roman_numerals() {
        let arabics = [1350, 9073, 7631, 3549];
        let expected = ["MCCCL", "MXLXXIII", "VMMDCXXXI", "MMMDXLIX"];
        for i in 0..arabics.len() {
            let roman = arabic_to_roman(arabics[i]);
            assert_eq!(&roman, expected[i]);
        }
    }
}
