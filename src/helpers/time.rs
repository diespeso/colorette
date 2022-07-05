use chrono::prelude::*;
use chrono::{Duration, DateTime, offset::Utc};
use std::ops::Add;

#[cfg(test)]
mod test {
    use chrono::prelude::*;
    use chrono::{Duration, DateTime, offset::Utc};
    use std::ops::Add;

    #[test]
    pub fn test_epoch_from_now() {
        let now = Utc::now();
        let later = get_epoch_from_now(10000);
        println!("{:?}", now);
        println!("{:?}", now.timestamp());
        println!("{:?}", later);
    }

    pub fn get_epoch_from_now(milis: i64)  -> i64 {
        let now = Utc::now();
        let res = now.checked_add_signed(Duration::milliseconds(milis)); //into is genius!
        res.unwrap().timestamp()
    }
}
