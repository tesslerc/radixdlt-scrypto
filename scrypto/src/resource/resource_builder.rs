use crate::resource::*;
use crate::rust::borrow::ToOwned;
use crate::rust::collections::HashMap;
use crate::rust::string::String;
use crate::types::*;

/// Utility for creating a resource
pub struct ResourceBuilder {
    metadata: HashMap<String, String>,
}

impl ResourceBuilder {
    /// Starts a new builder.
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata attribute.
    pub fn metadata<K: AsRef<str>, V: AsRef<str>>(&mut self, name: K, value: V) -> &mut Self {
        self.metadata
            .insert(name.as_ref().to_owned(), value.as_ref().to_owned());
        self
    }

    /// Creates a token resource with mutable supply.
    pub fn new_token_mutable<A: Into<ResourceDef>>(&self, minter: A) -> ResourceDef {
        ResourceDef::new_mutable(1, self.metadata.clone(), minter)
    }

    /// Creates a token resource with fixed supply.
    pub fn new_token_fixed<T: Into<Decimal>>(&self, supply: T) -> Bucket {
        ResourceDef::new_fixed(1, self.metadata.clone(), supply.into()).1
    }

    /// Creates a badge resource with mutable supply.
    pub fn new_badge_mutable<A: Into<ResourceDef>>(&self, minter: A) -> ResourceDef {
        ResourceDef::new_mutable(19, self.metadata.clone(), minter)
    }

    /// Creates a badge resource with fixed supply.
    pub fn new_badge_fixed<T: Into<Decimal>>(&self, supply: T) -> Bucket {
        ResourceDef::new_fixed(19, self.metadata.clone(), supply.into()).1
    }
}

impl Default for ResourceBuilder {
    fn default() -> Self {
        Self::new()
    }
}
