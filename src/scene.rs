use crate::cube::Cube;
use crate::material::Material;
use crate::vector::Vec3;

#[derive(Clone, Copy)]
pub struct SceneObject {
    pub cube: Cube,
    pub material: Material,
}

impl SceneObject {
    pub fn new(cube: Cube, material: Material) -> Self {
        Self { cube, material }
    }
}

fn add_monitor(objects: &mut Vec<SceneObject>) {
    objects.push(SceneObject::new(
        Cube::new(Vec3::new(-1.65, -0.35, -4.15), Vec3::new(1.65, 1.45, -3.75)),
        Material::plastic(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(-1.40, -0.10, -3.73), Vec3::new(1.40, 1.20, -3.65)),
        Material::screen(),
    ));

    objects.push(SceneObject::new(
        Cube::new(
            Vec3::new(-0.18, -0.65, -4.00),
            Vec3::new(0.18, -0.30, -3.82),
        ),
        Material::plastic(),
    ));

    objects.push(SceneObject::new(
        Cube::new(
            Vec3::new(-0.75, -0.72, -4.15),
            Vec3::new(0.75, -0.62, -3.65),
        ),
        Material::plastic(),
    ));
}

fn add_pc(objects: &mut Vec<SceneObject>) {
    objects.push(SceneObject::new(
        Cube::new(Vec3::new(2.05, -0.65, -4.35), Vec3::new(3.30, 1.55, -3.05)),
        Material::plastic(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(2.25, 0.85, -3.03), Vec3::new(3.10, 1.15, -2.98)),
        Material::server(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(2.25, 0.35, -3.03), Vec3::new(3.10, 0.60, -2.98)),
        Material::server(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(2.65, -0.25, -3.02), Vec3::new(2.82, -0.08, -2.96)),
        Material::green_led(),
    ));
}

fn add_keyboard(objects: &mut Vec<SceneObject>) {
    objects.push(SceneObject::new(
        Cube::new(
            Vec3::new(-1.70, -0.48, -2.65),
            Vec3::new(1.45, -0.32, -1.75),
        ),
        Material::plastic(),
    ));

    let start_x = -1.48;
    let start_z = -2.45;

    for row in 0..3 {
        for col in 0..10 {
            let x = start_x + col as f32 * 0.29;
            let z = start_z + row as f32 * 0.23;

            objects.push(SceneObject::new(
                Cube::new(Vec3::new(x, -0.30, z), Vec3::new(x + 0.22, -0.23, z + 0.16)),
                Material::plastic(),
            ));
        }
    }
}

fn add_rack(objects: &mut Vec<SceneObject>, min_x: f32, max_x: f32) {
    objects.push(SceneObject::new(
        Cube::new(Vec3::new(min_x, -1.0, -7.0), Vec3::new(max_x, 3.3, -5.7)),
        Material::dark_metal(),
    ));

    for row in 0..6 {
        let y = -0.65 + row as f32 * 0.58;

        objects.push(SceneObject::new(
            Cube::new(
                Vec3::new(min_x + 0.10, y, -5.68),
                Vec3::new(max_x - 0.10, y + 0.38, -5.60),
            ),
            Material::server(),
        ));

        objects.push(SceneObject::new(
            Cube::new(
                Vec3::new(max_x - 0.28, y + 0.12, -5.58),
                Vec3::new(max_x - 0.18, y + 0.22, -5.54),
            ),
            Material::blue_led(),
        ));
    }
}

fn add_lamp(objects: &mut Vec<SceneObject>) {
    objects.push(SceneObject::new(
        Cube::new(
            Vec3::new(-3.05, -0.58, -2.80),
            Vec3::new(-2.25, -0.48, -2.05),
        ),
        Material::dark_metal(),
    ));

    objects.push(SceneObject::new(
        Cube::new(
            Vec3::new(-2.72, -0.48, -2.48),
            Vec3::new(-2.58, 0.70, -2.34),
        ),
        Material::dark_metal(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(-2.68, 0.55, -2.42), Vec3::new(-2.05, 0.70, -2.28)),
        Material::dark_metal(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(-2.15, 0.35, -2.55), Vec3::new(-1.75, 0.72, -2.15)),
        Material::dark_metal(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(-2.08, 0.30, -2.48), Vec3::new(-1.82, 0.36, -2.22)),
        Material::screen(),
    ));
}

pub fn build_scene() -> Vec<SceneObject> {
    let mut objects = Vec::new();

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(-6.5, -1.25, -8.0), Vec3::new(6.5, -1.05, 2.0)),
        Material::floor(),
    ));

    objects.push(SceneObject::new(
        Cube::new(Vec3::new(-3.6, -0.90, -4.8), Vec3::new(3.6, -0.60, -1.35)),
        Material::wood(),
    ));

    add_monitor(&mut objects);
    add_pc(&mut objects);
    add_keyboard(&mut objects);
    add_lamp(&mut objects);

    add_rack(&mut objects, -5.0, -3.7);
    add_rack(&mut objects, 3.8, 5.1);

    objects
}
