use fswatch_sys::*;
use std::ffi::CString;
// use std::ffi::c_void;
// use std::os::raw::c_uint;
use std::path::Path;

// extern "C" fn callback(events: *const fsw_cevent, event_num: c_uint, data: *mut c_void) {
//     println!("{:?}", event_num);
// }

#[test]
fn init_session() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("listening to: {}", dir.display());
    let dir_c = CString::new(dir.to_str().unwrap()).unwrap();
    unsafe {
        let status = fsw_init_library();
        assert_eq!(status, FSW_OK as i32);
        let handle = fsw_init_session(fsw_monitor_type_system_default_monitor_type);
        assert_ne!(handle as i64, FSW_INVALID_HANDLE as i64);
        fsw_set_recursive(handle, true);
        fsw_add_path(handle, dir_c.as_ptr());
        // fsw_set_callback(handle, Some(callback), std::ptr::null_mut());
        // without setting a callback the monitor does not start
        fsw_start_monitor(handle);
    }
}
