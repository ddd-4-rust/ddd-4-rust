//! Identifies an entity within all entities of the same type.
//!
//! 1:1 translation of `org.fuin.ddd4j.core.EntityId`.

use crate::entity_id_factory::EntityIdFactory;
use crate::entity_type::EntityType;
use std::fmt::{Debug, Display};

/// Identifies an entity within all entities of the same type.
///
/// Java: `EntityId extends TechnicalId, AsStringCapable, Serializable`
///
/// Each implementor must also implement `Display` and `Debug`.
pub trait EntityId: Debug + Display + Send + Sync + 'static {
    /// Returns the type represented by this identifier.
    ///
    /// Java: `getType() -> EntityType`
    fn entity_type(&self) -> &dyn EntityType;

    /// Returns the entity identifier as string.
    ///
    /// Java: `asString()`
    fn as_string(&self) -> String;

    /// Returns the entity identifier as string with type and identifier.
    ///
    /// Java: `asTypedString()`
    fn as_typed_string(&self) -> String {
        format!("{} {}", self.entity_type().as_string(), self.as_string())
    }

    /// Verifies that the given value can be converted into an entity identifier.
    ///
    /// Java: `EntityId.isValid(EntityIdFactory factory, String value)`
    fn is_valid(factory: &dyn EntityIdFactory, value: Option<&str>) -> bool
    where
        Self: Sized,
    {
        let value = match value {
            None => return true,
            Some(v) => v,
        };
        let p = value.find(' ');
        if p.is_none() {
            return false;
        }
        let p = p.unwrap();
        let r#type = &value[..p];
        let id = &value[p + 1..];
        if !factory.contains_type(r#type) {
            return false;
        }
        factory.is_valid(r#type, id)
    }

    /// Converts a string into an entity identifier using the given factory.
    ///
    /// Java: `EntityId.valueOf(EntityIdFactory factory, String value)`
    fn value_of(factory: &dyn EntityIdFactory, value: Option<&str>) -> Option<Box<dyn EntityId>>
    where
        Self: Sized,
    {
        let value = value?;
        let p = value.find(' ')?;
        let r#type = &value[..p];
        let id = &value[p + 1..];
        factory.create_entity_id(r#type, id)
    }
}
