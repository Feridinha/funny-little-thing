extern crate winapi;

use std::ffi::OsString;
use std::os::windows::ffi::{ OsStrExt, OsStringExt };
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winuser::{ EnumWindows, GetWindowTextW, GetWindowThreadProcessId, SetWindowTextW };
use winapi::shared::windef::HWND;
use winapi::um::winbase::QueryFullProcessImageNameW;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref MY_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();
        map.insert(String::from("firefox.exe"), String::from("Google - Mozilla Firefox"));
        map.insert(String::from("Spotify.exe"), String::from("Spotify"));
        map.insert(String::from("Code.exe"), String::from("Visual Studio Code"));
        map
    };
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _: isize) -> i32 {
    const MAX_TITLE_LENGTH: usize = 1000;
    let mut buffer: [u16; MAX_TITLE_LENGTH] = [0; MAX_TITLE_LENGTH];

    let len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), MAX_TITLE_LENGTH as i32);

    if len > 0 {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);

        // Open the process
        let h_proc = OpenProcess(winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if !h_proc.is_null() {
            const MAX_PATH_LEN: usize = 1000;
            let mut path_buffer: [u16; MAX_PATH_LEN] = [0; MAX_PATH_LEN];
            let mut size = path_buffer.len() as u32;
            let success = QueryFullProcessImageNameW(
                h_proc,
                0,
                path_buffer.as_mut_ptr(),
                &mut size
            );

            if success != 0 {
                let path = OsString::from_wide(&path_buffer[..size as usize]);
                let path_lossy = path.to_string_lossy();
                let executable_name = match path_lossy.split("\\").last() {
                    Some(name) => name,
                    None => "",
                };

                if let Some(value) = MY_MAP.get(executable_name) {
                    let mut new_title: Vec<u16> = OsString::from(value).encode_wide().collect();
                    new_title.push(0);
                    SetWindowTextW(hwnd, new_title.as_ptr());
                }
            }
        }
    }

    1
}

fn main() {
    unsafe {
        loop {
            // println!("Rodando woah");
            EnumWindows(Some(enum_windows_proc), 0);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
