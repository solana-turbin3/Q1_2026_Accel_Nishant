use std::error::Error;
use std::marker::PhantomData;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

/// Common Result type
type BoxError = Box<dyn Error + Send + Sync>;
type Result<T> = std::result::Result<T, BoxError>;

/// =======================================================
/// 1️⃣ Serializer Trait
/// =======================================================
///
/// Defines generic serialization behavior.
/// Any serializer must implement these two methods.
///
pub trait Serializer {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize;

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize;
}

/// =======================================================
/// 2️⃣ Borsh Serializer
/// =======================================================

pub struct Borsh;

impl Serializer for Borsh {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize,
    {
        Ok(borsh::to_vec(value)?)
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize,
    {
        Ok(borsh::from_slice(bytes)?)
    }
}

/// =======================================================
/// 3️⃣ Wincode Serializer
/// =======================================================

pub struct Wincode;

impl Serializer for Wincode {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize,
    {
        Ok(bincode::serialize(value)?)
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize,
    {
        Ok(bincode::deserialize(bytes)?)
    }
}

/// =======================================================
/// 4️⃣ JSON Serializer
/// =======================================================

pub struct Json;

impl Serializer for Json {
    fn to_bytes<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize,
    {
        Ok(serde_json::to_vec(value)?)
    }

    fn from_bytes<T>(&self, bytes: &[u8]) -> Result<T>
    where
        T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize,
    {
        Ok(serde_json::from_slice(bytes)?)
    }
}

/// =======================================================
/// 5️⃣ Generic Storage Container
/// =======================================================

pub struct Storage<T, S>
where
    S: Serializer,
{
    data: Option<Vec<u8>>,
    serializer: S,
    _marker: PhantomData<T>,
}

impl<T, S> Storage<T, S>
where
    S: Serializer,
    T: Serialize + for<'de> Deserialize<'de> + BorshSerialize + BorshDeserialize,
{
    /// Create new storage with chosen serializer
    pub fn new(serializer: S) -> Self {
        Self {
            data: None,
            serializer,
            _marker: PhantomData,
        }
    }

    /// Save value (serialize into bytes)
    pub fn save(&mut self, value: &T) -> Result<()> {
        let bytes = self.serializer.to_bytes(value)?;
        self.data = Some(bytes);
        Ok(())
    }

    /// Load value (deserialize from bytes)
    pub fn load(&self) -> Result<T> {
       let bytes = self
    .data
    .as_ref()
    .ok_or_else(|| "No data stored".to_string())?;

        self.serializer.from_bytes(bytes)
    }

    /// Check if storage contains data
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
}

/// =======================================================
/// 6️⃣ Test Data Type
/// =======================================================

#[derive(
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
    BorshSerialize,
    BorshDeserialize,
)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

/// =======================================================
/// 7️⃣ Unit Tests
/// =======================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_person() -> Person {
        Person {
            name: "Andre".to_string(),
            age: 30,
        }
    }

    #[test]
    fn test_borsh_storage() {
        let person = sample_person();

        let mut storage = Storage::new(Borsh);
        storage.save(&person).unwrap();

        assert!(storage.has_data());

        let loaded = storage.load().unwrap();
        assert_eq!(person, loaded);
    }

    #[test]
    fn test_wincode_storage() {
        let person = sample_person();

        let mut storage = Storage::new(Wincode);
        storage.save(&person).unwrap();

        assert!(storage.has_data());

        let loaded = storage.load().unwrap();
        assert_eq!(person, loaded);
    }

    #[test]
    fn test_json_storage() {
        let person = sample_person();

        let mut storage = Storage::new(Json);
        storage.save(&person).unwrap();

        assert!(storage.has_data());

        let loaded = storage.load().unwrap();
        assert_eq!(person, loaded);
    }
}