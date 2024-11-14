use super::{block, register};

pub mod access {
    pub trait Write {}
    pub trait Read {}

    pub struct Granted;
    pub struct Denied;

    impl Write for Granted {}
    impl Read for Granted {}

    impl Write for Denied {}
    impl Read for Denied {}
}

pub trait FieldSpec {
    const WIDTH: u8;

    type Write: access::Write;
    type Read: access::Read;
}

pub trait Field<Block, Register>: FieldSpec
where
    Block: block::Block,
    Register: register::Register<Block>,
{
    type Reset: Field<Block, Register>;
    const OFFSET: usize;
}
