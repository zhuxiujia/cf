#[cfg(windows)]
extern crate winapi;

use std::ffi::CString;
use std::io::{Bytes, Error};
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::SystemTime;

use winapi::_core::mem::size_of;
use winapi::_core::str::Chars;
use winapi::_core::time::Duration;
use winapi::ctypes::{c_char, c_void};
use winapi::shared::minwindef::BYTE;
use winapi::shared::windef::{HBITMAP, HBITMAP__, HGDIOBJ, POINT, RECT, SIZE};
use winapi::um::wingdi;
use winapi::um::wingdi::{BI_RGB, BitBlt, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleBitmap, CreateCompatibleDC, DeleteObject, DIB_RGB_COLORS, GetBValue, GetDIBits, GetGValue, GetPixel, GetRValue, GetTextColor, RGBQUAD, SelectObject, SRCCOPY};
use winapi::um::winuser::{GetCursorPos, GetDC, GetDesktopWindow, GetTopWindow, GetWindowDC, GetWindowRect, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, ReleaseDC};

use crate::time_util::count_time_qps;
use crate::util::{pixel_to_rgb, write_file};

pub mod time_util;
pub mod util;


///
///
///
/// diff: 误差值
/// step: 步长
///
#[cfg(windows)]
unsafe fn find_color(left: u32, top: u32, right: u32, bottom: u32, r: u8, g: u8, b: u8, diff: i32, step: usize) -> bool{
    let hDeskTopWnd = GetDesktopWindow();//获得屏幕的HWND
    let hScreenDC = GetDC(hDeskTopWnd);//获得屏幕的HDC
    let MemDC = CreateCompatibleDC(hScreenDC);//创建一个内存中的DC
    let mut rect: RECT = RECT {
        left: left as i32,
        top: top as i32,
        right: right as i32,
        bottom: bottom as i32,
    };
    //获取屏幕尺寸
    // GetWindowRect(hDeskTopWnd, &mut rect);
    let mut screensize = SIZE { cx: 0, cy: 0 };
    screensize.cx = rect.right - rect.left;
    screensize.cy = rect.bottom - rect.top;

    let mut hBitmap: HBITMAP;
    hBitmap = CreateCompatibleBitmap(hScreenDC, screensize.cx, screensize.cy);
    let hOldBMP = SelectObject(MemDC, hBitmap as HGDIOBJ);
    let bitBltSuccess = BitBlt(MemDC, 0, 0, screensize.cx, screensize.cy, hScreenDC, 0, 0, SRCCOPY);
    if bitBltSuccess as i32 == 0 {
        return;
    }
    let mut bitInfo: BITMAPINFO = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: rect.right - rect.left,
            biHeight: rect.bottom - rect.top,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }; 1],
    };


    let mut result = 0;
//第一次调用GetDIBits获得图片的大小
    result = GetDIBits(MemDC, hBitmap, 0, screensize.cy as u32, null_mut(), &mut bitInfo, DIB_RGB_COLORS);
    bitInfo.bmiHeader.biCompression = BI_RGB;
    bitInfo.bmiHeader.biPlanes = 1;
    if result != 0 {
        //do something
        let size: usize = bitInfo.bmiHeader.biSizeImage as usize;
        let mut buffer: Vec<u8> = vec![];
        for _ in 0..size {
            buffer.push(0 as u8);
        }
        let mut slice = buffer.as_mut_slice() as *mut [u8];
        //第二次调用GetDIBits取图片流数据
        result = GetDIBits(MemDC, hBitmap, 0, screensize.cy as u32, slice as *mut c_void, &mut bitInfo, DIB_RGB_COLORS);
        //gc
        SelectObject(MemDC, hOldBMP);
        DeleteObject(MemDC as HGDIOBJ);
        ReleaseDC(hDeskTopWnd, hScreenDC);
        let len = size / 4;
        for i in 0..len {
            if step != 0 && i % step != 0 {
                continue;
            }
            let rv = buffer[(size - i * 4) - 1] as i32;
            let gv = buffer[(size - i * 4) - 2]as i32;
            let bv = buffer[(size - i * 4) - 3]as i32;
            //println!("r:{},g:{},b:{}",rv,gv,bv);

            let rv_diff = (rv - r as i32);
            let gv_diff = (gv - g  as i32);
            let bv_diff = (bv - b  as i32);

            let rv_diff = rv_diff.abs();
            let gv_diff = gv_diff.abs();
            let bv_diff = bv_diff.abs();

            if rv_diff <= diff && gv_diff <= diff  && bv_diff <= diff {
                println!("find    r:{},g:{},b:{}", rv, gv, bv);
                return true;
            }
        }
    } else {
        //gc
        SelectObject(MemDC, hOldBMP);
        DeleteObject(MemDC as HGDIOBJ);
        ReleaseDC(hDeskTopWnd, hScreenDC);
    }
    return false;
}


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


unsafe fn loop_find_color() {
    loop {
        find_color(0, 0, 100, 100, 255, 255, 255, 10, 0);
    }
}


fn main() {
    //bench_rate();
    unsafe { loop_find_color(); }
}
