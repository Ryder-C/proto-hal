use darling::FromMeta;
use syn::{ExprRange, Ident, Item};

use super::{field::FieldArgs, Args, Spec};

#[derive(Debug, Clone, FromMeta)]
pub struct FieldArrayArgs {
    pub range: ExprRange,
    pub field: FieldArgs,
}

impl Args for FieldArrayArgs {
    const NAME: &str = "field_array";
}

#[derive(Debug)]
pub struct FieldArraySpec {
    pub ident: Ident,
}

impl Spec for FieldArraySpec {
    type Inherited = Ident;
    type Args = FieldArrayArgs;

    fn parse<'a>(
        inherited: Self::Inherited,
        args: Self::Args,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        todo!()
    }
}
