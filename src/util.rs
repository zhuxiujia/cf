use std::fs::File;
use std::io::Write;

use winapi::um::winuser::{INPUT_u, GetCursorPos, GetDC, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, SendInput, INPUT, INPUT_MOUSE, MOUSEINPUT, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_MOVE};
use std::thread::sleep;
use winapi::_core::mem::size_of;
use std::time::Duration;

pub unsafe fn click_send_input(dx: u32, dy: u32) {
    // mouse_event(MOUSEEVENTF_LEFTDOWN, dx, dy, 0, 0);
    // sleep(Duration::from_millis(30));
    // mouse_event(MOUSEEVENTF_LEFTUP, dx, dy, 0, 0);

    let mut input=INPUT{ type_: INPUT_MOUSE, u: INPUT_u::default()};
    let mi= input.u.mi_mut();
    mi.dx=0;
    mi.dy=0;
    mi.dwFlags= MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP;

    SendInput(1, &mut input, size_of::<INPUT>() as i32);
}

pub unsafe fn click_mouse_event(dx: u32, dy: u32) {
     mouse_event(MOUSEEVENTF_LEFTDOWN, dx, dy, 0, 0);
     sleep(Duration::from_millis(30));
     mouse_event(MOUSEEVENTF_LEFTUP, dx, dy, 0, 0);
}

pub fn pixel_to_rgb(pixel: u32) -> (i32, i32, i32) {
    let rv = pixel & 0xFF;
    let gv = (pixel & 0xFF00) / 256;
    let bv = (pixel & 0xFF0000) / 65536;

    let rv = rv as i32;
    let gv = gv as i32;
    let bv = bv as i32;

    println!("R_:{},G_:{},B_:{}", rv, gv, bv);

    //mouse::click(p.x as u32,p.y as u32);

    let rg: i32 = (rv - gv);
    let rb: i32 = (rv - bv);
    let gb: i32 = (gv - bv);
    if rg.abs() >= 40 && rb.abs() >= 40 && gb.abs() <= 40 {
        println!("may be red");
    }
    return (rv, gv, bv);
}

pub fn is_red(rv: i32, gv: i32, bv: i32,r_diff:i32,gb_diff:i32) -> bool {
    let rg: i32 = (rv - gv);
    let rb: i32 = (rv - bv);
    let gb: i32 = (gv - bv);
    if rg.abs() >= r_diff && rb.abs() >= r_diff && gb.abs() <= gb_diff {
        return true;
    }
    return false;
}


pub fn write_file(buf: &Vec<u8>) {
    let mut file = File::create("test.jpg").unwrap();
    file.write_all(buf);
}