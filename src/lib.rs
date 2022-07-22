//! Easy DI is dependency injection container for Rust.
//!
//! Setup:
//! ```
//! use std::sync::Arc;
//! use easy_di::{Container, ServiceProvider};
//!
//! pub trait Animal {
//!     fn make_sound(&self);
//! }
//!
//! #[derive(Clone)]
//! struct Dog;
//! impl Animal for Dog {
//!     fn make_sound(&self) {
//!         println!("woof woof!")
//!     }
//! }
//!
//! let mut container = Container::new();
//! let animal: Arc<dyn Animal + Sync + Send> = Arc::new(Dog);
//! container.inject(animal);
//!
//! let animal2 = container.find::<Arc<dyn Animal + Sync + Send>>();
//! animal2.unwrap().make_sound();
//!
//! ```
//!
use crate::service_provider_extensions::ServiceProviderExtensions;
use std::borrow::Borrow;
use std::sync::{Mutex, MutexGuard};

pub mod service_provider;
mod service_provider_extensions;

pub use service_provider::ServiceProvider;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("InjectLookupError({0})")]
    InjectLookupError(String),
}

#[derive(Default)]
pub struct Container {
    extensions: Mutex<ServiceProviderExtensions>,
}

impl Container {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ServiceProvider for Container {
    fn extensions(&self) -> MutexGuard<'_, ServiceProviderExtensions> {
        self.extensions.borrow().lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Container, ServiceProvider};
    use std::sync::Arc;

    pub trait Animal {
        fn make_sound(&self);
    }

    #[derive(Clone)]
    struct Dog;

    impl Animal for Dog {
        fn make_sound(&self) {
            println!("woof woof!")
        }
    }

    #[test]
    fn test_dependency_is_injected() {
        let mut container = Container::new();
        let animal: Arc<dyn Animal + Sync + Send> = Arc::new(Dog);
        container.inject(animal.clone());

        let animal2 = container.find::<Arc<dyn Animal + Sync + Send>>();

        assert!(animal2.is_ok())
    }
}
