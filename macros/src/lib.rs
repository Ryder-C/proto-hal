use std::collections::HashMap;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    meta::ParseNestedMeta,
    parse::Parse,
    parse2, parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Brace, Colon, Const, Eq, For, Gt, Impl, Lt, Paren, PathSep, Semi, Struct, Unsafe},
    Attribute, Block, Expr, ExprArray, ExprLit, Field, FieldMutability, Fields, FieldsNamed,
    GenericParam, Generics, Ident, ImplItem, ImplItemConst, Item, ItemConst, ItemEnum, ItemImpl,
    ItemMod, ItemStruct, Lit, LitInt, Meta, MetaList, Path, PathArguments, PathSegment, Token,
    Type, TypeParam, TypeTuple, Visibility,
};

#[derive(Debug, FromMeta)]
struct BlockArgs {
    base_addr: LitInt,
    infer_offsets: bool,
}

#[derive(Debug, FromMeta)]
struct RegisterArgs {
    #[darling(default)]
    infer_offsets: bool,
}

#[derive(Debug, Default)]
struct PathArray {
    elems: Vec<Path>,
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

impl PathArray {
    const fn new() -> Self {
        Self { elems: Vec::new() }
    }
}

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct Access {
    entitlements: PathArray,
    effect: Option<Meta>,
}

#[derive(Debug, FromMeta)]
struct StatesArgs {
    width: u8,
}

#[derive(Debug, Default, FromMeta)]
struct FieldArgs {
    offset: Option<u8>,
    read: Option<Access>,
    write: Option<Access>,
    reset: Option<Ident>,
}

#[derive(Debug)]
struct RegFieldInfo {
    args: FieldArgs,
    ident: Ident,
    ty: Ident,
    gen_ty: Ident,
}

#[derive(Debug, FromMeta)]
struct BlockRegArgs {
    offset: u8,
}

#[derive(Debug)]
struct BlockRegInfo {
    args: BlockRegArgs,
    ident: Ident,
    gen_ty: Ident,
}

#[derive(Debug, FromMeta)]
struct Reset;

#[derive(Debug)]
struct StateInfo {
    args: StateArgs,
    ident: Ident,
}

#[derive(Debug, Default, FromMeta)]
struct StateArgs {
    entitlements: PathArray,
}

#[derive(Debug)]
struct FieldInfo {
    args: StatesArgs,
    ident: Ident,
    states: Vec<StateInfo>,
    reset: Ident,
}

#[derive(Debug)]
struct RegisterInfo {
    args: RegisterArgs,
    fields: Vec<RegFieldInfo>,
}

struct BlockInfo {
    args: BlockArgs,
    ident: Ident,
    generics: Generics,
    registers: Vec<BlockRegInfo>,
}

// #[proc_macro_attribute]
// pub fn peripheral(_args: TokenStream, item: TokenStream) -> TokenStream {
//     let mut result = quote! {};

//     let mut module = parse_macro_input!(item as ItemMod);

//     let items = &mut module
//         .content
//         .as_mut()
//         .expect("module must contain items")
//         .1;

//     let mut fields_map = HashMap::new();

//     let mut new_items = Vec::new();

//     // parse and transform fields
//     for item in items.iter_mut() {
//         // enums can be fields
//         if let Item::Enum(e) = item {
//             let mut field_args = None;

//             // parse attrs
//             e.attrs = e
//                 .attrs
//                 .iter()
//                 .cloned()
//                 .filter(|attr| {
//                     if attr.meta.path().is_ident("field") {
//                         match FieldArgs::from_meta(&attr.meta) {
//                             Ok(args) => field_args = Some(args),
//                             Err(e) => {
//                                 result.extend(e.write_errors());
//                             }
//                         }

//                         false
//                     } else {
//                         true
//                     }
//                 })
//                 .collect();

//             // parse variants
//             let mut reset = None;
//             let mut states = Vec::new();

//             e.variants.iter_mut().for_each(|variant| {
//                 let mut state_args = StateArgs::default();

//                 variant.attrs = variant
//                     .attrs
//                     .iter()
//                     .cloned()
//                     .filter(|attr| {
//                         if attr.meta.path().is_ident("reset") {
//                             match Reset::from_meta(&attr.meta) {
//                                 Ok(_) => {
//                                     if reset.is_none() {
//                                         reset = Some(variant.ident.clone());
//                                     } else {
//                                         result.extend(
//                                             syn::Error::new(
//                                                 variant.span(),
//                                                 "multiple reset state annotaions",
//                                             )
//                                             .to_compile_error(),
//                                         );
//                                     }
//                                 }
//                                 Err(e) => result.extend(e.write_errors()),
//                             }

//                             false
//                         } else if attr.meta.path().is_ident("state") {
//                             match StateArgs::from_meta(&attr.meta) {
//                                 Ok(args) => state_args = args,
//                                 Err(e) => {
//                                     result.extend(e.write_errors());
//                                 }
//                             }

//                             false
//                         } else {
//                             true
//                         }
//                     })
//                     .collect();

//                 states.push(StateInfo {
//                     args: state_args,
//                     ident: variant.ident.clone(),
//                 });
//             });

//             // generate type-states
//             for variant in e.variants.iter() {
//                 new_items.push(Item::Struct(ItemStruct {
//                     attrs: Vec::new(),
//                     vis: module.vis.clone(),
//                     struct_token: Struct(Span::mixed_site()),
//                     ident: variant.ident.clone(),
//                     generics: Generics::default(),
//                     fields: Fields::Named(FieldsNamed {
//                         brace_token: Brace(Span::mixed_site()),
//                         named: {
//                             Punctuated::from_iter(vec![Field {
//                                 attrs: Vec::new(),
//                                 vis: Visibility::Inherited,
//                                 mutability: FieldMutability::None,
//                                 ident: Some(Ident::new("sealed", Span::mixed_site())),
//                                 colon_token: Some(Colon(Span::mixed_site())),
//                                 ty: Type::Tuple(TypeTuple {
//                                     paren_token: Paren(Span::mixed_site()),
//                                     elems: Punctuated::new(),
//                                 }),
//                             }])
//                         },
//                     }),
//                     semi_token: None,
//                 }));
//             }

//             match (reset, field_args) {
//                 (Some(reset), Some(args)) => {
//                     fields_map.insert(
//                         e.ident.clone(),
//                         FieldInfo {
//                             args,
//                             ident: e.ident.clone(),
//                             states,
//                             reset,
//                         },
//                     );
//                 }
//                 (None, _) => {
//                     result.extend(
//                         syn::Error::new(
//                             e.ident.span(),
//                             "exactly one state must be marked as reset",
//                         )
//                         .to_compile_error(),
//                     );
//                 }
//                 (_, _) => {}
//             };
//         }
//     }

//     items.extend(new_items);

//     let mut registers = Vec::new();

//     // parse and transform registers
//     for item in items.iter_mut() {
//         if let Item::Struct(s) = item {
//             let mut register_args = None;

//             s.attrs = s
//                 .attrs
//                 .iter()
//                 .cloned()
//                 .filter(|attr| {
//                     if attr.meta.path().is_ident("register") {
//                         match RegisterArgs::from_meta(&attr.meta) {
//                             Ok(args) => {
//                                 register_args = Some(args);
//                             }
//                             Err(e) => {
//                                 result.extend(e.write_errors());
//                             }
//                         }

//                         false
//                     } else {
//                         true
//                     }
//                 })
//                 .collect();

//             let Some(register_args) = register_args else {
//                 continue;
//             };

//             let Fields::Named(fields) = &mut s.fields else {
//                 result.extend(
//                     syn::Error::new(s.fields.span(), "register struct must contain named fields")
//                         .to_compile_error(),
//                 );
//                 break;
//             };

//             let mut field_infos = Vec::<RegFieldInfo>::new();

//             fields.named.iter_mut().for_each(|field| {
//                 let mut field_args = None;

//                 field.attrs = field
//                     .attrs
//                     .iter()
//                     .cloned()
//                     .filter(|attr| {
//                         if attr.path().is_ident("field") {
//                             match RegFieldArgs::from_meta(&attr.meta) {
//                                 Ok(args) => {
//                                     field_args = Some(args);
//                                 }
//                                 Err(e) => {
//                                     result.extend(e.write_errors());
//                                 }
//                             }

//                             false
//                         } else {
//                             true
//                         }
//                     })
//                     .collect();

//                 let field_args = if let Some(field_args) = field_args {
//                     field_args
//                 } else {
//                     if let Some(last) = field_infos.last() {
//                         if let Some(info) = fields_map.get(&last.ty) {
//                             RegFieldArgs {
//                                 offset: last.args.offset + info.args.width,
//                             }
//                         } else {
//                             result.extend(
//                                 syn::Error::new(field.ty.span(), "field is undefined")
//                                     .to_compile_error(),
//                             );

//                             RegFieldArgs { offset: 0 }
//                         }
//                     } else {
//                         RegFieldArgs { offset: 0 }
//                     }
//                 };

//                 field_infos.push(RegFieldInfo {
//                     args: field_args,
//                     ident: field.ident.clone().unwrap(),
//                     ty: parse2(field.ty.to_token_stream()).unwrap(),
//                     gen_ty: Ident::new(
//                         inflector::cases::pascalcase::to_pascal_case(
//                             field.ident.as_ref().unwrap().to_string().as_str(),
//                         )
//                         .as_str(),
//                         Span::mixed_site(),
//                     ),
//                 });
//             });

//             // transform register struct to hold generics

//             // add generics
//             s.generics = Generics {
//                 lt_token: Some(Lt(Span::mixed_site())),
//                 params: Punctuated::from_iter(field_infos.iter().map(|info| {
//                     GenericParam::Type(TypeParam {
//                         attrs: Vec::new(),
//                         ident: info.gen_ty.clone(),
//                         colon_token: None,
//                         bounds: Punctuated::new(),
//                         eq_token: None,
//                         default: None,
//                     })
//                 })),
//                 gt_token: Some(Gt(Span::mixed_site())),
//                 where_clause: None,
//             };

//             // change field types to generics
//             fields
//                 .named
//                 .iter_mut()
//                 .zip(field_infos.iter())
//                 .for_each(|(field, info)| {
//                     field.ty = Type::Verbatim(info.gen_ty.to_token_stream());
//                 });

//             registers.push(RegisterInfo {
//                 args: register_args,
//                 fields: field_infos,
//             });
//         }
//     }

//     let mut blocks = Vec::new();

//     // parse and transform blocks
//     for item in items.iter_mut() {
//         if let Item::Struct(s) = item {
//             let mut block_args = None;

//             s.attrs = s
//                 .attrs
//                 .iter()
//                 .cloned()
//                 .filter(|attr| {
//                     if attr.meta.path().is_ident("block") {
//                         match BlockArgs::from_meta(&attr.meta) {
//                             Ok(args) => {
//                                 block_args = Some(args);
//                             }
//                             Err(e) => {
//                                 result.extend(e.write_errors());
//                             }
//                         }

//                         false
//                     } else {
//                         true
//                     }
//                 })
//                 .collect();

//             let Some(block_args) = block_args else {
//                 continue;
//             };

//             let Fields::Named(fields) = &mut s.fields else {
//                 result.extend(
//                     syn::Error::new(s.fields.span(), "block struct must contain named fields")
//                         .to_compile_error(),
//                 );
//                 break;
//             };

//             let mut register_infos = Vec::<BlockRegInfo>::new();

//             fields.named.iter_mut().for_each(|field| {
//                 let mut register_args = None;

//                 field.attrs = field
//                     .attrs
//                     .iter()
//                     .cloned()
//                     .filter(|attr| {
//                         if attr.path().is_ident("register") {
//                             match BlockRegArgs::from_meta(&attr.meta) {
//                                 Ok(args) => {
//                                     register_args = Some(args);
//                                 }
//                                 Err(e) => {
//                                     result.extend(e.write_errors());
//                                 }
//                             }

//                             false
//                         } else {
//                             true
//                         }
//                     })
//                     .collect();

//                 let register_args = if let Some(register_args) = register_args {
//                     register_args
//                 } else {
//                     if let Some(last) = register_infos.last() {
//                         BlockRegArgs {
//                             offset: last.args.offset + 4, /* are all regs 4 bytes wide? */
//                         }
//                     } else {
//                         BlockRegArgs { offset: 0 }
//                     }
//                 };

//                 register_infos.push(BlockRegInfo {
//                     args: register_args,
//                     ident: field.ident.clone().unwrap(),
//                     gen_ty: Ident::new(
//                         inflector::cases::pascalcase::to_pascal_case(
//                             field.ident.as_ref().unwrap().to_string().as_str(),
//                         )
//                         .as_str(),
//                         Span::mixed_site(),
//                     ),
//                 });
//             });

//             // transform register struct to hold generics

//             // add generics
//             s.generics = Generics {
//                 lt_token: Some(Lt(Span::mixed_site())),
//                 params: Punctuated::from_iter(register_infos.iter().map(|info| {
//                     GenericParam::Type(TypeParam {
//                         attrs: Vec::new(),
//                         ident: info.gen_ty.clone(),
//                         colon_token: None,
//                         bounds: Punctuated::new(),
//                         eq_token: None,
//                         default: None,
//                     })
//                 })),
//                 gt_token: Some(Gt(Span::mixed_site())),
//                 where_clause: None,
//             };

//             // change register types to generics
//             fields
//                 .named
//                 .iter_mut()
//                 .zip(register_infos.iter())
//                 .for_each(|(field, info)| {
//                     field.ty = Type::Verbatim(info.gen_ty.to_token_stream());
//                 });

//             blocks.push(BlockInfo {
//                 args: block_args,
//                 ident: s.ident.clone(),
//                 generics: s.generics.clone(),
//                 registers: register_infos,
//             });
//         }
//     }

//     for block in blocks {
//         // generate `Block` impl
//         items.push(Item::Impl(ItemImpl {
//             attrs: Vec::new(),
//             defaultness: None,
//             unsafety: Some(Unsafe(Span::mixed_site())),
//             impl_token: Impl(Span::mixed_site()),
//             generics: block.generics.clone(),
//             trait_: Some((
//                 None,
//                 Path {
//                     leading_colon: Some(PathSep(Span::mixed_site())),
//                     segments: Punctuated::from_iter(vec![
//                         PathSegment {
//                             ident: Ident::new("proto_hal", Span::mixed_site()),
//                             arguments: PathArguments::None,
//                         },
//                         PathSegment {
//                             ident: Ident::new("regs", Span::mixed_site()),
//                             arguments: PathArguments::None,
//                         },
//                         PathSegment {
//                             ident: Ident::new("block", Span::mixed_site()),
//                             arguments: PathArguments::None,
//                         },
//                         PathSegment {
//                             ident: Ident::new("Block", Span::mixed_site()),
//                             arguments: PathArguments::None,
//                         },
//                     ]),
//                 },
//                 For(Span::mixed_site()),
//             )),
//             self_ty: Box::new(Type::Verbatim({
//                 let ident = &block.ident;
//                 let generics = &block.generics;
//                 quote! {
//                     #ident #generics
//                 }
//             })),
//             brace_token: Brace(Span::mixed_site()),
//             items: vec![ImplItem::Const(ImplItemConst {
//                 attrs: Vec::new(),
//                 vis: Visibility::Inherited,
//                 defaultness: None,
//                 const_token: Const(Span::mixed_site()),
//                 ident: Ident::new("BASE", Span::mixed_site()),
//                 generics: Generics {
//                     lt_token: None,
//                     params: Punctuated::new(),
//                     gt_token: None,
//                     where_clause: None,
//                 },
//                 colon_token: Colon(Span::mixed_site()),
//                 ty: Type::Verbatim(quote! { usize }),
//                 eq_token: Eq(Span::mixed_site()),
//                 expr: Expr::Lit(ExprLit {
//                     attrs: Vec::new(),
//                     lit: Lit::Int(block.args.base_addr.clone()),
//                 }),
//                 semi_token: Semi(Span::mixed_site()),
//             })],
//         }));
//     }

//     result.extend(quote! {
//         #module
//     });

//     result.into()
// }

fn states_inner(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream2> {
    let states_args = StatesArgs::from_list(&NestedMeta::parse_meta_list(args.into())?)?;

    let mut e = parse2::<ItemEnum>(item.into())?;
    let vis = e.vis.clone();

    let mut errors = Vec::<syn::Error>::new();

    let mut states = Vec::new();

    e.variants.iter_mut().for_each(|variant| {
        let mut state_args = StateArgs::default();

        variant.attrs = variant
            .attrs
            .iter()
            .cloned()
            .filter(|attr| {
                if attr.meta.path().is_ident("state") {
                    match StateArgs::from_meta(&attr.meta) {
                        Ok(args) => {
                            state_args = args;
                        }
                        Err(e) => {
                            errors.push(e.into());
                        }
                    }

                    false
                } else {
                    true
                }
            })
            .collect();

        states.push(StateInfo {
            args: state_args,
            ident: variant.ident.clone(),
        });
    });

    if let Some(error) = errors.iter().cloned().reduce(|mut acc, e| {
        acc.combine(e);
        acc
    }) {
        Err(error)?
    }

    let mod_ident = Ident::new(
        inflector::cases::snakecase::to_snake_case(e.ident.to_string().as_str()).as_str(),
        Span::call_site(),
    );
    let width = states_args.width;
    let variants = e.variants;

    let mut result = quote! {
        const WIDTH: u8 = #width;

        pub enum States {
            #variants
        }

        pub trait State {}
    };

    for state in states {
        let ident = state.ident;

        result.extend(quote! {
            pub struct #ident {
                sealed: (),
            }

            impl State for #ident {}
        });
    }

    Ok(quote! {
        #vis mod #mod_ident {
            #result
        }
    })
}

fn register_inner(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream2> {
    let register_args = RegisterArgs::from_list(&NestedMeta::parse_meta_list(args.into())?)?;

    let mut s = parse2::<ItemStruct>(item.into())?;

    let mut errors = Vec::<syn::Error>::new();

    let Fields::Named(mut fields) = s.fields else {
        Err(syn::Error::new(
            s.ident.span(),
            "register structs must have named fields",
        ))?
    };

    fields.named.iter_mut().for_each(|field| {
        let mut field_args = FieldArgs::default();

        field.attrs = field
            .attrs
            .iter()
            .cloned()
            .filter(|attr| {
                if attr.path().is_ident("field") {
                    match FieldArgs::from_meta(&attr.meta) {
                        Ok(args) => {
                            field_args = args;
                        }
                        Err(e) => {
                            errors.push(e.into());
                        }
                    }

                    false
                } else {
                    true
                }
            })
            .collect();

        if field_args.offset.is_none() && !register_args.infer_offsets {
            errors.push(syn::Error::new(
                field.span(),
                "field offset must be specified. to infer offsets add the `infer_offsets` argument to the `register` attribute macro.",
            ));
        }
    });

    s.fields = Fields::Named(fields);

    if let Some(error) = errors.iter().cloned().reduce(|mut acc, e| {
        acc.combine(e);
        acc
    }) {
        Err(error)?
    }

    Ok(quote! {
        #s
    })
}

#[proc_macro_attribute]
pub fn states(args: TokenStream, item: TokenStream) -> TokenStream {
    match states_inner(args, item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

#[proc_macro_attribute]
pub fn register(args: TokenStream, item: TokenStream) -> TokenStream {
    match register_inner(args, item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
