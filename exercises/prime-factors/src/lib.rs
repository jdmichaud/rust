pub fn factors(n: usize) -> Vec<usize> {
  let mut result: Vec<usize> = vec![];
  let mut remainder = n;
  let mut d = 2;
  if n == 1 {
    return result;
  }
  while d <= n && remainder > 1 {
    if remainder % d == 0 {
      result.push(d);
      remainder = remainder / d;
    } else {
      d = d + 1;
    }
  }
  return result;
}