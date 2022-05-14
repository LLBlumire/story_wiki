use std::{ops::Deref, rc::Rc};

/// An immutable reference counter, while it does not prevent you from doing
/// interior mutability, its PartialEq implementation will make any interior
/// changes not be detected by anything that relies on PartialEq for change
/// detection.
#[derive(Debug, Ord, PartialOrd)]
pub struct Irc<T> {
    inner: Rc<T>,
}
impl<T> PartialEq for Irc<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}
impl<T> Eq for Irc<T> {}
impl<T> Clone for Irc<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<T> Deref for Irc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        Deref::deref(&self.inner)
    }
}
impl<T> Irc<T> {
    pub fn new(t: T) -> Self {
        Irc { inner: Rc::new(t) }
    }
}
