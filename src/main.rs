use core::{mem::zeroed, ptr::null};

use windows_sys::Win32::Graphics::Gdi::HBITMAP;
use windows_sys::Win32::System::Diagnostics::Debug::MessageBeep;
use windows_sys::{
    Win32::Foundation::*, Win32::System::LibraryLoader::GetModuleHandleW, Win32::UI::HiDpi::*,
    Win32::UI::Input::KeyboardAndMouse::EnableWindow, Win32::UI::WindowsAndMessaging::*,
};

const FILE_MENU_NEW: usize = 1;
// const FILE_MENU_OPEN: usize = 2;
const FILE_MENU_EXIT: usize = 3;
const GENERATE_BUTTON: usize = 4;

static mut H_MENU: HMENU = 0;
static mut H_NAME: HWND = 0;
static mut H_AGE: HWND = 0;
static mut H_OUT: HWND = 0;
static mut H_LOGO: HWND = 0;
static mut H_BUT: HWND = 0;
static mut H_MAIN_WINDOW: HWND = 0;

static mut H_LOGO_IMAGE: HBITMAP = 0;
static mut H_GENERATE_IMAGE: HBITMAP = 0;

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
        register_dialog_class(hinst);
        H_MAIN_WINDOW = CreateWindowExW(
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
                FILE_MENU_EXIT => {
                    match MessageBoxW(
                        0,
                        wide_str_ptr!("Are you sure?"),
                        wide_str_ptr!("Wait!"),
                        MB_YESNO | MB_ICONEXCLAMATION,
                    ) {
                        IDYES => DestroyWindow(hwnd),
                        _ => 0,
                    }
                }
                FILE_MENU_NEW => {
                    display_dialog(hwnd);
                    MessageBeep(MB_ICONINFORMATION)
                }
                GENERATE_BUTTON => {
                    let (mut name, mut age, mut out): ([u16; 30], [u16; 10], [u16; 50]) =
                        ([0; 30], [0; 10], [0; 50]);
                    GetWindowTextW(H_NAME, name.as_mut_ptr(), name.len() as i32);
                    GetWindowTextW(H_AGE, age.as_mut_ptr(), age.len() as i32);
                    if name[0] == 0 || age[0] == 0 {
                        match MessageBoxW(
                            hwnd,
                            wide_str_ptr!("You did not enter anything !"),
                            null(),
                            MB_ABORTRETRYIGNORE | MB_ICONERROR,
                        ) {
                            IDABORT => return DestroyWindow(hwnd) as LRESULT,
                            IDRETRY => return 0,
                            IDIGNORE => {}
                            _ => return 0,
                        }
                    }
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
            load_images();
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
    H_BUT = CreateWindowExW(
        0,
        wide_str_ptr!("Button"),
        wide_str_ptr!("Generate"),
        WS_VISIBLE | WS_CHILD | BS_BITMAP as WINDOW_EX_STYLE,
        150,
        140,
        98,
        38,
        hwnd,
        GENERATE_BUTTON as HMENU,
        0,
        null(),
    );
    SendMessageW(H_BUT, BM_SETIMAGE, IMAGE_BITMAP as WPARAM, H_GENERATE_IMAGE);
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
    H_LOGO = CreateWindowExW(
        0,
        wide_str_ptr!("Static"),
        null(),
        WS_VISIBLE | WS_CHILD | SS_BITMAP as WINDOW_EX_STYLE,
        350,
        60,
        100,
        100,
        hwnd,
        0,
        0,
        null(),
    );
    SendMessageW(H_LOGO, STM_SETIMAGE, IMAGE_BITMAP as WPARAM, H_LOGO_IMAGE);
}

unsafe fn load_images() {
    H_LOGO_IMAGE = LoadImageW(
        0,
        wide_str_ptr!("Logo.bmp"),
        IMAGE_BITMAP,
        100,
        100,
        LR_LOADFROMFILE,
    );
    H_GENERATE_IMAGE = LoadImageW(
        0,
        wide_str_ptr!("Generate.bmp"),
        IMAGE_BITMAP,
        98,
        38,
        LR_LOADFROMFILE,
    );
}

unsafe fn register_dialog_class(hinst: HINSTANCE) {
    let dialog_class = "myDialogClass";
    let dialog = WNDCLASSW {
        hbrBackground: COLOR_WINDOW as isize,
        hCursor: LoadCursorW(0, IDC_CROSS),
        hInstance: hinst,
        lpszClassName: wide_str_ptr!(dialog_class),
        lpfnWndProc: Some(dialog_procedure),
        ..zeroed()
    };
    RegisterClassW(&dialog);
}

unsafe extern "system" fn dialog_procedure(
    hwnd: HWND,
    msg: u32,
    wp: WPARAM,
    lp: LPARAM,
) -> LRESULT {
    match msg {
        WM_COMMAND => match wp {
            1 => {
                EnableWindow(H_MAIN_WINDOW, true.into());
                DestroyWindow(hwnd) as LRESULT
            }
            _ => DefWindowProcW(hwnd, msg, wp, lp),
        },
        WM_CLOSE => {
            EnableWindow(H_MAIN_WINDOW, true.into());
            DestroyWindow(hwnd) as LRESULT
        }
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}

unsafe fn display_dialog(hwnd: HWND) {
    let dialog_class = "myDialogClass";
    let dialog_name = "Dialog";
    let h_dlg = CreateWindowExW(
        0,
        wide_str_ptr!(dialog_class),
        wide_str_ptr!(dialog_name),
        WS_VISIBLE | WS_OVERLAPPEDWINDOW,
        400,
        400,
        200,
        200,
        hwnd,
        0,
        0,
        null(),
    );
    CreateWindowExW(
        0,
        wide_str_ptr!("Button"),
        wide_str_ptr!("Close"),
        WS_VISIBLE | WS_CHILD,
        20,
        20,
        100,
        40,
        h_dlg,
        1,
        0,
        null(),
    );
    EnableWindow(hwnd, false.into());
}
