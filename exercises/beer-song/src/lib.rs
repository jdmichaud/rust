pub fn verse(n: i32) -> String {
  pub fn plural(m: i32) -> String {
    if m == 1 { "".to_string() } else { "s".to_string() }
  }
  pub fn count(m: i32, beginning: bool) -> String {
    match (m, beginning) {
      (0, true) => "No more".to_string(),
      (0, false) => "no more".to_string(),
      (x, _) => x.to_string(),
    }
  }
  pub fn it(m: i32) -> String {
    if m == 1 { "it".to_string() } else { "one".to_string() }
  }
  let first_verse = format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\n",
        count(n, true), plural(n), count(n, false), plural(n));
  let second_verse = if n != 0 {
    format!("Take {} down and pass it around, {} bottle{} of beer on the wall.\n", it(n), count(n - 1, false), plural(n - 1))
  } else {
    "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
  };

  return first_verse + &second_verse;
}

pub fn sing(start: i32, end: i32) -> String {
  (end..start + 1).rev().map(|n| verse(n)).collect::<Vec<String>>().join("\n")
}
