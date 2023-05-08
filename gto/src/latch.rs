use std::{cell::UnsafeCell, ops::{Deref, DerefMut}, fmt::Debug};

// Like a mutex, but without the actual locking.
// Need to make sure manually that the data is not accessed concurrently.
#[derive(Debug)]
#[repr(transparent)]
pub struct Latch<T: ?Sized> {
    data: UnsafeCell<T>
}

unsafe impl<T: ?Sized + Send> Send for Latch<T> {}
unsafe impl<T: ?Sized + Sync> Sync for Latch<T> {}

pub struct LatchGuard<'a, T: ?Sized + 'a> {
    latch: &'a Latch<T>,
}

unsafe impl<'a, T: ?Sized + Sync> Sync for LatchGuard<'a, T> {}

impl<T> Latch<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> Latch<T> {
    pub fn lock(&self) -> LatchGuard<T> {
        LatchGuard { latch: self }
    }
}

impl<'a, T: ?Sized + Debug> Debug for LatchGuard<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { &*self.latch.data.get() }.fmt(f)
    }
}

impl<T: ?Sized + Default> Default for Latch<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<'a, T: ?Sized + 'a> Deref for LatchGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.latch.data.get() }
    }
}

impl<'a, T: ?Sized + 'a> DerefMut for LatchGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.latch.data.get() }
    }
}
