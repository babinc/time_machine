use chrono_tz::{Tz};
use std::error::Error;
use std::io::ErrorKind;
use chrono::{TimeZone, DateTime, Local, Utc, NaiveDate, NaiveDateTime, Datelike, NaiveTime, Timelike};
use std::mem::zeroed;
use libc::{suseconds_t, time_t, timeval, timezone};

pub struct App {
    tz: Tz
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let local_time_zone = match datetime::sys_timezone() {
            None => {
                return Err(Box::new(std::io::Error::new(ErrorKind::Other, "Could not figure out local timezone")));
            }
            Some(res) => res
        };

        println!("Using {} for time zone", local_time_zone);

        let tz: chrono_tz::Tz = match local_time_zone.parse() {
            Ok(res) => res,
            Err(e) => {
                let error = format!("Could not parse timezone: {}, Error: {}", local_time_zone, e.to_string());
                return Err(Box::new(std::io::Error::new(ErrorKind::Other, error)));
            }
        };

        let app = App {
            tz
        };

        Ok(app)
    }

    pub fn parse_arguments(&self, args: &Vec<String>) -> Result<(), Box<dyn Error>> {
        const TIME_ARGUMENT: &str = "-t";
        const REST_ARGUMENT: &str = "-r";
        const HELP: &str = "-h";
        const VERSION: &str = "-v";

        if args.contains(&TIME_ARGUMENT.to_string()) == true {
            let index = args.iter().position(|r| r == &TIME_ARGUMENT.to_string()).unwrap();
            if args.len() > index + 1 {
                self.change_time_with_time_argument(args[index + 1].as_str())?;
            }
            else {
                eprintln!("Error: missing time parameter");
                println!();
                App::display_usage();
            }
        }
        else if args.contains(&REST_ARGUMENT.to_string()) == true {
            self.reset_time_using_ntp()?;
        }
        else if args.contains(&HELP.to_string()) == true {
            App::display_usage();
        }
        else if args.contains(&VERSION.to_string()) == true {
            App::display_version();
        }
        else {
            eprintln!("Error: Argument not valid");
            println!();
            App::display_usage();
        }

        Ok(())
    }

    fn reset_time_using_ntp(&self) -> Result<(), Box<dyn Error>> {
        let ntp_result = sntpc::request("pool.ntp.org", 123)?;

        println!("NTP server time: {}.{}, Offset: {}", ntp_result.sec, ntp_result.nsec, ntp_result.offset);

        let datetime = convert_from_unix(ntp_result.sec as i64);

        println!("hour: {}", datetime.hour());
        println!("minute: {}", datetime.minute());

        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ntp_result.sec as i64, ntp_result.nsec), Utc);

        match App::change_system_time(dt) {
            Ok(_) => println!("Time Successfully Set"),
            Err(e) => eprintln!("Error settimeofday: {}", e.to_string())
        };

        Ok(())
    }

    fn change_time_with_time_argument(&self, time_arg: &str) -> Result<(), Box<dyn Error>> {
        let parsed_time = App::parse_human_time(time_arg)?;
        let now = Utc::now();

        let date = NaiveDate::from_ymd(now.year(), now.month(), now.day());
        let date_time = NaiveDateTime::new(date, parsed_time);

        let tz_aware = self.tz.from_local_datetime(&date_time).unwrap();

        match App::change_system_time(tz_aware) {
            Ok(_) => println!("Time Successfully Set"),
            Err(e) => eprintln!("Error settimeofday: {}", e.to_string())
        };

        Ok(())
    }

    fn parse_human_time(time: &str) -> Result<NaiveTime, Box<dyn Error>> {
        let parsed_time: NaiveTime = match NaiveTime::parse_from_str(time.to_lowercase().as_str(), "%I:%M:%p") {
            Ok(res) => res,
            Err(err) => {
                eprintln!("Could not parse time input. Example: 10:00:am");
                return Err(Box::new(err));
            }
        };

        Ok(parsed_time)
    }

    fn change_system_time<Tz: TimeZone>(t: DateTime<Tz>) -> Result<(), i32> {
        let date_time = t.with_timezone(&Local);
        let mut time_value: timeval = unsafe { zeroed() };

        time_value.tv_sec = date_time.timestamp() as time_t;
        time_value.tv_usec = date_time.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            let i = libc::settimeofday(&time_value as *const timeval, mock_tz);
            return if i == 0 {
                Ok(())
            } else {
                Err(i)
            }
        }
    }

    fn display_usage() {
        let version = env!("CARGO_PKG_VERSION");

        println!("Time Management Tool");
        println!("Version: {}", version);
        println!("Author: Carman Babin");
        println!();
        println!("USAGE:");
        println!("    tm <ARGUMENT> [OPTION]");
        println!();
        println!("FLAGS:");
        println!("    -h\t\tPrints help information");
        println!("    -v\t\tPrints version information");
        println!();
        println!("OPTIONS:");
        println!("    -t <Time>\texample: 5:30:pm");
        println!("    -r\tResets time using web api");
        println!();
        println!("EXAMPLES:");
        println!("    time_machine -t 5:30:pm");
        println!("    time_machine -r");
        println!();
    }

    fn display_version() {
        let version = env!("CARGO_PKG_VERSION");
        println!("v: {}", version);
    }
}

fn convert_from_unix(unix_time: i64) -> NaiveDateTime {
    let unix_str = unix_time.to_string();
    let epoch_seconds;
    if unix_str.len() > 10 {
        let epoch_str = &unix_time.to_string()[..10];
        epoch_seconds = epoch_str.parse::<i64>().unwrap();
    }
    else {
        epoch_seconds = unix_time;
    }
    let naive_datetime = NaiveDateTime::from_timestamp(epoch_seconds, 0);

    naive_datetime
}