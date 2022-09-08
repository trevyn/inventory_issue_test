use std::marker::PhantomData;

use super::{new_plugin, Factory, Plugin};

inventory::submit!(Factory(new_plugin::<MyPlugin<()>>));

pub struct MyPlugin<T>(PhantomData<T>);

impl<T> Plugin for MyPlugin<T> {
    const NAME: &'static str = "MyPlugin";
}
