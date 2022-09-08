mod my_plugin;
mod my_plugin2;

use std::any::TypeId;

pub use self::{my_plugin::MyPlugin, my_plugin2::MyPlugin2};

inventory::collect!(Factory);

#[derive(Clone, Copy, Debug)]
pub struct Factory(pub fn() -> Meta);

#[derive(Clone, Copy, Debug)]
pub struct Meta {
    pub id: TypeId,

    pub name: &'static str,
}

impl Meta {
    pub fn collect() -> Vec<Self> {
        inventory::iter::<Factory>
            .into_iter()
            .map(|factory| (factory.0)())
            .collect()
    }
}

pub trait Plugin {
    const NAME: &'static str;
}

pub fn new_plugin<T: Plugin + 'static>() -> Meta {
    println!("registered {}: {:?}", T::NAME, TypeId::of::<T>());

    Meta {
        id: TypeId::of::<T>(),
        name: T::NAME,
    }
}
