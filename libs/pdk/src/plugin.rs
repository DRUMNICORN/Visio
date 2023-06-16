use crate::StaticStr;
pub trait NodiumPlugin: Send + Sync {
  extern "C" fn name(&self) -> StaticStr;
}

#[repr(C)]
pub struct DynNodiumPlugin  {
  pointer: *const (),
  name_fn: extern "C" fn(*const()) -> StaticStr,
}

unsafe impl Send for DynNodiumPlugin {}
unsafe impl Sync for DynNodiumPlugin {}

impl DynNodiumPlugin {
  pub fn new<T: NodiumPlugin>(val: &'static T) -> Self {
      DynNodiumPlugin {
          pointer: (val as *const _ as *const ()),
          name_fn: unsafe { std::mem::transmute(<T as NodiumPlugin>::name as *const ()) },
      }
  }

  pub fn name(&self) -> &'static str {
      (self.name_fn)(self.pointer).as_str()
  }

}
