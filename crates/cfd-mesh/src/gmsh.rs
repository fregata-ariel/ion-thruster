//! Gmsh `.msh` file reader (MSH 2.2 ASCII format).
//!
//! Reads nodes, elements, and physical names from Gmsh mesh files
//! and converts them into the internal `Mesh` representation.
//!
//! We implement a simple parser for MSH 2.2 (ASCII) because it's the most
//! widely supported format. MSH 4.x support can be added later via the `mshio` crate.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use cfd_core::{CfdError, Vec3};

use crate::topology::{self, RawBoundaryFace, RawElement};
use crate::{CellType, Mesh};

/// Read a Gmsh `.msh` file (MSH 2.2 ASCII format) and build a `Mesh`.
pub fn read_msh(path: &Path) -> Result<Mesh, CfdError> {
    let content =
        fs::read_to_string(path).map_err(|e| CfdError::Mesh(format!("Cannot read {path:?}: {e}")))?;
    parse_msh2(&content)
}

/// Parse MSH 2.2 ASCII content.
fn parse_msh2(content: &str) -> Result<Mesh, CfdError> {
    let mut lines = content.lines().peekable();

    let mut nodes: Vec<Vec3> = Vec::new();
    let mut physical_names: HashMap<(i32, i32), String> = HashMap::new(); // (dim, tag) -> name
    let mut volume_elements: Vec<RawElement> = Vec::new();
    let mut boundary_faces: Vec<RawBoundaryFace> = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        match line {
            "$MeshFormat" => {
                // Read format line, verify it's 2.x ASCII
                if let Some(fmt_line) = lines.next() {
                    let parts: Vec<&str> = fmt_line.trim().split_whitespace().collect();
                    if parts.len() >= 2 {
                        let version: f64 = parts[0]
                            .parse()
                            .map_err(|_| CfdError::Mesh("Invalid format version".into()))?;
                        let file_type: i32 = parts[1]
                            .parse()
                            .map_err(|_| CfdError::Mesh("Invalid file type".into()))?;
                        if !(2.0..3.0).contains(&version) {
                            return Err(CfdError::Mesh(format!(
                                "Unsupported MSH version {version}. Expected 2.x"
                            )));
                        }
                        if file_type != 0 {
                            return Err(CfdError::Mesh(
                                "Only ASCII MSH format is supported".into(),
                            ));
                        }
                    }
                }
                skip_to_end(&mut lines, "$EndMeshFormat");
            }

            "$PhysicalNames" => {
                if let Some(count_line) = lines.next() {
                    let count: usize = count_line
                        .trim()
                        .parse()
                        .map_err(|_| CfdError::Mesh("Invalid physical name count".into()))?;
                    for _ in 0..count {
                        if let Some(pn_line) = lines.next() {
                            let parts: Vec<&str> = pn_line.trim().split_whitespace().collect();
                            if parts.len() >= 3 {
                                let dim: i32 = parts[0].parse().unwrap_or(0);
                                let tag: i32 = parts[1].parse().unwrap_or(0);
                                let name = parts[2].trim_matches('"').to_string();
                                physical_names.insert((dim, tag), name);
                            }
                        }
                    }
                }
                skip_to_end(&mut lines, "$EndPhysicalNames");
            }

            "$Nodes" => {
                if let Some(count_line) = lines.next() {
                    let count: usize = count_line
                        .trim()
                        .parse()
                        .map_err(|_| CfdError::Mesh("Invalid node count".into()))?;
                    // Gmsh node indices start at 1; we store 0-based
                    let mut node_map: Vec<Vec3> = vec![[0.0; 3]; count];
                    for _ in 0..count {
                        if let Some(node_line) = lines.next() {
                            let parts: Vec<&str> = node_line.trim().split_whitespace().collect();
                            if parts.len() >= 4 {
                                let _idx: usize = parts[0].parse().unwrap_or(1);
                                let x: f64 = parts[1].parse().unwrap_or(0.0);
                                let y: f64 = parts[2].parse().unwrap_or(0.0);
                                let z: f64 = parts[3].parse().unwrap_or(0.0);
                                // Gmsh indices are 1-based; store as 0-based
                                if _idx >= 1 && _idx <= count {
                                    node_map[_idx - 1] = [x, y, z];
                                }
                            }
                        }
                    }
                    nodes = node_map;
                }
                skip_to_end(&mut lines, "$EndNodes");
            }

            "$Elements" => {
                if let Some(count_line) = lines.next() {
                    let count: usize = count_line
                        .trim()
                        .parse()
                        .map_err(|_| CfdError::Mesh("Invalid element count".into()))?;
                    for _ in 0..count {
                        if let Some(elem_line) = lines.next() {
                            parse_element(
                                elem_line.trim(),
                                &physical_names,
                                &mut volume_elements,
                                &mut boundary_faces,
                            );
                        }
                    }
                }
                skip_to_end(&mut lines, "$EndElements");
            }

            _ => {
                // Skip unknown sections
                if line.starts_with('$') && !line.starts_with("$End") {
                    let end_tag = format!("$End{}", &line[1..]);
                    skip_to_end(&mut lines, &end_tag);
                }
            }
        }
    }

    tracing::info!(
        n_nodes = nodes.len(),
        n_volume_elements = volume_elements.len(),
        n_boundary_faces = boundary_faces.len(),
        "Parsed Gmsh MSH 2.2 file"
    );

    topology::build_mesh(nodes, &volume_elements, &boundary_faces)
}

/// Parse a single element line from MSH 2.2.
///
/// Format: `elem-id elem-type n-tags <tags...> <node-indices...>`
fn parse_element(
    line: &str,
    physical_names: &HashMap<(i32, i32), String>,
    volume_elements: &mut Vec<RawElement>,
    boundary_faces: &mut Vec<RawBoundaryFace>,
) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        return;
    }

    let _elem_id: usize = parts[0].parse().unwrap_or(0);
    let elem_type: i32 = parts[1].parse().unwrap_or(0);
    let n_tags: usize = parts[2].parse().unwrap_or(0);

    let physical_tag = if n_tags > 0 {
        parts.get(3).and_then(|s| s.parse::<i32>().ok())
    } else {
        None
    };

    let node_start = 3 + n_tags;
    let node_indices: Vec<u32> = parts[node_start..]
        .iter()
        .filter_map(|s| s.parse::<u32>().ok())
        .map(|idx| idx - 1) // Convert from 1-based to 0-based
        .collect();

    match elem_type {
        // 1D elements (lines) — used as boundary edges in 2D
        1 => {
            if let Some(tag) = physical_tag {
                let dim = 1;
                let name = physical_names
                    .get(&(dim, tag))
                    .cloned()
                    .unwrap_or_else(|| format!("boundary_{tag}"));
                boundary_faces.push(RawBoundaryFace {
                    node_indices,
                    physical_tag: tag,
                    physical_name: name,
                });
            }
        }
        // 2D elements: triangles and quads
        2 => {
            // Triangle — could be surface element (boundary in 3D) or volume element (2D mesh)
            if node_indices.len() == 3 {
                // Check if it's a boundary face (physical dim = 2 in 3D)
                // or a volume element (physical dim = 2 in 2D).
                // Heuristic: if physical_names has this tag with dim=2, treat as volume
                // in a 2D simulation. Otherwise treat as boundary face in 3D.
                let is_volume = physical_tag
                    .map(|tag| physical_names.contains_key(&(2, tag)))
                    .unwrap_or(false);

                if is_volume {
                    volume_elements.push(RawElement {
                        cell_type: CellType::Triangle,
                        node_indices,
                        physical_tag,
                    });
                } else if let Some(tag) = physical_tag {
                    let name = physical_names
                        .get(&(2, tag))
                        .cloned()
                        .unwrap_or_else(|| format!("boundary_{tag}"));
                    boundary_faces.push(RawBoundaryFace {
                        node_indices,
                        physical_tag: tag,
                        physical_name: name,
                    });
                }
            }
        }
        3 => {
            // Quad (2D volume or 3D boundary)
            if node_indices.len() == 4 {
                let is_volume = physical_tag
                    .map(|tag| physical_names.contains_key(&(2, tag)))
                    .unwrap_or(false);
                if is_volume {
                    volume_elements.push(RawElement {
                        cell_type: CellType::Quad,
                        node_indices,
                        physical_tag,
                    });
                } else if let Some(tag) = physical_tag {
                    let name = physical_names
                        .get(&(2, tag))
                        .cloned()
                        .unwrap_or_else(|| format!("boundary_{tag}"));
                    boundary_faces.push(RawBoundaryFace {
                        node_indices,
                        physical_tag: tag,
                        physical_name: name,
                    });
                }
            }
        }
        // 3D elements
        4 => {
            // Tetrahedron
            volume_elements.push(RawElement {
                cell_type: CellType::Tetrahedron,
                node_indices,
                physical_tag,
            });
        }
        5 => {
            // Hexahedron
            volume_elements.push(RawElement {
                cell_type: CellType::Hexahedron,
                node_indices,
                physical_tag,
            });
        }
        6 => {
            // Wedge / Prism
            volume_elements.push(RawElement {
                cell_type: CellType::Wedge,
                node_indices,
                physical_tag,
            });
        }
        7 => {
            // Pyramid
            volume_elements.push(RawElement {
                cell_type: CellType::Pyramid,
                node_indices,
                physical_tag,
            });
        }
        15 => {
            // Point — ignore
        }
        _ => {
            tracing::trace!(elem_type, "Skipping unsupported element type");
        }
    }
}

fn skip_to_end<'a, I: Iterator<Item = &'a str>>(lines: &mut I, end_tag: &str) {
    for line in lines.by_ref() {
        if line.trim() == end_tag {
            return;
        }
    }
}
