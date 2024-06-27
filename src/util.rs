pub(crate) mod criteria;
pub(crate) mod math_trait_ext;

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
