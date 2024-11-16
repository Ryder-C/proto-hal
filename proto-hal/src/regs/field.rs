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

pub unsafe trait FieldSpec {
    const WIDTH: u8;
}

pub unsafe trait Field<Block, Register>
where
    Block: block::Block,
    Register: register::Register<Block>,
{
    const OFFSET: usize;

    type Spec: FieldSpec;
    type Reset: Field<Block, Register>;

    type Read: access::Read;
    type Write: access::Write;
}
