/// Super minimal "any map" inspired by the (now unmaintained and incompatible with rust-nightly) `anymap` crate
extern crate alloc;

use alloc::boxed::Box;
use core::any::{Any, TypeId};
use core::hash::{BuildHasherDefault, Hasher};
use hashbrown::HashMap;

/// FIXME: ditch the any::core::TypeId (a u128) for our own version that fits better
/// into GBA memory registers. We'll only have maybe 10-30 components ever, so a u8 would
/// be sufficient and match our CPU instructions much better

/// FIXME: should we also use `agb_hashmap::HashMap` instead of the one from `hashbrown`?

// A trivial hasher for TypeId -- TypeId is already a well-distributed integer,
// so we just pass it through without mixing.
#[derive(Default)]
struct TypeIdHasher(u64);
impl Hasher for TypeIdHasher {
    fn write(&mut self, bytes: &[u8]) {
        // TypeId::of::<T>() writes 8 or 16 bytes depending on target; fold into u64
        for chunk in bytes.chunks(8) {
            let mut v = 0u64;
            for (i, &b) in chunk.iter().enumerate() {
                v |= (b as u64) << (i * 8);
            }
            self.0 ^= v;
        }
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

pub struct AnyMap(HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<TypeIdHasher>>);

impl AnyMap {
    pub fn new() -> Self {
        AnyMap(HashMap::with_hasher(BuildHasherDefault::default()))
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
