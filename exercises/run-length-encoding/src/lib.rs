pub fn encode(message: &str) -> String {
  if message == "" { return "".to_string(); }
  let mut code = String::new();
  let mut count = 1;
  let mut iter = message.chars();
  let mut lastc = iter.next().unwrap();
  for c in iter {
    if lastc == c { 
      count += 1
    } else {
      if count > 1 {
        code.push_str(&count.to_string());
      }
      code.push_str(&lastc.to_string());
      count = 1;
    }
    lastc = c;
  }
  if count != 0 {
    if count > 1 {
      code.push_str(&count.to_string());
    }
    code.push_str(&message.chars().last().unwrap().to_string());
  }
  return code
}

pub fn decode(code: &str) -> String {
  let mut count = 0;
  let mut message = String::new();
  for c in code.chars() {
    if c.is_digit(10) {
      count = count * 10 + c.to_digit(10).unwrap();
    } else {
      if count > 0 {
        message.push_str((0..count).map(|_| c).collect::<String>().as_str());
      } else {
        message.push_str(&c.to_string());
      }
      count = 0;
    }
  }
  message
}