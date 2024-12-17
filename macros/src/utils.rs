use darling::FromMeta;
use syn::{Expr, ExprArray, Item, ItemMod, ItemStruct, Meta, Path};

pub fn require_module(item: &Item) -> syn::Result<&ItemMod> {
    if let Item::Mod(module) = item {
        Ok(module)
    } else {
        Err(syn::Error::new_spanned(item, "item must be a module"))
    }
}

pub fn require_struct(item: &Item) -> syn::Result<&ItemStruct> {
    if let Item::Struct(s) = item {
        Ok(s)
    } else {
        Err(syn::Error::new_spanned(item, "item must be a struct"))
    }
}

pub fn extract_items_from(module: &ItemMod) -> syn::Result<&Vec<Item>> {
    Ok(&module
        .content
        .as_ref()
        .ok_or(syn::Error::new_spanned(
            module,
            "module must be a definition, not an import",
        ))?
        .1)
}

#[derive(Debug, Clone, Default)]
pub struct PathArray {
    pub elems: Vec<Path>,
}

impl FromMeta for PathArray {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        let arr = ExprArray::from_meta(item)?;

        Ok(Self {
            elems: arr
                .elems
                .iter()
                .cloned()
                .map(|expr| {
                    if let Expr::Path(path) = expr {
                        Ok(path.path)
                    } else {
                        Err(darling::Error::custom("expected path").with_span(&expr))
                    }
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct Access {
    pub entitlements: PathArray,
    pub effect: Option<Meta>,
}

pub type Offset = u8;
pub type Width = u8;

#[derive(Debug)]
pub struct SynErrorCombinator {
    errors: Vec<syn::Error>,
}

impl SynErrorCombinator {
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    // pub fn push(&mut self, error: syn::Error) {
    //     self.errors.push(error);
    // }

    // TODO: better name
    pub fn try_maybe_then<F, T, E>(&mut self, result: Result<T, E>, mut f: F)
    where
        E: Into<syn::Error>,
        F: FnMut(T) -> Result<(), E>,
    {
        match result {
            Ok(t) => {
                if let Err(e) = f(t) {
                    self.errors.push(e.into());
                }
            }
            Err(e) => {
                self.errors.push(e.into());
            }
        }
    }

    // TODO: better name
    pub fn maybe_then<F, T, E>(&mut self, result: Result<T, E>, mut f: F)
    where
        E: Into<syn::Error>,
        F: FnMut(T),
    {
        self.try_maybe_then(result, |t| {
            f(t);

            Ok(())
        });
    }

    // TODO: better name
    // pub fn maybe<T, E>(&mut self, result: Result<T, E>)
    // where
    //     E: Into<syn::Error>,
    // {
    //     self.maybe_then(result, |_| {});
    // }

    pub fn coalesce(self) -> Result<(), syn::Error> {
        if let Some(error) = self.errors.iter().cloned().reduce(|mut acc, e| {
            acc.combine(e);
            acc
        }) {
            Err(error)?
        } else {
            Ok(())
        }
    }
}
