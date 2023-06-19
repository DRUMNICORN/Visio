// dyn_node.rs

use crate::DynNodiumNode;

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
