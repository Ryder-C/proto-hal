use std::collections::HashSet;

use darling::FromMeta;
use syn::{Meta, Path};

use crate::utils::PathArray;

#[derive(Debug, Clone, Default, FromMeta)]
#[darling(default)]
pub struct AccessArgs {
    pub entitlements: PathArray,
    pub effect: Option<Meta>,
}

#[derive(Debug, Clone)]
pub struct Read {
    pub entitlements: HashSet<Path>,
    pub effects: (),
}

#[derive(Debug, Clone)]
pub struct Write {
    pub entitlements: HashSet<Path>,
    pub effects: (),
}

#[derive(Debug, Clone)]
pub struct ReadWrite {
    pub entitlements: HashSet<Path>,
    pub effects: (),
}

#[derive(Debug, Clone)]
pub enum Access {
    Read(Read),
    Write(Write),
    ReadWrite(ReadWrite),
}
