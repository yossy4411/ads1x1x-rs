#[derive(Debug, Clone, Copy)]
pub(crate) enum OperatingMode {
    OneShot,
    Continuous,
}

mod common;
mod features;
mod mode;
