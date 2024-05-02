pub use chrono::{ Datelike, Days, NaiveDate, Weekday as wd };

pub fn middle_day(year: i32) -> Option<wd>{
    if  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0){
        return None;
    }
    let num_of_day = NaiveDate::from_ymd_opt(year, 1, 1)
        .unwrap()
        .signed_duration_since(NaiveDate::from_ymd_opt(year, 1, 1).unwrap())
        .num_days();

    let middle_day =  num_of_day / 2;

    Some(NaiveDate::from_ymd_opt(year, 1, 1).unwrap().checked_add_days(Days::new(middle_day as u64)).unwrap().weekday())
}