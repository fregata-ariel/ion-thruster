// Gmsh .geo file: 2D wire-to-plate electrode configuration
// Wire (emitter) at top, plate (collector) at bottom

// Parameters
gap = 0.02;        // 20mm electrode gap
plate_width = 0.06; // 60mm plate width
wire_radius = 0.001; // 1mm wire radius
domain_height = 0.04; // 40mm total domain height

// Mesh sizes
lc_wire = 0.0005;   // Fine near wire (0.5mm)
lc_plate = 0.001;   // Fine near plate (1mm)
lc_far = 0.005;     // Coarse far field (5mm)

// --- Geometry ---

// Domain boundary
Point(1) = {-plate_width/2, 0, 0, lc_plate};
Point(2) = { plate_width/2, 0, 0, lc_plate};
Point(3) = { plate_width/2, domain_height, 0, lc_far};
Point(4) = {-plate_width/2, domain_height, 0, lc_far};

// Collector plate (bottom)
Line(1) = {1, 2};

// Right wall
Line(2) = {2, 3};

// Top (open)
Line(3) = {3, 4};

// Left wall
Line(4) = {4, 1};

// Wire (emitter) - circle at center top
wire_y = gap;
Point(5) = {0, wire_y, 0, lc_wire};
Point(6) = { wire_radius, wire_y, 0, lc_wire};
Point(7) = {0, wire_y + wire_radius, 0, lc_wire};
Point(8) = {-wire_radius, wire_y, 0, lc_wire};
Point(9) = {0, wire_y - wire_radius, 0, lc_wire};

Circle(5) = {6, 5, 7};
Circle(6) = {7, 5, 8};
Circle(7) = {8, 5, 9};
Circle(8) = {9, 5, 6};

// Curve loops
Curve Loop(1) = {1, 2, 3, 4};   // outer boundary
Curve Loop(2) = {5, 6, 7, 8};   // wire hole

// Surface (domain minus wire)
Plane Surface(1) = {1, 2};

// Physical groups
Physical Curve("collector") = {1};
Physical Curve("emitter") = {5, 6, 7, 8};
Physical Curve("farfield") = {2, 3, 4};
Physical Surface("fluid") = {1};
