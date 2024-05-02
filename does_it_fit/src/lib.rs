pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {

    let zone_rectangulaire = areas_volumes::rectangle_area(x, y);
    let area_of_object = match objects{
        areas_volumes::GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b),
        areas_volumes::GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b) as usize,
        areas_volumes::GeometricalShapes::Circle => areas_volumes::circle_area(a) as usize,
        areas_volumes::GeometricalShapes::Square => areas_volumes::square_area(a)
    };

    times * area_of_object <= zone_rectangulaire
}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {

    let volume_geom = areas_volumes::parallelepiped_volume(x, y, z);
    let  area_of_object = match objects{
        areas_volumes::GeometricalVolumes::Cube => areas_volumes::cube_volume(a),
        areas_volumes::GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b) as usize,
        areas_volumes::GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c),
        areas_volumes::GeometricalVolumes::Pyramid => areas_volumes::triangular_pyramid_volume(a as f64, b) as usize,
        areas_volumes::GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a ) as usize
    };
    times * area_of_object <= volume_geom
}