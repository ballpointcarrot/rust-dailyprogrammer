/*
 * datespan.rs
 * A String Formatting library for formatting spans of dates.
 */
#![feature(collections)]
#![feature(core)]

extern crate time;
use std::str::FromStr;
use std::cmp::Ordering;

static MONTHS: [&'static str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"];

fn ordinal(value: usize) -> Option<String> {
    match value {
        0 => Some(String::from_str("th")),
        1 => Some(String::from_str("st")),
        2 => Some(String::from_str("nd")),
        3 => Some(String::from_str("rd")),
        4...20 => Some(String::from_str("th")),
        21...31 => ordinal(value % 10),
        _ => None
    }
}

fn current_year() -> usize {
    (time::now().tm_year + 1900) as usize
}

pub fn print_ordinal(value: usize) -> String {
    match ordinal(value) {
        Some(v) => format!("{}{}", value, v),
        None => panic!("Invalid input number")
    }
}

pub fn span(start_dt: &str, end_dt: &str) -> String {
    let cur_year: usize = current_year();
    let start_values: Vec<usize> = start_dt.split("-").map(|value| FromStr::from_str(value).unwrap()).collect();
    let end_values: Vec<usize> = end_dt.split("-").map(|value| FromStr::from_str(value).unwrap()).collect();
    if start_values.len() == 3 || end_values.len() == 3 {
        let (start_yr, start_mo, start_dy) = (start_values[0], start_values[1], start_values[2]);
        let (end_yr, end_mo, end_dy) = (end_values[0], end_values[1], end_values[2]);
        if start_yr == cur_year {
            match (end_yr-start_yr, (end_mo as isize)-(start_mo as isize), (end_dy as isize)-(start_dy as isize)) {
                (0, 0, 0) => format!("{} {}",
                                     MONTHS[start_mo-1],
                                     print_ordinal(start_dy)),
                (0, 0, _) => format!("{} {} - {}",
                                     MONTHS[start_mo-1],
                                     print_ordinal(start_dy),
                                     print_ordinal(end_dy)),
                (0, _, _) => format!("{} {} - {} {}",
                                     MONTHS[start_mo-1],
                                     print_ordinal(start_dy),
                                     MONTHS[end_mo-1],
                                     print_ordinal(end_dy)),
                (1, month, day) => {
                    let use_yr = match (0.cmp(&month) , 0.cmp(&day)) {
                        (Ordering::Greater, _) => false,
                        (Ordering::Equal, Ordering::Greater) => false,
                        (_,_) => true
                    };
                    if use_yr {
                        format!("{} {}, {} - {} {}, {}",
                                MONTHS[start_mo-1],
                                print_ordinal(start_dy),
                                start_yr,
                                MONTHS[end_mo-1],
                                print_ordinal(end_dy),
                                end_yr)
                    } else {
                        format!("{} {} - {} {}",
                                MONTHS[start_mo-1],
                                print_ordinal(start_dy),
                                MONTHS[end_mo-1],
                                print_ordinal(end_dy))
                    }
                },
                (_,_,_) => format!("{} {}, {} - {} {}, {}",
                                MONTHS[start_mo-1],
                                print_ordinal(start_dy),
                                start_yr,
                                MONTHS[end_mo-1],
                                print_ordinal(end_dy),
                                end_yr)
            }
        } else {
            match (end_yr-start_yr, (end_mo as isize)-(start_mo as isize), (end_dy as isize)-(start_dy as isize)) {
                (0,0,0) => format!("{} {}, {}",
                                 MONTHS[start_mo-1],
                                 print_ordinal(start_dy),
                                 start_yr),
                (0,0,_) => format!("{} {} - {}, {}",
                                   MONTHS[start_mo-1],
                                   print_ordinal(start_dy),
                                   print_ordinal(end_dy),
                                   start_yr),
                (0,_,_) => format!("{} {} - {} {}, {}",
                                   MONTHS[start_mo-1],
                                   print_ordinal(start_dy),
                                   MONTHS[end_mo-1],
                                   print_ordinal(end_dy),
                                   start_yr),
                (_,_,_) => format!("{} {}, {} - {} {}, {}",
                                   MONTHS[start_mo-1],
                                   print_ordinal(start_dy),
                                   start_yr,
                                   MONTHS[end_mo-1],
                                   print_ordinal(end_dy),
                                   end_yr)
            }
        }

    } else {
        panic!("Input values incorrectly formatted. Requires \"YYYY-MM-DD\" format.");
    }

}

#[test]
fn uses_correct_ordinal() {
    let mut value = 1;
    assert_eq!("1st", print_ordinal(value));
    value = 2;
    assert_eq!("2nd", print_ordinal(value));
    value = 3;
    assert_eq!("3rd", print_ordinal(value));
    value = 4;
    assert_eq!("4th", print_ordinal(value));
    value = 11;
    assert_eq!("11th", print_ordinal(value));
    value = 21;
    assert_eq!("21st", print_ordinal(value));
    value = 23;
    assert_eq!("23rd", print_ordinal(value));
    value = 24;
    assert_eq!("24th", print_ordinal(value));
    value = 30;
    assert_eq!("30th", print_ordinal(value));
    value = 31;
    assert_eq!("31st", print_ordinal(value));
}

#[test]
#[should_panic]
fn ordinal_out_of_bounds() {
    let value = 32;
    assert_eq!("32nd", print_ordinal(value));
}

#[test]
fn test_cur_year() {
    assert_eq!(2015, current_year());
}

#[test]
fn within_same_month_current_year() {
    let yr = current_year();
    let start_dt = format!("{}-07-01", yr);
    let end_dt = format!("{}-07-04", yr);
    assert_eq!("July 1st - 4th", span(start_dt.as_slice(), end_dt.as_slice()));
}

#[test]
fn within_twelve_months_starting_current_year() {
    let start_dt = format!("{}-12-01", current_year());
    let end_dt = "2016-02-03";
    assert_eq!("December 1st - February 3rd", span(start_dt.as_slice(), end_dt));
}

#[test]
fn exactly_twelve_months_starting_current_year() {
    let start_dt = format!("{}-12-01", current_year());
    let end_dt = format!("{}-12-01", current_year()+1);
    assert_eq!("December 1st, 2015 - December 1st, 2016", span(start_dt.as_slice(), end_dt.as_slice()));
}

#[test]
fn over_twelve_months_starting_current_year() {
    let start_dt = format!("{}-12-01", current_year());
    let end_dt = "2017-02-03";
    assert_eq!("December 1st, 2015 - February 3rd, 2017", span(start_dt.as_slice(), end_dt));
}

#[test]
fn same_day_cur_year() {
    let yr = current_year();
    let start_dt = format!("{}-12-01", yr);
    let end_dt = format!("{}-12-01", yr);
    assert_eq!("December 1st", span(start_dt.as_slice(), end_dt.as_slice()));
}

#[test]
fn same_day_future() {
    let start_dt = "2017-01-01";
    let end_dt = "2017-01-01";
    assert_eq!("January 1st, 2017", span(start_dt, end_dt));
}

#[test]
fn within_month_future() {
    let start_dt = "2017-01-01";
    let end_dt = "2017-01-31";
    assert_eq!("January 1st - 31st, 2017", span(start_dt, end_dt));
}

#[test]
fn within_year_future() {
    let start_dt = "2017-01-01";
    let end_dt = "2017-04-30";
    assert_eq!("January 1st - April 30th, 2017", span(start_dt, end_dt));
}

#[test]
fn over_year_future() {
    let start_dt = "2017-01-01";
    let end_dt = "2018-04-30";
    assert_eq!("January 1st, 2017 - April 30th, 2018", span(start_dt, end_dt));
}
