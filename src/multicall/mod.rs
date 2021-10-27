//! Rtorrent multicalls

#![allow(dead_code)]

mod ops;
mod raw_impl;

mod raw {
    use super::raw_impl;

    use crate::{value_conversion::{self, TryFromValue}, Error, Result};
    use std::marker::PhantomData;
    use xmlrpc::Value;

    pub(crate) use raw_impl::MultiBuilder;
    use raw_impl::MultiBuilderInternal;

    raw_impl::define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
    raw_impl::define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    raw_impl::define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    raw_impl::define_builder!(MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
}

pub mod d {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::d::*;

    define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
}

pub mod f {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::f::*;

    define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
}

pub mod p {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::p::*;

    define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
}

pub mod t {
    use crate::{value_conversion::TryFromValue, Result};
    use super::{ops, raw};

    pub use ops::t::*;

    define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
    define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
    define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
    define_builder!(MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
}
