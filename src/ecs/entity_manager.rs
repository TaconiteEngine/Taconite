use crate::ecs::{
    component::Component,
    component_manager::{cast_manager, cast_manager_mut, ComponentManager, ComponentManagerT},
    entity::Entity,
};
use std::{
    any::TypeId,
    collections::{hash_map, HashMap},
    vec,
};
use tracing::*;
struct Entities {
    entities: Vec<Entity>,
    available_indexes: Vec<usize>,
}

impl Entities {
    fn new() -> Self {
        Entities {
            entities: vec![],
            available_indexes: vec![],
        }
    }

    fn entity_exists(&self, entity_id: usize) -> bool {
        entity_id < self.entities.len() && self.entities[entity_id].is_alive()
    }

    fn create(&mut self) -> usize {
        if !self.available_indexes.is_empty() {
            let index = self.available_indexes.remove(0);
            self.entities[index].enable();
            return index;
        }

        self.entities.push(Entity::default());

        self.entities.len() - 1
    }

    fn remove(&mut self, entity_id: usize) {
        if !self.entity_exists(entity_id) {
            error!("Can't remove this ID, it is not a used ID.");
            return;
        }

        self.entities[entity_id].disable();
        self.available_indexes.push(entity_id);
    }
}

#[derive(Default)]
pub struct EntityIdAccessor {
    caching_map: HashMap<TypeId, Vec<usize>>,
    updated_map: HashMap<TypeId, u64>,
}

impl EntityIdAccessor {
    pub fn borrow_ids<T: 'static + Component>(
        &mut self,
        manager: &EntityManager,
    ) -> Option<&Vec<usize>> {
        if !manager.has_component_manager::<T>() {
            return None;
        }

        let type_id = TypeId::of::<T>();
        let needs_update = if let hash_map::Entry::Vacant(e) = self.caching_map.entry(type_id) {
            e.insert(Vec::new());
            true
        } else {
            let updated_frame = *self.updated_map.get(&type_id).unwrap();
            manager.get_updated_frame::<T>() != updated_frame
        };

        if needs_update {
            let src = &manager.borrow_entity_ids::<T>().unwrap();
            let dst = self.caching_map.get_mut(&type_id).unwrap();
            dst.clear();
            for id in src.iter() {
                dst.push(*id);
            }
            self.updated_map.insert(type_id, manager.get_frame());
        }

        self.caching_map.get(&type_id)
    }

    pub fn borrow_ids_for_pair<T1: 'static + Component, T2: 'static + Component>(
        &mut self,
        manager: &EntityManager,
    ) -> Option<&Vec<usize>> {
        if !manager.has_component_manager::<T1>() || !manager.has_component_manager::<T2>() {
            return None;
        }

        let type_id = TypeId::of::<(T1, T2)>();
        let needs_update =
            if let std::collections::hash_map::Entry::Vacant(e) = self.caching_map.entry(type_id) {
                e.insert(Vec::new());
                true
            } else {
                let updated_frame = *self.updated_map.get(&type_id).unwrap();
                manager.get_updated_frame::<T1>() != updated_frame
                    || manager.get_updated_frame::<T2>() != updated_frame
            };

        if needs_update {
            // TODO: Can be optimized if iterating a shorter array
            let src = &manager.borrow_entity_ids::<T1>().unwrap();
            let manager2 = manager.borrow_component_manager::<T2>();
            let dst = self.caching_map.get_mut(&type_id).unwrap();
            dst.clear();
            for id in src.iter() {
                if manager2.component_exists(*id) {
                    dst.push(*id);
                }
            }
            self.updated_map.insert(type_id, manager.get_frame());
        }

        self.caching_map.get(&type_id)
    }

    pub fn borrow_ids_for_triple<
        T1: 'static + Component,
        T2: 'static + Component,
        T3: 'static + Component,
    >(
        &mut self,
        manager: &EntityManager,
    ) -> Option<&Vec<usize>> {
        if !manager.has_component_manager::<T1>()
            || !manager.has_component_manager::<T2>()
            || !manager.has_component_manager::<T3>()
        {
            return None;
        }

        let type_id = TypeId::of::<(T1, T2, T3)>();
        let needs_update =
            if let std::collections::hash_map::Entry::Vacant(e) = self.caching_map.entry(type_id) {
                e.insert(Vec::new());
                true
            } else {
                let updated_frame = *self.updated_map.get(&type_id).unwrap();
                manager.get_updated_frame::<T1>() != updated_frame
                    || manager.get_updated_frame::<T2>() != updated_frame
                    || manager.get_updated_frame::<T3>() != updated_frame
            };

        if needs_update {
            // TODO: Can be optimized if iterating the shortest array
            let src = &manager.borrow_entity_ids::<T1>().unwrap();
            let manager2 = manager.borrow_component_manager::<T2>();
            let manager3 = manager.borrow_component_manager::<T3>();
            let dst = self.caching_map.get_mut(&type_id).unwrap();
            dst.clear();
            for id in src.iter() {
                if manager2.component_exists(*id) && manager3.component_exists(*id) {
                    dst.push(*id);
                }
            }
            self.updated_map.insert(type_id, manager.get_frame());
        }

        self.caching_map.get(&type_id)
    }

    pub fn borrow_ids_for_quad<
        T1: 'static + Component,
        T2: 'static + Component,
        T3: 'static + Component,
        T4: 'static + Component,
    >(
        &mut self,
        manager: &EntityManager,
    ) -> Option<&Vec<usize>> {
        if !manager.has_component_manager::<T1>()
            || !manager.has_component_manager::<T2>()
            || !manager.has_component_manager::<T3>()
            || !manager.has_component_manager::<T4>()
        {
            return None;
        }

        let type_id = TypeId::of::<(T1, T2, T3, T4)>();
        let needs_update =
            if let std::collections::hash_map::Entry::Vacant(e) = self.caching_map.entry(type_id) {
                e.insert(Vec::new());
                true
            } else {
                let updated_frame = *self.updated_map.get(&type_id).unwrap();
                manager.get_updated_frame::<T1>() != updated_frame
                    || manager.get_updated_frame::<T2>() != updated_frame
                    || manager.get_updated_frame::<T3>() != updated_frame
                    || manager.get_updated_frame::<T4>() != updated_frame
            };

        if needs_update {
            // TODO: Can be optimized if iterating the shortest array
            let src = &manager.borrow_entity_ids::<T1>().unwrap();
            let manager2 = manager.borrow_component_manager::<T2>();
            let manager3 = manager.borrow_component_manager::<T3>();
            let manager4 = manager.borrow_component_manager::<T4>();
            let dst = self.caching_map.get_mut(&type_id).unwrap();
            dst.clear();
            for id in src.iter() {
                if manager2.component_exists(*id)
                    && manager3.component_exists(*id)
                    && manager4.component_exists(*id)
                {
                    dst.push(*id);
                }
            }
            self.updated_map.insert(type_id, manager.get_frame());
        }

        self.caching_map.get(&type_id)
    }
}

pub struct EntityManager {
    entities: Entities,
    manager_map: HashMap<TypeId, Box<dyn ComponentManagerT>>,
    frame: u64,                              // Rename
    updated_frame_map: HashMap<TypeId, u64>, // Rename
}

impl Default for EntityManager {
    fn default() -> Self {
        Self {
            entities: Entities::new(),
            manager_map: HashMap::new(),

            frame: 0,

            updated_frame_map: HashMap::new(),
        }
    }
}

impl EntityManager {
    pub fn increment_frame(&mut self) {
        self.frame += 1;
    }

    fn get_frame(&self) -> u64 {
        self.frame
    }

    fn get_updated_frame<T: 'static + Component>(&self) -> u64 {
        *self.updated_frame_map.get(&TypeId::of::<T>()).unwrap()
    }

    pub fn register<T: 'static + Component>(&mut self) -> &mut Self {
        // TODO: Error handling if already registered?
        if !self.has_component_manager::<T>() {
            let type_id = TypeId::of::<T>();
            self.manager_map
                .insert(type_id, Box::new(ComponentManager::<T>::new()));
            self.updated_frame_map.insert(type_id, self.get_frame());
        }
        self
    }

    pub fn create_entity(&mut self) -> usize {
        self.entities.create()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        let frame = self.get_frame();
        for (_, manager) in self.manager_map.iter_mut() {
            if manager.component_exists(entity_id) {
                manager.remove(entity_id);
                self.updated_frame_map
                    .insert(manager.get_type_id(), frame + 1);
            }
        }
        self.entities.remove(entity_id);
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut Self {
        if !self.has_component_manager::<T>() {
            // TODO: Better error handling
            error!("Failed to add component to entity. It does not exist.");
            return self;
        }
        self.borrow_component_manager_mut::<T>()
            .add(entity_id, component);
        self.updated_frame_map
            .insert(TypeId::of::<T>(), self.get_frame());

        self
    }

    fn borrow_entity_ids<T: 'static + Component>(&self) -> Option<&Vec<usize>> {
        if !self.has_component_manager::<T>() {
            error!("Failed to get component from entity. It does not exist.");
            println!("Unknown component");
            return None;
        }
        Some(self.borrow_component_manager::<T>().borrow_entity_ids())
    }

    pub fn borrow_component<T: 'static + Component>(&self, entity_id: usize) -> Option<&T> {
        match self.has_component_manager::<T>() {
            true => self
                .borrow_component_manager::<T>()
                .borrow_component(entity_id),
            false => None,
        }
    }

    pub fn borrow_component_mut<T: 'static + Component>(
        &mut self,
        entity_id: usize,
    ) -> Option<&mut T> {
        match self.has_component_manager::<T>() {
            true => self
                .borrow_component_manager_mut::<T>()
                .borrow_component_mut(entity_id),
            false => None,
        }
    }

    pub fn borrow_components<T: 'static + Component>(&self) -> Option<&Vec<T>> {
        match self.has_component_manager::<T>() {
            true => Some(self.borrow_component_manager::<T>().borrow_components()),
            false => None,
        }
    }

    pub fn borrow_components_mut<T: 'static + Component>(&mut self) -> Option<&mut Vec<T>> {
        match self.has_component_manager::<T>() {
            true => Some(
                self.borrow_component_manager_mut::<T>()
                    .borrow_components_mut(),
            ),
            false => None,
        }
    }

    pub fn borrow_components_pair_mut<T1: 'static + Component, T2: 'static + Component>(
        &mut self,
    ) -> Option<(&mut Vec<T1>, &mut Vec<T2>)> {
        if !self.has_component_manager::<T1>() || !self.has_component_manager::<T2>() {
            return None;
        }

        let type_id1 = TypeId::of::<T1>();
        let type_id2 = TypeId::of::<T2>();

        let manager1 = cast_manager_mut_unsafe(self.manager_map.get(&type_id1).unwrap());
        let manager2 = cast_manager_mut_unsafe(self.manager_map.get(&type_id2).unwrap());

        Some((
            manager1.borrow_components_mut(),
            manager2.borrow_components_mut(),
        ))
    }

    pub fn borrow_components_triple_mut<
        T1: 'static + Component,
        T2: 'static + Component,
        T3: 'static + Component,
    >(
        &mut self,
    ) -> Option<(&mut Vec<T1>, &mut Vec<T2>, &mut Vec<T3>)> {
        if !self.has_component_manager::<T1>()
            || !self.has_component_manager::<T2>()
            || !self.has_component_manager::<T3>()
        {
            return None;
        }

        let type_id1 = TypeId::of::<T1>();
        let type_id2 = TypeId::of::<T2>();
        let type_id3 = TypeId::of::<T3>();

        let manager1 = cast_manager_mut_unsafe(self.manager_map.get(&type_id1).unwrap());
        let manager2 = cast_manager_mut_unsafe(self.manager_map.get(&type_id2).unwrap());
        let manager3 = cast_manager_mut_unsafe(self.manager_map.get(&type_id3).unwrap());

        Some((
            manager1.borrow_components_mut(),
            manager2.borrow_components_mut(),
            manager3.borrow_components_mut(),
        ))
    }
    #[allow(clippy::type_complexity)]
    pub fn borrow_components_quad_mut<
        T1: 'static + Component,
        T2: 'static + Component,
        T3: 'static + Component,
        T4: 'static + Component,
    >(
        &mut self,
    ) -> Option<(&mut Vec<T1>, &mut Vec<T2>, &mut Vec<T3>, &mut Vec<T4>)> {
        if !self.has_component_manager::<T1>()
            || !self.has_component_manager::<T2>()
            || !self.has_component_manager::<T3>()
            || !self.has_component_manager::<T4>()
        {
            return None;
        }

        let type_id1 = TypeId::of::<T1>();
        let type_id2 = TypeId::of::<T2>();
        let type_id3 = TypeId::of::<T3>();
        let type_id4 = TypeId::of::<T4>();

        let manager1 = cast_manager_mut_unsafe(self.manager_map.get(&type_id1).unwrap());
        let manager2 = cast_manager_mut_unsafe(self.manager_map.get(&type_id2).unwrap());
        let manager3 = cast_manager_mut_unsafe(self.manager_map.get(&type_id3).unwrap());
        let manager4 = cast_manager_mut_unsafe(self.manager_map.get(&type_id4).unwrap());

        Some((
            manager1.borrow_components_mut(),
            manager2.borrow_components_mut(),
            manager3.borrow_components_mut(),
            manager4.borrow_components_mut(),
        ))
    }

    pub fn borrow_component_pair_mut<T1: 'static + Component, T2: 'static + Component>(
        &mut self,
        entity_id: usize,
    ) -> Option<(&mut T1, &mut T2)> {
        if !self.has_component_manager::<T1>() || !self.has_component_manager::<T2>() {
            return None;
        }

        let type_id1 = TypeId::of::<T1>();
        let type_id2 = TypeId::of::<T2>();

        let manager1 = cast_manager_mut_unsafe(self.manager_map.get(&type_id1).unwrap());
        let manager2 = cast_manager_mut_unsafe(self.manager_map.get(&type_id2).unwrap());

        if !manager1.component_exists(entity_id) || !manager2.component_exists(entity_id) {
            return None;
        }

        Some((
            manager1.borrow_component_mut(entity_id).unwrap(),
            manager2.borrow_component_mut(entity_id).unwrap(),
        ))
    }

    pub fn borrow_component_triple_mut<
        T1: 'static + Component,
        T2: 'static + Component,
        T3: 'static + Component,
    >(
        &mut self,
        entity_id: usize,
    ) -> Option<(&mut T1, &mut T2, &mut T3)> {
        if !self.has_component_manager::<T1>()
            || !self.has_component_manager::<T2>()
            || !self.has_component_manager::<T3>()
        {
            return None;
        }

        let type_id1 = TypeId::of::<T1>();
        let type_id2 = TypeId::of::<T2>();
        let type_id3 = TypeId::of::<T3>();

        let manager1 = cast_manager_mut_unsafe(self.manager_map.get(&type_id1).unwrap());
        let manager2 = cast_manager_mut_unsafe(self.manager_map.get(&type_id2).unwrap());
        let manager3 = cast_manager_mut_unsafe(self.manager_map.get(&type_id3).unwrap());

        if !manager1.component_exists(entity_id)
            || !manager2.component_exists(entity_id)
            || !manager3.component_exists(entity_id)
        {
            return None;
        }

        Some((
            manager1.borrow_component_mut(entity_id).unwrap(),
            manager2.borrow_component_mut(entity_id).unwrap(),
            manager3.borrow_component_mut(entity_id).unwrap(),
        ))
    }

    pub fn borrow_component_quad_mut<
        T1: 'static + Component,
        T2: 'static + Component,
        T3: 'static + Component,
        T4: 'static + Component,
    >(
        &mut self,
        entity_id: usize,
    ) -> Option<(&mut T1, &mut T2, &mut T3, &mut T4)> {
        if !self.has_component_manager::<T1>()
            || !self.has_component_manager::<T2>()
            || !self.has_component_manager::<T3>()
            || !self.has_component_manager::<T4>()
        {
            return None;
        }

        let type_id1 = TypeId::of::<T1>();
        let type_id2 = TypeId::of::<T2>();
        let type_id3 = TypeId::of::<T3>();
        let type_id4 = TypeId::of::<T4>();

        let manager1 = cast_manager_mut_unsafe(self.manager_map.get(&type_id1).unwrap());
        let manager2 = cast_manager_mut_unsafe(self.manager_map.get(&type_id2).unwrap());
        let manager3 = cast_manager_mut_unsafe(self.manager_map.get(&type_id3).unwrap());
        let manager4 = cast_manager_mut_unsafe(self.manager_map.get(&type_id4).unwrap());

        if !manager1.component_exists(entity_id)
            || !manager2.component_exists(entity_id)
            || !manager3.component_exists(entity_id)
            || !manager4.component_exists(entity_id)
        {
            return None;
        }

        Some((
            manager1.borrow_component_mut(entity_id).unwrap(),
            manager2.borrow_component_mut(entity_id).unwrap(),
            manager3.borrow_component_mut(entity_id).unwrap(),
            manager4.borrow_component_mut(entity_id).unwrap(),
        ))
    }

    fn has_component_manager<T: 'static + Component>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.manager_map.contains_key(&type_id)
    }

    fn borrow_component_manager<T: 'static + Component>(&self) -> &ComponentManager<T> {
        let type_id = TypeId::of::<T>();
        cast_manager(self.manager_map.get(&type_id).unwrap().as_ref())
    }

    fn borrow_component_manager_mut<T: 'static + Component>(&mut self) -> &mut ComponentManager<T> {
        let type_id = TypeId::of::<T>();
        cast_manager_mut(self.manager_map.get_mut(&type_id).unwrap().as_mut())
    }
}

#[allow(clippy::mut_from_ref, clippy::borrowed_box)]
fn cast_manager_mut_unsafe<T: 'static + Component>(
    manager: &Box<dyn ComponentManagerT>,
) -> &mut ComponentManager<T> {
    let ptr =
        cast_manager(manager.as_ref()) as *const ComponentManager<T> as *mut ComponentManager<T>;
    unsafe { &mut *ptr }
}
