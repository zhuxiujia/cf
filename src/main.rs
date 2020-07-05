#[cfg(windows)]
extern crate winapi;

pub mod time_util;

use std::io::Error;
use std::thread::sleep;

use winapi::_core::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::wingdi;
use winapi::um::wingdi::{GetPixel, GetTextColor, GetRValue, GetGValue, GetBValue};
use winapi::um::winuser::{GetCursorPos, GetDC, MOUSEEVENTF_LEFTDOWN, mouse_event, MOUSEEVENTF_LEFTUP};
use std::time::SystemTime;
use std::ptr::null_mut;
use crate::time_util::count_time_qps;

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();

    let ret = unsafe {
        // let pixel=  GetPixel(null_mut(),0,512);
        // println!("{}",pixel);
        let mut p = POINT {
            x: 0,
            y: 0,
        };
        let hdc= GetDC(null_mut());
        loop {
            GetCursorPos(&mut p);
            println!("{},{}", &p.x, &p.y);

            let pixel = GetPixel(hdc, p.x, p.y);
            println!("{}", pixel);

            let rv=  GetRValue(pixel);
            let gv=  GetGValue(pixel);
            let bv=  GetBValue(pixel);

            println!("R:{},G:{},B:{}", rv,gv,bv);


            mouse_event(MOUSEEVENTF_LEFTDOWN,p.x as u32,p.y as u32,0,0);
            mouse_event(MOUSEEVENTF_LEFTUP,p.x as u32,p.y as u32,0,0);



            sleep(Duration::from_secs(1));
        }
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) } else { Ok(ret) }
}

#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

#[test]
fn bench_rate(){
    let total=120;
    println!("start");
    let now=SystemTime::now();

    let hdc;
    unsafe {
        hdc = GetDC(null_mut());
    }
    for _ in 0..total{
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

                let rv=  GetRValue(pixel);
                let gv=   GetGValue(pixel);
                let bv=  GetBValue(pixel);
        };
    }
    count_time_qps("",total,now);
}


fn main() {
    //bench_rate();
    print_message("Hello, world!").unwrap();
}
