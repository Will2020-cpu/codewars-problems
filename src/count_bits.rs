
fn main() {
   println!("{}",count_bits(1234));
}


fn count_bits(n: i64) -> u32 {
  let convert = format!("{n:b}");
  let mut result = 0;
  for (i,c) in convert.chars().enumerate(){
    result += c.to_digit(10).unwrap();
  }
  
  return result;
}

// Add your tests here.
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}