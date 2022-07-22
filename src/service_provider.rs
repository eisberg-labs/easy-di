use std::sync::MutexGuard;

use crate::{Error, ServiceProviderExtensions};

type ServiceProviderResult<T> = Result<T, Error>;

pub trait ServiceProvider {
    /// Service provider's extensions container
    fn extensions(&self) -> MutexGuard<'_, ServiceProviderExtensions>;

    /// Inserts element into service provider's extensions
    fn inject<T>(&mut self, elem: T)
    where
        T: 'static + Send + Sync + Clone,
        Self: Sized,
    {
        self.extensions().insert(elem);
    }

    /// Returns Ok(`T`) if value already hashed, otherwise returns Err.
    fn find<T>(&self) -> ServiceProviderResult<T>
    where
        T: 'static + Send + Sync + Clone,
    {
        let ext = self.extensions();
        let maybe_wrapper = ext.get::<T>();
        match maybe_wrapper {
            Some(service) => Ok(service.clone()),
            None => Err(Error::InjectLookupError(format!(
                "{:?}",
                std::any::type_name::<T>()
            ))),
        }
    }

    fn count(&self) -> usize {
        self.extensions().len()
    }
}
