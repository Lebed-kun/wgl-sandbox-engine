use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};
use js_sys::{Float32Array};
use crate::types::{Vector4, Matrix4x4, DrawProps};
use crate::program::{Program};

struct Uniforms {
    pub viewport: Vector4,
    pub position: Matrix4x4,
    pub rotation: Matrix4x4,
    pub scale: Matrix4x4,
    pub color: Vector4,
}

mod constants {
    use super::{Matrix4x4, Vector4};

    pub const default_matrix4x4: Matrix4x4 = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    pub const default_vector4: Vector4 = [0.0, 0.0, 0.0, 0.0];

    pub const vertex_size: i32 = 4;
}

// TODO: profile memory size for call stack optimization (?)

/// Struct which stores WebGl level data for rendering shape
pub struct Form<'a> {
    uniforms: Uniforms,
    vertex_buffer: WebGlBuffer,
    vertex_data: Float32Array,
    program: &'a Program,
}

impl<'a> Form<'a> {
    pub fn init(
        gl: &WebGlRenderingContext,
        vertex_data: &[f32],
        program: &'a Program
    ) -> Option<Self> {
        let vertex_buffer = try_unwrap!(
            @dev;
            gl.create_buffer(),
            "Unable to find location of attribute \"a_vertex\""
        );

        unsafe {
            let vertex_array = Float32Array::view(vertex_data);

            Some(Self {
                uniforms: Uniforms {
                    viewport: constants::default_vector4,
                    position: constants::default_matrix4x4,
                    rotation: constants::default_matrix4x4,
                    scale: constants::default_matrix4x4,
                    color: constants::default_vector4
                },
                vertex_buffer,
                vertex_data: vertex_array,
                program
            })
        }
    }

    pub fn prepare(
        &mut self,
        viewport: Vector4,
        draw_props: &DrawProps
    ) {
        self.uniforms.viewport = viewport;

        self.uniforms.position[(3 << 2) + 0] = draw_props.position.x;
        self.uniforms.position[(3 << 2) + 1] = draw_props.position.y;
        
        let cos_angle = draw_props.rotation.cos();
        let sin_angle = draw_props.rotation.sin();
        self.uniforms.rotation[(0 << 2) + 0] = cos_angle;
        self.uniforms.rotation[(0 << 2) + 1] = sin_angle;
        self.uniforms.rotation[(1 << 2) + 0] = -sin_angle;
        self.uniforms.rotation[(1 << 2) + 1] = cos_angle;

        self.uniforms.scale[(0 << 2) + 0] = draw_props.scale.x;
        self.uniforms.scale[(1 << 2) + 1] = draw_props.scale.y;

        self.uniforms.color = draw_props.color;
    }

    pub fn draw(
        &self, 
        gl: &WebGlRenderingContext,
    ) {
        gl.use_program(Some(&self.program.program));

        // Setup vertex data
        gl.enable_vertex_attrib_array(self.program.a_vertex_location);
        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer)
        );
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &self.vertex_data,
            WebGlRenderingContext::STATIC_DRAW
        );
        gl.vertex_attrib_pointer_with_f64(
            self.program.a_vertex_location,
            constants::vertex_size,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0.0
        );

        // Setup uniform data
        
        gl.uniform4fv_with_f32_array(
            Some(&self.program.uniform_locations.u_viewport),
            &self.uniforms.viewport
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.program.uniform_locations.u_position),
            false,
            &self.uniforms.position
        );
        
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.program.uniform_locations.u_rotation),
            false,
            &self.uniforms.rotation
        );

        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.program.uniform_locations.u_scale),
            false,
            &self.uniforms.scale
        );

        gl.uniform4fv_with_f32_array(
            Some(&self.program.uniform_locations.u_color),
            &self.uniforms.color
        );

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (self.vertex_data.length() as i32 / constants::vertex_size) as i32
        );
    }
}
