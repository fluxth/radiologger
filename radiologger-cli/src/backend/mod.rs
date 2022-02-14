#[cfg(feature = "direct")]
mod direct;

#[cfg(feature = "remote")]
mod remote;

pub trait Connection {}
