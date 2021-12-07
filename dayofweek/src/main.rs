/* 요일 구하기 
   서기 1년 1월 1일은 월요일이다 
   윤년을 구하는 공식 
     1. 4로 나누어지는 해는 윤년이다.
     2. 100으로 나누어지는 해는 윤년이 아니다.
     3. 400으로 나누어지는 해는 윤년이다.       */


fn main() { 
    let mut target_date = TargetDate{year: 0, month: 0, day: 0, total_days: 0};

    input_date(&mut target_date);
    calc_total_days(&mut target_date);
    println!("Target Date {:?}", target_date);
    println!("The {}, {}, {} is {}" , target_date.year, target_date.month, target_date.day,
        find_day_of_week(target_date.total_days));
}



