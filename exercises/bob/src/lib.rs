pub fn reply(message: &str) -> &str {
  if message.trim().len() == 0 { return "Fine. Be that way!"; }
  if message.chars().filter(|c| c.is_alphabetic()).count() != 0 && 
    message == message.to_uppercase() { return "Whoa, chill out!"}
  if message.trim().ends_with("?") { return "Sure."}
  return "Whatever.";
}
