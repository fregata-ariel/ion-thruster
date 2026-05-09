//! `cfd-fields` — Field storage, registry, and simulation state.
//!
//! Fields are contiguous arrays (`Vec<f64>` / `Vec<Vec3>`) indexed by cell/face/node ID.
//! The `FieldRegistry` provides a string-keyed map for inter-module data sharing.
//! Physics modules register fields by name and access them via the registry.

use std::collections::HashMap;

use cfd_core::{CfdError, FieldLocation, Vec3};

/// A cell/face/node-centered scalar field.
#[derive(Clone, Debug)]
pub struct ScalarField {
    pub values: Vec<f64>,
    pub location: FieldLocation,
}

/// A cell/face/node-centered vector field.
#[derive(Clone, Debug)]
pub struct VectorField {
    pub values: Vec<Vec3>,
    pub location: FieldLocation,
}

impl ScalarField {
    pub fn new(size: usize, location: FieldLocation) -> Self {
        Self {
            values: vec![0.0; size],
            location,
        }
    }

    pub fn filled(size: usize, value: f64, location: FieldLocation) -> Self {
        Self {
            values: vec![value; size],
            location,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl VectorField {
    pub fn new(size: usize, location: FieldLocation) -> Self {
        Self {
            values: vec![[0.0, 0.0, 0.0]; size],
            location,
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// Type-erased field storage.
#[derive(Clone, Debug)]
pub enum Field {
    Scalar(ScalarField),
    Vector(VectorField),
}

impl Field {
    pub fn location(&self) -> FieldLocation {
        match self {
            Field::Scalar(f) => f.location,
            Field::Vector(f) => f.location,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Field::Scalar(f) => f.len(),
            Field::Vector(f) => f.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Central field registry: physics modules register and access fields by name.
///
/// Field access by name goes through `HashMap` lookup, which is acceptable because
/// it happens once per timestep per field (not in inner loops). Inside hot loops,
/// steps work with raw `&[f64]` / `&[[f64; 3]]` slices obtained from the registry.
#[derive(Clone, Debug, Default)]
pub struct FieldRegistry {
    fields: HashMap<String, Field>,
}

impl FieldRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a scalar field. Overwrites if name already exists.
    pub fn register_scalar(
        &mut self,
        name: impl Into<String>,
        size: usize,
        location: FieldLocation,
    ) {
        self.fields
            .insert(name.into(), Field::Scalar(ScalarField::new(size, location)));
    }

    /// Register a vector field. Overwrites if name already exists.
    pub fn register_vector(
        &mut self,
        name: impl Into<String>,
        size: usize,
        location: FieldLocation,
    ) {
        self.fields
            .insert(name.into(), Field::Vector(VectorField::new(size, location)));
    }

    /// Insert a pre-built field.
    pub fn insert(&mut self, name: impl Into<String>, field: Field) {
        self.fields.insert(name.into(), field);
    }

    pub fn get(&self, name: &str) -> Option<&Field> {
        self.fields.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Field> {
        self.fields.get_mut(name)
    }

    pub fn get_scalar(&self, name: &str) -> Result<&ScalarField, CfdError> {
        match self.fields.get(name) {
            Some(Field::Scalar(f)) => Ok(f),
            Some(_) => Err(CfdError::Other(format!(
                "Field '{name}' is not a scalar field"
            ))),
            None => Err(CfdError::FieldNotFound {
                name: name.to_string(),
            }),
        }
    }

    pub fn get_scalar_mut(&mut self, name: &str) -> Result<&mut ScalarField, CfdError> {
        match self.fields.get_mut(name) {
            Some(Field::Scalar(f)) => Ok(f),
            Some(_) => Err(CfdError::Other(format!(
                "Field '{name}' is not a scalar field"
            ))),
            None => Err(CfdError::FieldNotFound {
                name: name.to_string(),
            }),
        }
    }

    pub fn get_vector(&self, name: &str) -> Result<&VectorField, CfdError> {
        match self.fields.get(name) {
            Some(Field::Vector(f)) => Ok(f),
            Some(_) => Err(CfdError::Other(format!(
                "Field '{name}' is not a vector field"
            ))),
            None => Err(CfdError::FieldNotFound {
                name: name.to_string(),
            }),
        }
    }

    pub fn get_vector_mut(&mut self, name: &str) -> Result<&mut VectorField, CfdError> {
        match self.fields.get_mut(name) {
            Some(Field::Vector(f)) => Ok(f),
            Some(_) => Err(CfdError::Other(format!(
                "Field '{name}' is not a vector field"
            ))),
            None => Err(CfdError::FieldNotFound {
                name: name.to_string(),
            }),
        }
    }

    /// Get two mutable scalar fields simultaneously (panics if same name).
    pub fn get_scalar_pair_mut(
        &mut self,
        name_a: &str,
        name_b: &str,
    ) -> Result<(&mut ScalarField, &mut ScalarField), CfdError> {
        assert_ne!(name_a, name_b, "Cannot borrow same field twice");
        // Safety: we checked the names are different, so they point to different entries
        let ptr = &mut self.fields as *mut HashMap<String, Field>;
        unsafe {
            let a = (*ptr)
                .get_mut(name_a)
                .ok_or_else(|| CfdError::FieldNotFound {
                    name: name_a.to_string(),
                })?;
            let b = (*ptr)
                .get_mut(name_b)
                .ok_or_else(|| CfdError::FieldNotFound {
                    name: name_b.to_string(),
                })?;
            match (a, b) {
                (Field::Scalar(a), Field::Scalar(b)) => Ok((a, b)),
                _ => Err(CfdError::Other(
                    "One of the fields is not a scalar".to_string(),
                )),
            }
        }
    }

    /// List all registered field names.
    pub fn names(&self) -> Vec<&str> {
        self.fields.keys().map(|s| s.as_str()).collect()
    }

    /// Check if a field with the given name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }
}

/// Complete simulation state at a point in time.
pub struct SimState {
    pub fields: FieldRegistry,
    pub time: f64,
    pub step: usize,
    pub dt: f64,
}

impl SimState {
    pub fn new() -> Self {
        Self {
            fields: FieldRegistry::new(),
            time: 0.0,
            step: 0,
            dt: 0.0,
        }
    }
}

impl Default for SimState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_access() {
        let mut reg = FieldRegistry::new();
        reg.register_scalar("pressure", 100, FieldLocation::Cell);
        reg.register_vector("velocity", 100, FieldLocation::Cell);

        let p = reg.get_scalar("pressure").unwrap();
        assert_eq!(p.len(), 100);
        assert_eq!(p.location, FieldLocation::Cell);

        let v = reg.get_vector("velocity").unwrap();
        assert_eq!(v.len(), 100);
    }

    #[test]
    fn test_missing_field() {
        let reg = FieldRegistry::new();
        assert!(reg.get_scalar("nonexistent").is_err());
    }
}
