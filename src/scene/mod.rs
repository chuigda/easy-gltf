mod camera;
mod light;
/// Contains model and material
/// # Usage
/// Check [Model](struct.Model.html) for more information about how to use this module.
pub mod model;

use crate::utils::transform_to_matrix;
use crate::GltfData;
pub use camera::{Camera, Projection};
pub use light::Light;
pub use model::{Material, Model};

use cgmath::*;
use gltf::scene::{Node, Transform};

/// Contains cameras, models and lights of a scene.
#[derive(Default, Clone, Debug)]
pub struct Scene {
    #[cfg(feature = "names")]
    /// Scene name. Requires the `names` feature.
    pub name: Option<String>,
    #[cfg(feature = "extras")]
    /// Scene extra data. Requires the `extras` feature.
    pub extras: gltf::json::extras::Extras,
    /// List of models in the scene
    pub models: Vec<Model>,
    /// List of cameras in the scene
    pub cameras: Vec<Camera>,
    /// List of lights in the scene
    pub lights: Vec<Light>,
}

impl Scene {
    pub(crate) fn load(gltf_scene: gltf::Scene, data: &mut GltfData) -> Self {
        let mut scene = Self::default();

        #[cfg(feature = "names")]
        {
            scene.name = gltf_scene.name().map(String::from);
        }
        #[cfg(feature = "extras")]
        {
            scene.extras = gltf_scene.extras().clone();
        }

        let mut transform = vec![];
        for node in gltf_scene.nodes() {
            scene.read_node(&node, &mut transform, &One::one(), data);
        }
        scene
    }

    fn read_node(
        &mut self,
        node: &Node,
        transform: &mut Vec<Transform>,
        transform_matrix: &Matrix4<f32>,
        data: &mut GltfData
    ) {
        transform.push(node.transform());
        // Compute transform of the current node
        let transform_matrix = transform_matrix * transform_to_matrix(node.transform());

        // Recurse on children
        for child in node.children() {
            self.read_node(&child, transform, &transform_matrix, data);
        }

        // Load camera
        if let Some(camera) = node.camera() {
            self.cameras.push(Camera::load(camera, &transform_matrix));
        }

        // Load light
        if let Some(light) = node.light() {
            self.lights.push(Light::load(light, &transform_matrix));
        }

        // Load model
        if let Some(mesh) = node.mesh() {
            for (i, primitive) in mesh.primitives().enumerate() {
                self.models
                    .push(Model::load(&mesh, i, primitive, transform.clone(), data));
            }
        }

        transform.pop();
    }
}
