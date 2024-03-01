#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Bytes(Vec<u8>),
    F32(f32),
    ISize(isize),
    None,
    String(String),
    USize(usize),
    Vec(Vec<Value>),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::F32(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Value::ISize(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::None
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::USize(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Vec(value)
    }
}

impl TryFrom<Value> for bool {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bytes(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for f32 {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::F32(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for isize {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::ISize(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for () {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::None => Ok(()),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for usize {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::USize(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for Vec<Value> {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Vec(value) => Ok(value),
            _ => Err(()),
        }
    }
}
