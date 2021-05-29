use crate::constants;
use crate::form::Form;
use crate::types::{Vector4, XYTuple};

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

    fn init_prototype(shape_props: &ShapeType<'a>) -> Form<'a> {
        let dot_components = match shape_props {
            ShapeType::Polygon(dot_components) => Some(dot_components),
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
    }
    
    pub fn init(shape_props: ShapeType<'a>) -> Self<'a> {
        



    }
}
