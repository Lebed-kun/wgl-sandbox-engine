use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation,
};
use js_sys::{Object};
use crate::types::{Vector4, Matrix4x4, DrawProps};

struct Uniform<T> {
    pub location: WebGlUniformLocation,
    pub value: T,
}

struct Attribute<T> {
    pub location: u32,
    pub value: T,
}

struct Uniforms {
    pub viewport_x_scale: Uniform<f32>,
    pub viewport_y_scale: Uniform<f32>,
    pub position: Uniform<Matrix4x4>,
    pub rotation: Uniform<Matrix4x4>,
    pub scale: Uniform<Matrix4x4>,
    pub color: Uniform<Vector4>,
}

mod constants {
    use super::{Matrix4x4, Vector4};

    pub const u_viewport_x_scale: &'static str = "u_viewport_x_scale";
    pub const u_viewport_y_scale: &'static str = "u_viewport_y_scale";
    pub const u_position: &'static str = "u_position";
    pub const u_rotation: &'static str = "u_rotation";
    pub const u_scale: &'static str = "u_scale";
    pub const u_color: &'static str = "u_color";

    pub const a_vertex: &'static str = "a_vertex";

    pub const default_matrix4x4: Matrix4x4 = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    pub const default_vector4: Vector4 = [0.0, 0.0, 0.0, 0.0];

    pub const vertex_size: i32 = 4;
}

fn is_valid_u_location(v: &WebGlUniformLocation) -> bool {
    let num = v.as_f64();
    num.is_some() && num.unwrap() >= 0.0
}

macro_rules! try_locate_uniform {
    ($gl:expr, $program:expr, $u_name:expr) => {
        {
            try_unwrap!(
                @dev;
                ($gl).get_uniform_location($program, $u_name),
                format!(
                    "Unable to get uniform location of parameter \"{}\"",
                    $u_name
                ),
                is_valid_u_location
            )
        }
    };
}

// TODO: profile memory size for call stack optimization (?)
pub struct Form {
    uniforms: Uniforms,
    vertex_attribute: Attribute<Vector4>,
    vertex_buffer: WebGlBuffer,
    vertex_data: Object,
    vertex_data_length: i32,
    program: WebGlProgram,
}

impl Form {
    fn compile_shader(
        gl: &WebGlRenderingContext,
        src: &str,
        shader_type: u32,
    ) -> Option<WebGlShader> {
        let shader = try_unwrap!(
            @dev;
            gl.create_shader(shader_type),
            "Unable to create shader"
        );

        gl.shader_source(&shader, src);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .is_falsy()
        {
            return None;
        }

        Some(shader)
    }

    fn init_program(
        gl: &WebGlRenderingContext,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
    ) -> Option<WebGlProgram> {
        let vertex_shader = try_unwrap!(
            @dev;
            Self::compile_shader(
                gl,
                vertex_shader_src,
                WebGlRenderingContext::VERTEX_SHADER
            ),
            "Unable to compile vertex shader"
        );

        let fragment_shader = try_unwrap!(
            @dev;
            Self::compile_shader(
                gl,
                vertex_shader_src,
                WebGlRenderingContext::VERTEX_SHADER
            ),
            "Unable to compile fragment shader"
        );

        let program = try_unwrap!(
            @dev;
            gl.create_program(),
            "Unable to create program"
        );

        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .is_falsy()
        {
            return None;
        }

        Some(program)
    }

    fn retrieve_uniforms(gl: &WebGlRenderingContext, program: &WebGlProgram) -> Option<Uniforms> {
        let u_viewport_x_scale = try_locate_uniform!(gl, program, constants::u_viewport_x_scale);
        let u_viewport_y_scale = try_locate_uniform!(gl, program, constants::u_viewport_y_scale);
        let u_position = try_locate_uniform!(gl, program, constants::u_position);
        let u_rotation = try_locate_uniform!(gl, program, constants::u_rotation);
        let u_scale = try_locate_uniform!(gl, program, constants::u_scale);
        let u_color = try_locate_uniform!(gl, program, constants::u_color);

        Some(Uniforms {
            viewport_x_scale: Uniform {
                location: u_viewport_x_scale,
                value: 0.0,
            },
            viewport_y_scale: Uniform {
                location: u_viewport_y_scale,
                value: 0.0,
            },
            position: Uniform {
                location: u_position,
                value: constants::default_matrix4x4,
            },
            rotation: Uniform {
                location: u_rotation,
                value: constants::default_matrix4x4,
            },
            scale: Uniform {
                location: u_scale,
                value: constants::default_matrix4x4,
            },
            color: Uniform {
                location: u_color,
                value: constants::default_vector4,
            },
        })
    }

    pub fn init(
        gl: &WebGlRenderingContext,
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        vertex_data: Object,
        vertex_data_length: i32
    ) -> Option<Self> {
        let program = try_unwrap!(
            @dev;
            Self::init_program(gl, vertex_shader_src, fragment_shader_src),
            "Unable to initialize program"
        );

        let vertex_attribute_location = try_unwrap!(
            @dev;
            Some(gl.get_attrib_location(&program, constants::a_vertex)),
            "Unable to find location of attribute \"a_vertex\"",
            |v| *v >= 0
        );
        let vertex_buffer = try_unwrap!(
            @dev;
            gl.create_buffer(),
            "Unable to find location of attribute \"a_vertex\""
        );

        let uniforms = try_unwrap!(
            @dev;
            Self::retrieve_uniforms(gl, &program),
            "Unable to retrieve uniforms"
        );

        Some(Self {
            uniforms,
            vertex_attribute: Attribute {
                location: vertex_attribute_location as u32,
                value: constants::default_vector4
            },
            vertex_buffer,
            vertex_data,
            vertex_data_length,
            program
        })
    }

    pub fn draw(
        &mut self, 
        gl: &WebGlRenderingContext,
        viewport_x_scale: f32,
        viewport_y_scale: f32,
        draw_props: &DrawProps
    ) {
        gl.use_program(Some(&self.program));

        // Setup vertex data
        gl.enable_vertex_attrib_array(self.vertex_attribute.location);
        gl.vertex_attrib_pointer_with_f64(
            self.vertex_attribute.location,
            constants::vertex_size,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0.0
        );
        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer)
        );
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &self.vertex_data,
            WebGlRenderingContext::STATIC_DRAW
        );

        // Setup uniform data
        self.uniforms.viewport_x_scale.value = viewport_x_scale;
        self.uniforms.viewport_y_scale.value = viewport_y_scale;
        gl.uniform1f(
            Some(&self.uniforms.viewport_x_scale.location),
            self.uniforms.viewport_x_scale.value
        );
        gl.uniform1f(
            Some(&self.uniforms.viewport_y_scale.location),
            self.uniforms.viewport_y_scale.value
        );

        self.uniforms.position.value[(0 << 2) + 3] = draw_props.position.x;
        self.uniforms.position.value[(1 << 2) + 3] = draw_props.position.y;
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniforms.position.location),
            false,
            &self.uniforms.position.value
        );
        
        let cos_angle = draw_props.rotation.cos();
        let sin_angle = draw_props.rotation.sin();
        self.uniforms.rotation.value[(0 << 2) + 0] = cos_angle;
        self.uniforms.rotation.value[(0 << 2) + 1] = -sin_angle;
        self.uniforms.rotation.value[(1 << 2) + 0] = sin_angle;
        self.uniforms.rotation.value[(1 << 2) + 1] = cos_angle;
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniforms.rotation.location),
            false,
            &self.uniforms.rotation.value
        );

        self.uniforms.scale.value[(0 << 2) + 0] = draw_props.scale.x;
        self.uniforms.scale.value[(1 << 2) + 1] = draw_props.scale.y;
        gl.uniform_matrix4fv_with_f32_array(
            Some(&self.uniforms.scale.location),
            false,
            &self.uniforms.scale.value
        );

        self.uniforms.color.value = draw_props.color;
        gl.uniform4fv_with_f32_array(
            Some(&self.uniforms.color.location),
            &self.uniforms.color.value
        );

        gl.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            self.vertex_data_length / 2
        );
    }
}
