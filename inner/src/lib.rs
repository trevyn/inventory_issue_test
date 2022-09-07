use std::any::TypeId;

use once_cell::sync::Lazy;

inventory::collect!(Factory);

#[derive(Clone, Copy, Debug)]
pub struct Factory(pub fn() -> Meta);

#[derive(Clone, Copy, Debug)]
pub struct Meta {
    pub id: TypeId,

    pub name: &'static str,
}

impl Meta {
    pub fn collect() -> &'static Vec<Self> {
        static RESULT: Lazy<Vec<Meta>> =
            Lazy::new(|| {
                inventory::iter::<Factory>
                    .into_iter()
                    .inspect(|_| { dbg!("aaa"); })
                    .map(|factory| (factory.0)())
                    .collect()
            });

        &RESULT
    }
}

pub trait Plugin {
    const NAME: &'static str;
}
