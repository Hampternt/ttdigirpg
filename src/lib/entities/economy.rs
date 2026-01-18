//! Resource management and transaction tracking system.
//!
//! This module defines entities for tracking resources, their values, and the
//! interactions/transactions between different game elements. It's not about
//! traditional currency, but rather a system for associating and tracking
//! any valued resources and their relationships.

use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Represents a tracked resource or element in the game's resource management system.
///
/// An `EconomicEntity` is not about money or traditional economy, but rather
/// serves as a way to track any game resource (items, abilities, story elements)
/// along with their associated values and the transactions/interactions between them.
///
/// Each entity has a unique ID and can be serialized for storage or transmission.
///
/// # Fields
///
/// * `id` - Unique identifier for this entity
/// * `name` - The name/label of this resource
/// * `value` - The quantitative value associated with this resource
#[derive(Serialize, Deserialize)]
pub struct EconomicEntity {
    /// Unique identifier for this economic entity
    pub id: EntityId,
    /// Name of the resource
    pub name: String,
    /// Quantitative value associated with this resource
    pub value: i32,
}

// impl _EconomicEntity {
// }

/// A unique identifier for economic entities using UUIDs.
///
/// This newtype wrapper around `Uuid` provides a strongly-typed ID for
/// `EconomicEntity` instances. It uses UUID v4 for random, globally unique
/// identifiers and supports serialization via serde.
///
/// # Examples
///
/// ```
/// use ttdigirpg::entities::economy::EntityId;
///
/// let id1 = EntityId::new();
/// let id2 = EntityId::new();
/// // id1 and id2 will have different unique values
/// ```
#[derive(Serialize, Deserialize)]
pub struct EntityId(Uuid);

impl EntityId {
    /// Creates a new random UUID-based entity identifier.
    ///
    /// Uses UUID version 4 (random) to generate a unique identifier.
    ///
    /// # Returns
    ///
    /// A new `EntityId` containing a freshly generated UUID.
    pub fn new() -> Self {
        EntityId(Uuid::new_v4())
    }
}



