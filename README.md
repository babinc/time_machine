# time_machine
Easy way to change sytem time on linux and reset back to the correct time using NTP.

## Description
I needed a quick and easy way to change time on linux in order to run integration test for another project. 

## Install
```sh
git clone git@github.com:babinc/time_machine.git
cd time_machine
cargo build --release
```

If you want to run time_machine without root privilages add the CAP_SYS_TIME capabilities to the executable
```
sudo setcap CAP_SYS_TIME+ep <path to time_machine executable>
```

## Usage
Set time to 8:30 am:
```sh
time_machine -t 8:30:am
```

Reset to correct time using NTP:
```sh
time_machine -r
```
