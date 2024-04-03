use alloc::boxed::Box;

use anymap::hashbrown::AnyMap;
use itertools::Itertools;

use super::builder::EntityBuilder;
use super::core::{Component, ComponentMap, EntityId, EntityMap};
use super::filter::EntityMapFilter;
use super::view::View;

pub type EntityView<'r, V> = Box<<V as View>::Result<'r>>;
pub type ComponentView<'r, V> = Box<dyn Iterator<Item = <V as View>::Result<'r>> + 'r>;
pub type CombinationComponentView<'r, V> =
    Box<dyn Iterator<Item = (<V as View>::Result<'r>, <V as View>::Result<'r>)> + 'r>;

pub struct World {
    entities: EntityMap,
    pub components: AnyMap,
}
impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> World {
        Self {
            entities: EntityMap::with_key(),
            components: AnyMap::new(),
        }
    }

    // TODOs:
    // * do we need `is_alive`? (checks entity_id against current in entity array?)
    // * how can we kill entity
    //

    // TODO: move into ComponentRegistry() as part of ::new()
    //       this can help us with e.g. bitmask creation
    pub fn register_component<T: Component>(&mut self) {
        if !self.components.contains::<T>() {
            self.components.insert(ComponentMap::<T>::default());
        }
    }

    pub fn create(&mut self) -> EntityBuilder {
        let entity = self.entities.insert(());
        let builder = EntityBuilder::new(entity, &mut self.entities, &mut self.components);
        builder
    }

    pub fn destroy(&mut self, entity_id: EntityId) {
        self.entities.remove(entity_id);
    }

    pub fn entry<'r, V>(&self, entity_id: EntityId) -> EntityView<'_, V>
    where
        V: View,
    {
        Box::new(V::borrow(entity_id, &self.components))
    }

    fn filter<'f, V, F>(&'f self, entity_filter: &'f F) -> Box<dyn Iterator<Item = EntityId> + '_>
    where
        V: View,
        F: EntityMapFilter,
    {
        // this is where a bitmask might help? before going to archetype
        Box::new(
            self.entities
                .keys()
                .filter(|id| entity_filter.filter(*id))
                .filter(|id| V::filter(*id, &self.components)),
        )
    }

    pub fn query<'f, V, F>(&'f self, entity_filter: &'f F) -> ComponentView<V>
    where
        V: View,
        F: EntityMapFilter,
    {
        Box::new(
            self.filter::<V, F>(entity_filter)
                .map(|id| V::borrow(id, &self.components)),
        )
    }

    pub fn combinations<V>(&self) -> CombinationComponentView<V>
    where
        V: View,
    {
        let entities = self
            .entities
            .keys()
            .tuple_combinations()
            .filter(|(id_a, id_b)| {
                V::filter(*id_a, &self.components) && V::filter(*id_b, &self.components)
            });

        Box::new(entities.map(|(id_a, id_b)| {
            (
                V::borrow(id_a, &self.components),
                V::borrow(id_b, &self.components),
            )
        }))
    }

    fn filter_components<V>(&self) -> Box<dyn Iterator<Item = EntityId> + '_>
    where
        V: View,
    {
        // this is where a bitmask might help? before going to archetype
        Box::new(
            self.entities
                .keys()
                .filter(|id| V::filter(*id, &self.components)),
        )
    }

    pub fn components<V>(&self) -> ComponentView<V>
    where
        V: View,
    {
        Box::new(
            self.filter_components::<V>()
                .map(|id| V::borrow(id, &self.components)),
        )
    }
}
