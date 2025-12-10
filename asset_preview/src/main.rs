use ahash;
use image::{self, GenericImageView, ImageError, ImageReader};
use macroquad::{prelude::*};
use std::env;
use std::io::{Cursor};
use std::path::Path;

fn process_mesh(mesh: &tobj::Mesh, texture: &Texture2D) -> Mesh {
    let vertex_positions: Vec<Vec3> = mesh
        .positions
        .chunks(3)
        .map(|x| Vec3::new(x[0], x[1], x[2]))
        .collect();

    let texcoords: Vec<Vec2> = mesh
        .texcoords
        .chunks(2)
        .map(|x| Vec2::new(x[0], 1.0 - x[1]))
        .collect();

    let normals: Vec<Vec3> = mesh
        .normals
        .chunks(3)
        .map(|x| Vec3::new(x[0], x[1], x[2]))
        .collect();

    let mut vertices = Vec::new();

    assert_eq!(vertex_positions.len(), texcoords.len());
    assert_eq!(vertex_positions.len(), normals.len());

    for i in 0..vertex_positions.len() {
        vertices.push(Vertex {
            position: vertex_positions[i],
            uv: texcoords[i],
            color: [255, 255, 255, 255],
            normal: vec4(normals[i].x, normals[i].y, normals[i].z, 3.0),
        });
    }

    Mesh {
        vertices,
        indices: mesh.indices.iter().map(|x| *x as u16).collect::<Vec<u16>>(),
        texture: Some(texture.clone()),
    }
}

fn load_head(bytes: &[u8], texture: &Texture2D) -> Mesh {
    let mut reader = Cursor::new(bytes);

    let meshes: Vec<tobj::Model> = tobj::load_obj_buf(&mut reader, &tobj::GPU_LOAD_OPTIONS, |_p| {
        Ok((vec![], ahash::AHashMap::new()))
    })
    .expect("can't load file from bytes")
    .0;

    let mesh: &tobj::Mesh = &meshes[0].mesh;
    process_mesh(mesh, texture)
}

fn load_accessory<P>(path: P, texture: &Texture2D) -> Mesh
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    let meshes: Vec<tobj::Model> = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
        .expect("can't load file")
        .0;

    let mesh: &tobj::Mesh = &meshes[0].mesh;
    process_mesh(mesh, texture)
}

const PROGRAM_NAME: &str = "LSDBLOX Asset Previewer 1.1";

fn window_conf() -> Conf {
    Conf {
        window_title: PROGRAM_NAME.to_owned(),
        window_width: 512,
        window_height: 512,
        ..Default::default()
    }
}

fn program_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: window_conf(),
        draw_call_vertex_capacity: 100000,
        draw_call_index_capacity: 50000,
        default_filter_mode: FilterMode::Nearest,
        ..Default::default()
    }
}

fn draw_text_shadow(text: String, height: f32) {
    let new_height = height * 16.0;
    draw_text(&text, 1.0, new_height, 24.0, BLACK);
    let new_height = new_height + 1.0;
    draw_text(&text, 0.0, new_height, 24.0, WHITE);
}

fn process_img(img_path: &Path) -> Result<(u32, u32, Vec<u8>), ImageError> {
    let img = ImageReader::open(img_path)?.decode()?;
    let bytes = img.to_rgba8().into_vec();
    let (width, height) = img.dimensions();
    Ok((width, height, bytes))
}

#[macroquad::main(program_conf)]
async fn main() {
    println!("{}", PROGRAM_NAME);
    println!("Licensed under the GPLv3, Rendering: Macroquad");
    println!("acidbox 2025 (admin@lsdblox.cc)\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} asset.obj [asset.png]", args[0]);
        std::process::exit(1);
    }

    let hair_text: Texture2D;

    println!("Loading asset texture...");

    if args.len() < 3 {
        eprintln!(
            "Warning! You have not set a texture for your asset. Its texture will be a checkerboard."
        );
        hair_text = Texture2D::from_file_with_format(include_bytes!("checker.png"), None);
    } else {
        println!("Processing texture...");
        let img_path = Path::new(&args[2]);
        if img_path.exists() {
            match process_img(img_path) {
                Ok(e) => {
                    let (width, height, bytes) = e;
                    let image = &Image {
                        bytes,
                        width: (width as u16),
                        height: (height as u16),
                    };
                    hair_text = Texture2D::from_image(image);
                },
                Err(err) => {
                    eprintln!("Hey dingus. Had an error processing yyoyr texture. Sorry about that. Falling back once more on that checkerboard. Here's your issue if you wanna check why I couldnt: {}", err);
                    hair_text = Texture2D::from_file_with_format(include_bytes!("checker.png"), None);
                }
            };

        } else {
            eprintln!("COULD NOT FIND TEXTURE FILE. Using fallback.");
            hair_text = Texture2D::from_file_with_format(include_bytes!("checker.png"), None);
        }
    }

    println!("Loading models...");

    let head_text: Texture2D = Texture2D::from_rgba8(1, 1, &[255, 0, 255, 255]);

    let head_byte: &[u8] = include_bytes!("default.obj");
    let hair_path: &Path = Path::new(&args[1]);

    let head_mesh: Mesh = load_head(&head_byte, &head_text);
    let hair_mesh: Mesh = load_accessory(&hair_path, &hair_text);

    println!("Starting preview...");

    request_new_screen_size(512.0, 512.0);

    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;
    let mut radius: f32 = 4.0;

    let target = vec3(0.0, 0.5, 0.0);
    let world_up = vec3(0.0, 1.0, 0.0);

    let max_pitch = std::f32::consts::FRAC_PI_2 - 0.1;
    let min_pitch = -max_pitch;

    let mut sens = 0.05;
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut cam_invert = -1.0;
    loop {
        let delta = get_frame_time();

        if is_key_down(KeyCode::W) {
            pitch += sens * cam_invert;
        }

        if is_key_down(KeyCode::S) {
            pitch -= sens * cam_invert;
        }

        if is_key_down(KeyCode::A) {
            yaw += sens * cam_invert;
        }

        if is_key_down(KeyCode::D) {
            yaw -= sens * cam_invert;
        }

        if is_key_down(KeyCode::LeftShift) {
            sens = 0.01;
        } else {
            sens = 0.05;
        }

        if is_key_down(KeyCode::Q) {
            radius -= sens;
        }

        if is_key_down(KeyCode::E) {
            radius += sens;
        }

        if is_key_pressed(KeyCode::Tab) {
            cam_invert = -cam_invert;
        }

        if is_mouse_button_down(MouseButton::Right) {
            set_cursor_grab(true);
            show_mouse(false);
            let mouse_position: Vec2 = mouse_position().into();
            let mouse_delta = mouse_position - last_mouse_position;
            yaw += (delta * mouse_delta.x * sens) * -cam_invert;
            pitch += (delta * mouse_delta.y * sens) * -cam_invert;

            last_mouse_position = mouse_position;
        } else {
            set_cursor_grab(false);
            show_mouse(true);
        }

        let (_, mouse_wheel_y) = mouse_wheel();

        if mouse_wheel_y == 1.0 {
            radius -= sens;
        }

        if mouse_wheel_y == -1.0 {
            radius += sens;
        }

        radius = radius.clamp(1.0, 10.0);
        pitch = pitch.clamp(min_pitch, max_pitch);

        let new_pos = vec3(
            radius * yaw.cos() * pitch.cos(),
            radius * pitch.sin(),
            radius * yaw.sin() * pitch.cos(),
        ) + target;

        clear_background(Color::from_hex(0x189BCC));

        set_camera(&Camera3D {
            position: new_pos,
            up: world_up,
            target: target,
            ..Default::default()
        });

        draw_mesh(&head_mesh);
        draw_mesh(&hair_mesh);
        gl_use_default_material();

        set_default_camera();
        let rad_txt = format!("Zoom: {}", radius);
        draw_text_shadow(PROGRAM_NAME.to_string(), 1.0);
        draw_text_shadow(rad_txt, 2.0);
        next_frame().await
    }
}
