/// Super minimal "any map" inspired by the (now unmaintained and incompatible with rust-nightly) `anymap` crate
extern crate alloc;

use alloc::boxed::Box;
use core::any::{Any, TypeId};
use hashbrown::HashMap;

pub struct AnyMap(HashMap<TypeId, Box<dyn Any>>);

impl AnyMap {
    pub fn new() -> Self {
        AnyMap(HashMap::new())
    }

    pub fn insert<T: Any + 'static>(&mut self, value: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.0.get(&TypeId::of::<T>())?.downcast_ref()
    }

    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }

    pub fn contains<T: Any + 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }
}
