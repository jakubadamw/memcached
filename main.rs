#![no_main]

extern "C" {
    static mut uriencode_map: [*const libc::c_char; 256];
    static mut uriencode_str: [libc::c_char; 768];
}

#[no_mangle]
pub extern "C" fn uriencode_init() {
    use std::fmt::Write;

    unsafe {
        let mut current_str = uriencode_str.as_mut_ptr();

        for x in 0..=255u8 {
            let c = x as char;
            if c.is_alphanumeric() || c == '-' || c == '.' || c == '_' || c == '~' {
                uriencode_map[x as usize] = std::ptr::null();
            } else {
                let mut s = String::new();
                write!(&mut s, "%{:X}", x).unwrap();
                libc::memcpy(current_str as *mut libc::c_void, s.as_bytes().as_ptr() as *const libc::c_void, 3);
                *current_str.add(3) = 0 as i8;
                uriencode_map[x as usize] = current_str;
                current_str = current_str.add(3);
            }
        }
    }
}
