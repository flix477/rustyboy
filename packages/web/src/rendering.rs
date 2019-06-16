use js_sys::{Float32Array, WebAssembly};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture,
};

use rustyboy_core::video::screen::SCREEN_SIZE;

pub struct Renderer {
    context: WebGlRenderingContext,
    texture: WebGlTexture,
}

impl Renderer {
    pub fn new() -> Result<Renderer, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        let program = initialize_program(&context)?;
        let position_buffer = initialize_position_buffer(&context)?;
        let texture_coord_buffer = initialize_texture_coord_buffer(&context)?;

        let position_attribute_location = context.get_attrib_location(&program, "position") as u32;
        let texture_attribute_location =
            context.get_attrib_location(&program, "textureCoord") as u32;
        // position buffer
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        context.vertex_attrib_pointer_with_i32(
            position_attribute_location,
            2, // number of elements per attribute
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location);

        // texture coordinates buffer
        context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&texture_coord_buffer),
        );
        context.vertex_attrib_pointer_with_i32(
            texture_attribute_location,
            2, // number of elements per attribute
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(texture_attribute_location);

        let texture = create_texture(&context)?;

        context.use_program(Some(&program));

        Ok(Renderer { context, texture })
    }

    pub fn update(&self, buffer: &[u8]) -> Result<(), JsValue> {
        self.context.clear_color(1.0, 1.0, 1.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.update_texture(buffer)?;

        self.context
            .draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        Ok(())
    }

    fn update_texture(&self, buffer: &[u8]) -> Result<(), JsValue> {
        self.context
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.texture));
        self.context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGB as i32,
                SCREEN_SIZE.0 as i32,
                SCREEN_SIZE.1 as i32,
                0,
                WebGlRenderingContext::RGB,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(buffer),
            )
    }
}

fn initialize_position_buffer(context: &WebGlRenderingContext) -> Result<WebGlBuffer, JsValue> {
    let vertices: [f32; 12] = [
        -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
    ];

    initialize_buffer(context, &vertices)
}

fn initialize_texture_coord_buffer(
    context: &WebGlRenderingContext,
) -> Result<WebGlBuffer, JsValue> {
    let vertices: [f32; 12] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0];

    initialize_buffer(context, &vertices)
}

fn initialize_buffer(
    context: &WebGlRenderingContext,
    vertices: &[f32],
) -> Result<WebGlBuffer, JsValue> {
    let buffer = context
        .create_buffer()
        .ok_or("failed to create position buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    let vertices = vertices_to_typedarray(vertices)?;

    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices,
        WebGlRenderingContext::STATIC_DRAW,
    );

    Ok(buffer)
}

fn vertices_to_typedarray(vertices: &[f32]) -> Result<Float32Array, JsValue> {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()?
        .buffer();
    let vertices_location = vertices.as_ptr() as u32 / 4;
    Ok(Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32))
}

fn create_texture(context: &WebGlRenderingContext) -> Result<WebGlTexture, JsValue> {
    let texture = context.create_texture().ok_or("failed to create texture")?;
    context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_S,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_WRAP_T,
        WebGlRenderingContext::CLAMP_TO_EDGE as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MIN_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );
    context.tex_parameteri(
        WebGlRenderingContext::TEXTURE_2D,
        WebGlRenderingContext::TEXTURE_MAG_FILTER,
        WebGlRenderingContext::NEAREST as i32,
    );

    context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGlRenderingContext::TEXTURE_2D,
        0,
        WebGlRenderingContext::RGB as i32,
        SCREEN_SIZE.0 as i32,
        SCREEN_SIZE.1 as i32,
        0,
        WebGlRenderingContext::RGB,
        WebGlRenderingContext::UNSIGNED_BYTE,
        None,
    )?;

    context.active_texture(WebGlRenderingContext::TEXTURE0);

    Ok(texture)
}

fn initialize_program(context: &WebGlRenderingContext) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(
        context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec2 position;
        attribute vec2 textureCoord;

        varying vec2 fragTextureCoord;

        void main() {
            fragTextureCoord = textureCoord;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#,
    )?;

    let frag_shader = compile_shader(
        context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;

        varying vec2 fragTextureCoord;
        uniform sampler2D sampler;

        void main() {
            gl_FragColor = texture2D(sampler, fragTextureCoord);
        }
    "#,
    )?;

    link_program(context, &vert_shader, &frag_shader)
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
