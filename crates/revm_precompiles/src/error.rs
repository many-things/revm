use alloc::borrow::Cow;

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "no-derive"), derive(Debug,))]
pub enum Return {
    Exit,
    OutOfGas,
    /// Other normal errors.
    Other(Cow<'static, str>),
}
