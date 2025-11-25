use macroquad::prelude::*;
use std::io::Cursor;
use std::path::Path;
use std::env;
use ahash;
use image::{self, GenericImageView, ImageReader};

fn load_head(bytes: &[u8], texture: &Texture2D) -> Mesh {
    let mut reader = Cursor::new(bytes);

    let meshes: Vec<tobj::Model> = tobj::load_obj_buf(&mut reader, &tobj::GPU_LOAD_OPTIONS, |_p| {
        Ok((vec![], ahash::AHashMap::new()))
    })
    .expect("can't load file from bytes")
    .0;

    let mesh: &tobj::Mesh = &meshes[0].mesh;
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

    assert_eq!(vertex_positions.len(), texcoords.len());
    assert_eq!(vertex_positions.len(), normals.len());

    for i in 0..vertex_positions.len() {
        vertices.push(Vertex {
            position: vertex_positions[i],
            uv: texcoords[i],
            color: [255, 0, 255, 255],
            normal: vec4(normals[i].x, normals[i].y, normals[i].z, 1.0),
        });
    }

    Mesh {
        vertices,
        indices: mesh.indices.iter().map(|x| *x as u16).collect::<Vec<u16>>(),
        texture: Some(texture.clone()),
    }
}

fn load_mesh<P>(path: P, texture: &Texture2D) -> Mesh
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    let meshes: Vec<tobj::Model> = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
        .expect("can't load file")
        .0;

    let mesh: &tobj::Mesh = &meshes[0].mesh;

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

    assert_eq!(vertex_positions.len(), texcoords.len());

    assert_eq!(vertex_positions.len(), normals.len());

    for i in 0..vertex_positions.len() {
        vertices.push(Vertex {
            position: vertex_positions[i],

            uv: texcoords[i],

            color: [255, 255, 255, 255],

            normal: vec4(normals[i].x, normals[i].y, normals[i].z, 1.0),
        });
    }

    Mesh {
        vertices,

        indices: mesh.indices.iter().map(|x| *x as u16).collect::<Vec<u16>>(),

        texture: Some(texture.clone()),
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "LSDBLOX asset previewer".to_owned(),
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
        ..Default::default()
    }
}

#[macroquad::main(program_conf)]
async fn main() {
    println!("LSDBLOX asset previewer 1.0");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} asset.obj [asset.png]", args[0]);
        std::process::exit(1);
    }

    let hair_text: Texture2D;

    println!("Loading asset texture...");
    
    if args.len() < 3 {
        eprintln!("Warning! You have not set a texture for your asset. Its texture will be a checkerboard.");
        hair_text = Texture2D::from_file_with_format(include_bytes!("checker.png"), None);
    } else {
        println!("Processing texture...");
        let img_path = Path::new(&args[2]);
        let img = ImageReader::open(img_path).unwrap().decode();
        let bytes = img.as_ref().unwrap().to_rgba8();
        let (width, height) = img.unwrap().dimensions();
        let bytes = bytes.into_vec();
        let image = &Image { bytes, width: (width as u16), height: (height as u16) };
        hair_text = Texture2D::from_image(image);
    }


    println!("Loading models...");

    let head_text: Texture2D = Texture2D::from_rgba8(1, 1, &[255, 255, 255, 255]);

    let head_byte: &[u8] = include_bytes!("default.obj");
    let hair_path: &Path = Path::new(&args[1]);

    let head_mesh: Mesh = load_head(&head_byte, &head_text);
    let hair_mesh: Mesh = load_mesh(&hair_path, &hair_text);

    println!("Starting preview...");

    request_new_screen_size(512.0, 512.0);


    loop {
        clear_background(SKYBLUE);

        set_camera(&Camera3D {
            position: vec3(1.5, 1.0, 2.5),
            target: vec3(0.0, 0.5, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&head_mesh);
        draw_mesh(&hair_mesh);
        gl_use_default_material();

        next_frame().await
    }
}
