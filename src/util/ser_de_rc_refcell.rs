
use serde::{Serializer, Deserializer, Deserialize, Serialize};
use std::rc::Rc;
use std::cell::RefCell;

// Custom serialization for Rc<RefCell<T>>
pub fn serialize<T, S>(value: &Rc<RefCell<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
{
    // Directly serialize the value inside Rc<RefCell<T>>
    value.borrow().serialize(serializer)
}

// Custom deserialization for Rc<RefCell<T>>
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Rc<RefCell<T>>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
{
    // Deserialize directly into the type T
    let value = T::deserialize(deserializer)?;
    Ok(Rc::new(RefCell::new(value)))
}

