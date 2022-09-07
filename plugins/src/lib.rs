use std::{any::TypeId, marker::PhantomData};

use inner::{Meta, Factory, Plugin};

pub fn new_plugin<T: Plugin + 'static>() -> Meta {
    println!("registered {}: {:?}", T::NAME, TypeId::of::<T>());

    Meta {
        id: TypeId::of::<T>(),
        name: T::NAME,
    }
}

inventory::submit!(Factory(new_plugin::<MyPlugin<()>>));
inventory::submit!(Factory(new_plugin::<MyPlugin2>));

pub struct MyPlugin<T>(PhantomData<T>);

pub struct MyPlugin2;

impl<T> Plugin for MyPlugin<T> {
    const NAME: &'static str = "MyPlugin";
}

impl Plugin for MyPlugin2 {
    const NAME: &'static str = "MyPlugin2";
}
