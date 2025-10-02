#![allow(unused_variables)]
pub mod logger;

// Import required crates and modules
use jni::JavaVM;
use windows::Win32::System::Console::FreeConsole;
use windows::Win32::{Foundation::HINSTANCE, System::{Console::AllocConsole, SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}}};

// Entry point for the DLL. Called by the OS when the DLL is loaded or unloaded.
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn DllMain(_h_inst: HINSTANCE, reason: u32, _reserved: *mut ()) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => {
            // When the DLL is loaded, allocate a new console window.
            unsafe  {
                AllocConsole().expect("Cannot alloc console.");
            }

            // Prepare to get the created Java VMs
            let mut raw_vm: *mut jni::sys::JavaVM = std::ptr::null_mut();
            let mut size = 0;

            // Get the list of created Java VMs
            let vms = unsafe { jni::sys::JNI_GetCreatedJavaVMs(&mut raw_vm, 1, &mut size) };
            logger::info(format!("VMs count: {:?}", size).as_str());

            // Create a JavaVM instance from the raw pointer
            let jvm = unsafe { JavaVM::from_raw(raw_vm).expect("Cannot get Java VM.") };
            logger::info("Got Java VM.");

            // Attach the current thread to the JVM as a daemon
            let mut env = jvm.attach_current_thread_as_daemon().expect("Cannot get Java Env.");
            logger::info("Got Java Env.");

            // Find the Java class java.lang.String
            let class = env.find_class("java/lang/String").expect("Cannot find class.");
            logger::info("Found class.");
            // Get the method ID for String.valueOf(int)
            let method_id = env.get_method_id(class, "valueOf", "(I)Ljava/lang/String;").expect("Cannot get method ID.");
            logger::info("Got method ID.");
            // You can now use `method_id` to call the method on Java objects.
            // ...
        },
        DLL_PROCESS_DETACH => {
            // When the DLL is unloaded, free the console window.
            unsafe { FreeConsole().expect("Cannot free console.") }
        },
        _ => {}
    }

    // Return true to indicate successful handling
    true
}