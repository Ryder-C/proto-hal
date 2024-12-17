use darling::FromMeta;
use syn::Item;

use crate::utils::PathArray;

use super::{Args, Spec};

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct StateArgs {
    #[darling(default)]
    pub bits: Option<u32>,
    pub reset: bool,
    pub entitlements: PathArray,
}

impl Args for StateArgs {
    const NAME: &str = "state";
}

#[derive(Debug)]
pub struct StateSpec {}

impl Spec for StateSpec {
    type Inherited = ();
    type Args = StateArgs;

    fn parse<'a>(
        inherited: Self::Inherited,
        args: Self::Args,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        todo!()
    }
}
