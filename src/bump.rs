use slotmap::{Key, KeyData};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

#[repr(transparent)]
pub struct Node<T>(KeyData, std::marker::PhantomData<T>);

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node(self.0, std::marker::PhantomData)
    }
}

impl<T> Copy for Node<T> {}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Node<T> {}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node(KeyData::default(), std::marker::PhantomData)
    }
}

impl<T> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = std::any::type_name::<T>();
        f.debug_tuple(&format!("Node<{ty}>"))
            .field(&self.0)
            .finish()
    }
}

impl<T> From<KeyData> for Node<T> {
    fn from(k: KeyData) -> Self {
        Node(k, std::marker::PhantomData)
    }
}

impl<T> Hash for Node<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

unsafe impl<T> Key for Node<T> {
    fn data(&self) -> KeyData {
        self.0
    }
}

pub struct BumpMap {
    bump: bumpalo::Bump,
    slots: slotmap::SlotMap<Node<()>, *mut ()>,
}

impl BumpMap {
    pub fn new() -> Self {
        Self {
            bump: bumpalo::Bump::new(),
            slots: slotmap::SlotMap::with_key(),
        }
    }

    pub fn insert<T: 'static>(&mut self, val: T) -> Node<T> {
        let ptr = self.bump.alloc(val);
        let node = self
            .slots
            .insert(unsafe { (ptr as *mut T as *mut ()).as_mut().unwrap() });
        Node(node.0, std::marker::PhantomData)
    }

    pub fn get<T: 'static>(&self, node: Node<T>) -> Option<&T> {
        unsafe {
            self.slots
                .get(Node(node.0, PhantomData))?
                .cast::<T>()
                .as_ref()
        }
    }

    pub fn get_mut<T: 'static>(&mut self, node: Node<T>) -> Option<&mut T> {
        unsafe {
            self.slots
                .get_mut(Node(node.0, PhantomData))?
                .cast::<T>()
                .as_mut()
        }
    }
}
