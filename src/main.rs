#[cfg(windows)]
extern crate winapi;

use std::io::Error;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::SystemTime;

use winapi::_core::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::wingdi;
use winapi::um::wingdi::{GetBValue, GetGValue, GetPixel, GetRValue, GetTextColor};
use winapi::um::winuser::{GetCursorPos, GetDC, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, GetWindowDC, GetDesktopWindow, GetTopWindow};

use crate::time_util::count_time_qps;

pub mod time_util;
pub mod mouse;

#[cfg(windows)]
unsafe fn print_message() {
    // let pixel=  GetPixel(null_mut(),0,512);
    // println!("{}",pixel);
    let mut p = POINT {
        x: 0,
        y: 0,
    };
    let hdc = GetDC(null_mut());
    loop {
        GetCursorPos(&mut p);
        println!("point: {},{}", &p.x, &p.y);

        let pixel = GetPixel(hdc, p.x, p.y);
        println!("pixel: {}", pixel);

        let rv=pixel & 0xFF;
        let gv=(pixel & 0xFF00) / 256;
        let bv=(pixel & 0xFF0000) / 65536;

        let rv = rv as i32;
        let gv = gv as i32;
        let bv = bv as i32;

        println!("R_:{},G_:{},B_:{}", rv, gv,bv);

        //mouse::click(p.x as u32,p.y as u32);

        let rg:i32=(rv - gv);
        let rb:i32=(rv - bv);
        let gb:i32=(gv - bv);
        if rg.abs() >= 40 && rb.abs() >= 40  && gb.abs() <= 40 {
            println!("may be red");
        }


        sleep(Duration::from_secs(1));
    }
}


#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

#[test]
fn bench_rate() {
    let total = 120;
    println!("start");
    let now = SystemTime::now();

    let hdc;
    unsafe {
        hdc = GetDC(null_mut());
    }
    for _ in 0..total {
        unsafe {
            // let pixel=  GetPixel(null_mut(),0,512);
            // println!("{}",pixel);
            let mut p = POINT {
                x: 0,
                y: 0,
            };
            //GetCursorPos(&mut p);
            //println!("{},{}", &p.x, &p.y);

            let pixel = GetPixel(hdc, p.x, p.y);

            let rv = GetRValue(pixel);
            let gv = GetGValue(pixel);
            let bv = GetBValue(pixel);
        };
    }
    count_time_qps("", total, now);
}


fn main() {
    //bench_rate();
    unsafe { print_message(); }
}
