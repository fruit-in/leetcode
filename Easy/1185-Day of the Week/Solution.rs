impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let mut day = day;
        let mut months = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let week = ["Thursday", "Friday", "Saturday",
            "Sunday", "Monday", "Tuesday", "Wednesday"];
 
        day += (year - 1971) * 365 + (year - 1969) / 4 + months[month as usize - 1];
        if year % 4 == 0 && month > 2 {
            day += 1;
        }
        week[day as usize % 7].to_string()
    }
}
