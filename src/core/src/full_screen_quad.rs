use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::shader::Shader;

pub struct FullScreenQuad {
    position_buffer: WebGlBuffer,
}

impl FullScreenQuad {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, wasm_bindgen::JsValue> {
        let positions: Vec<f32> = vec![-1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0];

        let position_buffer = upload_array_f32(gl, positions)?;

        Ok(Self { position_buffer })
    }

    pub fn render(&mut self, gl: &WebGl2RenderingContext, shade: &impl Shader) {
        gl.bind_buffer(
            WebGl2RenderingContext::ARRAY_BUFFER,
            Some(&self.position_buffer),
        );

        gl.vertex_attrib_pointer_with_i32(
            shade.get_attrib_position(),
            2, // num components
            WebGl2RenderingContext::FLOAT,
            false, // normalize
            0,     // stride
            0,     // offset
        );
        gl.enable_vertex_attrib_array(shade.get_attrib_position());

        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLE_STRIP,
            0, //offset,
            4, // vertex count
        );
    }
}

fn upload_array_f32(
    gl: &WebGl2RenderingContext,
    vertices: Vec<f32>,
) -> Result<WebGlBuffer, wasm_bindgen::JsValue> {
    let position_buffer = gl
        .create_buffer()
        .expect("Failed to create position buffer");

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();

    let vertices_location = vertices.as_ptr() as u32 / 4;

    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);

    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &vert_array,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    Ok(position_buffer)
}
