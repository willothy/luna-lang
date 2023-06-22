use std::marker::PhantomData;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

pub struct Arena<T, const N: usize> {
    /// Next index in the current segment
    next: AtomicUsize,
    /// The current segment
    current: AtomicPtr<[T; N]>,
    /// Old segments
    rest: boxcar::Vec<*mut [T; N]>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id<T> {
    seg: u32,
    idx: u32,
    ty: PhantomData<T>,
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self {
            seg: 0,
            idx: 0,
            ty: PhantomData,
        }
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            seg: self.seg,
            idx: self.idx,
            ty: self.ty,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T, const N: usize> Arena<T, N> {
    fn alloc_segment() -> *mut [T; N] {
        use std::alloc::{alloc, Layout};
        let layout = Layout::array::<T>(N).expect("to have a non-zero size");
        unsafe { alloc(layout).cast() }
    }

    pub fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
            current: AtomicPtr::new(Self::alloc_segment()),
            rest: boxcar::vec![],
        }
    }

    pub fn insert(&self, value: T) -> Id<T> {
        if self.next.fetch_add(1, Ordering::SeqCst) >= N {}
        let slot = match self
            .next
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| {
                if v > N {
                    self.rest
                        .push(self.current.swap(Self::alloc_segment(), Ordering::SeqCst));
                    Some(0)
                } else {
                    Some(v + 1)
                }
            }) {
            Ok(slot) => slot,
            Err(slot) => slot,
        };
        unsafe {
            self.current
                .load(Ordering::SeqCst)
                .cast::<T>()
                .add(slot)
                .write(value);
        }
        Id {
            seg: self.rest.len() as u32,
            idx: slot as u32,
            ty: PhantomData,
        }
    }

    pub fn get(&self, id: Id<T>) -> &T {
        let seg = if id.seg == self.rest.len() as u32 {
            self.current.load(Ordering::SeqCst)
        } else {
            self.rest[id.seg as usize]
        };
        unsafe { &*seg.cast::<T>().add(id.idx as usize) }
    }
}

impl<T, const N: usize> Drop for Arena<T, N> {
    fn drop(&mut self) {
        use std::{
            alloc::{dealloc, Layout},
            ptr::drop_in_place,
        };
        unsafe {
            let layout = Layout::array::<T>(N).unwrap();
            for seg in self.rest.iter() {
                for i in 0..N {
                    drop_in_place(seg.cast::<T>().add(i));
                }
                dealloc(seg.cast(), layout);
            }
            let current = self.current.load(Ordering::SeqCst);
            for i in 0..N {
                drop_in_place(current.cast::<T>().add(i));
            }
            dealloc(current.cast(), layout);
        }
    }
}

#[test]
fn bruh() {
    let arena = Arena::<u32, 4>::new();
    let id = arena.insert(0);
    let id2 = arena.insert(1);
    let id3 = arena.insert(2);
    let id4 = arena.insert(3);
    let id5 = arena.insert(4);
    let id6 = arena.insert(5);
    let id7 = arena.insert(6);

    assert_eq!(arena.get(id), &0);
    assert_eq!(arena.get(id2), &1);
    assert_eq!(arena.get(id3), &2);
    assert_eq!(arena.get(id4), &3);
    assert_eq!(arena.get(id5), &4);
    assert_eq!(arena.get(id6), &5);
    assert_eq!(arena.get(id7), &6);
}
