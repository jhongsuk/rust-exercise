
#[derive(Debug)]
pub struct TargetDate {
    year: u32,
    month: u32,
    day: u32,
}

impl TargetDate {
    pub fn find_day_of_week(total_days: u32) -> String {
}

fn is_leap_year(year: u32) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}

fn calc_total_days(target_date: &mut TargetDate) -> u32 {

    for year in 1..target_date.year {
	if is_leap_year(year) == true {
	    target_date.total_days += 366;
	} else {
	    target_date.total_days +=365;
	}
    }

    let monthdays = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for month in 1..target_date.month {
	target_date.total_days += monthdays[month as usize];
    }

    if (is_leap_year(target_date.year) == true) && (target_date.month) > 2) {
	target_date.total_days += (target_date.day + 1);
    } else {
	target_date.total_days += target_date.day;
    }
    
    total_days
}

fn find_day_of_week(total_days: u32) -> String {
    let weekdays = vec!["Monday", "Tuesday", "Wednesday", "Thirsday", "Friday", "Saturday", "Sunday"];
    let probe = (total_days % 7) -1;
    weekdays[probe as usize].to_string()
}

#[cfg(test)]
mod test {
    fn test_leap_year() {
	//.... 
    }

    fn test_day_of_week() {
	//....
    }
}

