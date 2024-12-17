use darling::FromMeta;
use syn::{Expr, Ident, Item};

use crate::utils::Access;

use super::{Args, Spec};

#[derive(Debug, Clone, Default, FromMeta)]
pub struct FieldArgs {
    #[darling(default)]
    pub auto_increment: bool,
    pub offset: Option<u8>,
    pub width: Option<u8>,
    pub read: Option<Access>,
    pub write: Option<Access>,
    pub reset: Option<Expr>,
    pub schema: Option<Ident>,
}

impl Args for FieldArgs {
    const NAME: &str = "field";
}

#[derive(Debug)]
pub struct FieldSpec {
    pub ident: Ident,
}

impl Spec for FieldSpec {
    type Inherited = Ident;
    type Args = FieldArgs;

    fn parse<'a>(
        inherited: Self::Inherited,
        args: Self::Args,
        items: impl Iterator<Item = &'a Item>,
    ) -> syn::Result<Self> {
        todo!()
    }
}
