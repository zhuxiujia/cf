use winapi::um::winuser::{GetCursorPos, GetDC, MOUSEEVENTF_LEFTDOWN, mouse_event, MOUSEEVENTF_LEFTUP};
pub unsafe fn click(dx:u32,dy:u32){
    mouse_event(MOUSEEVENTF_LEFTDOWN,dx,dy,0,0);
    mouse_event(MOUSEEVENTF_LEFTUP,dx,dy,0,0);
}