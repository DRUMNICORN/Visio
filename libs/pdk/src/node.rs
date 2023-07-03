use std::ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DynNodiumNodeList {
    nodes: *const DynNodiumNode,
    length: usize,
}

impl DynNodiumNodeList {
    pub fn new(nodes: Vec<DynNodiumNode>) -> Self {
        DynNodiumNodeList {
            nodes: nodes.as_ptr(),
            length: nodes.len(),
        }
    }

    pub fn get(&self, index: usize) -> *const DynNodiumNode {
        unsafe {
            if index < self.length {
                self.nodes.add(index)
            } else {
                std::ptr::null()
            }
        }
    }
}

#[repr(C)]
pub struct DynNodiumNodeListIterator {
    nodes: *const DynNodiumNode,
    length: usize,
    index: usize,
}

impl DynNodiumNodeListIterator {
    pub fn new(node_list: DynNodiumNodeList) -> Self {
        DynNodiumNodeListIterator {
            nodes: node_list.nodes,
            length: node_list.length,
            index: 0,
        }
    }

    pub fn next(&mut self) -> *const DynNodiumNode {
        if self.index < self.length {
            unsafe {
                let node = self.nodes.add(self.index);
                self.index += 1;
                node
            }
        } else {
            std::ptr::null()
        }
    }
}

impl<'a> IntoIterator for &'a DynNodiumNodeList {
    type Item = *const DynNodiumNode;
    type IntoIter = DynNodiumNodeListIterator;

    fn into_iter(self) -> Self::IntoIter {
        DynNodiumNodeListIterator::new(*self)
    }
}

impl Iterator for DynNodiumNodeListIterator {
    type Item = *const DynNodiumNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.length {
            unsafe {
                let node = self.nodes.add(self.index);
                self.index += 1;
                Some(node)
            }
        } else {
            None
        }
    }
}

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
    name_fn: unsafe fn(*const ()) -> &'static str,
    description_fn: unsafe fn(*const ()) -> &'static str,
    input_params_fn: unsafe fn(*const (), *mut usize) -> *mut NodiumParam,
    output_params_fn: unsafe fn(*const (), *mut usize) -> *const NodiumParam,
    process_fn: unsafe fn(*mut ()),
}


impl DynNodiumNode {
    pub fn new<T: NodiumNode>(val: &'static T) -> Self {
        DynNodiumNode {
            pointer: val as *const _ as *const (),
            name_fn: unsafe { std::mem::transmute(<T as NodiumNode>::name as *const ()) },
            description_fn: unsafe {
                std::mem::transmute(<T as NodiumNode>::description as *const ())
            },
            input_params_fn: unsafe {
                std::mem::transmute(<T as NodiumNode>::input_params as *const ())
            },
            output_params_fn: unsafe {
                std::mem::transmute(<T as NodiumNode>::output_params as *const ())
            },
            process_fn: unsafe { std::mem::transmute(<T as NodiumNode>::process as *const ()) },
        }
    }

    pub fn name(&self) -> &'static str {
        unsafe { (self.name_fn)(self.pointer) }
    }

    pub fn description(&self) -> &'static str {
        unsafe { (self.description_fn)(self.pointer) }
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
        unsafe { (self.process_fn)(self.pointer as *mut ()) };
    }
    
    pub fn input_params_count(&self) -> usize {
        unsafe {
            let mut count: usize = 0;
            (self.input_params_fn)(self.pointer, &mut count);
            count
        }
    }

    pub fn output_params_count(&self) -> usize {
        unsafe {
            let mut count: usize = 0;
            (self.output_params_fn)(self.pointer, &mut count);
            count
        }
    }
}
unsafe impl Send for DynNodiumNode {}
unsafe impl Sync for DynNodiumNode {}

unsafe impl Sync for NodiumParam {}
unsafe impl Send for NodiumParam {}

use std::fmt;

impl fmt::Debug for DynNodiumNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DynNodiumNode")
            .field("name", &self.name())
            .field("input_params_count", &self.input_params_count())
            .field("output_params_count", &self.output_params_count())
            .finish()
    }
}

