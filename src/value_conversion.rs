use crate::{Error, Result, Value};

// Essentially TryFrom<Value> with crate::Error, but we need our own trait because crates are not
// allowed to define implementations of traits from foreign crates on types from forein crates.
pub(crate) trait TryFromValue: Sized {
    fn try_from_value(val: &Value) -> Result<Self>;
}

// rtorrent primitives are "Value," which are integers, and String, which... are strings.
impl TryFromValue for i64 {
    fn try_from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Int(i) => Ok(*i as _),
            Value::Int64(i) => Ok(*i),
            _ => Err(Error::UnexpectedStructure(
                format!("Got {:?}, expected integer", val)
            )),
        }
    }
}

// rtorrent expresses a few statistics as fixed-point decimal; treat as floats.
impl TryFromValue for f64 {
    fn try_from_value(val: &Value) -> Result<Self> {
        Ok((i64::try_from_value(val)? as f64) / 1000.)
    }
}

// "Bools" are expressed as integer Values, but we'll accept an actual bool in case they choose
// to use that eventually.
impl TryFromValue for bool {
    fn try_from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Int(i) => Ok(*i != 0),
            Value::Int64(i) => Ok(*i != 0),
            Value::Bool(b) => Ok(*b),
            _ => Err(Error::UnexpectedStructure(
                format!("Got {:?}, expected bool or integer type", val)
            )),
        }
    }
}

pub(crate) fn string(val: &Value) -> Result<&str> {
    match val {
        Value::String(s) => Ok(s),
        _ => Err(Error::UnexpectedStructure(
            format!("Got {:?}, expected string", val)
        )),
    }
}

impl TryFromValue for String {
    fn try_from_value(val: &Value) -> Result<Self> {
        string(val).map(|s| s.to_owned())
    }
}

// Void is represented as zero-valued int, but we'll accept nil.
impl TryFromValue for () {
    fn try_from_value(val: &Value) -> Result<Self> {
        match val {
            Value::Int(0) => Ok(()),
            Value::Nil => Ok(()),
            _ => Err(Error::UnexpectedStructure(
                format!("Got {:?}, expected int(0) or nil", val)
            )),
        }
    }
}

pub(crate) fn list(val: &Value) -> Result<&Vec<Value>> {
    match val {
        Value::Array(a) => Ok(a),
        _ => Err(Error::UnexpectedStructure(
            format!("Got {:?}, expected array", val)
        )),
    }
}
