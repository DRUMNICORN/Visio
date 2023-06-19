use nodium_pdk::node::{NodiumNode, NodiumParam};
use crate::utils::download::process_download_node;
use std::{ffi::{CString, CStr}};

use std::sync::{Arc, Mutex};

pub struct NodiumDownloadNode {
    input_params: Arc<Mutex<Vec<NodiumParam>>>,
    output_params: Arc<Mutex<Vec<NodiumParam>>>,
}


impl NodiumDownloadNode {
    pub fn new() -> Self {
        let input_params = vec![
            NodiumParam {
                name: CString::new("crate_name").unwrap().into_raw(),
                param_type: CString::new("string").unwrap().into_raw(),
                value: CString::new("").unwrap().into_raw(),
            },
            NodiumParam {
                name: CString::new("param2").unwrap().into_raw(),
                param_type: CString::new("number").unwrap().into_raw(),
                value: CString::new("").unwrap().into_raw(),
            },
        ];

        let output_params = vec![NodiumParam {
            name: CString::new("success").unwrap().into_raw(),
            param_type: CString::new("string").unwrap().into_raw(),
            value: CString::new("").unwrap().into_raw(),
        }];

        let input_params = Arc::new(Mutex::new(input_params));
        let output_params = Arc::new(Mutex::new(output_params));

        NodiumDownloadNode {
            input_params,
            output_params,
        }
    }
    
    fn get_crate_name(&mut self) -> &str {
        let crate_name = unsafe {
            let name = CString::new("crate_name").unwrap();
            let mut count: usize = 0;
            let input_params_ptr = self.input_params(&mut count);
            let param = (0..count)
                .map(|i| input_params_ptr.add(i))
                .find(|p| CStr::from_ptr((*(*p)).name as *const i8) == name.as_c_str())
                .expect("Missing input parameter 'crate_name'");
            let value = (*param).value as *const i8;
            CStr::from_ptr(value).to_str().unwrap()
        };
        crate_name
    }
    
    
}

impl NodiumNode for NodiumDownloadNode {
    fn name(&self) -> &'static str {
        "download_node"
    }

    fn description(&self) -> &'static str {
        "Downloads a cargo crate."
    }

    unsafe fn input_params(&self, count: *mut usize) -> *mut NodiumParam {
        *count = self.input_params.lock().unwrap().len();
        self.input_params.lock().unwrap().as_mut_ptr()
    }
    
    unsafe fn output_params(&self, count: *mut usize) -> *const NodiumParam {
        *count = self.output_params.lock().unwrap().len();
        self.output_params.lock().unwrap().as_ptr()
    }

    fn process(&mut self) {
        let crate_name = self.get_crate_name();
        let success_name = CString::new("success").unwrap();
        let success_value = CString::new("true").unwrap();

        let download_future = process_download_node(crate_name.to_owned());

        if !download_future.is_ok() {
            println!("Failed to download");
            return;
        }

        let mut output_params = self.output_params.lock().unwrap();
        let param = output_params.iter_mut()
            .find(|p| unsafe { CStr::from_ptr(p.name) } == success_name.as_c_str())
            .expect("Missing output parameter 'success'");
        param.value = success_value.into_raw();
    }

}