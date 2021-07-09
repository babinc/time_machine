use libc::{suseconds_t, time_t, timeval, timezone};
use std::mem::zeroed;
use chrono::{TimeZone, DateTime, Local, NaiveTime, NaiveDate, Utc, Timelike, Datelike, NaiveDateTime, FixedOffset, Offset, Weekday};
use chrono_tz::America::Chicago;
use std::error::Error;
use std::{env, ptr};

//sudo setcap CAP_SYS_TIME+ep ./time_machine

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    parse_arguments(&args);

    Ok(())
}

fn parse_arguments(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    const TIME_ARGUMENT: &str = "-t";
    const HELP: &str = "-h";
    const VERSION: &str = "-v";

    if args.contains(&TIME_ARGUMENT.to_string()) == true {
        let index = args.iter().position(|r| r == &TIME_ARGUMENT.to_string()).unwrap();
        println!("Time Argument Found");
        if args.len() > index + 1 {
            set_time_with_time_argument(args[index + 1].as_str());
        }
        else {
            eprintln!("Error: missing time parameter");
            println!();
            display_usage();
        }
    }
    else if args.contains(&HELP.to_string()) == true {
        display_usage();
    }
    else if args.contains(&VERSION.to_string()) == true {
        display_version();
    }
    else {
        eprintln!("Error: Argument not valid");
        println!();
        display_usage();
    }

    Ok(())
}

fn set_time_with_time_argument(time_arg: &str) {
    let parsed_time = parse_time(time_arg)?;
    let now = Utc::now();

    let date = NaiveDate::from_ymd(now.year(), now.month(), now.day());
    let date_time = NaiveDateTime::new(date, parsed_time);

    let tz_aware = Chicago.from_local_datetime(&date_time).unwrap();
    set_system_time(tz_aware);
}

fn parse_time(time: &str) -> Result<NaiveTime, Box<dyn Error>> {
    let parsed_time: NaiveTime = match NaiveTime::parse_from_str(time.to_lowercase().as_str(), "%I:%M:%p") {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Could not parse time input. Example: 10:00:am");
            return Err(Box::new(err));
        }
    };

    Ok(parsed_time)
}

fn set_system_time<Tz: TimeZone>(t: DateTime<Tz>) {
    let date_time = t.with_timezone(&Local);
    let mut time_value: timeval = unsafe { zeroed() };

    time_value.tv_sec = date_time.timestamp() as time_t;
    time_value.tv_usec = date_time.timestamp_subsec_micros() as suseconds_t;

    unsafe {
        let mock_tz: *const timezone = std::ptr::null();
        let i = libc::settimeofday(&time_value as *const timeval, mock_tz);
        println!("output: {}", i);
    }
}

fn display_usage() {
    let version = env!("CARGO_PKG_VERSION");

    println!("Time Management Tool");
    println!("Version: {}", version);
    println!("Author: Carman Babin");
    println!();
    println!("USAGE:");
    println!("    tm [OPTION] <ARGUMENT>");
    println!();
    println!("FLAGS:");
    println!("    -h\t\t\tPrints help information");
    println!("    -v\t\t\tPrints version information");
    println!();
    println!("OPTIONS:");
    println!("    -t <Time>\t\texample: 5:30:pm");
    println!();
    println!("EXAMPLES:");
    println!("    tm -t 5:30:pm");
    println!();
}

fn display_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("v: {}", version);
}