use std::any::{Any, TypeId};

use ahash::AHashMap;

/// A type map for dependency injections.
///
/// All entries into this map must be owned types (or static references).
#[derive(Default)]
pub struct ServiceProviderExtensions {
    /// Use AHasher with a std HashMap with for faster lookups on the small `TypeId` keys.
    map: AHashMap<TypeId, Box<dyn Any + Sync + Send>>,
}

impl ServiceProviderExtensions {
    pub fn insert<T: 'static + Sync + Send>(&mut self, val: T) -> Option<T> {
        self.insert_type_id(TypeId::of::<T>(), val)
    }

    fn insert_type_id<T: 'static + Sync + Send>(&mut self, type_id: TypeId, val: T) -> Option<T> {
        self.map
            .insert(type_id, Box::new(val))
            .and_then(downcast_owned)
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        let elem = self.map.get(&TypeId::of::<T>());
        elem.and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

fn downcast_owned<T: 'static + Sync>(boxed: Box<dyn Any + Sync + Send>) -> Option<T> {
    boxed.downcast().ok().map(|boxed| *boxed)
}
