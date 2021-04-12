use core::time;
use std::{alloc::System, ffi::IntoStringError, fmt::DebugSet, ops::{Add, Sub}, thread::sleep, time::{Duration, Instant, SystemTime}};

use chrono::{DateTime, Local, LocalResult, TimeZone, Utc, Weekday};



fn main() {
    // // 秒 + 纳秒
    // let timmer  = Duration::new(0,0);
    // assert_eq!(timmer.as_secs(), 0);
    // assert_eq!(timmer.as_nanos(),0);


    // // 毫秒
    // let timmer_millis = Duration::from_millis(10);
    // assert_eq!(timmer_millis.as_millis(),10);


    
    // // =============
    // // 测量过去的时间
    // // =============
    // use std::thread::sleep;
    // use std::time::Instant;
    // let now = Instant::now();

    // // we sleep for 2 seconds
    // sleep(Duration::new(2, 0));
    // // it prints '2'
    // println!("{}", now.elapsed().as_secs());

    // // =============

    // let now = Instant::now();
    // sleep(Duration::new(2,10));
    // let new_now = Instant::now();
    // println!("{:?}",new_now.duration_since(now));


    // // 
    // let now = std::time::SystemTime::now();
    // println!("{:?}",now);

    use chrono::prelude::*;

    let utc:DateTime<Utc> = Utc::now();
    println!("{:?}",utc); // 2021-04-08T02:32:41.839558Z
    let local_time:DateTime<Local> = Local::now(); // 2021-04-08T10:33:47.740477+08:00 东八区
    println!("{:?}",local_time);  

    let dt = Utc.ymd(2010, 10,30).and_hms(9, 10, 11); //年月日 时分秒
    println!("{:?}",dt); // 2010-10-30T09:10:11Z
    println!("{:?}",dt.time()); // 09:10:11
    println!("{:?}",dt.timestamp()); // 1288429811

    let db_local = Local.ymd(2010, 10, 10);
    println!("{:?}",db_local); //2010-10-10+08:00 
    // println!("{:?}",db_local.timestamp); // error no .time()  -> no timestamp()
    println!("{:?}",db_local.timezone()); // Local time  timezone is Local
    let dt_local = Local.ymd(2010, 10, 30).and_hms(9, 10, 11);
    println!("{:?}", dt_local);  // 2010-10-30T09:10:11+08:00
    println!("{:?}",dt_local.time()); // 09:10:11
    println!("{:?}", dt_local.timestamp()); //  1288401011

    // humanized time set
    let dt_local = Local.isoywd(2010, 10, Weekday::Tue); // 表示2010年10月的周二

    // 添加时间到秒
    let dt_local = Local.isoywd(2010, 10, Weekday::Tue).and_hms_milli(10,9, 8,7);

    // 动态验证
    let dt_local_res = LocalResult::Single(Utc.ymd(2014, 7, 8).and_hms(21, 15, 33));
    let dt_local_opt = Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33);

    if let LocalResult::None = dt_local_opt{
        println!("Invalid format",)
    }

    // 毫秒
    // let dt_local_res_micro = Utc.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000);
    // 纳秒
    // let dt_local_res_nanos = Utc.ymd(2014, 1, 10).and_hms_nano(91, 19, 11, 12_000_000);


    // FixedOffset 根据其他时区来定义当前时区


    // formating and parsing
    let utc = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
    let utc_format_string = utc.format("%Y-%m-%d %H:%M:%S");
    assert_eq!(utc_format_string.to_string(),"2014-11-28 12:00:09");
    assert_eq!(utc.to_string(), "2014-11-28 12:00:09 UTC");
    assert_eq!(utc.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
    assert_eq!(utc.to_rfc3339(), "2014-11-28T12:00:09+00:00");


    // timestamp or unix time
    let local  = Local.ymd(1970, 1,1).and_hms(0, 0, 0);
    assert_eq!(-28800,local.timestamp());
    let utc  = Utc.ymd(1970, 1,1).and_hms(0, 0, 0);
    assert_eq!(0,utc.timestamp());

    let Local_timestamp = Local.timestamp(-28800, 0);
    assert_eq!(Local.ymd(1970,1,1).and_hms(0,0,0),Local_timestamp);


    // 时间戳与日期之间的转换
    let utc_time = Utc.timestamp(331155752, 0);
    // 转换成日期
    let utc_date_string = utc_time.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{:?}",utc_date_string); // "1980-06-29 19:42:32"
    // 日期转换成时间戳
    let local_time = Local.timestamp(331155752, 0);
    let local_date_string = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("{:?}",local_date_string); // "1980-06-30 03:42:32"

    // 日期转换成时间戳
    let utc_time = Utc.ymd(2010, 10, 10).and_hms(10, 10, 10);
    let utc_time_unix = utc_time.timestamp();
    println!("{:?}",utc_time_unix); // 1286705410

    // thread sleep some time 
    // sleep(Duration::new(20,0));
    

    // 单秒
    let one_second = Duration::new(1,0);
    println!("{:?}",one_second); // 1s
    let two_second = Duration::add(one_second,Duration::new(1,0));
    println!("{:?}",two_second); // 2s



}


