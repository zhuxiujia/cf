#[cfg(windows)]
extern crate winapi;

use std::io::Error;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::SystemTime;

use winapi::_core::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::wingdi;
use winapi::um::wingdi::{GetBValue, GetGValue, GetPixel, GetRValue, GetTextColor, GetDIBits};
use winapi::um::winuser::{GetCursorPos, GetDC, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, GetWindowDC, GetDesktopWindow, GetTopWindow};

use crate::time_util::count_time_qps;
use crate::util::pixel_to_rgb;

pub mod time_util;
pub mod util;

#[cfg(windows)]
unsafe fn print_message() {
    // let pixel=  GetPixel(null_mut(),0,512);
    // println!("{}",pixel);
    let mut p = POINT {
        x: 0,
        y: 0,
    };
    let hDeskTopWnd = GetDesktopWindow();//获得屏幕的HWND
    let hScreenDC = GetDC(hDeskTopWnd);//获得屏幕的HDC
    loop {
        GetCursorPos(&mut p);
        println!("point: {},{}", &p.x, &p.y);

        let pixel = GetPixel(hScreenDC, p.x, p.y);
        println!("pixel: {}", pixel);

        pixel_to_rgb(pixel as u32);

        sleep(Duration::from_secs(1));
    }
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
        };
    }
    count_time_qps("", total, now);
}




fn main() {
    //bench_rate();
    unsafe { print_message(); }
}
