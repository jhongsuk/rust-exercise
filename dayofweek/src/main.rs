/* 요일 구하기 
   서기 1년 1월 1일은 월요일이다 
   윤년을 구하는 공식 
     1. 4로 나누어지는 해는 윤년이다.
     2. 100으로 나누어지는 해는 윤년이 아니다.
     3. 400으로 나누어지는 해는 윤년이다.       */


use std::io;

#[derive(Debug)]
struct TargetDate {
    year: u32,
    month: u32,
    day: u32,
    total_days: u32,
}

fn is_leap_year(year: u32) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }  
}

fn input_date(target_date: &mut TargetDate) {
    println!("Please input date... year, month, day");

    let mut input = String::new();
    match (io::stdin()).read_line(&mut input){
        Ok(_) => {
            let mut ymd = input.trim_end().split(", ");
            let year = ymd.next();
            let month = ymd.next();
            let day = ymd.next();
 
            target_date.year = year.as_deref().unwrap_or(" ").parse().unwrap();
            target_date.month = month.as_deref().unwrap_or(" ").parse().unwrap();
            target_date.day = day.as_deref().unwrap_or(" ").parse().unwrap();
        }
        Err(err) => println!("error: {}",err),
    }
}

fn calc_total_days(target_date: &mut TargetDate) {
    
    for year in 1..target_date.year {
        if is_leap_year(year) == true {
            target_date.total_days += 366;
        } else {
            target_date.total_days += 365;
        }
    }

    let monthdays = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for month in 1..target_date.month {
        target_date.total_days += monthdays[month as usize];
    }
 
    if (is_leap_year(target_date.year) == true) && (target_date.month > 2) {
        target_date.total_days += (target_date.day + 1);
    } else {
        target_date.total_days += target_date.day;
    }  

    println!("calc total{}", target_date.total_days);
}

fn find_day_of_week(total_days: u32) -> String {
    let weekdays = vec!["Monday", "Tuesday", "Wednesday", "Thirsday", "Friday", "Saturday", "Sunday"];
    let probe = (total_days % 7) - 1;
    weekdays[probe as usize].to_string()
}

fn main() { 
    let mut target_date = TargetDate{year: 0, month: 0, day: 0, total_days: 0};

    input_date(&mut target_date);
    calc_total_days(&mut target_date);
    println!("Target Date {:?}", target_date);
    println!("The {}, {}, {} is {}" , target_date.year, target_date.month, target_date.day,
        find_day_of_week(target_date.total_days));
}



