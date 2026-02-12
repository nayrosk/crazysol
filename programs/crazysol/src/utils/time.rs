use crate::constants::SECONDS_IN_24H;

pub fn has_24_hours_passed(last_activity: i64, current_time: i64) -> bool {
    current_time - last_activity >= SECONDS_IN_24H.try_into().unwrap_or(864_00)
}
