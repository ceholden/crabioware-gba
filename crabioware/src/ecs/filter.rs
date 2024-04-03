use super::EntityId;

pub trait EntityMapFilter {
    fn filter(&self, entity: EntityId) -> bool;
}

pub struct IsEntity {
    entity: EntityId,
}
impl IsEntity {
    pub fn new(entity: EntityId) -> IsEntity {
        Self { entity }
    }
}
impl EntityMapFilter for IsEntity {
    fn filter(&self, entity: EntityId) -> bool {
        entity == self.entity
    }
}

pub struct IsNotEntity {
    entity: EntityId,
}
impl IsNotEntity {
    pub fn new(entity: EntityId) -> IsNotEntity {
        Self { entity }
    }
}
impl EntityMapFilter for IsNotEntity {
    fn filter(&self, entity: EntityId) -> bool {
        entity != self.entity
    }
}
