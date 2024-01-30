/*! Rtorrent multicalls

This module defines the [`MultiBuilder`] type, which is a tool for building queries of multiple
fields across a single XMLRPC call.  The query results are nicely typed.
!*/

#![allow(dead_code)]

use crate::{value_conversion, Result, Server};
use xmlrpc::{Request, Value};

pub(super) struct MultiBuilderInternal {
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

    pub(super) fn push_arg(&mut self, val: Value) {
        self.args.push(val);
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

    pub(crate) fn invoke(&self) -> Result<Vec<Value>> {
        let list = self.server.execute(self.as_request())?;
        Ok(value_conversion::list(&list)?.clone())
    }
}

// The `MultiBuilder` type is a tool for building queries of one or more fields across many items,
// in a single XMLRPC call.  The query results are nicely typed.
//
// ## Usage
//
// ```no_run
// use rtorrent_xmlrpc_bindings as rtorrent;
//
// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
//
// // This call's result rows will consist of downloads in the "default" view.
// let callbuilder = MultiBuilder::new(&my_handle, "d.multicall2", "", "default");
//
// let rows = callbuilder.call::<String>("d.name")
//     .call::<f64>("d.ratio")
//     .call::<i64>("d.size_bytes")
//     .invoke()?;
// for (name, ratio, bytes) in rows {
//     println!("{}: {} bytes, {} ratio", name, bytes, ratio);
// }
// # Ok::<(), rtorrent::Error>(())
// ```
//
// The `call()` method can be invoked repeatedly to add more columns to the query -- in the above
// example, selecting the "d.name", "d.ratio", and "d.size_bytes" columns.
pub(crate) struct MultiBuilder {
    pub(super) inner: MultiBuilderInternal,
}

impl MultiBuilder {
    /// Start building a multicall against `server` using the XMLRPC function specified by `multicall`.
    ///
    /// There are always some `call_target` and `call_filter` strings.
    ///
    /// `call_target` is often a torrent identifier (SHA1 hex) for `t.*`, `p.*`, and `f.*` queries,
    /// but it is the empty string (`""`) for `d.*` queries.
    ///
    /// `call_filter` behavior varies according to the specific `multicall` operation.  Usually the
    /// empty string is equivalent to unfiltered.  For `d.*` queries, the filter corresponds to
    /// some rtorrent "view."
    pub(crate) fn new(
        server: &Server,
        multicall: &str,
        call_target: &str,
        call_filter: &str,
    ) -> Self {
        Self {
            inner: MultiBuilderInternal::new(
                server,
                multicall,
                call_target.into(),
                call_filter.into(),
            ),
        }
    }
}

macro_rules! define_builder {
    // The pipe is an ugly kludge to allow us to list types left-to-right but avoid Rust macro
    // parsing ambiguity.
    ( $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        pub(crate) struct $name<$($ty: TryFromValue,)* $ty_last: TryFromValue> {
            inner: MultiBuilderInternal,
            $($phantoms: PhantomData<$ty>,)*
            $phantom_last: PhantomData<$ty_last>,
        }

        impl<$($ty: TryFromValue,)* $ty_last: TryFromValue> $name<$($ty,)* $ty_last> {
            pub(crate) fn invoke(&self) -> Result<Vec<($($ty,)* $ty_last,)>> {
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
            /// Add a column (an accessor for `getter`) of type `T` to the query represented by
            /// this builder.
            ///
            /// `call()` can be invoked again, repeatedly, on the result of `call()` invocations,
            /// to build queries with more columns.
            ///
            /// (The higher-order builder types are invisible in Rustdoc because they are generated
            /// by macros.)
            pub(crate) fn call<T: TryFromValue>(self, getter: &str) -> $name<$($ty,)* T> {
                let mut inner = self.inner;
                inner.push_arg(Value::from(format!("{}=", getter)));
                $name {
                    inner,
                    $($phantoms: PhantomData,)*
                    $phantom_last: PhantomData,
                }
            }
        }
    }
}
pub(super) use define_builder;
