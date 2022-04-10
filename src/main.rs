use core::{mem::zeroed, ptr::null};

use windows_sys::Win32::System::Diagnostics::Debug::MessageBeep;
use windows_sys::{
    Win32::Foundation::*, Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::HiDpi::*,
    Win32::UI::WindowsAndMessaging::*,
};

const FILE_MENU_NEW: usize = 1;
// const FILE_MENU_OPEN: usize = 2;
const FILE_MENU_EXIT: usize = 3;
const GENERATE_BUTTON: usize = 4;

static mut H_MENU: HMENU = 0;
static mut H_NAME: HWND = 0;
static mut H_AGE: HWND = 0;
static mut H_OUT: HWND = 0;

macro_rules! wide_str_ptr {
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
            lpszClassName: wide_str_ptr!(window_class),
            lpfnWndProc: Some(window_procedure),
            ..zeroed()
        };
        if RegisterClassW(&wc) == 0 {
            panic!("{}", GetLastError());
        }
        CreateWindowExW(
            0,
            wide_str_ptr!(window_class),
            wide_str_ptr!(window_name),
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
                GENERATE_BUTTON => {
                    let (mut name, mut age, mut out): ([u16; 30], [u16; 10], [u16; 50]) =
                        ([0; 30], [0; 10], [0; 50]);
                    GetWindowTextW(H_NAME, name.as_mut_ptr(), name.len() as i32);
                    GetWindowTextW(H_AGE, age.as_mut_ptr(), age.len() as i32);
                    let mut i = 0;
                    while name[i] != 0 {
                        out[i] = name[i];
                        i += 1;
                    }
                    for x in " is ".encode_utf16() {
                        out[i] = x;
                        i += 1;
                    }
                    let mut j = 0;
                    while age[j] != 0 {
                        out[i] = age[j];
                        j += 1;
                        i += 1;
                    }
                    for x in " years old.".encode_utf16() {
                        out[i] = x;
                        i += 1;
                    }
                    SetWindowTextW(H_OUT, out.as_ptr())
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
    AppendMenuW(sub_menu, MF_STRING, 0, wide_str_ptr!("SubMenu Item"));
    AppendMenuW(file_menu, MF_STRING, FILE_MENU_NEW, wide_str_ptr!("New"));
    AppendMenuW(
        file_menu,
        MF_POPUP,
        sub_menu as usize,
        wide_str_ptr!("Open SubMenu"),
    );
    AppendMenuW(file_menu, MF_SEPARATOR, 0, null());
    AppendMenuW(file_menu, MF_STRING, FILE_MENU_EXIT, wide_str_ptr!("Exit"));
    AppendMenuW(H_MENU, MF_POPUP, file_menu as usize, wide_str_ptr!("File"));
    AppendMenuW(H_MENU, MF_STRING, 0, wide_str_ptr!("Help"));
    SetMenu(hwnd, H_MENU);
}

unsafe fn add_controls(hwnd: HWND) {
    CreateWindowExW(
        0,
        wide_str_ptr!("Static"),
        wide_str_ptr!("Name :"),
        WS_VISIBLE | WS_CHILD,
        100,
        50,
        98,
        38,
        hwnd,
        0,
        0,
        null(),
    );
    H_NAME = CreateWindowExW(
        0,
        wide_str_ptr!("Edit"),
        null(),
        WS_VISIBLE | WS_CHILD | WS_BORDER,
        200,
        50,
        98,
        38,
        hwnd,
        0,
        0,
        null(),
    );
    CreateWindowExW(
        0,
        wide_str_ptr!("Static"),
        wide_str_ptr!("Age :"),
        WS_VISIBLE | WS_CHILD,
        100,
        90,
        98,
        38,
        hwnd,
        0,
        0,
        null(),
    );
    H_AGE = CreateWindowExW(
        0,
        wide_str_ptr!("Edit"),
        null(),
        WS_VISIBLE | WS_CHILD | WS_BORDER,
        200,
        90,
        98,
        38,
        hwnd,
        0,
        0,
        null(),
    );
    CreateWindowExW(
        0,
        wide_str_ptr!("Button"),
        wide_str_ptr!("Generate"),
        WS_VISIBLE | WS_CHILD,
        150,
        140,
        98,
        38,
        hwnd,
        GENERATE_BUTTON as isize,
        0,
        null(),
    );
    H_OUT = CreateWindowExW(
        0,
        wide_str_ptr!("Edit"),
        null(),
        WS_VISIBLE | WS_CHILD | WS_BORDER,
        100,
        200,
        300,
        200,
        hwnd,
        0,
        0,
        null(),
    );
}
