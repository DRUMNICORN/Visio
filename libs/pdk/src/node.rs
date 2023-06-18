use crate::StaticStr;
use crate::FfiSafeHashMap;

pub trait NodiumNode: Send + Sync {
    extern "C" fn name(&self) -> StaticStr;
    extern "C" fn description(&self) -> StaticStr;
    extern "C" fn input_params(&self) -> FfiSafeHashMap;
    extern "C" fn output_params(&self) -> FfiSafeHashMap;
    extern "C" fn process(&mut self);
}

#[repr(C)]
pub struct DynNodiumNode {
    pointer: *const (),
    name_fn: extern "C" fn(*const ()) -> StaticStr,
    description_fn: extern "C" fn(*const ()) -> StaticStr,
    input_params_fn: extern "C" fn(*const ()) -> FfiSafeHashMap,
    output_params_fn: extern "C" fn(*const ()) -> FfiSafeHashMap,
    process_fn: extern "C" fn(*mut ()),
}

unsafe impl Send for DynNodiumNode {}
unsafe impl Sync for DynNodiumNode {}

impl DynNodiumNode {
    pub fn new<T: NodiumNode>(val: &'static T) -> Self {
        DynNodiumNode {
            pointer: (val as *const _ as *const ()),
            name_fn: unsafe { std::mem::transmute(<T as NodiumNode>::name as *const ()) },
            description_fn: unsafe { std::mem::transmute(<T as NodiumNode>::description as *const ()) },
            input_params_fn: unsafe { std::mem::transmute(<T as NodiumNode>::input_params as *const ()) },
            output_params_fn: unsafe { std::mem::transmute(<T as NodiumNode>::output_params as *const ()) },
            process_fn: unsafe { std::mem::transmute(<T as NodiumNode>::process as *const ()) },
        }
    }

    pub fn name(&self) -> &'static str {
        (self.name_fn)(self.pointer).as_str()
    }

    pub fn description(&self) -> &'static str {
        (self.description_fn)(self.pointer).as_str()
    }

    pub fn input_params(&self) -> FfiSafeHashMap {
        (self.input_params_fn)(self.pointer)
    }

    pub fn output_params(&self) -> FfiSafeHashMap {
        (self.output_params_fn)(self.pointer)
    }

    pub fn process(&mut self) {
        (self.process_fn)(self.pointer as *mut ())
    }
}
