use crate::{Error, Result, Value};

// rtorrent primitives are "Value," which are integers, and String, which... are strings.
pub(crate) fn int(val: &Value) -> Result<i64> {
    match val {
        Value::Int(i) => Ok(*i as _),
        Value::Int64(i) => Ok(*i),
        _ => Err(Error::UnexpectedStructure(
            format!("Got {:?}, expected integer", val)
        )),
    }
}

pub(crate) fn fraction1000(val: &Value) -> Result<f64> {
    Ok((int(val)? as f64) / 1000.)
}

// "Bools" are expressed as integer Values, but we'll accept an actual bool in case they choose
// to use that eventually.
pub(crate) fn bool(val: &Value) -> Result<bool> {
    match val {
        Value::Int(i) => Ok(*i != 0),
        Value::Int64(i) => Ok(*i != 0),
        Value::Bool(b) => Ok(*b),
        _ => Err(Error::UnexpectedStructure(
            format!("Got {:?}, expected bool or integer type", val)
        )),
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

pub(crate) fn string_owned(val: &Value) -> Result<String> {
    string(val).map(|s| s.to_owned())
}

// Void is represented as zero-valued int, but we'll accept nil.
pub(crate) fn void(val: &Value) -> Result<()> {
    match val {
        Value::Int(0) => Ok(()),
        Value::Nil => Ok(()),
        _ => Err(Error::UnexpectedStructure(
            format!("Got {:?}, expected int(0) or nil", val)
        )),
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
