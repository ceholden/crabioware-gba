// Find and borrow component data by entity ID(s)
use anymap::hashbrown::AnyMap;
use core::cell::{Ref, RefMut};

use super::core::{Component, ComponentMap, EntityId};

pub trait View {
    type Result<'r>;

    fn borrow(id: EntityId, component_maps: &AnyMap) -> Self::Result<'_>;
    fn filter(id: EntityId, component_maps: &AnyMap) -> bool;
}

impl View for EntityId {
    type Result<'r> = EntityId;

    fn borrow(id: EntityId, _: &AnyMap) -> Self::Result<'_> {
        id
    }
    fn filter(_: EntityId, _: &AnyMap) -> bool {
        true
    }
}

impl<C> View for &C
where
    C: Component + 'static,
{
    type Result<'r> = Ref<'r, C>;

    fn borrow(id: EntityId, component_maps: &AnyMap) -> Self::Result<'_> {
        component_maps
            .get::<ComponentMap<C>>()
            .unwrap()
            .get(id)
            .unwrap()
            .borrow()
    }

    fn filter(id: EntityId, component_maps: &AnyMap) -> bool {
        component_maps
            .get::<ComponentMap<C>>()
            .is_some_and(|cm| cm.contains_key(id))
    }
}

impl<C> View for &mut C
where
    C: Component + 'static,
{
    type Result<'r> = RefMut<'r, C>;

    fn borrow(id: EntityId, component_maps: &AnyMap) -> Self::Result<'_> {
        component_maps
            .get::<ComponentMap<C>>()
            .unwrap()
            .get(id)
            .unwrap()
            .borrow_mut()
    }

    fn filter(id: EntityId, component_maps: &AnyMap) -> bool {
        component_maps
            .get::<ComponentMap<C>>()
            .is_some_and(|cm| cm.contains_key(id))
    }
}


macro_rules! impl_view_for_tuple {
  ($($name:ident)*) => {
      #[allow(unused)]
      #[allow(clippy::unused_unit)]
      impl<$($name,)*> View for ($($name,)*)
      where $($name: View + 'static,)*
      {
          type Result<'r> = ($($name::Result<'r>,)*);

          fn borrow(id: EntityId, component_maps: &AnyMap) -> Self::Result<'_> {
              ($($name::borrow(id, component_maps),)*)
          }

          // This is an "AND" operation
          fn filter(id: EntityId, component_maps: &AnyMap) -> bool {
              match ($($name::filter(id, component_maps),)*) {
                  ($(replace_expr!($name true),)*) => true,
                  _ => false,
              }
          }
      }
  };
}

macro_rules! replace_expr {
    ($_t:tt $repl:expr) => {
        $repl
    };
}

impl_view_for_tuple!(A);
impl_view_for_tuple!(A B);
impl_view_for_tuple!(A B C);
impl_view_for_tuple!(A B C D);
impl_view_for_tuple!(A B C D E);
impl_view_for_tuple!(A B C D E F);
impl_view_for_tuple!(A B C D E F G);
