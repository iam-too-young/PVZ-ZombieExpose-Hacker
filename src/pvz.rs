use crate::open_pvz;

use std::mem;
use std::ptr::null_mut;


use open_pvz::eval_final_address;


use winapi::um::memoryapi::ReadProcessMemory;
use winapi::ctypes::c_void;
// 内存基址
const BASE_ADDRESS: usize = 0x006A9EC0;


pub struct Zombie {
    pub x: f32,
    pub y: f32,
}

pub fn get_zombies_info(handle: *mut c_void) -> Vec<Zombie>{
    // 有效僵尸Vec
    let mut zombies_info: Vec<Zombie> = Vec::<Zombie>::new();
    // 本体血量
    let final_addr_zombie_self_hp: usize =
        unsafe { eval_final_address(handle, BASE_ADDRESS, [0x768, 0x90, 0xC8].to_vec()) };
    // 护甲1血量
    let final_addr_zombie_defend1: usize =
        unsafe { eval_final_address(handle, BASE_ADDRESS, [0x768, 0x90, 0xD0].to_vec()) };
    // 护甲2血量
    let final_addr_zombie_defend2: usize =
        unsafe { eval_final_address(handle, BASE_ADDRESS, [0x768, 0x90, 0xDC].to_vec()) };
    // 是否存在
    let final_addr_zombie_is_dead: usize =
        unsafe { eval_final_address(handle, BASE_ADDRESS, [0x768, 0x90, 0xEC].to_vec()) };
    // 僵尸X坐标
    let final_addr_zombie_x =
        unsafe { eval_final_address(handle, BASE_ADDRESS, [0x768, 0x90, 0x2C].to_vec()) };
    // 僵尸Y坐标
    let final_addr_zombie_y =
        unsafe { eval_final_address(handle, BASE_ADDRESS, [0x768, 0x90, 0x30].to_vec()) };

    // 本体hp，1级护甲hp，2级护甲hp，是否消失（死亡）,f32 x，f32 y
    let mut zombie_big_array: [[usize; 1]; 6] = [[0; 1]; 6];
    unsafe {
        for i in 0..1024 {
            ReadProcessMemory(
                handle,
                (final_addr_zombie_self_hp + (0x15C * i)) as *mut winapi::ctypes::c_void,
                zombie_big_array[0].as_mut_ptr() as *mut winapi::ctypes::c_void,
                4,
                null_mut(),
            );
            ReadProcessMemory(
                handle,
                (final_addr_zombie_defend1 + (0x15C * i)) as *mut winapi::ctypes::c_void,
                zombie_big_array[1].as_mut_ptr() as *mut winapi::ctypes::c_void,
                4,
                null_mut(),
            );
            ReadProcessMemory(
                handle,
                (final_addr_zombie_defend2 + (0x15C * i)) as *mut winapi::ctypes::c_void,
                zombie_big_array[2].as_mut_ptr() as *mut winapi::ctypes::c_void,
                4,
                null_mut(),
            );
            ReadProcessMemory(
                handle,
                (final_addr_zombie_is_dead + (0x15C * i)) as *mut winapi::ctypes::c_void,
                zombie_big_array[3].as_mut_ptr() as *mut winapi::ctypes::c_void,
                4,
                null_mut(),
            );

            ReadProcessMemory(
                handle,
                (final_addr_zombie_x + (0x15C * i)) as *mut winapi::ctypes::c_void,
                zombie_big_array[4].as_mut_ptr() as *mut winapi::ctypes::c_void,
                4,
                null_mut(),
            );
            ReadProcessMemory(
                handle,
                (final_addr_zombie_y + (0x15C * i)) as *mut winapi::ctypes::c_void,
                zombie_big_array[5].as_mut_ptr() as *mut winapi::ctypes::c_void,
                4,
                null_mut(),
            );

            let hp =
                (zombie_big_array[0][0] + zombie_big_array[1][0] + zombie_big_array[2][0]) as u32;
            let is_dead = zombie_big_array[3][0] as u32;
            let x: f32 = mem::transmute_copy(&(zombie_big_array[4][0] as u32));
            let y: f32 = mem::transmute_copy(&(zombie_big_array[5][0] as u32));

            if is_dead == 0 && hp > 0 && hp <= 60000 {
                // 判断僵尸是否存在并且hp在正常范围
                // 若是，则是正确僵尸
                zombies_info.push(Zombie{x, y});

            }
        }
        
    }
    return zombies_info;
}
