use chrono::{Datelike, Timelike};
use colored::Colorize;
use std::{fmt, thread::sleep, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "linear_clock",
    about = "A program to show a linear clock with date.
Sources and licenses are found here: https://github.com/functional-tim/linear_clock"
)]
struct Opt {
    #[structopt(short = "c", long = "continous", help = "Runs the program continously")]
    continous: bool,

    #[structopt(short = "d", long = "date", help = "Prints also the date")]
    date: bool,
}

struct LinearClock {
    date: bool,
    year: i32,
    month: u32,
    day: u32,
    weekday: chrono::Weekday,
    hour: u32,
    minute: u32,
    second: u32,
}

impl fmt::Display for LinearClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.date {
            writeln!(f, "Year:    {}", self.year)?;
            writeln!(f, "Month:   {}", self.month)?;
            writeln!(f, "Day:     {}", self.day)?;
            writeln!(f, "Weekday: {}", self.weekday)?;
            writeln!(f)?;
        }

//        for i in 0..24 {
//            if i == 23 && i == self.hour {
//                write!(f, "{}", "|".truecolor(214, 93, 14))?;
//            } else if i == 23 {
//                write!(f, "–")?;
//            } else if i == self.hour {
//                write!(f, "{}––", "|".truecolor(214, 93, 14))?;
//            } else {
//                write!(f, "–––")?;
//            }
//        }
        writeln!(
            f,
            "{}{}",
            (0..self.hour).map(|_| "███").collect::<String>().truecolor(213, 93, 14),
            (0..(24-self.hour)).map(|_| "   ").collect::<String>().on_truecolor(235, 219, 178),
        )?;
        writeln!(
            f,
            "00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24"
        )?;
//        for i in 0..60 {
//            if i == self.minute {
//                write!(f, "{}", "|".truecolor(214, 93, 14))?;
//            } else if i % 5 == 0 && i != 0 {
//                write!(f, "{}", "–".truecolor(214, 93, 14))?;
//            } else {
//                write!(f, "–")?;
//            }
//        }
        writeln!(
            f,
            "{}{}",
            (0..self.minute).map(|_| "█").collect::<String>().truecolor(213, 93, 14),
            (0..(60-self.minute)).map(|_| "█").collect::<String>().truecolor(235, 219, 178),
        )?;
        writeln!(
            f,
            "00{spacer}05{spacer}10{spacer}15{spacer}20{spacer}25{spacer}30{spacer}35{spacer}40{spacer}45{spacer}50{spacer}55{spacer}60",
            spacer = (0..3).map(|_| " ").collect::<String>()
        )?;
//        for i in 0..60 {
//            if i == self.second {
//                write!(f, "{}", "|".truecolor(214, 93, 14))?;
//            } else if i % 5 == 0 && i != 0 {
//                write!(f, "{}", "–".truecolor(214, 93, 14))?;
//            } else {
//                write!(f, "–")?;
//            }
//        }
        writeln!(
            f,
            "{}{}",
            (0..self.second).map(|_| "█").collect::<String>().truecolor(213, 93, 14),
            (0..(60-self.second)).map(|_| "█").collect::<String>().truecolor(235, 219, 178),
        )?;
        writeln!(
            f,
            "00{spacer}05{spacer}10{spacer}15{spacer}20{spacer}25{spacer}30{spacer}35{spacer}40{spacer}45{spacer}50{spacer}55{spacer}60",
            spacer = (0..3).map(|_| " ").collect::<String>()
        )?;
        writeln!(f)
    }
}

fn print_linear_clock(time: chrono::DateTime<chrono::Local>, date: bool) {
    let clock = LinearClock {
        date,
        year: time.year(),
        month: time.month(),
        day: time.day(),
        weekday: time.weekday(),
        hour: time.hour(),
        minute: time.minute(),
        second: time.second(),
    };
    print!("{}", clock);
}

fn main() {
    let opt = Opt::from_args();
    let mut time = chrono::Local::now();

    if opt.continous {
        loop {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            print_linear_clock(time, opt.date);
            sleep(Duration::from_millis(1000));
            time = chrono::Local::now();
        }
    } else {
        print_linear_clock(time, opt.date);
    }
}
