use std::rc::Rc;

use libraytracer::{
    cpu::{
        shape::Shape,
        top_level_acceleration_structure::{Instance, TopLevelAccelerationStructure},
    },
    types::{HdrColor, Mat4},
};
use tracey_utils::{
    material::{Lambertian, Material},
    texture::UniformColorTexture,
};

#[derive(Default)]
pub struct Scene {
    pub shapes: Vec<Rc<dyn Shape>>,
    pub instances: Vec<Instance>,
    pub materials: Vec<Rc<dyn Material>>,
    pub instance_to_mesh: Vec<usize>,
    pub instance_to_material: Vec<usize>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            instances: Vec::new(),
            materials: vec![Rc::new(Lambertian::new(Rc::new(UniformColorTexture::new(
                HdrColor::new(0.5, 0.5, 0.5, 1.0),
            ))))],
            instance_to_mesh: Vec::new(),
            instance_to_material: Vec::new(),
        }
    }

    pub fn build(&mut self) -> TopLevelAccelerationStructure {
        TopLevelAccelerationStructure::new(&self.instances)
    }

    pub fn add_shape(&mut self, shape: Rc<dyn Shape>) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn create_instance(&mut self, object_id: usize, transform: Mat4) -> usize {
        let instance_id = self.instances.len();
        let instance = Instance::new(
            self.shapes[object_id].clone(),
            self.instances.len() as u32,
            transform,
        );
        self.instances.push(instance);
        self.instance_to_mesh.push(object_id);
        self.instance_to_material.push(0);
        instance_id
    }

    fn shape_id_from_instance(&self, instance_id: usize) -> usize {
        self.instance_to_mesh[instance_id]
    }

    pub fn shape_from_instance(&self, instance_id: usize) -> &dyn Shape {
        let shape_id = self.shape_id_from_instance(instance_id);
        self.shape(shape_id)
    }

    pub fn shape(&self, shape_id: usize) -> &dyn Shape {
        self.shapes[shape_id].as_ref()
    }

    pub fn add_material(&mut self, material: Rc<dyn Material>) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn set_material(&mut self, instance_id: usize, material_id: usize) {
        self.instance_to_material[instance_id] = material_id;
    }

    pub fn material(&self, instance_id: usize) -> &dyn Material {
        let material_id = self.instance_to_material[instance_id];
        self.materials[material_id].as_ref()
    }
}
