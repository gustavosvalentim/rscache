use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    String(String),
    Float(f64),
    Boolean(bool),
}

pub trait ValueConvert<T> {
    fn to(&self) -> Result<&T, ()>;
}

impl ValueConvert<String> for Value {
    fn to(&self) -> Result<&String, ()> {
        match self {
            Value::String(value) => return Ok(value),
            _ => return Err(()),
        }
    }
}

impl ValueConvert<i32> for Value {
    fn to(&self) -> Result<&i32, ()> {
        match self {
            Value::Integer(value) => return Ok(value),
            _ => return Err(()),
        }
    }
}

impl ValueConvert<f64> for Value {
    fn to(&self) -> Result<&f64, ()> {
        match self {
            Value::Float(value) => return Ok(value),
            _ => return Err(()),
        }
    }
}

impl ValueConvert<bool> for Value {
    fn to(&self) -> Result<&bool, ()> {
        match self {
            Value::Boolean(value) => return Ok(value),
            _ => return Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    KeyDoesNotExist,
    MaxSizeExceeded,
}

pub struct MemoryDatabase {
    items: HashMap<String, Value>,
    size: i32,
    max_size: i32,
}

impl MemoryDatabase {
    /// Creates a new instance of MemoryDatabase
    ///
    /// # Examples
    /// ```
    /// use rscache::{MemoryDatabase, Value, ValueConvert};
    ///
    /// let mut db = MemoryDatabase::new();
    /// db.set(String::from("test"), Value::String(String::from("test")));
    ///
    /// // Use type inference to get the value with the expected type
    /// let item: &String = db.get("test").unwrap().to().unwrap();
    ///
    /// assert_eq!(item, "test");
    /// ```
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            size: 0,
            max_size: 1024 * 1024,
        }
    }

    /// Returns the current size of the database
    ///
    /// # Examples
    /// ```
    /// use rscache::{MemoryDatabase, Value};
    ///
    /// let mut db = MemoryDatabase::new();
    /// db.set(String::from("test"), Value::String(String::from("test")));
    ///
    /// assert_eq!(db.size(), 8);
    /// ```
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Sets a new key-value pair in the database
    ///
    /// # Examples
    /// ```
    /// use rscache::{MemoryDatabase, Value};
    ///
    /// let mut db = MemoryDatabase::new();
    ///
    /// db.set(String::from("test"), Value::String(String::from("test")));
    /// db.set(String::from("pi"), Value::Float(3.14));
    /// db.set(String::from("is_active"), Value::Boolean(true));
    /// db.set(String::from("age"), Value::Integer(25));
    ///
    /// assert_eq!(db.size(), 35);
    /// ```
    pub fn set(&mut self, key: String, value: Value) -> Result<(), DatabaseError> {
        let item_size = Self::calculate_value_size(key.as_ref(), &value);

        if self.size + item_size > self.max_size {
            return Err(DatabaseError::MaxSizeExceeded);
        }

        self.items.insert(key, value);
        self.size += item_size;

        return Ok(());
    }

    /// Gets a value from the database
    ///
    /// # Examples
    /// ```
    /// use rscache::{MemoryDatabase, Value, ValueConvert};
    ///
    /// let mut db = MemoryDatabase::new();
    ///
    /// db.set(String::from("test"), Value::String(String::from("test")));
    ///
    /// let value: &String = db.get("test").unwrap().to().unwrap();
    ///
    /// assert_eq!(value, "test");
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        let item = self.items.get(key);

        if let Some(item) = item {
            return Some(&item);
        }

        None
    }

    /// Removes a key-value pair from the database
    ///
    /// # Examples
    /// ```
    /// use rscache::{MemoryDatabase, Value};
    ///
    /// let mut db = MemoryDatabase::new();
    ///
    /// db.set(String::from("test"), Value::String(String::from("test")));
    ///
    /// db.remove("test");
    ///
    /// assert_eq!(db.size(), 0);
    /// ```
    pub fn remove(&mut self, key: &str) -> Result<(), DatabaseError> {
        if !self.items.contains_key(key) {
            return Err(DatabaseError::KeyDoesNotExist);
        }

        let item_size = Self::calculate_value_size(key, self.items.get(key).unwrap());

        self.items.remove(key);
        self.size -= item_size;

        Ok(())
    }

    /// Calculates the value size based on the key and value
    fn calculate_value_size(key: &str, value: &Value) -> i32 {
        let value_size: i32;

        match value {
            Value::Integer(_) => value_size = 4,
            Value::String(value) => value_size = value.len() as i32,
            Value::Float(_) => value_size = 8,
            Value::Boolean(_) => value_size = 1,
        }

        value_size + key.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_memory_database_add() {
        let mut db = MemoryDatabase::new();

        db.set(String::from("test"), Value::String(String::from("test")))
            .unwrap();

        assert_eq!(db.size(), 8);
    }

    #[test]
    fn test_get_cache_item() {
        let mut db = MemoryDatabase::new();

        db.set(String::from("test"), Value::String(String::from("test")))
            .unwrap();

        let value: &String = db.get("test").unwrap().to().unwrap();

        assert_eq!(value, "test");
    }

    #[test]
    fn test_get_float_cache_item() {
        let mut db = MemoryDatabase::new();

        db.set(String::from("pi"), Value::Float(3.14)).unwrap();

        let value: f64 = *db.get("pi").unwrap().to().unwrap();

        assert_eq!(value, 3.14);
    }

    #[test]
    fn test_get_boolean_cache_item() {
        let mut db = MemoryDatabase::new();

        db.set(String::from("is_active"), Value::Boolean(true))
            .unwrap();

        let value: bool = *db.get("is_active").unwrap().to().unwrap();

        assert_eq!(value, true);
    }

    #[test]
    fn test_memory_db_max_size_exceeded() {
        let mut db = MemoryDatabase::new();
        db.max_size = 1;

        let result = db
            .set(String::from("test"), Value::String(String::from("test")))
            .unwrap_err();

        assert_eq!(result, DatabaseError::MaxSizeExceeded);
    }
}
