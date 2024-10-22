use ego_tree::NodeId;
use std::mem::transmute;
use std::num::NonZeroUsize;

pub(crate) fn node_id_to_u64(node_id: NodeId) -> u64 {
    // unsafe transmute because `node_id` cannot access internal usize
    let node_id: NonZeroUsize = unsafe { transmute(node_id) };
    node_id.get() as u64
}

pub(crate) fn u64_to_node_id(node_id: u64) -> NodeId {
    unsafe {
        // unsafe transmute because no public constructor
        let node_id = NonZeroUsize::new_unchecked(node_id as usize);
        transmute(node_id)
    }
}