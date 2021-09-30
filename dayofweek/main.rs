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
}

/*
fn is_leap_year(year: i32) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }  
}
*/

fn input_date(target_date: &mut TargetDate) {
    println!("Please input date... year, month, day");

    let mut input = String::new();
    match (io::stdin()).read_line(&mut input){
        Ok(_) => {
            let mut ymd = input.split(", ");
            let mut year = ymd.next();
            let mut month = ymd.next();
            let mut day = ymd.next();
 
            println!("{:?}, {:?}, {:?}", year, month, day);

//            target_date.year = year.as_deref().map(|x| &**x).unwrap_or("default string");
//            target_date.month = month.parse::<i32>().unwrap();
//            target_date.day = day.parse::<i32>().unwrap();
            
//            println!("{:?}, {:?}, {:?}", target_date.year, target_date.month, target_date.day);
        }
        Err(err) => println!("error: {}",err),
    }
}

//fn calc_total_days() {


//}

fn main() { 
    let mut target_date = TargetDate{year: 0, month: 0, day: 0};
    input_date(&mut target_date);
    println!("input date{:?}", target_date);
}



