//! Mesh topology construction from element connectivity.
//!
//! Builds the face-based FVM topology from raw element data:
//! - Identifies shared faces between cells (internal faces)
//! - Identifies boundary faces (faces with only one adjacent cell)
//! - Orders faces: internal first, then boundary grouped by patch
//! - Builds CSR-compressed cell-to-face and face-to-node connectivity

use std::collections::HashMap;

use cfd_core::{CellId, FaceId, NodeId, Vec3};

use crate::geometry;
use crate::{BoundaryPatch, CellType, Mesh};

/// Raw element data from a mesh file reader.
#[derive(Clone, Debug)]
pub struct RawElement {
    pub cell_type: CellType,
    pub node_indices: Vec<u32>,
    /// Physical group tag (for boundary identification).
    pub physical_tag: Option<i32>,
}

/// Raw boundary face from a mesh file reader (surface elements).
#[derive(Clone, Debug)]
pub struct RawBoundaryFace {
    pub node_indices: Vec<u32>,
    pub physical_tag: i32,
    pub physical_name: String,
}

/// Build a complete `Mesh` from raw node coordinates and element connectivity.
///
/// This is the main topology construction entry point. It:
/// 1. Extracts faces from each element's local connectivity
/// 2. Identifies internal faces (shared between two cells)
/// 3. Identifies boundary faces and groups them into patches
/// 4. Computes geometric quantities
pub fn build_mesh(
    node_coords: Vec<Vec3>,
    volume_elements: &[RawElement],
    boundary_faces: &[RawBoundaryFace],
) -> Result<Mesh, cfd_core::CfdError> {
    let n_nodes = node_coords.len();
    let n_cells = volume_elements.len();

    // --- Step 1: Extract faces from elements and find internal/boundary ---
    // A face is identified by its sorted node set.
    // If two elements share a face, it's internal. Otherwise it's boundary.

    // Map from sorted face nodes → (owner_cell, face_nodes_ordered)
    // If a second cell claims the same face, it becomes internal.
    #[derive(Debug)]
    struct FaceInfo {
        nodes: Vec<NodeId>, // ordered (not sorted)
        owner: CellId,
        neighbor: Option<CellId>,
    }

    let mut face_map: HashMap<Vec<u32>, usize> = HashMap::new();
    let mut all_faces: Vec<FaceInfo> = Vec::new();

    // Cell-to-face connectivity (temporary, dense)
    let mut cell_faces_tmp: Vec<Vec<FaceId>> = vec![Vec::new(); n_cells];

    for (ci, elem) in volume_elements.iter().enumerate() {
        let cell_id = CellId::from(ci);
        let local_faces = element_faces(elem);

        for face_nodes in local_faces {
            let mut sorted = face_nodes.iter().map(|n| n.0).collect::<Vec<_>>();
            sorted.sort();

            if let Some(&face_idx) = face_map.get(&sorted) {
                // This face already exists — it's internal
                all_faces[face_idx].neighbor = Some(cell_id);
                cell_faces_tmp[ci].push(FaceId::from(face_idx));
            } else {
                let face_idx = all_faces.len();
                face_map.insert(sorted, face_idx);
                all_faces.push(FaceInfo {
                    nodes: face_nodes,
                    owner: cell_id,
                    neighbor: None,
                });
                cell_faces_tmp[ci].push(FaceId::from(face_idx));
            }
        }
    }

    // --- Step 2: Separate internal and boundary faces, reorder ---
    // Internal faces first, then boundary faces grouped by patch.

    let mut internal_face_indices: Vec<usize> = Vec::new();
    let mut boundary_face_indices: Vec<usize> = Vec::new();

    for (i, face) in all_faces.iter().enumerate() {
        if face.neighbor.is_some() {
            internal_face_indices.push(i);
        } else {
            boundary_face_indices.push(i);
        }
    }

    // Build boundary name lookup from raw boundary faces
    let mut boundary_node_to_name: HashMap<Vec<u32>, String> = HashMap::new();
    for bf in boundary_faces {
        let mut sorted = bf.node_indices.clone();
        sorted.sort();
        boundary_node_to_name.insert(sorted, bf.physical_name.clone());
    }

    // Group boundary faces by patch name
    let mut patch_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for &bi in &boundary_face_indices {
        let mut sorted: Vec<u32> = all_faces[bi].nodes.iter().map(|n| n.0).collect();
        sorted.sort();
        let name = boundary_node_to_name
            .get(&sorted)
            .cloned()
            .unwrap_or_else(|| "default".to_string());
        patch_groups.entry(name).or_default().push(bi);
    }

    // Build the reordering: internal faces, then boundary patches
    let n_internal_faces = internal_face_indices.len();
    let mut ordered_indices: Vec<usize> = internal_face_indices;

    let mut patches: Vec<BoundaryPatch> = Vec::new();
    let mut boundary_offset = 0;

    let mut patch_names: Vec<String> = patch_groups.keys().cloned().collect();
    patch_names.sort(); // deterministic ordering

    for name in &patch_names {
        let faces = &patch_groups[name];
        patches.push(BoundaryPatch {
            name: name.clone(),
            start_face: boundary_offset,
            n_faces: faces.len(),
        });
        ordered_indices.extend(faces);
        boundary_offset += faces.len();
    }

    let n_faces = ordered_indices.len();

    // Build old→new face index mapping
    let mut old_to_new = vec![0usize; all_faces.len()];
    for (new_idx, &old_idx) in ordered_indices.iter().enumerate() {
        old_to_new[old_idx] = new_idx;
    }

    // --- Step 3: Build final arrays ---

    let mut face_owner = Vec::with_capacity(n_faces);
    let mut face_neighbor = Vec::with_capacity(n_internal_faces);
    let mut face_node_offsets: Vec<u32> = vec![0];
    let mut face_node_indices: Vec<NodeId> = Vec::new();

    for &old_idx in &ordered_indices {
        let fi = &all_faces[old_idx];
        face_owner.push(fi.owner);
        if let Some(nb) = fi.neighbor {
            face_neighbor.push(nb);
        }
        for &node in &fi.nodes {
            face_node_indices.push(node);
        }
        face_node_offsets.push(face_node_indices.len() as u32);
    }

    // Cell-to-face with remapped indices
    let mut cell_face_offsets: Vec<u32> = vec![0];
    let mut cell_face_indices_flat: Vec<FaceId> = Vec::new();

    for cell_faces in &cell_faces_tmp {
        for &old_face_id in cell_faces {
            let new_idx = old_to_new[old_face_id.as_usize()];
            cell_face_indices_flat.push(FaceId::from(new_idx));
        }
        cell_face_offsets.push(cell_face_indices_flat.len() as u32);
    }

    // Cell-to-node connectivity
    let mut cell_node_offsets: Vec<u32> = vec![0];
    let mut cell_node_indices_flat: Vec<NodeId> = Vec::new();
    let mut cell_types = Vec::with_capacity(n_cells);

    for elem in volume_elements {
        for &ni in &elem.node_indices {
            cell_node_indices_flat.push(NodeId(ni));
        }
        cell_node_offsets.push(cell_node_indices_flat.len() as u32);
        cell_types.push(elem.cell_type);
    }

    // --- Step 4: Compute geometry ---

    // Face geometry
    let mut face_areas = Vec::with_capacity(n_faces);
    let mut face_normals = Vec::with_capacity(n_faces);
    let mut face_centroids = Vec::with_capacity(n_faces);

    for fi in 0..n_faces {
        let start = face_node_offsets[fi] as usize;
        let end = face_node_offsets[fi + 1] as usize;
        let face_nodes: Vec<Vec3> = face_node_indices[start..end]
            .iter()
            .map(|ni| node_coords[ni.as_usize()])
            .collect();

        let (area, normal) = geometry::polygon_area_normal(&face_nodes);
        let centroid = geometry::polygon_centroid(&face_nodes);

        face_areas.push(area);
        face_normals.push(normal);
        face_centroids.push(centroid);
    }

    // Cell geometry (decompose into tetrahedra from centroid)
    let mut cell_volumes = vec![0.0; n_cells];
    let mut cell_centroids = vec![[0.0, 0.0, 0.0]; n_cells];

    for ci in 0..n_cells {
        let start = cell_node_offsets[ci] as usize;
        let end = cell_node_offsets[ci + 1] as usize;
        let _cell_nodes: Vec<usize> = cell_node_indices_flat[start..end]
            .iter()
            .map(|ni| ni.as_usize())
            .collect();

        // Get the faces for this cell
        let cf_start = cell_face_offsets[ci] as usize;
        let cf_end = cell_face_offsets[ci + 1] as usize;
        let cell_face_list = &cell_face_indices_flat[cf_start..cf_end];

        // Build face node lists for polyhedron volume computation
        let mut face_node_lists: Vec<Vec<usize>> = Vec::new();
        for &fi in cell_face_list {
            let fs = face_node_offsets[fi.as_usize()] as usize;
            let fe = face_node_offsets[fi.as_usize() + 1] as usize;
            let nodes: Vec<usize> = face_node_indices[fs..fe]
                .iter()
                .map(|ni| ni.as_usize())
                .collect();
            face_node_lists.push(nodes);
        }

        let faces_ref: Vec<&[usize]> = face_node_lists.iter().map(|v| v.as_slice()).collect();
        let (vol, cen) = geometry::polyhedron_volume_centroid(&node_coords, &faces_ref);

        cell_volumes[ci] = vol;
        cell_centroids[ci] = cen;
    }

    // Inter-cell geometry (delta vectors, weights) for internal faces
    let mut face_delta = Vec::with_capacity(n_internal_faces);
    let mut face_delta_mag = Vec::with_capacity(n_internal_faces);
    let mut face_weight = Vec::with_capacity(n_internal_faces);

    use cfd_core::vec3::ops as v;

    for fi in 0..n_internal_faces {
        let owner = face_owner[fi].as_usize();
        let neighbor = face_neighbor[fi].as_usize();

        let delta = v::sub(cell_centroids[neighbor], cell_centroids[owner]);
        let mag = v::magnitude(delta);
        face_delta.push(delta);
        face_delta_mag.push(mag);

        // Weight based on distance: w = |face - owner| / |neighbor - owner|
        if mag > 1e-30 {
            let d_of = v::distance(face_centroids[fi], cell_centroids[owner]);
            let w = 1.0 - d_of / mag;
            face_weight.push(w.clamp(0.0, 1.0));
        } else {
            face_weight.push(0.5);
        }
    }

    tracing::info!(
        n_nodes,
        n_cells,
        n_faces,
        n_internal_faces,
        n_boundary_patches = patches.len(),
        "Mesh topology built"
    );

    Ok(Mesh {
        n_nodes,
        n_cells,
        n_faces,
        n_internal_faces,
        node_coords,
        cell_volumes,
        cell_centroids,
        cell_types,
        cell_face_offsets,
        cell_face_indices: cell_face_indices_flat,
        cell_node_offsets,
        cell_node_indices: cell_node_indices_flat,
        face_areas,
        face_normals,
        face_centroids,
        face_owner,
        face_neighbor,
        face_node_offsets,
        face_node_indices,
        boundary_patches: patches,
        face_delta,
        face_delta_mag,
        face_weight,
    })
}

/// Extract local face connectivity from an element's node list.
///
/// Returns a list of faces, each face being a list of `NodeId` in winding order.
fn element_faces(elem: &RawElement) -> Vec<Vec<NodeId>> {
    let ni = &elem.node_indices;
    match elem.cell_type {
        CellType::Triangle => {
            // 2D triangle: 3 edges as "faces"
            vec![
                vec![NodeId(ni[0]), NodeId(ni[1])],
                vec![NodeId(ni[1]), NodeId(ni[2])],
                vec![NodeId(ni[2]), NodeId(ni[0])],
            ]
        }
        CellType::Quad => {
            vec![
                vec![NodeId(ni[0]), NodeId(ni[1])],
                vec![NodeId(ni[1]), NodeId(ni[2])],
                vec![NodeId(ni[2]), NodeId(ni[3])],
                vec![NodeId(ni[3]), NodeId(ni[0])],
            ]
        }
        CellType::Tetrahedron => {
            // 4 triangular faces (outward-facing convention)
            vec![
                vec![NodeId(ni[0]), NodeId(ni[2]), NodeId(ni[1])],
                vec![NodeId(ni[0]), NodeId(ni[1]), NodeId(ni[3])],
                vec![NodeId(ni[1]), NodeId(ni[2]), NodeId(ni[3])],
                vec![NodeId(ni[0]), NodeId(ni[3]), NodeId(ni[2])],
            ]
        }
        CellType::Hexahedron => {
            // 6 quad faces
            vec![
                vec![NodeId(ni[0]), NodeId(ni[3]), NodeId(ni[2]), NodeId(ni[1])],
                vec![NodeId(ni[4]), NodeId(ni[5]), NodeId(ni[6]), NodeId(ni[7])],
                vec![NodeId(ni[0]), NodeId(ni[1]), NodeId(ni[5]), NodeId(ni[4])],
                vec![NodeId(ni[2]), NodeId(ni[3]), NodeId(ni[7]), NodeId(ni[6])],
                vec![NodeId(ni[0]), NodeId(ni[4]), NodeId(ni[7]), NodeId(ni[3])],
                vec![NodeId(ni[1]), NodeId(ni[2]), NodeId(ni[6]), NodeId(ni[5])],
            ]
        }
        CellType::Wedge => {
            // 2 triangular + 3 quad faces
            vec![
                vec![NodeId(ni[0]), NodeId(ni[2]), NodeId(ni[1])],
                vec![NodeId(ni[3]), NodeId(ni[4]), NodeId(ni[5])],
                vec![NodeId(ni[0]), NodeId(ni[1]), NodeId(ni[4]), NodeId(ni[3])],
                vec![NodeId(ni[1]), NodeId(ni[2]), NodeId(ni[5]), NodeId(ni[4])],
                vec![NodeId(ni[0]), NodeId(ni[3]), NodeId(ni[5]), NodeId(ni[2])],
            ]
        }
        CellType::Pyramid => {
            // 1 quad base + 4 triangular sides
            vec![
                vec![NodeId(ni[0]), NodeId(ni[3]), NodeId(ni[2]), NodeId(ni[1])],
                vec![NodeId(ni[0]), NodeId(ni[1]), NodeId(ni[4])],
                vec![NodeId(ni[1]), NodeId(ni[2]), NodeId(ni[4])],
                vec![NodeId(ni[2]), NodeId(ni[3]), NodeId(ni[4])],
                vec![NodeId(ni[3]), NodeId(ni[0]), NodeId(ni[4])],
            ]
        }
    }
}
