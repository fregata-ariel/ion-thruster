//! Newtype indices for type-safe mesh element addressing.
//!
//! Using newtypes prevents accidentally mixing cell, face, and node indices.
//! Internally `u32` to save memory in large meshes (supports up to ~4 billion elements).

use std::fmt;

macro_rules! define_index {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name(pub u32);

        impl $name {
            #[inline]
            pub fn as_usize(self) -> usize {
                self.0 as usize
            }
        }

        impl From<usize> for $name {
            #[inline]
            fn from(v: usize) -> Self {
                debug_assert!(v <= u32::MAX as usize, "Index overflow: {v}");
                Self(v as u32)
            }
        }

        impl From<$name> for usize {
            #[inline]
            fn from(id: $name) -> usize {
                id.0 as usize
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

define_index!(CellId, "Index into cell arrays.");
define_index!(FaceId, "Index into face arrays.");
define_index!(NodeId, "Index into node/vertex arrays.");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_id_roundtrip() {
        let id = CellId::from(42usize);
        assert_eq!(id.as_usize(), 42);
        let back: usize = id.into();
        assert_eq!(back, 42);
    }

    #[test]
    fn test_type_safety() {
        // These are different types — can't be mixed accidentally
        let _cell = CellId(0);
        let _face = FaceId(0);
        let _node = NodeId(0);
        // cell == face would not compile
    }
}
