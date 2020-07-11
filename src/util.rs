use winapi::um::winuser::{GetCursorPos, GetDC, mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};

pub unsafe fn click(dx: u32, dy: u32) {
    mouse_event(MOUSEEVENTF_LEFTDOWN, dx, dy, 0, 0);
    mouse_event(MOUSEEVENTF_LEFTUP, dx, dy, 0, 0);
}

pub  fn pixel_to_rgb(pixel: u32) -> (i32,i32,i32) {
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
    return (rv,gv,bv);
}