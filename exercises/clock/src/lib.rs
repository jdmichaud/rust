#[derive(Eq,PartialEq,Debug)]
pub struct Clock {
  minute: i32,
}

impl Clock {
  pub fn new(hour: i32, minute: i32) -> Clock {
    let i = hour * 60 + minute;
    if i >= 0 {
      Clock { minute: i % 1440 }
    } else {
      Clock { minute: 1440 + (i % 1440) }      
    }
  }

  pub fn to_string(self: &Clock) -> String {
    format!("{:>02}:{:>02}", 
            ((1440 + self.minute) % 1440) / 60,
            ((1440 + self.minute) % 1440) % 60)
  }

  pub fn add_minutes(self: &Clock, minute: i32) -> Clock {
    Clock { minute: (self.minute + minute) % 1440 }
  }
}