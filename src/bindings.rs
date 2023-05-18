mod wasm {
    wit_bindgen::generate!("heathen");
    struct WasmInterface;
    impl Heathen for WasmInterface {
        fn rpc(request: wit_bindgen::rt::string::String) -> wit_bindgen::rt::string::String {
            crate::rpc::call(request.as_str())
        }
    }
    
    export_heathen!(WasmInterface);
}

mod c {
    #[no_mangle]
    pub extern "C" fn call(request: *const std::ffi::c_char) -> *mut std::ffi::c_char {
        let request = unsafe {
            if request.is_null() {
                return std::ffi::CString::new(crate::rpc::Response::ok(None, serde_json::Value::Null)).unwrap().into_raw();
            }
            
            let Ok(request) = std::ffi::CStr::from_ptr(request).to_str() else {
                return std::ffi::CString::new(crate::rpc::Response::ok(None, serde_json::Value::Null)).unwrap().into_raw();
            };
            
            request
        };
        
        let json_response = crate::rpc::call(request);
        
        std::ffi::CString::new(json_response).unwrap().into_raw()
    }
}
