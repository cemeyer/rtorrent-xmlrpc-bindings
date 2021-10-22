/*! Rtorrent multicalls

This module defines the [`MultiBuilder`] type, which is a tool for building queries of multiple
fields across a single XMLRPC call.  The query results are nicely typed.

## Usage

```rust
use rtorrent_xmlrpc_bindings as rtorrent;

let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");

// This call's result rows will consist of downloads in the "default" view.
let callbuilder = MultiBuilder::new(&my_handle, "d.multicall2", "", "default");

let rows = callbuilder.call::<String>("d.name")
    .call::<f64>("d.ratio")
    .call::<i64>("d.size_bytes")
    .invoke()?;
for (name, ratio, bytes) in rows {
    println!("{}: {} bytes, {} ratio", name, bytes, ratio);
}
```

## Current Limitations

* It might be nice to have some prepopulated singletons of API, type pairs.  I'm not yet sure what
  that would look like.

[`MultiBuilder`]: crate::MultiBuilder
!*/

#![allow(dead_code)]

use crate::{value_conversion::{self, TryFromValue}, Error, Result, Server};
use std::marker::PhantomData;
use xmlrpc::{Request, Value};

struct MultiBuilderInternal {
    server: Server,
    multicall: String,
    call_target: Value,
    call_filter: Value,
    args: Vec<Value>,
}

impl MultiBuilderInternal {
    fn new(server: &Server, multicall: &str, call_target: Value, call_filter: Value) -> Self {
        Self {
            server: server.clone(),
            multicall: multicall.to_owned(),
            call_target,
            call_filter,
            args: Vec::new(),
        }
    }

    fn as_request(&self) -> Request {
        let mut req = Request::new(&self.multicall)
            .arg(self.call_target.clone())
            .arg(self.call_filter.clone());
        for arg in &self.args {
            req = req.arg(arg.clone());
        }
        req
    }

    fn invoke(&self) -> Result<Vec<Value>> {
        let list = self.as_request()
            .call_url(self.server.endpoint())?;
        Ok(value_conversion::list(&list)?.clone())
    }
}

/// Builder to construct a query of the same accessor(s) across many target objects.
pub struct MultiBuilder {
    inner: MultiBuilderInternal,
}

impl MultiBuilder {
    /// Start building a multicall against `server` using the XMLRPC function `multicall`.  The
    /// `call_target` is some rtorrent identifier (SHA1 hex).  The `call_filter` behavior will
    /// depend on the specific `multicall` operation.  Usually the empty string is equivalent to
    /// unfiltered.
    pub fn new(server: &Server, multicall: &str, call_target: &str, call_filter: &str) -> Self {
        Self {
            inner: MultiBuilderInternal::new(server,
                                             multicall,
                                             call_target.into(),
                                             call_filter.into()),
        }
    }
}

macro_rules! define_builder {
    // The pipe is an ugly kludge to allow us to list types left-to-right but avoid Rust macro
    // parsing ambiguity.
    ( $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        pub struct $name<$($ty: TryFromValue,)* $ty_last: TryFromValue> {
            inner: MultiBuilderInternal,
            $($phantoms: PhantomData<$ty>,)*
            $phantom_last: PhantomData<$ty_last>,
        }

        impl<$($ty: TryFromValue,)* $ty_last: TryFromValue> $name<$($ty,)* $ty_last> {
            pub fn invoke(&self) -> Result<Vec<($($ty,)* $ty_last,)>> {
                let list = self.inner.invoke()?;
                let mut res = Vec::new();

                for row in list {
                    let row = value_conversion::list(&row)?;
                    // Repurposing (abusing) existing phantom names for temp variables.
                    if let [$($phantoms,)* $phantom_last] = row.as_slice() {
                        res.push((
                                $($ty::try_from_value(&$phantoms)?,)*
                                $ty_last::try_from_value(&$phantom_last)?,
                            ));
                    } else {
                        return Err(Error::UnexpectedStructure(
                                format!("row missing columns ({:?})", row)));
                    }
                }
                Ok(res)
            }
        }

        impl<$($ty: TryFromValue,)*> $prev<$($ty,)*> {
            /// Add an accessor for `getter` (resulting in type `T`) to the query represented by this builder.
            pub fn call<T: TryFromValue>(self, getter: &str) -> $name<$($ty,)* T> {
                let mut inner = self.inner;
                inner.args.push(Value::from(format!("{}=", getter)));
                $name {
                    inner,
                    $($phantoms: PhantomData,)*
                    $phantom_last: PhantomData,
                }
            }
        }
    }
}

define_builder!(MultiBuilder,  MultiBuilder1, | phantom_a A);
define_builder!(MultiBuilder1, MultiBuilder2, phantom_a A | phantom_b B);
define_builder!(MultiBuilder2, MultiBuilder3, phantom_a A, phantom_b B | phantom_c C);
