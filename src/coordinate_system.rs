// TODO: Look into f64?
type CoordType = f32;

pub struct SphericalCoordinates {
    pub radius: CoordType,
    pub theta: CoordType,
    pub phi: CoordType,
}

pub fn spherical_to_cartesian(coords: &SphericalCoordinates) -> [CoordType; 3] {
    [
        coords.radius * CoordType::sin(coords.theta) * CoordType::cos(coords.phi),
        coords.radius * CoordType::sin(coords.theta) * CoordType::sin(coords.phi),
        coords.radius * CoordType::cos(coords.theta),
    ]
}
