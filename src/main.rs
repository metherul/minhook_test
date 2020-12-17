mod minhook;

fn main() {
    let target = winapi::um::fileapi::CreateFileW;

    let result = minhook::initialize();
    println!("initialize: {}", result as i16);

    let result = minhook::create_hook(target as _, detour_createfilew as _, target as _);
    println!("create_hook: {}", result as i16);

    let result = minhook::enable_hook(target as _);
    println!("enable_hook: {}", result as i16);

    std::fs::File::create("C:/test.txt");
    std::fs::File::create("test.txt");

    let result = minhook::remove_hook(target as _);
    println!("remove_hook: {}", result as i16);
}

#[allow(non_snake_case)]
use winapi::um::winnt::{ LPCWSTR, HANDLE };
use winapi::um::minwinbase::LPSECURITY_ATTRIBUTES;
use winapi::shared::minwindef::DWORD;
unsafe extern "C" fn detour_createfilew(
    file_name: LPCWSTR, 
    _: DWORD, 
    _: DWORD, 
    _: LPSECURITY_ATTRIBUTES, 
    _: DWORD, 
    _: DWORD, 
    _: HANDLE) -> i32 {
        let test = widestring::U16CStr::from_ptr_str(file_name).to_string().unwrap();
        println!("-> CreateFileW: {}", test);
        0
}
