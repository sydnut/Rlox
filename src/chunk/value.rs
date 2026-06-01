pub type Value = f64;
#[derive(Debug)]
pub struct ValueArray {
    values: Vec<Value>,
}
impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }
    pub fn write_value(&mut self, value: Value) {
        self.values.push(value);
    }
    pub fn count(&self) -> usize {
        self.values.len()
    }
    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }
    pub fn values(&self) -> &[Value] {
        &self.values
    }
}
