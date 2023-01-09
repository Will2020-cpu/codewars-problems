fn main (){ 
    println!("{}", square_digits(9119));
}

fn square_digits(num: u64) -> u64 {
    let mut string = String::new();

    for (_i, c) in num.to_string().chars().enumerate(){
        string.push_str(&(c.to_digit(10).unwrap_or(0) * c.to_digit(10).unwrap_or(0)).to_string());
    }
    return string.parse().unwrap_or(0);
}

// Best solution
// fn square_digits(num: u64) -> u64 {
//     num
//         .to_string()
//         .chars()
//         .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
//         .collect::<String>()
//         .parse()
//         .expect("result isnt u64 parsable")
// }

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
