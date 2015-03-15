    extern crate chrono;

    use std::old_io as io;
    use chrono::{Local, UTC, Offset};

    fn main() {
      let mut rdr = io::stdin();
      loop {
        let line = match rdr.read_line() {
          Err(e) => {
            println!("End of file, exiting...");
            break;
          },
          Ok(data) => data
        };
        let ymd: Vec<isize> = line.split(' ').map(|x| x.trim().parse().unwrap() ).collect();
        let today = Local::today();
        let cmp = UTC.ymd(ymd[0] as i32, ymd[1] as u32, ymd[2] as u32);
        let duration = (cmp - today).num_days();
        println!("{} days {} {}",
          duration,
          match duration > 0 {
            true => "until",
            false => "since"
          },
          line.trim());
      }
    }
