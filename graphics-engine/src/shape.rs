use crate::constants;
use crate::form::Form;
use crate::types::{Vector4, XYTuple};
use web_sys::{WebGlRenderingContext};
use crate::program::Program;
use crate::utils;

pub type ShapeGenerator<'a> = &'a dyn Fn() -> Box<[f32]>;

#[derive(Copy, Clone)]
pub enum ShapeType<'a> {
    Rectangle { width: f32, height: f32 },
    Ellipse { radius: f32 },
    Polygon(&'a [f32]),
    Procedural(ShapeGenerator<'a>),
}

pub struct Shape<'a> {
    prototype: Form<'a>,
    shape_props: ShapeType<'a>,
    position: XYTuple,
    angle: f32,
    color: Vector4,
}

fn generate_circle_data<'a>(origin: &XYTuple) -> Box<[f32]> {
    let steps = (
        constants::max_radius * 2.0 * 
        std::f32::consts::PI / constants::mesh_compression_factor
    ) as i32;
    
    let mut buffer_data: Vec<f32> = vec![
        0.0; 
        (steps * constants::vertex_size * constants::triangle_verticies) as usize
    ];

    let mut i = 0_usize;
    let angle_step = 2.0 * std::f32::consts::PI / (steps as f32);
    let mut angle: f32 = 0.0;
    let radius = 1.0;
    for _ in 0..steps {
        // origin vertex
        buffer_data[i] = origin.x;
        buffer_data[i + 1] = origin.y;
        buffer_data[i + 2] = 1.0;
        buffer_data[i + 3] = 1.0;

        // current vertex
        buffer_data[i + 4] = origin.x + radius * angle.cos();
        buffer_data[i + 5] = origin.y - radius * angle.sin();
        buffer_data[i + 6] = 1.0;
        buffer_data[i + 7] = 1.0;

        // next vertex
        angle += angle_step;
        buffer_data[i + 8] = origin.x + radius * angle.cos();
        buffer_data[i + 9] = origin.y - radius * angle.sin();
        buffer_data[i + 10] = 1.0;
        buffer_data[i + 11] = 1.0;

        i += (constants::vertex_size * constants::triangle_verticies) as usize;
    }

    buffer_data.into_boxed_slice()
}

impl<'a> Shape<'a> {
    fn center_of_mesh(dot_components: &[f32]) -> XYTuple {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut i = 0;
        let dots_count = dot_components.len() / constants::vertex_size as usize;

        for _ in 0..dots_count {
            sum_x += dot_components[i];
            sum_y += dot_components[i + 1];
            i += constants::vertex_size as usize;
        }

        XYTuple {
            x: sum_x / dots_count as f32,
            y: sum_y / dots_count as f32,
        }
    }

    fn radius_of_mesh(dot_components: &[f32], center: &XYTuple) -> f32 {
        let mut radius = 0.0;
        let mut i = 0;
        let dots_count = dot_components.len() / constants::vertex_size as usize;

        for _ in 0..dots_count {
            let dx = dot_components[i] - center.x;
            let dy = dot_components[i + 1] - center.y;
            let next_radius = (dx * dx + dy * dy).sqrt();

            if next_radius > radius {
                radius = next_radius;
            }
        }

        radius
    }

    fn init_prototype(
        shape_props: &ShapeType<'a>, 
        gl: &WebGlRenderingContext,
        program: &'a Program
    ) -> Option<Form<'a>> {
        let dot_components = match shape_props {
            ShapeType::Polygon(dot_components) => Some(*dot_components),
            ShapeType::Procedural(procedure) => {
                let res = (procedure)();
                Some(res.as_ref())
            },
            _ => None
        };

        let center = {
            if let Some(dot_components) = dot_components {
                Self::center_of_mesh(dot_components)
            } else {
                XYTuple {
                    x: 0.0,
                    y: 0.0,
                }
            }
        };

        let radius = {
            if let Some(dot_components) = dot_components {
                Self::radius_of_mesh(dot_components, &center)
            } else {
                1.0
            }
        };

        

        let vertex_data = match (shape_props, dot_components) {
            (ShapeType::Rectangle { .. }, _) => Box::new(constants::rectangle_array),
            (ShapeType::Ellipse { .. }, _) => generate_circle_data(&center),
            // TODO: normalize data
            (_, Some(dot_components)) => Box::<[f32]>::from(dot_components),
        };

        Form::init(gl, vertex_data.as_ref(), program)
    }
    
    pub fn init(
        shape_props: ShapeType<'a>,
        gl: &WebGlRenderingContext,
        program: &Program
    ) -> Option<Self<'a>> {
        let prototype = try_unwrap!(
            @dev;
            Self::init_prototype(
                &shape_props, 
                gl, 
                program
            ),
            "Prototype can't be initialized"
        );





    }
}
