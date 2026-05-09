//! Geometry computation for mesh elements.
//!
//! Computes cell volumes, centroids, face areas, normals, and
//! inter-cell geometric quantities needed by FVM discretization.

use cfd_core::vec3::ops;
use cfd_core::Vec3;

/// Compute the area and outward normal of a triangular face.
///
/// Returns `(area, unit_normal)`.
pub fn triangle_area_normal(p0: Vec3, p1: Vec3, p2: Vec3) -> (f64, Vec3) {
    let e1 = ops::sub(p1, p0);
    let e2 = ops::sub(p2, p0);
    let cross = ops::cross(e1, e2);
    let area = ops::magnitude(cross) * 0.5;
    if area > 1e-30 {
        let normal = ops::scale(cross, 1.0 / (2.0 * area));
        (area, normal)
    } else {
        (0.0, [0.0, 0.0, 1.0])
    }
}

/// Compute the area and outward normal of a polygonal face (3+ nodes).
///
/// Uses the Newell method for general polygons.
/// Returns `(area, unit_normal)`.
pub fn polygon_area_normal(nodes: &[Vec3]) -> (f64, Vec3) {
    let n = nodes.len();
    assert!(n >= 3, "Face must have at least 3 nodes");

    if n == 3 {
        return triangle_area_normal(nodes[0], nodes[1], nodes[2]);
    }

    // Newell method: sum cross products of edge pairs
    let mut normal = ops::ZERO;
    for i in 0..n {
        let curr = nodes[i];
        let next = nodes[(i + 1) % n];
        normal[0] += (curr[1] - next[1]) * (curr[2] + next[2]);
        normal[1] += (curr[2] - next[2]) * (curr[0] + next[0]);
        normal[2] += (curr[0] - next[0]) * (curr[1] + next[1]);
    }
    let area = ops::magnitude(normal) * 0.5;
    if area > 1e-30 {
        let unit = ops::scale(normal, 1.0 / (2.0 * area));
        (area, unit)
    } else {
        (0.0, [0.0, 0.0, 1.0])
    }
}

/// Compute the centroid of a polygon (average of vertices).
pub fn polygon_centroid(nodes: &[Vec3]) -> Vec3 {
    let n = nodes.len() as f64;
    let mut c = ops::ZERO;
    for &node in nodes {
        ops::add_assign(&mut c, node);
    }
    ops::scale(c, 1.0 / n)
}

/// Compute the volume and centroid of a tetrahedron.
pub fn tetrahedron_volume_centroid(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> (f64, Vec3) {
    let e1 = ops::sub(p1, p0);
    let e2 = ops::sub(p2, p0);
    let e3 = ops::sub(p3, p0);
    let volume = ops::dot(e1, ops::cross(e2, e3)).abs() / 6.0;
    let centroid = ops::scale(
        ops::add(ops::add(p0, p1), ops::add(p2, p3)),
        0.25,
    );
    (volume, centroid)
}

/// Compute the volume and centroid of a general polyhedron by decomposing
/// into tetrahedra from the geometric center.
///
/// `face_nodes`: for each face, the list of node coordinates in order.
/// `face_normals_outward`: whether each face normal points outward.
pub fn polyhedron_volume_centroid(
    all_nodes: &[Vec3],
    faces: &[&[usize]], // each face is a list of node indices
) -> (f64, Vec3) {
    // Compute geometric center as reference point
    let mut center = ops::ZERO;
    let mut n_unique = 0;
    let mut seen = std::collections::HashSet::new();
    for face in faces {
        for &ni in *face {
            if seen.insert(ni) {
                ops::add_assign(&mut center, all_nodes[ni]);
                n_unique += 1;
            }
        }
    }
    center = ops::scale(center, 1.0 / n_unique as f64);

    let mut total_volume = 0.0;
    let mut weighted_centroid = ops::ZERO;

    // Decompose each face into triangles, form tetrahedra with center
    for face in faces {
        let nf = face.len();
        for i in 1..nf - 1 {
            let p0 = center;
            let p1 = all_nodes[face[0]];
            let p2 = all_nodes[face[i]];
            let p3 = all_nodes[face[i + 1]];

            let (vol, cen) = tetrahedron_volume_centroid(p0, p1, p2, p3);
            total_volume += vol;
            ops::add_scaled(&mut weighted_centroid, cen, vol);
        }
    }

    if total_volume > 1e-30 {
        let centroid = ops::scale(weighted_centroid, 1.0 / total_volume);
        (total_volume, centroid)
    } else {
        (0.0, center)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area() {
        let p0 = [0.0, 0.0, 0.0];
        let p1 = [1.0, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let (area, normal) = triangle_area_normal(p0, p1, p2);
        assert!((area - 0.5).abs() < 1e-12);
        assert!((normal[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_tet_volume() {
        let p0 = [0.0, 0.0, 0.0];
        let p1 = [1.0, 0.0, 0.0];
        let p2 = [0.0, 1.0, 0.0];
        let p3 = [0.0, 0.0, 1.0];
        let (vol, cen) = tetrahedron_volume_centroid(p0, p1, p2, p3);
        assert!((vol - 1.0 / 6.0).abs() < 1e-12);
        assert!((cen[0] - 0.25).abs() < 1e-12);
        assert!((cen[1] - 0.25).abs() < 1e-12);
        assert!((cen[2] - 0.25).abs() < 1e-12);
    }
}
