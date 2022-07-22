#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::{Container, ServiceProvider};

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
    fn test_dependency_is_injected(){
        let mut container = Container::new();
        let animal: Arc<dyn Animal + Sync + Send> = Arc::new(Dog);
        container.inject(animal.clone());

        let animal2 = container.find::<Arc<dyn Animal + Sync + Send>>();

        assert!(animal2.is_ok())
    }
}
