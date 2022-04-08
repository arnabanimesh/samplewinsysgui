use core::{mem::zeroed, ptr::null};

use windows_sys::Win32::System::Diagnostics::Debug::MessageBeep;
use windows_sys::{
    Win32::Foundation::*, Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::HiDpi::*,
    Win32::UI::WindowsAndMessaging::*,
};

const NULL: usize = 0;
const FILE_MENU_NEW: usize = 1;
const FILE_MENU_EXIT: usize = 3;
const CHANGE_TITLE: usize = 4;

static mut H_MENU: HMENU = 0;
static mut H_EDIT: HWND = 0;

macro_rules! wide_str {
    ($str:expr) => {
        $str.encode_utf16()
            .chain(Some(0))
            .collect::<Vec<u16>>()
            .as_ptr()
    };
}

fn main() {
    unsafe {
        SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
        let hinst = GetModuleHandleW(null());
        let window_class = "window";
        let window_name = "My window";
        let wc = WNDCLASSW {
            hbrBackground: COLOR_WINDOW as isize,
            hCursor: LoadCursorW(0, IDC_ARROW),
            hInstance: hinst,
            lpszClassName: wide_str!(window_class),
            lpfnWndProc: Some(window_procedure),
            ..zeroed()
        };
        if RegisterClassW(&wc) == 0 {
            panic!("{}", GetLastError());
        }
        CreateWindowExW(
            0,
            wide_str!(window_class),
            wide_str!(window_name),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            100,
            100,
            500,
            500,
            0,
            0,
            hinst,
            null(),
        );
        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, 0, 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn window_procedure(
    hwnd: HWND,
    msg: u32,
    wp: WPARAM,
    lp: LPARAM,
) -> LRESULT {
    match msg {
        WM_COMMAND => {
            match wp {
                FILE_MENU_EXIT => DestroyWindow(hwnd),
                FILE_MENU_NEW => MessageBeep(MB_ICONINFORMATION),
                CHANGE_TITLE => {
                    let mut text: [u16; 100] = [0; 100]; // max array length should be limited to i32::MAX
                    GetWindowTextW(H_EDIT, text.as_mut_ptr(), text.len() as i32);
                    SetWindowTextW(hwnd, text.as_ptr())
                }
                _ => 0,
            };
        }
        WM_CREATE => {
            add_menus(hwnd);
            add_controls(hwnd);
        }
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, wp, lp),
    }
    0
}

unsafe fn add_menus(hwnd: HWND) {
    H_MENU = CreateMenu();
    let file_menu = CreateMenu();
    let sub_menu = CreateMenu();
    AppendMenuW(sub_menu, MF_STRING, NULL, wide_str!("SubMenu Item"));
    AppendMenuW(file_menu, MF_STRING, FILE_MENU_NEW, wide_str!("New"));
    AppendMenuW(
        file_menu,
        MF_POPUP,
        sub_menu as usize,
        wide_str!("Open SubMenu"),
    );
    AppendMenuW(file_menu, MF_SEPARATOR, NULL, null());
    AppendMenuW(file_menu, MF_STRING, FILE_MENU_EXIT, wide_str!("Exit"));
    AppendMenuW(H_MENU, MF_POPUP, file_menu as usize, wide_str!("File"));
    AppendMenuW(H_MENU, MF_STRING, NULL, wide_str!("Help"));
    SetMenu(hwnd, H_MENU);
}

unsafe fn add_controls(hwnd: HWND) {
    CreateWindowExW(
        0,
        wide_str!("static"),
        wide_str!("Enter Text Here :"),
        WS_VISIBLE | WS_CHILD | WS_BORDER | SS_CENTER as u32,
        200,
        100,
        100,
        50,
        hwnd,
        0,
        0,
        null(),
    );
    H_EDIT = CreateWindowExW(
        0,
        wide_str!("Edit"),
        wide_str!("..."),
        WS_VISIBLE | WS_CHILD | WS_BORDER | (ES_MULTILINE | ES_AUTOVSCROLL | ES_AUTOHSCROLL) as u32,
        200,
        152,
        100,
        50,
        hwnd,
        0,
        0,
        null(),
    );
    CreateWindowExW(
        0,
        wide_str!("Button"),
        wide_str!("Change Title"),
        WS_VISIBLE | WS_CHILD,
        200,
        204,
        100,
        50,
        hwnd,
        CHANGE_TITLE as isize,
        0,
        null(),
    );
}
