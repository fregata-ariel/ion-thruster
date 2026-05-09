//! VTU (VTK XML Unstructured Grid) output for ParaView.
//!
//! Writes mesh geometry and field data in VTU format using the `vtkio` crate.
//! Also generates PVD collection files for time series animation.

use std::fs;
use std::path::PathBuf;

use vtkio::model::*;
use vtkio::IOBuffer;

use cfd_core::CfdError;
use cfd_fields::SimState;
use cfd_mesh::Mesh;
use cfd_time::FieldWriter;

/// VTU file writer with PVD time series support.
pub struct VtuWriter {
    output_dir: PathBuf,
    /// Names of scalar fields to output.
    scalar_fields: Vec<String>,
    /// Names of vector fields to output.
    vector_fields: Vec<String>,
    /// PVD entries: (timestep, time, filename).
    pvd_entries: Vec<(usize, f64, String)>,
}

impl VtuWriter {
    pub fn new(
        output_dir: impl Into<PathBuf>,
        scalar_fields: Vec<String>,
        vector_fields: Vec<String>,
    ) -> Self {
        let dir = output_dir.into();
        fs::create_dir_all(&dir).ok();
        Self {
            output_dir: dir,
            scalar_fields,
            vector_fields,
            pvd_entries: Vec::new(),
        }
    }

    /// Write a PVD collection file referencing all written frames.
    pub fn write_pvd(&self) -> Result<(), CfdError> {
        let pvd_path = self.output_dir.join("output.pvd");
        let mut content = String::new();
        content.push_str("<?xml version=\"1.0\"?>\n");
        content.push_str("<VTKFile type=\"Collection\" version=\"0.1\">\n");
        content.push_str("  <Collection>\n");

        for (_step, time, filename) in &self.pvd_entries {
            content.push_str(&format!(
                "    <DataSet timestep=\"{time}\" group=\"\" part=\"0\" file=\"{filename}\"/>\n"
            ));
        }

        content.push_str("  </Collection>\n");
        content.push_str("</VTKFile>\n");

        fs::write(&pvd_path, content)?;
        tracing::info!(?pvd_path, entries = self.pvd_entries.len(), "Wrote PVD file");
        Ok(())
    }
}

impl FieldWriter for VtuWriter {
    fn write_frame(
        &mut self,
        mesh: &Mesh,
        state: &SimState,
        step: usize,
    ) -> Result<(), CfdError> {
        let filename = format!("frame_{step:06}.vtu");
        let filepath = self.output_dir.join(&filename);

        let vtk_data = build_vtk(mesh, state, &self.scalar_fields, &self.vector_fields)?;

        vtk_data
            .export_be(&filepath)
            .map_err(|e| CfdError::Other(format!("VTU write error: {e}")))?;

        self.pvd_entries.push((step, state.time, filename));

        // Write PVD after each frame (so it's always up-to-date)
        self.write_pvd()?;

        Ok(())
    }
}

/// Build a VTK data structure from mesh and state.
fn build_vtk(
    mesh: &Mesh,
    state: &SimState,
    scalar_fields: &[String],
    vector_fields: &[String],
) -> Result<Vtk, CfdError> {
    // --- Points ---
    let points: Vec<f64> = mesh
        .node_coords
        .iter()
        .flat_map(|p| p.iter().copied())
        .collect();

    // --- Cells ---
    // VTK cell connectivity and types
    let mut connectivity: Vec<u64> = Vec::new();
    let mut offsets: Vec<u64> = Vec::new();
    let mut cell_types: Vec<CellType> = Vec::new();

    for ci in 0..mesh.n_cells {
        let start = mesh.cell_node_offsets[ci] as usize;
        let end = mesh.cell_node_offsets[ci + 1] as usize;
        let nodes = &mesh.cell_node_indices[start..end];

        for node in nodes {
            connectivity.push(node.0 as u64);
        }
        offsets.push(connectivity.len() as u64);

        let vtk_type = match mesh.cell_types[ci] {
            cfd_mesh::CellType::Triangle => CellType::Triangle,
            cfd_mesh::CellType::Quad => CellType::Quad,
            cfd_mesh::CellType::Tetrahedron => CellType::Tetra,
            cfd_mesh::CellType::Hexahedron => CellType::Hexahedron,
            cfd_mesh::CellType::Wedge => CellType::Wedge,
            cfd_mesh::CellType::Pyramid => CellType::Pyramid,
        };
        cell_types.push(vtk_type);
    }

    // --- Cell data (field values) ---
    let mut attributes: Vec<Attribute> = Vec::new();

    for name in scalar_fields {
        if let Ok(field) = state.fields.get_scalar(name) {
            attributes.push(Attribute::DataArray(DataArray {
                name: name.clone(),
                elem: ElementType::Scalars {
                    num_comp: 1,
                    lookup_table: None,
                },
                data: IOBuffer::F64(field.values.clone()),
            }));
        }
    }

    for name in vector_fields {
        if let Ok(field) = state.fields.get_vector(name) {
            let flat: Vec<f64> = field
                .values
                .iter()
                .flat_map(|v| v.iter().copied())
                .collect();
            attributes.push(Attribute::DataArray(DataArray {
                name: name.clone(),
                elem: ElementType::Vectors,
                data: IOBuffer::F64(flat),
            }));
        }
    }

    Ok(Vtk {
        version: Version::new((4, 2)),
        title: String::new(),
        byte_order: ByteOrder::BigEndian,
        file_path: None,
        data: DataSet::inline(UnstructuredGridPiece {
            points: IOBuffer::F64(points),
            cells: Cells {
                cell_verts: VertexNumbers::XML {
                    connectivity: connectivity.into_iter().map(|x| x as u64).collect(),
                    offsets: offsets.into_iter().map(|x| x as u64).collect(),
                },
                types: cell_types,
            },
            data: Attributes {
                point: Vec::new(),
                cell: attributes,
            },
        }),
    })
}
