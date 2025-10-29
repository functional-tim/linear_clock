use chrono::{Datelike, Timelike};
use std::{fmt, thread::sleep, time::Duration};

struct LinearClock {
    year: i32,
    month: u32,
    day: u32,
    weekday: chrono::Weekday,
    hour: u32,
    minute: u32,
    second: u32
}

impl fmt::Display for LinearClock {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Year:    {}\n", self.year)?;
        write!(f, "Month:   {}\n", self.month)?;
        write!(f, "Day:     {}\n", self.day)?;
        write!(f, "Weekday: {}\n", self.weekday)?;
        write!(f, "\n")?;
//        for i in 0..12 {
//            if self.hour != 11 && i == self.hour || i == self.hour - 12 {
//                write!(f, "|--")?;
//            } else if i == 11 {
//                write!(f, "-")?;
//            } else if self.hour == 11 || self.hour - 12 == 11 {
//                write!(f, "|")?;
//            } else {
//                write!(f, "---")?;
//            }
//        }
//        write!(f, "\n12 01 02 03 04 05 06 07 08 09 10 11")?;
//        if self.hour > 12 {
//            write!(f, " PM\n")?;
//        } else {
//            write!(f, " AM\n")?;
//        }
        for i in 0..24 {
            if i == 23 && i == self.hour {
                write!(f, "|")?;
            } else if i == 23 {
                write!(f, "–")?;
            } else if i == self.hour {
                write!(f, "|––")?;
            } else {
                write!(f, "–––")?;
            }
        }
        write!(f, "\n00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23\n")?;
        for i in 0..60 {
            if i == self.minute {
                write!(f, "|")?;
            } else {
                write!(f, "–")?;
            }
        }
        write!(f, "\n00{spacer}15{spacer}30{spacer}45{spacer}60\n", spacer = (0..13).map(|_| " ").collect::<String>())?;
        for i in 0..60 {
            if i == self.second {
                write!(f, "|")?;
            } else {
                write!(f, "–")?;
            }
        }
        write!(f, "\n00{spacer}15{spacer}30{spacer}45{spacer}60\n", spacer = (0..13).map(|_| " ").collect::<String>())?;
        write!(f, "\n")
    }
}


fn main() {
    let mut time = chrono::Local::now();

    loop {
        let clock = LinearClock {
            year: time.year(),
            month: time.month(),
            day: time.day(),
            weekday: time.weekday(),
            hour: time.hour(),
            minute: time.minute(),
            second: time.second(),
        };

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("{}", clock);
        sleep(Duration::from_millis(20));
        time = chrono::Local::now();
    }
}
