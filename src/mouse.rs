use winapi::um::winuser::{GetCursorPos, GetDC, MOUSEEVENTF_LEFTDOWN, mouse_event, MOUSEEVENTF_LEFTUP};

pub unsafe fn click(){
    mouse_event(MOUSEEVENTF_LEFTDOWN,p.x as u32,p.y as u32,0,0);
    mouse_event(MOUSEEVENTF_LEFTUP,p.x as u32,p.y as u32,0,0);
}