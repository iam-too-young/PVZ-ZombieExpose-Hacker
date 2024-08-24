
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::{BOOL, LPARAM, MAX_PATH};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::windef::HWND;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winbase::QueryFullProcessImageNameW;
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::winuser::{EnumWindows, FindWindowW, GetWindowTextW, GetWindowThreadProcessId};
use winapi::{shared::minwindef::DWORD, um::psapi::EnumProcesses};

pub unsafe fn eval_final_address(handle: HANDLE, base_addr: usize, offset_vec: Vec<usize>) -> usize {
    let mut buf: [usize; 1] = [base_addr; 1];
    let mut i = 0;
    while i < offset_vec.len() {
        ReadProcessMemory(
            handle,
            buf[0] as *mut winapi::ctypes::c_void,
            buf.as_ptr() as *mut winapi::ctypes::c_void,
            4,
            null_mut(),
        );
        buf[0] += offset_vec[i];
        i += 1;
    }
    return buf[0];
}


unsafe fn get_game_handle_by_process_name() -> HANDLE {
    // Query Name while enum
    let mut pids: [DWORD; 1024] = [0; 1024];
    let mut real_size: DWORD = 0;
    EnumProcesses(
        pids.as_mut_ptr(),
        (size_of::<DWORD>() * 1024) as u32,
        &mut real_size,
    );
    for p in pids {
        if p != 0 {
            // Open then Query
            let handle = OpenProcess(PROCESS_ALL_ACCESS, 0, p);
            if handle.is_null() {
                continue;
            }
            let mut image_name: [u16; MAX_PATH] = [0; MAX_PATH];
            let mut size: DWORD = MAX_PATH as DWORD;
            QueryFullProcessImageNameW(handle, 0, image_name.as_mut_ptr(), &mut size);
            let name = OsString::from_wide(&image_name);
            let name_str = name.to_string_lossy();
            if name_str.contains("PlantsVsZombies.exe") {
                return handle;
            }
        }
    }
    return null_mut();
}

unsafe fn get_game_handle_by_find_window() -> HANDLE {
    let mut class_name = OsString::from("MainWindow")
        .encode_wide()
        .collect::<Vec<_>>();
    class_name.push(0); //添加\0
    let mut window_name = OsString::from("植物大战僵尸中文版")
        .encode_wide()
        .collect::<Vec<_>>();
    window_name.push(0);
    let hwnd: HWND = FindWindowW(class_name.as_ptr(), window_name.as_ptr());
    if hwnd.is_null() {
        return null_mut();
    }
    let mut process_id = 0;
    GetWindowThreadProcessId(hwnd, &mut process_id);
    if process_id == 0 {
        return null_mut();
    }

    let handle: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, 0, process_id);
    return handle;
}

pub unsafe fn get_game_handle() -> HANDLE {
    let handle: HANDLE = get_game_handle_by_process_name();
    if handle.is_null(){
        return get_game_handle_by_find_window();
    }
    return handle;
}

pub unsafe fn get_game_hwnd() -> HWND{
    
    unsafe extern "system" fn enum_window_call_back_proc(hwnd: HWND, l_param: LPARAM) -> BOOL{
        let mut window_name: [u16; 9] = [0; 9];
        GetWindowTextW(hwnd, window_name.as_mut_ptr(), 9);
        if OsString::from_wide(&window_name).to_string_lossy().contains("植物大战僵尸")
        {
            let param = l_param as *mut HWND;
            *param = hwnd;
            return 0;
        }
        return 1;
    }
    let mut hwnd: HWND = null_mut();
    EnumWindows(Some(enum_window_call_back_proc), (&mut hwnd as *mut HWND) as LPARAM);
    return hwnd;
}