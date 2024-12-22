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

impl Access {
    pub fn is_read(&self) -> bool {
        match self {
            Self::Read(_) | Self::ReadWrite(_) => true,
            Self::Write(_) => false,
        }
    }

    pub fn is_write(&self) -> bool {
        match self {
            Self::Write(_) | Self::ReadWrite(_) => true,
            Self::Read(_) => false,
        }
    }
}
