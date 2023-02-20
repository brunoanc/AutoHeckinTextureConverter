// Simulates the 'pause' system command on Windows
#[cfg(target_os = "windows")]
pub fn press_any_key() {
    use windows_sys::Win32::System::Console::GetConsoleProcessList;
    use std::io::{stdin, Read};

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
