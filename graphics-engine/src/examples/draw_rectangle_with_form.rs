use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::{program::Program, form::Form, types::DrawProps, types::XYTuple};
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext};

const example_vertex_shader: &'static str = "
    precision mediump float;
    attribute vec4 a_vertex;

    uniform vec4 u_viewport;
    uniform mat4 u_position;
    uniform mat4 u_rotation;
    uniform mat4 u_scale;

    void main() {
        vec4 norm_vertex = a_vertex / u_viewport;
        norm_vertex = u_position * norm_vertex;
        norm_vertex = u_rotation * norm_vertex;
        norm_vertex = u_scale * norm_vertex;

        gl_Position = norm_vertex;
    }
";

const example_fragment_shader: &'static str = "
    precision mediump float;
    uniform vec4 u_color;

    void main() {
        gl_FragColor = u_color;
    }
";

pub fn example() -> Result<(), JsValue> {
    let canvas = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("root")
        .unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap();
    let context = context.dyn_into::<WebGlRenderingContext>()?;

    let program = Program::init(
        &context,
        example_vertex_shader,
        example_fragment_shader
    );
    let program = program.unwrap();

    let vertex_data: [f32; 24] = [
        0.0,
        0.0,
        1.0,
        1.0,

        0.5,
        0.0,
        1.0,
        1.0,

        0.5,
        0.5,
        1.0,
        1.0, // Triangle 1

        0.0,
        0.0,
        1.0,
        1.0,

        0.0,
        0.5,
        1.0,
        1.0,
        
        0.5,
        0.5,
        1.0,
        1.0, // Triangle 2
    ];
    let mut form = Form::init(
        &context,
        &vertex_data,
        &program
    ).unwrap();

    context.clear_color(0.0, 0.0, 0.0, 0.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

    let draw_props = DrawProps {
        position: XYTuple {
            x: 320.0,
            y: 200.0
        },
        rotation: 0.0,
        scale: XYTuple {
            x: 1.0,
            y: 1.0
        },
        color: [1.0, 0.0, 0.0, 1.0]
    };
    form.draw(
        &context,
        [800.0, 600.0, 1.0, 1.0],
        &draw_props
    );

    Ok(())
}
