mod criteria;
mod math_trait_ext;
mod pipe;

pub(crate) use self::{criteria::*, math_trait_ext::*, pipe::*};

macro_rules! single {
    ($query:expr) => {
        match $query.get_single() {
            Ok(q) => q,
            _ => {
                return;
            }
        }
    };
}

macro_rules! single_mut {
    ($query:expr) => {
        match $query.get_single_mut() {
            Ok(q) => q,
            _ => {
                return;
            }
        }
    };
}

pub(crate) use single;
pub(crate) use single_mut;
