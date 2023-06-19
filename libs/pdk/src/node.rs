use std::ffi::c_char;

#[repr(C)]
pub struct NodiumParam {
    pub name: *const c_char,
    pub param_type: *const c_char,
    pub value: *mut c_char,
}

pub trait NodiumNode: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    unsafe fn input_params(&self, count: *mut usize) -> *mut NodiumParam;
    unsafe fn output_params(&self, count: *mut usize) -> *const NodiumParam;
    fn process(&mut self);
}

pub struct DynNodiumNode {
    pointer: *const (),
    name_fn: fn(*const ()) -> &'static str,
    description_fn: fn(*const ()) -> &'static str,
    input_params_fn: fn(*const (), *mut usize) -> *mut NodiumParam,
    output_params_fn: fn(*const (), *mut usize) -> *const NodiumParam,
    process_fn: fn(*mut ()),
}

impl DynNodiumNode {
    pub fn new<T: NodiumNode>(val: &'static T) -> Self {
        DynNodiumNode {
            pointer: val as *const _ as *const (),
            name_fn: unsafe { std::mem::transmute(<T as NodiumNode>::name as *const ()) },
            description_fn: unsafe { std::mem::transmute(<T as NodiumNode>::description as *const ()) },
            input_params_fn: unsafe {
                std::mem::transmute(<T as NodiumNode>::input_params as *const ())
            },
            output_params_fn: unsafe { std::mem::transmute(<T as NodiumNode>::output_params as *const ()) },
            process_fn: unsafe { std::mem::transmute(<T as NodiumNode>::process as *const ()) },
        }
    }

    pub fn name(&self) -> &'static str {
        (self.name_fn)(self.pointer)
    }

    pub fn description(&self) -> &'static str {
        (self.description_fn)(self.pointer)
    }

    pub unsafe fn input_params(&self, count: *mut usize) -> *mut NodiumParam {
        (self.input_params_fn)(self.pointer, count)
    }

    pub fn output_params(&self) -> &'static [NodiumParam] {
        unsafe {
            let mut count: usize = 0;
            let params = (self.output_params_fn)(self.pointer, &mut count);
            std::slice::from_raw_parts(params, count)
        }
    }

    pub fn process(&mut self) {
        (self.process_fn)(self.pointer as *mut ());
    }
}

unsafe impl Send for DynNodiumNode {}
unsafe impl Sync for DynNodiumNode {}

unsafe impl Sync for NodiumParam {}
unsafe impl Send for NodiumParam {}
