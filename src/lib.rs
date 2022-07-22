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
use std::borrow::Borrow;
use std::sync::{Mutex, MutexGuard};

pub use service_provider::ServiceProvider;

use crate::service_provider_extensions::ServiceProviderExtensions;

pub mod service_provider;
mod service_provider_extensions;

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
    use std::borrow::Borrow;
    use std::sync::{Arc, Mutex, MutexGuard};

    use totems::assert_err;

    use crate::{Container, ServiceProvider, ServiceProviderExtensions};

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

    #[derive(Default)]
    struct ServiceProviderStub {
        extensions: Mutex<ServiceProviderExtensions>,
    }

    impl ServiceProvider for ServiceProviderStub {
        fn extensions(&self) -> MutexGuard<'_, ServiceProviderExtensions> {
            self.extensions.borrow().lock().unwrap()
        }
    }

    #[test]
    fn empty_service_provider() {
        let sp = ServiceProviderStub::default();

        assert_err!(sp.find::<String>());
        assert_err!(sp.find::<Arc<String>>());
    }

    #[test]
    fn value_is_injected_by_type_not_by_value() {
        let mut sp = ServiceProviderStub::default();
        sp.inject("String".to_string());
        sp.inject("Ae".to_string());

        assert_eq!(sp.count(), 1);
        assert_ne!(*sp.find::<String>().unwrap(), "String".to_string());
        assert_eq!(*sp.find::<String>().unwrap(), "Ae".to_string());
    }

    #[test]
    fn inject_arc() {
        struct MyStub {}
        let mut sp = ServiceProviderStub::default();
        sp.inject(Arc::new(MyStub {}));

        assert!(sp.find::<Arc<MyStub>>().is_ok());
    }
}
