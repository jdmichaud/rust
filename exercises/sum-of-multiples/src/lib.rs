pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  (1..limit).fold(0, |acc, i| acc + factors.iter().map(|f| f * i).filter(|s| s < &limit).fold(0, |acc2, j| acc2 + j))
}
