use crate::{Entity, Event, NodeMap, State};
use fnv::FnvHashMap;

use std::any::{Any, TypeId};
pub trait Node: Any {
    fn on_mutate(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, node: &dyn Node, nodes: &NodeMap) {

    }

    //fn on_update(&self, state: &mut State, entity: Entity, data: &Box<dyn Node>) {}

    fn build(mut self, state: &mut State, parent: Entity) -> Entity 
    where Self: std::marker::Sized + 'static
    {
        let entity = state.entity_manager.create_entity().unwrap();

        state.data_graph.add(entity, parent);

        state.data_nodes.insert(entity, Box::new(self));

        entity
    }
}

impl dyn Node {
    // Check if a message is a certain type
    pub fn is<T: Any + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Casts a message to the specified type if the message is of that type
    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: Node + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Node as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Node as *const T)) }
        } else {
            None
        }
    }
}

trait Downcast {
    fn as_any (self: &'_ Self)
      -> &'_ dyn Any
    where
        Self : 'static,
    ;
}

impl<T: Node> Downcast for T {
    fn as_any (self: &'_ Self)
      -> &'_ dyn Any
    where
        Self : 'static,
    {
        self
    }
}

// Can't do this apparently, that's annoying
// impl<T: Node> EventHandler for T
// where
//     T: Node + 'static,
// {
//     fn update(&mut self, data: &Self) -> bool {
//         <T as Node>::update(self, data)
//     }
// }