use serde_json::Value;

#[non_exhaustive]
#[derive(Debug)]
pub struct Message(Value);

impl Message {
    pub fn new(value: Value) -> Self {
        Self(value)
    }

    pub fn inner(&self) -> Value {
        self.0.to_owned()
    }
}
