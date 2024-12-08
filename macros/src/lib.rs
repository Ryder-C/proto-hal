use std::collections::HashSet;

use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse::Parse, parse2, parse_macro_input, spanned::Spanned, Expr, ExprArray, Fields, Ident,
    Index, Item, ItemMod, ItemStruct, LitInt, Meta, Path, Token, Type, Visibility,
};

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
struct BlockArgs {
    base_addr: u32,
    auto_increment: bool,
    entitlements: PathArray,
    erase_mod: bool,
}

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
struct RegisterArgs {
    offset: Option<u8>,
    #[darling(default)]
    auto_increment: bool,
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
struct Access {
    entitlements: PathArray,
    effect: Option<Meta>,
}

#[derive(Debug, Clone, Default, FromMeta)]
struct FieldArgs {
    #[darling(default)]
    auto_increment: bool,
    offset: Option<u8>,
    width: u8,
    read: Option<Access>,
    write: Option<Access>,
    reset: Option<u32>,
}

// TODO: should probably encode statefulness
// and reset requirement explicitly somehow
#[derive(Debug, Clone)]
struct FieldInfo {
    args: FieldArgs,
    ident: Ident,
    reset_state: Option<Ident>,
    entitlement_fields: HashSet<Ident>,
    stateful: bool,
}

#[derive(Debug, Clone)]
struct StateInfo {
    args: StateArgs,
    ident: Ident,
    entitlement_fields: HashSet<Ident>,
}

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
struct StateArgs {
    #[darling(default)]
    bits: Option<u32>,
    reset: bool,
    entitlements: PathArray,
}

#[derive(Debug, Default, FromMeta)]
struct ValueArgs;

#[derive(Debug, Clone)]
struct RegisterInfo {
    args: RegisterArgs,
    ident: Ident,
    stateful: bool,
}

struct BlockInfo {
    args: BlockArgs,
    ident: Ident,
}

struct GenPrimitiveModsArgs {
    ty: Type,
    comma: Token![,],
    width: LitInt,
}

impl Parse for GenPrimitiveModsArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty: input.parse()?,
            comma: input.parse()?,
            width: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn gen_primitive_mods(item: TokenStream) -> TokenStream {
    let GenPrimitiveModsArgs {
        ty,
        comma: _comma,
        width,
    } = parse_macro_input!(item as GenPrimitiveModsArgs);

    let mod_ident = Ident::new(
        inflector::cases::snakecase::to_snake_case(ty.to_token_stream().to_string().as_str())
            .as_str(),
        Span::call_site(),
    );

    quote! {
        pub mod #mod_ident {
            pub const WIDTH: u8 = #width;
        }
    }
    .into()
}

#[derive(Debug)]
struct SynErrorCombinator {
    errors: Vec<syn::Error>,
}

impl SynErrorCombinator {
    pub const fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn push(&mut self, error: syn::Error) {
        self.errors.push(error);
    }

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
    pub fn maybe<T, E>(&mut self, result: Result<T, E>)
    where
        E: Into<syn::Error>,
    {
        self.maybe_then(result, |_| {});
    }

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

fn process_state(
    state_args: StateArgs,
    prev_state_info: Option<StateInfo>,
    field_width: u8,
    s: &mut ItemStruct,
) -> Result<(StateInfo, TokenStream2), syn::Error> {
    let Fields::Unit = s.fields else {
        Err(syn::Error::new_spanned(
            s.fields.clone(),
            "state must be a unit struct",
        ))?
    };

    // Q: gross that this is technically fallible
    // but the correct way is too verbose.
    // is there a better strategy?
    s.fields = Fields::Named(
        parse2(quote! {
            {
                sealed: (),
            }
        })
        .unwrap(),
    );
    s.vis = Visibility::Public(Token![pub](s.span()));

    *s = parse2(quote! {
        #[cfg_attr(feature = "defmt", derive(::defmt::Format))]
        #s
    })
    .unwrap();

    let state_impl = {
        let ident = &s.ident;
        quote! {
            impl State for #ident {
                const RAW: States = States::#ident;

                unsafe fn conjure() -> Self {
                    Self {
                        sealed: (),
                    }
                }
            }
        }
    };

    let entitlement_impl = if !state_args.entitlements.elems.is_empty() {
        let ident = &s.ident;
        let entitlement_paths = &state_args.entitlements.elems;

        Some(quote! {
            #(
                unsafe impl ::proto_hal::stasis::Entitled<super::#entitlement_paths> for #ident {}
            )*
        })
    } else {
        None
    };

    let order_assertion = if let Some(prev_state) = prev_state_info {
        let current_ident = &s.ident;
        let prev_ident = prev_state.ident;

        let span = s.ident.span();
        Some(quote_spanned! { span =>
            const _: () = assert!(
                (States::#prev_ident as u32) < (States::#current_ident as u32),
                "state bit values must be unique and in ascending order"
            );
        })
    } else {
        None
    };

    let msg = format!(
        "state bit value is larger than the maximum value supported by a field of width {}",
        field_width
    );

    let bounds_assertion = {
        let span = s.ident.span();
        let ident = &s.ident;
        quote_spanned! { span =>
            const _: () = assert!(
                ((States::#ident as u32) >> #field_width) == 0,
                #msg,
            );
        }
    };

    let mut entitlement_fields = HashSet::new();

    state_args.entitlements.elems.iter().for_each(|path| {
        entitlement_fields.insert(path.segments.first().unwrap().ident.clone());
    });

    Ok((
        StateInfo {
            args: state_args,
            ident: s.ident.clone(),
            entitlement_fields,
        },
        quote! {
            #state_impl
            #entitlement_impl

            #order_assertion
            #bounds_assertion
        },
    ))
}

fn process_field(
    field_args: FieldArgs,
    prev_field_info: Option<FieldInfo>,
    module: &mut ItemMod,
) -> Result<FieldInfo, syn::Error> {
    module.vis = Visibility::Public(Token![pub](module.span()));
    let items = &mut module
        .content
        .as_mut()
        .expect("module must be a definition, not an import")
        .1;

    let mut error_combinator = SynErrorCombinator::new();

    let mut reset_state = None;

    let mut states = Vec::new();
    let mut extras = Vec::new();

    items.iter_mut().for_each(|item| {
        let Item::Struct(s) = item else { return };

        // 1. try to extract state annotation args
        let mut state_args = None;

        s.attrs.retain(|attr| {
                if attr.path().is_ident("state") {
                    error_combinator.try_maybe_then(StateArgs::from_meta(&attr.meta), |args| {
                        // store reset and validate single occurance
                        if args.reset {
                            if reset_state.is_none() {
                                reset_state = Some(s.ident.clone());
                            } else {
                                Err(syn::Error::new_spanned(attr, "reset is already specified"))?
                            }
                        }

                        // validate bits specification
                        if args.bits.is_none() && !field_args.auto_increment {
                            Err(syn::Error::new_spanned(attr.path(), "state bit value `bits` must be specified. to infer the bit value, add the `auto_increment` argument to the field attribute macro"))?
                        }

                        state_args.replace(args);

                        Ok(())
                    });

                    false
                } else {
                    true
                }
            });

        if let Some(args) = state_args {
            // 2. pass the module over to the state parser
            error_combinator.maybe_then(
                process_state(args, states.last().cloned(), field_args.width, s),
                |(state, extra)| {
                    states.push(state);
                    extras.push(extra);
                },
            );
        }
    });

    error_combinator.coalesce()?;

    // offset and width
    {
        let offset_tokens = match (field_args.offset, &prev_field_info) {
            (Some(offset), _) => {
                quote! { #offset }
            }
            (_, Some(prev)) => {
                let prev_ident = &prev.ident;
                quote! { super::#prev_ident::OFFSET + super::#prev_ident::WIDTH }
            }
            (None, None) => {
                quote! { 0 }
            }
        };
        let width = field_args.width;

        items.push(Item::Verbatim(quote! {
            pub const OFFSET: u8 = #offset_tokens;
            pub const WIDTH: u8 = #width;
        }));
    }

    let stateful = !states.is_empty();

    if stateful {
        // field is stateful

        let state_idents = states
            .iter()
            .map(|state| state.ident.clone())
            .collect::<Vec<_>>();
        let state_bits_tokens = states
            .iter()
            .map(|state| state.args.bits.map(|bits| quote! { = #bits }))
            .collect::<Vec<_>>();

        items.push(Item::Verbatim(quote! {
            #[cfg_attr(feature = "defmt", derive(::defmt::Format))]
            pub struct Any {
                state: States,
            }

            pub type Reset = #reset_state;
            pub const RESET: u32 = Reset::RAW as u32;

            #[cfg_attr(feature = "defmt", derive(::defmt::Format))]
            #[repr(u32)]
            pub enum States {
                #(
                    #state_idents #state_bits_tokens,
                )*
            }

            pub trait State {
                const RAW: States;

                unsafe fn conjure() -> Self;
            }
        }));

        let state_accessor_idents = state_idents
            .iter()
            .map(|ident| {
                Ident::new(
                    &inflector::cases::snakecase::to_snake_case(&ident.to_string()),
                    Span::call_site(),
                )
            })
            .collect::<Vec<_>>();
        let field_ty = Ident::new(
            &inflector::cases::pascalcase::to_pascal_case(&module.ident.to_string()),
            Span::call_site(),
        );
        let field_ident = &module.ident;

        if field_args.write.is_some() {
            // state builder
            items.push(Item::Verbatim(quote! {
                pub struct StateBuilder<RegisterStateBuilder>(pub(crate) RegisterStateBuilder)
                where
                    RegisterStateBuilder: super::Refine;

                impl<RegisterStateBuilder> StateBuilder<RegisterStateBuilder>
                where
                    RegisterStateBuilder: super::Refine,
                {
                    #(
                        pub fn #state_accessor_idents(self) -> RegisterStateBuilder::#field_ty<#state_idents> {
                            self.0.#field_ident()
                        }
                    )*

                    pub fn generic<S>(self) -> RegisterStateBuilder::#field_ty<S>
                    where
                        S: State,
                    {
                        self.0.#field_ident()
                    }
                }
            }));
        }
    } else {
        // field is stateless

        if let Some(reset) = field_args.reset {
            items.push(Item::Verbatim(quote! {
                pub const RESET: u32 = #reset;
            }));
        }
    }

    // render extras to module
    for extra in extras {
        items.push(Item::Verbatim(extra));
    }

    // domain validation
    {
        let span = module.ident.span();
        if let Some(prev_field) = prev_field_info {
            let prev_ident = &prev_field.ident;

            let overlap_msg = format!(
                "field domains must be in order and non-overlapping. overlaps with {}",
                prev_ident,
            );

            // TODO: would be better if the span was
            // that of the field annotation, or better
            // yet the offset argument
            items.push(Item::Verbatim(quote_spanned! { span =>
                const _: () = assert!(
                    super::#prev_ident::OFFSET + super::#prev_ident::WIDTH <= OFFSET,
                    #overlap_msg
                );
            }));
        }

        items.push(Item::Verbatim(quote_spanned! { span =>
            const _: () = assert!(
                OFFSET + WIDTH <= 32,
                "field domain goes out of bounds of register domain"
            );
        }));
    }

    // Q: is this the best way to do this?
    if stateful && reset_state.is_none()
    // || (!stateful && field_args.read.is_some() && field_args.reset.is_none()) // why was this here?
    {
        return Err(syn::Error::new_spanned(module, "reset must be specified"));
    }

    // field docs
    // {
    //     let mut msg = format!("# Spec\n- width: {}\n# States\n", field_args.width);

    //     for state in states.iter() {
    //         msg.push_str(&format!("\t- {}::{}\n", module.ident.clone(), state.ident));
    //     }

    //     // TODO: i dislike the misleading
    //     // fallibility of this
    //     *module = parse2(quote! {
    //         #[doc = #msg]
    //         #module
    //     })
    //     .unwrap();
    // }

    Ok(FieldInfo {
        args: field_args,
        ident: module.ident.clone(),
        reset_state,
        entitlement_fields: {
            states
                .iter()
                .map(|state| state.entitlement_fields.clone())
                .fold(HashSet::new(), |acc, next_set| {
                    acc.union(&next_set).cloned().collect()
                })
        },
        stateful,
    })
}

fn process_register(
    register_args: RegisterArgs,
    prev_register_info: Option<RegisterInfo>,
    module: &mut ItemMod,
) -> Result<RegisterInfo, syn::Error> {
    module.vis = Visibility::Public(Token![pub](module.span()));
    let items = &mut module
        .content
        .as_mut()
        .expect("module must be a definition, not an import")
        .1;

    let mut error_combinator = SynErrorCombinator::new();

    let mut fields = Vec::new();

    items.iter_mut().for_each(|item| {
        let Item::Mod(inner_mod) = item else { return };

        // 1. try to extract field annotation args
        let mut field_args = None;

        inner_mod.attrs.retain(|attr| {
            if attr.path().is_ident("field") {
                error_combinator.try_maybe_then(FieldArgs::from_meta(&attr.meta), |args| {
                    // validate offset specification
                    if args.offset.is_none() && !register_args.auto_increment {
                        Err(syn::Error::new_spanned(attr.path(), "field offset must be specified. to infer offsets, add the `auto_increment` argument to the register attribute macro"))?
                    }

                    if args.read.is_none() && args.write.is_none() {
                        Err(syn::Error::new_spanned(
                            attr.path(),
                            "fields must be readable or writable.",
                        ))?
                    }

                    field_args.replace(args);

                    Ok(())
                });

                false
            } else {
                true
            }
        });

        let Some(field_args) = field_args else {
            return;
        };

        // 2. pass the module over to the field parser
        error_combinator.maybe_then(process_field(field_args, fields.last().cloned(), inner_mod), |field| {
            fields.push(field)
        });
    });

    error_combinator.coalesce()?;

    let stateful = fields.iter().any(|field| field.stateful);

    {
        let offset = if let Some(offset) = register_args.offset {
            quote! { #offset }
        } else if let Some(prev) = prev_register_info {
            let prev_ident = prev.ident;

            quote! { super::#prev_ident::OFFSET + 4 }
        } else {
            quote! { 0 }
        };

        let (stateful_fields, stateless_fields) =
            fields.iter().partition::<Vec<_>, _>(|field| field.stateful);

        let stateful_field_idents = stateful_fields
            .iter()
            .map(|field| field.ident.clone())
            .collect::<Vec<_>>();

        let stateless_field_idents = stateless_fields
            .iter()
            .map(|field| field.ident.clone())
            .collect::<Vec<_>>();

        let stateful_field_tys = stateful_field_idents
            .iter()
            .map(|ident| {
                Ident::new(
                    &inflector::cases::pascalcase::to_pascal_case(&ident.to_string()),
                    Span::call_site(),
                )
            })
            .collect::<Vec<_>>();

        let new_stateful_field_tys = stateful_field_tys
            .iter()
            .map(|ident| format_ident!("New{}", ident))
            .collect::<Vec<_>>();

        items.push(Item::Verbatim(quote! {
            pub const OFFSET: u32 = #offset as u32;

            #[cfg_attr(feature = "defmt", derive(::defmt::Format))]
            pub struct Register<#(#stateful_field_tys,)*> {
                #(
                    pub #stateful_field_idents: #stateful_field_tys,
                )*

                #(
                    #stateless_field_idents: (),
                )*
            }
        }));

        if stateful {
            let writable_stateful_fields = stateful_fields
                .iter()
                .filter(|field| field.args.write.is_some())
                .collect::<Vec<_>>();

            let writable_stateful_field_idents = writable_stateful_fields
                .iter()
                .map(|field| field.ident.clone())
                .collect::<Vec<_>>();

            let writable_stateful_field_tys = writable_stateful_field_idents
                .iter()
                .map(|ident| {
                    Ident::new(
                        &inflector::cases::pascalcase::to_pascal_case(&ident.to_string()),
                        Span::call_site(),
                    )
                })
                .collect::<Vec<_>>();

            items.push(Item::Verbatim(quote! {
                pub type Reset = Register<
                    #(
                        #stateful_field_idents::Reset,
                    )*
                >;

                pub struct StateBuilder<#(#stateful_field_tys,)*> {
                    #(
                        pub(crate) #stateful_field_idents: core::marker::PhantomData<#stateful_field_tys>,
                    )*
                }

                pub trait Refine {
                    #(
                        type #writable_stateful_field_tys<S>;
                    )*

                    #(
                        fn #writable_stateful_field_idents<S>(self) -> Self::#writable_stateful_field_tys<S>
                        where
                            S: #writable_stateful_field_idents::State;
                    )*
                }

                impl<#(#stateful_field_tys,)*> StateBuilder<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    pub(crate) unsafe fn conjure() -> Self {
                        Self {
                            #(
                                #stateful_field_idents: core::marker::PhantomData,
                            )*
                        }
                    }

                    pub fn finish(self) -> Register<#(#stateful_field_tys,)*> {
                        let reg_value = #(
                            ((#writable_stateful_field_tys::RAW as u32) << #writable_stateful_field_idents::OFFSET)
                        )|*;

                        // SAFETY: assumes the proc macro implementation is sound
                        // and that the peripheral description is accurate
                        unsafe {
                            core::ptr::write_volatile((super::BASE_ADDR + OFFSET) as *mut u32, reg_value);
                        }

                        // SAFETY: `self` is destroyed
                        Register {
                            #(
                                #stateful_field_idents: unsafe { #stateful_field_tys::conjure() },
                            )*
                        }
                    }
                }

                impl<#(#stateful_field_tys,)*> Register<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    pub fn transition<#(#new_stateful_field_tys,)*>(self) -> Register<#(#new_stateful_field_tys,)*>
                    where
                        #(
                            #new_stateful_field_tys: #stateful_field_idents::State,
                        )*
                    {
                        // SAFETY: `self` is destroyed
                        unsafe { StateBuilder::conjure() }.finish()
                    }

                    pub fn build_state(self) -> StateBuilder<#(#stateful_field_tys,)*> {
                        // SAFETY: `self` is destroyed
                        unsafe { StateBuilder::conjure() }
                    }
                }
            }));

            let mut refine_impl = quote! {};

            stateful_fields
                .iter()
                .enumerate()
                .filter(|(_, field)| field.args.write.is_some())
                .for_each(|(i, field)| {
                    let ident = &field.ident;
                    let ty = Ident::new(
                        &inflector::cases::pascalcase::to_pascal_case(&ident.to_string()),
                        Span::call_site(),
                    );

                    let prev_field_tys = stateful_field_tys.get(..i).unwrap();
                    let next_field_tys = stateful_field_tys.get(i + 1..).unwrap();

                    let entitlement_bounds = if !field.entitlement_fields.is_empty() {
                        let entitled_field_tys = field
                            .entitlement_fields
                            .iter()
                            .map(|ident| {
                                Ident::new(
                                    &inflector::cases::pascalcase::to_pascal_case(
                                        &ident.to_string(),
                                    ),
                                    Span::call_site(),
                                )
                            })
                            .collect::<Vec<_>>();

                        Some(quote! {
                            where
                                #ty: #(::proto_hal::stasis::Entitled<#entitled_field_tys>)+*,
                        })
                    } else {
                        None
                    };

                    items.push(Item::Verbatim(quote! {
                        impl<#(#stateful_field_tys,)*> StateBuilder<#(#stateful_field_tys,)*>
                        where
                            #(
                                #stateful_field_tys: #stateful_field_idents::State,
                            )*
                        {
                            pub fn #ident(self) -> #ident::StateBuilder<Self>
                            #entitlement_bounds
                            {
                                #ident::StateBuilder(self)
                            }
                        }
                    }));

                    refine_impl.extend(quote! {
                        type #ty<S> = StateBuilder<#(#prev_field_tys,)* S, #(#next_field_tys,)*>;

                        fn #ident<S>(self) -> Self::#ty<S>
                        where
                            S: #ident::State,
                        {
                            // SAFETY: `self` is destroyed
                            unsafe { StateBuilder::conjure() }
                        }
                    });
                });

            items.push(Item::Verbatim(quote! {
                impl<#(#stateful_field_tys,)*> Refine for StateBuilder<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    #refine_impl
                }
            }));
        }

        // reader
        {
            let readable_stateless_fields = stateless_fields
                .iter()
                .filter(|field| field.args.read.is_some())
                .collect::<Vec<_>>();

            let readable_stateless_field_idents = readable_stateless_fields
                .iter()
                .map(|field| field.ident.clone())
                .collect::<Vec<_>>();

            let value_tys = readable_stateless_fields
                .iter()
                .map(|field| {
                    let ident = format_ident!(
                        "u{}",
                        Index {
                            index: field.args.width as _,
                            span: Span::call_site(),
                        }
                    );

                    match field.args.width {
                        1 => quote! { bool },
                        8 | 16 | 32 => {
                            quote! { #ident }
                        }
                        _ => {
                            quote! { ::proto_hal::macro_utils::arbitrary_int::#ident }
                        }
                    }
                })
                .collect::<Vec<_>>();

            if !readable_stateless_fields.is_empty() {
                items.push(Item::Verbatim(quote! {
                    pub struct Reader {
                        value: ::proto_hal::macro_utils::RegisterValue,
                    }

                    impl Reader {
                        const fn new(value: u32) -> Self {
                            Self {
                                value: ::proto_hal::macro_utils::RegisterValue::new(value),
                            }
                        }

                        #(
                            pub fn #readable_stateless_field_idents(&self) -> #value_tys {
                                self.value.#value_tys(#readable_stateless_field_idents::OFFSET)
                            }
                        )*
                    }
                }));

                items.push(Item::Verbatim(quote! {
                impl<#(#stateful_field_tys,)*> Register<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    pub fn read<T>(&self, f: impl FnOnce(&Reader) -> T) -> T {
                        let reader = Reader::new(
                            // SAFETY: assumes the proc macro implementation is sound
                            // and that the peripheral description is accurate
                            unsafe {
                                core::ptr::read_volatile((super::BASE_ADDR + OFFSET) as *const u32)
                            }
                        );

                        f(&reader)
                    }
                }
            }));
            }
        }

        // writer
        {
            let writable_stateless_fields = stateless_fields
                .iter()
                .filter(|field| field.args.write.is_some())
                .collect::<Vec<_>>();

            let writable_stateless_field_idents = writable_stateless_fields
                .iter()
                .map(|field| field.ident.clone())
                .collect::<Vec<_>>();

            let value_tys = writable_stateless_fields
                .iter()
                .map(|field| {
                    let ident = format_ident!(
                        "u{}",
                        Index {
                            index: field.args.width as _,
                            span: Span::call_site(),
                        }
                    );

                    match field.args.width {
                        1 => quote! { bool },
                        8 | 16 | 32 => quote! { #ident },
                        _ => quote! { ::proto_hal::macro_utils::arbitrary_int::#ident },
                    }
                })
                .collect::<Vec<_>>();

            if !writable_stateless_fields.is_empty() {
                items.push(Item::Verbatim(quote! {
                pub struct Writer {
                    value: u32,
                }

                impl Writer {
                    const fn new() -> Self {
                        Self {
                            value: 0,
                        }
                    }

                    #(
                        pub fn #writable_stateless_field_idents(&mut self, value: #value_tys) -> &mut Self {
                            self.value |= (value as u32) << #writable_stateless_field_idents::OFFSET;
                            self
                        }
                    )*
                }
            }));

                items.push(Item::Verbatim(quote! {
                impl<#(#stateful_field_tys,)*> Register<#(#stateful_field_tys,)*>
                where
                    #(
                        #stateful_field_tys: #stateful_field_idents::State,
                    )*
                {
                    pub fn write(&self, f: impl FnOnce(&mut Writer) -> &mut Writer) {
                        let mut writer = Writer::new();

                        f(&mut writer);

                        // SAFETY: assumes the proc macro implementation is sound
                        // and that the peripheral description is accurate
                        unsafe {
                            core::ptr::write_volatile((super::BASE_ADDR + OFFSET) as *mut u32, writer.value);
                        }
                    }
                }
            }));
            }
        }
    }

    Ok(RegisterInfo {
        args: register_args,
        ident: module.ident.clone(),
        stateful,
    })
}

fn process_block(block_args: &BlockArgs, module: &mut ItemMod) -> Result<(), syn::Error> {
    let items = &mut module
        .content
        .as_mut()
        .expect("module must be a definition, not an import")
        .1;

    let mut error_combinator = SynErrorCombinator::new();

    let mut registers = Vec::new();

    items.iter_mut().for_each(|item| {
        let Item::Mod(inner_mod) = item else { return };

        // 1. try to extract register annotation args
        let mut register_args = None;

        inner_mod.attrs.retain(|attr| {
            if attr.path().is_ident("register") {
                error_combinator.try_maybe_then(RegisterArgs::from_meta(&attr.meta), |args| {
                    // validate offset specification
                    if args.offset.is_none() && !block_args.auto_increment {
                        Err(syn::Error::new_spanned(attr.path(), "register offset must be specified. to infer offsets, add the `auto_increment` argument to the block attribute macro"))?
                    }

                    register_args.replace(args);

                    Ok(())
                });

                false
            } else {
                true
            }
        });

        let Some(register_args) = register_args else {
            return;
        };

        // 2. pass the module over to the register parser
        error_combinator.maybe_then(process_register(register_args, registers.last().cloned(), inner_mod), |register| {
            registers.push(register);
        });
    });

    error_combinator.coalesce()?;

    {
        let base_addr = block_args.base_addr;

        let (stateful_registers, stateless_registers) = registers
            .iter()
            .partition::<Vec<_>, _>(|register| register.stateful);

        let stateful_register_idents = stateful_registers
            .iter()
            .map(|register| register.ident.clone())
            .collect::<Vec<_>>();

        let stateless_register_idents = stateless_registers
            .iter()
            .map(|register| register.ident.clone())
            .collect::<Vec<_>>();

        let stateful_register_tys = stateful_register_idents
            .iter()
            .map(|ident| {
                Ident::new(
                    &inflector::cases::pascalcase::to_pascal_case(&ident.to_string()),
                    Span::call_site(),
                )
            })
            .collect::<Vec<_>>();

        let entitlement_idents = (0..block_args.entitlements.elems.len())
            .map(|i| format_ident!("entitlement{}", i))
            .collect::<Vec<_>>();
        let entitlement_tys = (0..block_args.entitlements.elems.len())
            .map(|i| format_ident!("Entitlement{}", i))
            .collect::<Vec<_>>();

        let reset_entitlement_tys = entitlement_tys
            .iter()
            .map(|_| {
                quote! {
                    ::proto_hal::stasis::Unsatisfied
                }
            })
            .collect::<Vec<_>>();

        items.push(Item::Verbatim(quote! {
            pub const BASE_ADDR: u32 = #base_addr;

            #[cfg_attr(feature = "defmt", derive(::defmt::Format))]
            pub struct Block<
                #(
                    #stateful_register_tys,
                )*

                #(
                    #entitlement_tys,
                )*
            > {
                #(
                    pub #stateful_register_idents: #stateful_register_tys,
                )*

                #(
                    pub #stateless_register_idents: #stateless_register_idents::Register,
                )*

                #(
                    pub #entitlement_idents: #entitlement_tys,
                )*
            }

            pub type Reset = Block<
                #(
                    #stateful_register_idents::Reset,
                )*

                #(
                    #reset_entitlement_tys,
                )*
            >;
        }));

        let entitlements = block_args
            .entitlements
            .elems
            .iter()
            .map(|path| {
                quote! {
                    ::proto_hal::stasis::Entitlement<#path>
                }
            })
            .collect::<Vec<_>>();

        // Q: better way to do this?
        stateful_register_idents
            .iter()
            .zip(stateful_register_tys.iter())
            .enumerate()
            .for_each(|(i, (ident, ty))| {
                let prev_register_idents = stateful_register_idents.get(..i).unwrap();
                let next_register_idents = stateful_register_idents.get(i + 1..).unwrap();

                let prev_register_tys = stateful_register_tys.get(..i).unwrap();
                let next_register_tys = stateful_register_tys.get(i + 1..).unwrap();

                items.push(Item::Verbatim(quote! {
                    impl<#(#stateful_register_tys,)*> Block<#(#stateful_register_tys,)* #(#entitlements,)*> {
                        pub fn #ident<R>(self, f: impl FnOnce(#ty) -> R) -> Block<#(#prev_register_tys,)* R, #(#next_register_tys,)* #(#entitlements,)*> {
                            Block {
                                #(
                                    #prev_register_idents: self.#prev_register_idents,
                                )*

                                #ident: f(self.#ident),

                                #(
                                    #next_register_idents: self.#next_register_idents,
                                )*

                                #(
                                    #stateless_register_idents: self.#stateless_register_idents,
                                )*

                                #(
                                    #entitlement_idents: self.#entitlement_idents,
                                )*
                            }
                        }
                    }
                }));
            });

        if !block_args.entitlements.elems.is_empty() {
            items.push(Item::Verbatim(quote! {
                impl<#(#stateful_register_tys,)*> Block<#(#stateful_register_tys,)* #(#reset_entitlement_tys,)*> {
                    pub fn attach(self, #(#entitlement_idents: #entitlements,)*) -> Block<#(#stateful_register_tys,)* #(#entitlements,)*> {
                        Block {
                            #(
                                #stateful_register_idents: self.#stateful_register_idents,
                            )*

                            #(
                                #stateless_register_idents: self.#stateless_register_idents,
                            )*

                            #(
                                #entitlement_idents,
                            )*
                        }
                    }
                }
            }));
        }
    }

    Ok(())
}

fn block_inner(args: TokenStream, item: TokenStream) -> Result<TokenStream2, syn::Error> {
    let block_args = BlockArgs::from_list(&NestedMeta::parse_meta_list(args.into())?)?;

    let mut module = parse2::<ItemMod>(item.into())?;

    process_block(&block_args, &mut module)?;

    Ok(if !block_args.erase_mod {
        module.to_token_stream()
    } else {
        let items = module.content.unwrap().1;

        quote! {
            #(
                #items
            )*
        }
    })
}

#[proc_macro_attribute]
pub fn block(args: TokenStream, item: TokenStream) -> TokenStream {
    match block_inner(args, item) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
