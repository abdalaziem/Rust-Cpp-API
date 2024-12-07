use serde::Deserialize;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;

#[derive(Deserialize, Debug)]
struct TroubleCode {
    code: String,
    related_code: String,
    description: String,
    related_description: String,
}

/// Function to search for trouble codes by `code` or `related_code`
/// Exposed as C-compatible
#[no_mangle]
pub extern "C" fn search_trouble_codes(file_path: *const c_char, search_query: *const c_char) -> *mut c_char {
    // Convert C strings to Rust strings
    let file_path = unsafe {
        CStr::from_ptr(file_path)
            .to_str()
            .expect("Invalid UTF-8 in file_path")
    };

    let search_query = unsafe {
        CStr::from_ptr(search_query)
            .to_str()
            .expect("Invalid UTF-8 in search_query")
    };

    // Read and parse the JSON file
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return CString::new("Error: Failed to read file").unwrap().into_raw(),
    };

    let codes: Vec<TroubleCode> = match serde_json::from_str(&file_content) {
        Ok(codes) => codes,
        Err(_) => return CString::new("Error: Failed to parse JSON").unwrap().into_raw(),
    };

    // Filter matching codes
    let results: Vec<String> = codes
        .into_iter()
        .filter(|code| code.code == search_query || code.related_code == search_query)
        .map(|code| {
            format!(
                "Code: {}\nRelated Code: {}\nDescription: {}\nRelated Description: {}\n",
                code.code, code.related_code, code.description, code.related_description
            )
        })
        .collect();

    // Convert results to a single string
    let output = if results.is_empty() {
        "No entries found".to_string()
    } else {
        results.join("\n--------------------------------\n")
    };

    // Return the result as a C string
    CString::new(output).unwrap().into_raw()
}

/// Function to free the C string returned by `search_trouble_codes`
#[no_mangle]
pub extern "C" fn free_result(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(ptr); // Take ownership and drop
    }
}
