#[cfg(target_os = "windows")]
extern crate windows_sys;

pub mod utilities {
    use std::ptr;

    #[cfg(target_os = "windows")]
    use windows_sys::Win32::System::Console::GetConsoleProcessList;
    #[cfg(target_os = "windows")]
    use std::io::{stdin, Read};

    // Insert a slice at a specific location in a vec
    // From https://gist.github.com/frozolotl/22a051baa5153b92e0b0207ad462ec12
    pub fn insert_slice_at<T: Copy>(vec: &mut Vec<T>, index: usize, slice: &[T]) {
        unsafe {
            vec.reserve(slice.len());
            let insert_ptr = vec.as_mut_ptr().add(index);
            ptr::copy(insert_ptr, insert_ptr.add(slice.len()), vec.len() - index);
            ptr::copy_nonoverlapping(slice.as_ptr(), insert_ptr, slice.len());
            vec.set_len(vec.len() + slice.len());
        }
    }

    // Simulates the 'pause' system command on Windows
    #[cfg(target_os = "windows")]
    fn press_any_key() {
        // Get process count
        let process_count: u32;

        unsafe {
            let mut buffer = [0_u32, 1];
            process_count = GetConsoleProcessList(buffer.as_mut_ptr(), 1);
        }

        // If there's only one process, we're not running from terminal
        if process_count == 1 {
            println!("\nPress any key to exit...");
            let mut stdin = stdin();
            let _ = stdin.read(&mut [0u8]).unwrap();
        }
    }
}
