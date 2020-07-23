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
use winapi::shared::windef::{HBITMAP, HBITMAP__, HGDIOBJ, HWND, POINT, RECT, SIZE};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::wingdi;
use winapi::um::wingdi::{BI_RGB, BitBlt, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, DIB_RGB_COLORS, GetBValue, GetDIBits, GetGValue, GetPixel, GetRValue, GetTextColor, RGBQUAD, SelectObject, SRCCOPY};
use winapi::um::winuser::{GetCursorPos, GetDC, GetDesktopWindow, GetTopWindow, GetWindowDC, GetWindowRect, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, ReleaseDC};

use crate::time_util::count_time_qps;
use crate::util::{click_mouse_event, click_send_input, is_red, pixel_to_rgb, rgb_is_black, rgb_is_red, write_file};

pub mod time_util;
pub mod util;


/// step: 步长
#[cfg(windows)]
unsafe fn find_color(left: u32, top: u32, right: u32, bottom: u32, step: usize) -> bool {
    let h_screen_dc = GetDC(null_mut());//获得屏幕的HDC
    let mem_dc = CreateCompatibleDC(h_screen_dc);//创建一个内存中的DC
    let mut rect: RECT = RECT {
        left: left as i32,
        top: top as i32,
        right: right as i32,
        bottom: bottom as i32,
    };
    //获取屏幕尺寸
    // GetWindowRect(h_desk_top_wnd, &mut rect);
    let mut screensize = SIZE { cx: 0, cy: 0 };
    screensize.cx = rect.right - rect.left;
    screensize.cy = rect.bottom - rect.top;

    let mut h_bitmap: HBITMAP;
    h_bitmap = CreateCompatibleBitmap(h_screen_dc, screensize.cx, screensize.cy);
    let h_old_bmp = SelectObject(mem_dc, h_bitmap as HGDIOBJ);
    let bit_blt_success = BitBlt(mem_dc, 0, 0, screensize.cx, screensize.cy, h_screen_dc, rect.left, rect.top, SRCCOPY);
    if bit_blt_success as i32 == 0 {
        //注意释放资源
        DeleteDC(h_screen_dc);
        DeleteDC(mem_dc);
        ReleaseDC(h_screen_dc as HWND, mem_dc);
        ReleaseDC(null_mut(), h_screen_dc);
        DeleteObject(mem_dc as HGDIOBJ);
        DeleteObject(h_old_bmp);
        DeleteObject(h_bitmap as HGDIOBJ);
        return false;
    }
    let mut bit_info: BITMAPINFO = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: screensize.cx,
            biHeight: screensize.cy,
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

    let mut is_find_color = false;

    let mut result = 0;
    //第一次调用GetDIBits获得图片的大小
    result = GetDIBits(mem_dc, h_bitmap, 0, screensize.cy as u32, null_mut(), &mut bit_info, DIB_RGB_COLORS);
    if result != 0 {
        //第二次调用GetDIBits取图片流数据
        // 位图信息及调色板大小
        let info_size = bit_info.bmiHeader.biSize + bit_info.bmiHeader.biClrUsed * size_of::<RGBQUAD>() as u32;
        //缓冲区大小
        let size: usize = bit_info.bmiHeader.biSizeImage as usize + info_size as usize;

        //缓存
        let mut buffer = vec![0u8; size];
        let ptr = buffer.as_mut_ptr().cast();
        result = GetDIBits(mem_dc, h_bitmap, 0, screensize.cy as u32, ptr, &mut bit_info, DIB_RGB_COLORS);

        if result == 0 {
            println!("2设置图片信息出错");
            is_find_color = false;
        }else{
            let mut have_black = false;
            let mut last_i = 0;

            let len = size / 4;
            for i in 0..len {
                if step != 0 && i % step != 0 {
                    continue;
                }
                let rv = buffer[(size - i * 4) - 2] as i32;
                let gv = buffer[(size - i * 4) - 3] as i32;
                let bv = buffer[(size - i * 4) - 4] as i32;

                if rgb_is_black(rv, gv, bv) {
                    have_black = true;
                    last_i = i;
                }

                if rgb_is_red(rv, gv, bv) && have_black && last_i + 1 == i {
                    //println!("find  red   r:{},g:{},b:{}", rv, gv, bv);
                    is_find_color = true;
                    break;
                }
            }
        }
    } else {
        let e = GetLastError();
        println!("1设置图片信息出错,code: {}", e);
    }
    //gc
    DeleteDC(h_screen_dc);
    DeleteDC(mem_dc);
    ReleaseDC(h_screen_dc as HWND, mem_dc);
    ReleaseDC(null_mut(), h_screen_dc);
    DeleteObject(mem_dc as HGDIOBJ);
    DeleteObject(h_old_bmp);
    DeleteObject(h_bitmap as HGDIOBJ);
    return is_find_color;
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
        find_color(0, 0, 10, 100, 0);
    }
}


unsafe fn loop_find_cf_color() {
    println!("done");
    loop {
        //let find =  find_color(0, 0, 10, 100, 0);;
        let find = find_color(918, 570, 918 + 100, 570 + 57, 0);
        if find {
            click_mouse_event(0, 0);
            sleep(Duration::from_millis(80));
        }
    }
}

fn main() {
    println!("now start!");
    //bench_rate();
    unsafe { loop_find_cf_color(); }
}
