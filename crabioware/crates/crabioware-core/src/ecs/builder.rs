use core::cell::RefCell;

use anymap::hashbrown::AnyMap;

use super::core::{Component, ComponentMap, EntityId, EntityMap};

#[must_use = "Must call .build() to finish constructing the Entity"]
pub struct EntityBuilder<'a> {
    entity: EntityId,
    entities: &'a mut EntityMap,
    components: &'a mut AnyMap,
    is_built: bool,
}
impl<'a> EntityBuilder<'a> {
    // TODO: I think we need to Drop the entity_id if not build()

    pub fn new(
        entity: EntityId,
        entities: &'a mut EntityMap,
        components: &'a mut AnyMap,
    ) -> EntityBuilder<'a> {
        EntityBuilder {
            entity,
            entities,
            components,
            is_built: false,
        }
    }

    pub fn with<C>(&mut self, component: C) -> &mut Self
    where
        C: Component + 'static,
    {
        let storage = self.components.get_mut::<ComponentMap<C>>().unwrap();
        storage.insert(self.entity, RefCell::new(component));
        self
    }

    pub fn maybe_with<C>(&mut self, maybe_component: Option<C>) -> &mut Self
    where
        C: Component + 'static,
    {
        match maybe_component {
            Some(component) => {
                self.with(component);
            }
            _ => {}
        }
        self
    }

    pub fn build(&mut self) -> EntityId {
        self.is_built = true;
        self.entity
    }
}

impl<'a> Drop for EntityBuilder<'a> {
    fn drop(&mut self) {
        if !self.is_built {
            self.entities.remove(self.entity);
        }
    }
}
