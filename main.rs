use base64::Engine;
use dotenv::dotenv;
use image::{GenericImageView, ImageError, ImageReader};
use macroquad::prelude::*;
use png::{BitDepth, ColorType, Encoder};
use rouille::{post_input, router};
use sqlx::mysql::MySqlPool;
use sqlx::prelude::FromRow;
use sqlx::{MySql, Pool};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const PROGRAM_NAME: &str = "LSDBLOX Thumbnail Server 1.0";

fn window_conf() -> Conf {
    Conf {
        window_title: PROGRAM_NAME.to_owned(),
        window_width: 512,
        window_height: 512,
        window_resizable: false,
        ..Default::default()
    }
}

struct RenderRequest {
    mesh_path: String,
    texture_path: String,
    response_sender: Sender<String>,
}

#[derive(Debug, FromRow)]
struct Asset {
    item_type: i8,
    location: Option<String>,
    texture: Option<i32>,
}

async fn fetch_asset_info(
    pool: &Pool<MySql>,
    item_id: i32,
) -> Result<(String, String), Box<dyn Error>> {
    let item_row: Asset = sqlx::query_as::<_, Asset>(
        r#"
        SELECT type AS item_type, asset AS location, hat_texture as texture
        FROM items 
        WHERE id = ?
        "#,
    )
    .bind(item_id)
    .fetch_one(pool)
    .await?;

    let mesh_location = match item_row.location {
        Some(loc) => loc,
        None => {
            return Err(
                "Mesh location field is NULL in the database. THIS SHOULD NEVER HAPPEN.".into(),
            );
        }
    };

    if item_row.item_type != 9 {
        return Err("Item type is not implemented.".into());
    }

    let is_obj = Path::new(&mesh_location)
        .extension()
        .and_then(OsStr::to_str)
        == Some("obj");

    if !is_obj {
        return Err("Obj file not valid. May be in the old format?".into());
    }

    let texture_row: Asset = sqlx::query_as::<_, Asset>(
        r#"
        SELECT type AS item_type, asset AS location, hat_texture as texture
        FROM items 
        WHERE id = ?
        "#,
    )
    .bind(item_row.texture)
    .fetch_one(pool)
    .await?;

    let texture_location: String = match texture_row.location {
        Some(loc) => loc,
        None => return Err("Mesh location field is NULL in the database.".into()),
    };

    Ok((mesh_location, texture_location))
}

fn process_img(img_path: &Path) -> Result<(u32, u32, Vec<u8>), ImageError> {
    let img = ImageReader::open(img_path)?.decode()?;
    let bytes = img.to_rgba8().into_vec();
    let (width, height) = img.dimensions();
    Ok((width, height, bytes))
}

fn process_mesh(mesh: &tobj::Mesh, texture: &Texture2D) -> Mesh {
    let vertex_positions: Vec<Vec3> = mesh
        .positions
        .chunks(3)
        .map(|x| Vec3::new(x[0], x[1], x[2]))
        .collect();

    let texcoords: Vec<Vec2> = mesh
        .texcoords
        .chunks(2)
        .map(|x| Vec2::new(x[0], x[1]))
        .collect();

    let normals: Vec<Vec3> = mesh
        .normals
        .chunks(3)
        .map(|x| Vec3::new(x[0], x[1], x[2]))
        .collect();

    let mut vertices = Vec::new();

    let count = vertex_positions.len();
    for i in 0..count {
        let uv = if i < texcoords.len() {
            texcoords[i]
        } else {
            Vec2::ZERO
        };
        let normal = if i < normals.len() {
            vec4(normals[i].x, normals[i].y, normals[i].z, 1.0)
        } else {
            vec4(0.0, 1.0, 0.0, 1.0)
        };

        vertices.push(Vertex {
            position: vertex_positions[i],
            uv,
            color: [255, 255, 255, 255],
            normal,
        });
    }

    Mesh {
        vertices,
        indices: mesh.indices.iter().map(|x| *x as u16).collect::<Vec<u16>>(),
        texture: Some(texture.clone()),
    }
}

fn load_resources_and_mesh(
    mesh_filename: &str,
    texture_filename: &str,
) -> Result<Mesh, Box<dyn Error>> {
    let texture_full_path = format!("/srv/http/{}", texture_filename);
    let mesh_full_path = format!("/srv/http/{}", mesh_filename);

    let img_path = Path::new(&texture_full_path);
    println!("{}", texture_full_path);
    println!("{}", mesh_full_path);

    let texture = if img_path.exists() {
        match process_img(img_path) {
            Ok((w, h, bytes)) => {
                let img = Image {
                    bytes,
                    width: w as u16,
                    height: h as u16,
                };
                Texture2D::from_image(&img)
            }
            Err(_) => Texture2D::from_file_with_format(include_bytes!("checker.png"), None),
        }
    } else {
        Texture2D::from_file_with_format(include_bytes!("checker.png"), None)
    };

    let (meshes, _) = tobj::load_obj(&mesh_full_path, &tobj::GPU_LOAD_OPTIONS)?;

    if meshes.is_empty() {
        return Err("No data found in obj file. Weirddddddddd.".into());
    }

    let mesh_data = meshes[0].mesh.clone();
    Ok(process_mesh(&mesh_data, &texture))
}

fn perform_render(mesh_path: String, texture_path: String) -> String {
    println!("Rendering: {} with {}", mesh_path, texture_path);

    let yaw: f32 = 1.18;
    let pitch: f32 = 38.0;
    let radius: f32 = 4.0;

    let target = vec3(0.0, 1.5, 0.0);
    let world_up = vec3(0.0, 1.0, 0.0);

    let new_pos = vec3(
        radius * yaw.cos() * pitch.cos(),
        radius * pitch.sin(),
        radius * yaw.sin() * pitch.cos(),
    ) + target;

    clear_background(Color::with_alpha(&Color::from_hex(0x000000), 0.0));

    set_camera(&Camera3D {
        position: new_pos,
        up: world_up,
        target,
        ..Default::default()
    });

    match load_resources_and_mesh(&mesh_path, &texture_path) {
        Ok(m) => {
            draw_mesh(&m);
        }
        Err(e) => {
            println!("Render error: {}", e);
        }
    };

    gl_use_default_material();
    
    let img_data = get_screen_data();
    let width = img_data.width as u32;
    let height = img_data.height as u32;
    
    let mut image = match image::RgbaImage::from_raw(width, height, img_data.bytes) {
        Some(img) => image::DynamicImage::ImageRgba8(img),
        None => {
            println!("Failed to create image from screen data.");
            return "".to_string();
        }
    };

    image = image.flipv();
    let flipped_bytes = image.into_rgba8().into_vec();

    let mut png_data = Vec::new();

    {
        let mut encoder = Encoder::new(&mut png_data, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&flipped_bytes).unwrap();
    }

    base64::engine::general_purpose::STANDARD.encode(png_data)
}

#[macroquad::main(window_conf)]
async fn main() {
    dotenv().ok();

    println!("{}", PROGRAM_NAME);

    let (tx_work, rx_work) = channel::<RenderRequest>();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
        let url = format!("mysql://usr:{}@localhost:3306/appdb", db_password);

        let pool = rt.block_on(async {
            MySqlPool::connect(&url)
                .await
                .expect("Failed to connect to DB")
        });

        println!("Server listening on 127.0.0.1:8000");

        rouille::start_server("127.0.0.1:8000", move |request| {
            router!(request,
                (POST) (/) => {
                    let body = match post_input!(request, { id: String }) {
                        Ok(d) => d,
                        Err(_) => return rouille::Response::empty_400(),
                    };

                    let id_val = match body.id.parse::<i32>() {
                        Ok(i) => i,
                        Err(_) => return rouille::Response::text("Invalid Number").with_status_code(400),
                    };

                    let asset_result = rt.block_on(async {
                        fetch_asset_info(&pool, id_val).await
                    });


                    let (mesh_loc, tex_loc) = match asset_result {
                        Ok(data) => data,
                        Err(e) => {
                            eprintln!("DB Error: {}", e);
                            return rouille::Response::text("Asset not found").with_status_code(404);
                        }
                    };

                    let (tx_answer, rx_answer) = channel();

                    let req = RenderRequest {
                        mesh_path: mesh_loc,
                        texture_path: tex_loc,
                        response_sender: tx_answer
                    };

                    if tx_work.send(req).is_err() {
                        return rouille::Response::text("Server Shutting Down").with_status_code(500);
                    }

                    match rx_answer.recv() {
                        Ok(base64_img) => rouille::Response::text(base64_img),
                        Err(_) => rouille::Response::text("Render Failed").with_status_code(500),
                    }
                },
                _ => rouille::Response::empty_404()
            )
        });
    });

    loop {
        clear_background(BLACK);
        draw_text("Listening for requests...", 10.0, 20.0, 30.0, WHITE);

        if let Ok(work) = rx_work.try_recv() {
            let result_b64 = perform_render(work.mesh_path, work.texture_path);
            let _ = work.response_sender.send(result_b64);
        }

        next_frame().await;
    }
}
