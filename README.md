# Easy DI [![Continuous Integration](https://github.com/eisberg-labs/easy-di/actions/workflows/ci.yml/badge.svg)](https://github.com/eisberg-labs/easy-di/actions/workflows/ci.yml) [![cargo-badge][]][cargo] [![license-badge][]][license]
> Simple dependency injection container for Rust.

# Example
Code:

```rust
use std::sync::Arc;
use easy_di::{Container, ServiceProvider};
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
let mut container = Container::new();
let animal: Arc<dyn Animal + Sync + Send> = Arc::new(Dog);
container.inject(animal);
let animal2 = container.find::<Arc<dyn Animal + Sync + Send>>();
animal2.unwrap().make_sound();
}
```

## Contributing

This project welcomes all kinds of contributions. No contribution is too small!

If you want to contribute to this project but don't know how to begin or if you need help with something related to this project, 
feel free to send me an email <https://www.eisberg-labs.com/> (contact form at the bottom).

Some pointers on contribution are in [Contributing.md](./CONTRIBUTING.md)

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).


# License

Distributed under the terms of [MIT license](./LICENSE-MIT) and [Apache license](./LICENSE-APACHE).

[cargo-badge]: https://img.shields.io/crates/v/easy-di.svg?style=flat-square
[cargo]: https://crates.io/crates/easy-di
[license-badge]: https://img.shields.io/badge/license-MIT/Apache--2.0-lightgray.svg?style=flat-square
[license]: #license
