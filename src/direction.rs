use sealed::sealed;

#[sealed]
pub trait Direction {}
pub struct Ingoing;
pub struct Outgoing;
pub struct All;
#[sealed]
impl Direction for Ingoing {}
#[sealed]
impl Direction for Outgoing {}
#[sealed]
impl Direction for All {}
