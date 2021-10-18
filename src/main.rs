use std::fmt::{Debug};
use std::{thread, time};
use chrono::prelude::*;

const ASCII_NUMS: [&str; 10] = [
r"
  ___  
 / _ \ 
| | | |
| |_| |
 \___/ 
",
r"       
 _ 
/ |
| |
| |
|_|
",   
r"
 ____  
|___ \ 
  __) |
 / __/ 
|_____|
",
r"       
 _____ 
|___ / 
  |_ \ 
 ___) |
|____/ 
",
r"       
 _  _   
| || |  
| || |_ 
|__   _|
   |_|  
",
r"        
 ____  
| ___| 
|___ \ 
 ___) |
|____/ 
",       
r"
  __   
 / /_  
| '_ \ 
| (_) |
 \___/ 
",
r"       
 _____ 
|___  |
   / / 
  / /  
 /_/   
",
r"
  ___  
 ( _ ) 
 / _ \ 
| (_) |
 \___/ 
",       
r"
  ___  
 / _ \ 
| (_) |
 \__, |
   /_/ 
",
];

#[derive(Debug)]
struct Time(u32, u32, u32);

#[derive(Debug)]
struct Clock {
    time: Time,
}

impl Clock {
    pub fn new() -> Self {
        let dt = Local::now();
        Clock {
            time: Time(dt.hour(), dt.minute(), dt.second()),
        }
    }

    pub fn print_time(&self) {
        let Time(h, m, s) = self.time;

        let a_h = self.get_ascii_representation(&h);
        let a_m = self.get_ascii_representation(&m);
        let a_s = self.get_ascii_representation(&s);

        for i in 0..6 {
            println!("{}{} {}{} {}{}", 
                     a_h.0[i], a_h.1[i],
                     a_m.0[i], a_m.1[i],
                     a_s.0[i], a_s.1[i],
                     );
        }
    }

    pub fn refresh_time(&mut self) {
        let Time(h, m, s) = self.time;

        if s < 59 {
            self.time.2 = s + 1;
        } else if s == 59 {
            self.time.2 = 0;
            self.time.1 = m + 1;
        } else if m == 59 && s == 59 {
            self.time.2 = 0;
            self.time.1 = 0;
            self.time.0 += h + 1
        } else if h == 23 && m == 59 && s == 59 {
            self.time.2 = 0;
            self.time.1 = 0;
            self.time.0 = 0;
        }
    }

    fn get_ascii_representation<'a>(&self, time_val: &'a u32) -> (Vec<&'a str>, Vec<&'a str>) {
        let ascii_rep: (Vec<&str>, Vec<&str>);

        if *time_val <= 9 {
            let first = ASCII_NUMS[0].split("\n").collect::<Vec<&str>>();
            let second = ASCII_NUMS[*time_val as usize].split("\n").collect::<Vec<&str>>();

           ascii_rep = (first, second);
        } else {
            let RADIX: u32 = 10;
            let time_val = time_val.to_string();
            let mut iter = time_val
                        .chars()
                        .map(|c| c.to_digit(RADIX).unwrap());

            let first = ASCII_NUMS[iter.next().unwrap() as usize].split("\n").collect::<Vec<&str>>();
            let second = ASCII_NUMS[iter.next().unwrap() as usize].split("\n").collect::<Vec<&str>>();

           ascii_rep = (first, second);
        }

        ascii_rep
    }

}

fn main() {
    let mut cl = Clock::new();

    print!("\x1B[2J\x1B[1;1H");

    loop {
        cl.print_time();
        cl.refresh_time();
        print!("\x1B[2J\x1B[1;1H");
        thread::sleep(time::Duration::from_secs(1))
    }
}
