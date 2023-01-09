fn main (){
    println!("{}", digital_root(942));
}

fn digital_root(n: i64) -> i64 {
    let mut sum: i64 = 0;

    for (_i, c) in n.to_string().chars().enumerate(){
        sum += c.to_digit(10).unwrap_or(0) as i64;
    }

    if sum.to_string().len() != 1{
        sum = digital_root(sum as i64);
    }

    return sum;
}

// Best solution
// fn digital_root(n: i64) -> i64 {
//     (n - 1) % 9 + 1
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
      assert_eq!(digital_root(16), 7);
    }    
}