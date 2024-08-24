mod open_pvz;
use open_pvz::{get_game_handle, get_game_hwnd};
mod pvz;
use pvz::get_zombies_info;

use winapi::um::winuser::{
    DrawEdge, FillRect, GetDC,SetWindowPos, BF_RECT, EDGE_RAISED, HWND_TOPMOST,
    LWA_COLORKEY, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
};

extern crate winapi;

use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetStockObject, SelectObject, BLACK_BRUSH, RGB, SRCCOPY};
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, GetWindowRect, LoadCursorW,
    MoveWindow, PostQuitMessage, RegisterClassW, SetLayeredWindowAttributes, ShowWindow,
    TranslateMessage, UpdateWindow, CS_HREDRAW, CS_VREDRAW, IDC_ARROW, MSG, SW_SHOWNORMAL,
    WM_DESTROY, WM_PAINT, WNDCLASSW, WS_EX_LAYERED, WS_EX_TRANSPARENT, WS_POPUP,
};

use winapi::shared::windef::{HBRUSH__, HWND, RECT};

use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use winapi::shared::minwindef::{LPARAM, UINT, WPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> isize {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_PAINT => 0,
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

fn main() {
    println!("Make sure your dpi settings for this program is correct,");
    println!("or the draw will work incorrectly.");
    let handle = unsafe { get_game_handle() };
    if handle.is_null(){
        println!("Cannot find game! Please retry!");
        return;
    }
    let pvz_hwnd = unsafe { get_game_hwnd() };
    if pvz_hwnd.is_null(){
        println!("Find game handle but the window failed, how could happen...");
        return;
    }

    println!("Find Game! Start to init...");
    // 一个外透思路

    // 创建一个主窗口（透明，可穿透点击）
    let mut instance_class_name: Vec<u16> = OsString::from("Main_Class")
        .encode_wide()
        .collect::<Vec<_>>();
    instance_class_name.push(0); // \0
    let mut main_hwnd_window_name: Vec<u16> = OsString::from("Main_Window")
        .encode_wide()
        .collect::<Vec<_>>();
    main_hwnd_window_name.push(0); 
    unsafe {
        let main_instance = GetModuleHandleW(null_mut());

        let wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: main_instance,
            lpszClassName: instance_class_name.as_ptr(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            ..std::mem::zeroed()
        };

        RegisterClassW(&wc);

        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT,
            instance_class_name.as_ptr(),
            main_hwnd_window_name.as_ptr(),
            WS_POPUP,
            100,
            100,
            800,
            600,
            null_mut(),
            null_mut(),
            main_instance,
            null_mut(),
        );

        // 这个地方如果不这样子初始化样式好像会不显示窗口，即便你有WS_VISIBLE属性
        SetLayeredWindowAttributes(hwnd, RGB(0, 0, 0), 0, LWA_COLORKEY); //变透明


        ShowWindow(hwnd, SW_SHOWNORMAL);
        UpdateWindow(hwnd);


        let mut msg: MSG = std::mem::zeroed();

        // 获得pvz的窗口位置
        let mut pvz_window_rect: RECT = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        println!("Init successfully.");
        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
            
            
            
            
            GetWindowRect(pvz_hwnd, &mut pvz_window_rect);
            // 将外透窗口始终附在pvz窗口上
            SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                0,
                0,
                800,
                600,
                SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE,
            ); // 置顶显示
            MoveWindow(
                hwnd,
                pvz_window_rect.left + 3,
                pvz_window_rect.top + 32,
                800,
                600,
                0,
            );

            // 获得 DC，然后创建内存上下文和Bitmap区域，在内存中绘制后统一复制到源dc，解决了频闪问题
            let hdc = GetDC(hwnd); // local dc
            let mut scr_rect = RECT {
                left: 0,
                top: 0,
                right: 800,
                bottom: 600,
            }; // pvz窗口大小
            
            let hdc_mem: *mut winapi::shared::windef::HDC__ = CreateCompatibleDC(hdc);
            let hbitmap = CreateCompatibleBitmap(hdc, 800, 600);
            SelectObject(hdc_mem, hbitmap as *mut winapi::ctypes::c_void);
            //清屏 
            FillRect(
                hdc_mem,
                &mut scr_rect,
                GetStockObject(BLACK_BRUSH as i32) as *mut HBRUSH__,
            );
            DrawEdge(hdc_mem, &mut scr_rect, EDGE_RAISED, BF_RECT);
            for z in get_zombies_info(handle) {
                if z.x < 0.0 || z.y < 0.0 {
                    continue;
                }
                // 绘制僵尸
                let mut zombie_rect = RECT {
                    left: (z.x as i32)+18,
                    top: z.y as i32,
                    right: (z.x + 85.0) as i32, 
                    bottom: (z.y + 120.0) as i32,
                };
                DrawEdge(hdc_mem, &mut zombie_rect, EDGE_RAISED, BF_RECT);

            }
            BitBlt(hdc, 0, 0, 800, 600, hdc_mem, 0, 0, SRCCOPY);
            DeleteObject(hbitmap as *mut winapi::ctypes::c_void);
            DeleteDC(hdc_mem);
            DeleteDC(hdc);
        }
    }
    println!("Program exited.");
    std::thread::sleep(core::time::Duration::from_millis(2000));
}
