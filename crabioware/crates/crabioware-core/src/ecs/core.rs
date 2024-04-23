use core::any::Any;
use core::cell::RefCell;

use slotmap::{new_key_type, SecondaryMap, SlotMap};

new_key_type! { pub struct EntityId; }

// TODO: derive this as trait
// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
// e.g., https://github.com/leudz/shipyard/blob/master/shipyard_proc/src/component_expand.rs#L4
// TODO: define a type for storage on this trait
pub trait Component: Any + Sized {}

// Primary entity storage
pub type EntityMap = SlotMap<EntityId, ()>;
// Slotmap "secondary map"
pub type ComponentMap<C> = SecondaryMap<EntityId, RefCell<C>>;
