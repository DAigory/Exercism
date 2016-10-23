extern crate chrono;
use chrono::*;

pub fn after(data_in: chrono::DateTime<chrono::UTC>) -> chrono::DateTime<chrono::UTC> {
    let delta =  UTC.ymd(2043, 1, 1).and_hms(1,46,40) - UTC.ymd(2011, 4, 25).and_hms(0,0,0);
    data_in + delta
}
