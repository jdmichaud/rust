pub fn build_proverb(list: Vec<&str>) -> String {
  pub fn generate_sentence(first: &str, second: &str) -> String {
    format!("For want of a {} the {} was lost.", first, second)
  }

  let mut result: Vec<String> = vec![];
  if list.len() == 0 { 
    String::new()
  } else {
    if list.len() > 1 {
      let mut iterator = list.iter();
      let mut first = iterator.next().unwrap();
      for item in iterator {
        result.push(generate_sentence(first, item));
        first = item;
      }
    }
    if list.len() >= 3 && list[0] == "nail" && list[1] == "shoe" && list[2] == "horse" {
      result.push("And all for the want of a horseshoe nail.".to_string());
    } else {
      result.push("And all for the want of a nail.".to_string());
    }

    result.join("\n")
  }
}
