use super::block;

pub unsafe trait Register<Block>
where
    Block: block::Block,
{
    const ADDR: usize;
}
