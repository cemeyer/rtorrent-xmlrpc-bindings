//! Rtorrent multicalls

#![allow(dead_code)]

use crate::{value_conversion::{self, TryFromValue}, Error, Result};
use std::marker::PhantomData;
use xmlrpc::Value;

mod raw;

pub(crate) use raw::MultiBuilder;
use raw::MultiBuilderInternal;

raw::define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
raw::define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
raw::define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
raw::define_builder!(MultiBuilder3, MultiBuilder4, phantom_a A, phantom_b B, phantom_c C | phantom_d D);
