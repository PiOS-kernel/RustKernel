use core::marker::Sync;

pub trait Mux: Sync {
    type Wrapped;
    fn lock(&self, obj: &Self::Wrapped) -> &mut Self::Wrapped;
    fn unlock(&self);
}