use super::{new_plugin, Factory, Plugin};

inventory::submit!(Factory(new_plugin::<MyPlugin2>));

pub struct MyPlugin2;

impl Plugin for MyPlugin2 {
    const NAME: &'static str = "MyPlugin2";
}
