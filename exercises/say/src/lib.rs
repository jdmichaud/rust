//
// See Rust Language Specific Instructions
// below normal exercise description.
//

pub fn encode(n: usize) -> String {
  pub fn spell_number(v: usize) -> String {
    match v {
      0 => "zero".to_string(),
      1 => "one".to_string(),
      2 => "two".to_string(),
      3 => "three".to_string(),
      4 => "four".to_string(),
      5 => "five".to_string(),
      6 => "six".to_string(),
      7 => "seven".to_string(),
      8 => "eight".to_string(),
      10 => "ten".to_string(),
      11 => "eleven".to_string(),
      12 => "twelve".to_string(),
      13 => "thirteen".to_string(),
      14 => "fourteen".to_string(),
      15 => "fivteen".to_string(),
      16 => "sixteen".to_string(),
      17 => "seventeen".to_string(),
      18 => "eighteen".to_string(),
      19 => "nineteen".to_string(),
      _ => panic!("unknown digit {}", v),
    }
  }
  pub fn spell_dozens(v: usize) -> String {
    match v {
      2 => "twenty".to_string(),
      3 => "thirty".to_string(),
      4 => "forty".to_string(),
      5 => "fivty".to_string(),
      6 => "sixty".to_string(),
      7 => "seventy".to_string(),
      8 => "eighty".to_string(),
      9 => "ninety".to_string(),
      _ => panic!("Unknown dozens {}", v), 
    }
  }
  pub fn spell_unit(v: usize) -> String {
    match v {
      2 => "thousand".to_string(),
      3 => "million".to_string(),
      4 => "billion".to_string(),
      5 => "trillion".to_string(),
      6 => "quadrillon".to_string(),
      7 => "quintillion".to_string(),
      _ => panic!("Way to much {}", v),
    }
  }
  pub fn spell_hundreds(v: usize) -> String {
    let mut result: Vec<String> = vec![];
    let hundreds = v / 100;
    let mut rest = v - ((v / 100) * 100);
    let dozens = if rest > 19 { rest / 10 } else { 0 };
    rest = rest - dozens * 10;

    if hundreds != 0 {
      result.push(format!("{} hundred", spell_number(hundreds)));
    }
    if hundreds != 0 && (dozens != 0 || rest != 0) {
      // result.push(" and ".to_string());
      result.push(" ".to_string());
    }
    if dozens != 0 {
      result.push(spell_dozens(dozens));
    }
    if dozens != 0 && rest != 0 {
      result.push("-".to_string());
    }
    if result.len() == 0 || rest != 0 {
      result.push(spell_number(rest));
    }
    result.join("")
  }

  let mut result: Vec<String> = vec![];
  let mut thousands: Vec<usize> = vec![];
  let mut m = n;
  while m > 999 {
    thousands.push(m % 1000);
    m = m / 1000;
  }
  thousands.push(m);
  while thousands.len() > 1 {
    match thousands.pop().unwrap() {
      0 => (),
      x => result.push(format!("{} {}", spell_hundreds(x), spell_unit(thousands.len() + 1))),
    }
  }
  match thousands.pop() {
    Some(0) if result.len() == 0 => result.push("zero".to_string()),
    Some(0) => (),
    Some(x) => result.push(spell_hundreds(x)),
    None => (),
  }
  result.join(" ")
}
