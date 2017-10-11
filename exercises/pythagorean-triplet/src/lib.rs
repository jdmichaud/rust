pub fn find() -> Option<u32> {
  let i: u32 = 0;
  let j: u32 = 0;
  let k: u32 = 0;
  for i in 1..1001 {
    for j in 1..1001 {
      for k in 1..1001 {
        if (i + j + k == 1000) && (i * i + j * j == k * k) {
          println!("{:?}", i+j+k);
          return Some(i * j * k);
        }
      }
    }
  }
  None
}