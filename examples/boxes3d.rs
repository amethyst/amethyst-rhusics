extern crate amethyst;
extern crate amethyst_rhusics;
extern crate cgmath;
extern crate collision;
extern crate genmesh;
extern crate rand;
extern crate rhusics_core;
extern crate rhusics_ecs;
extern crate shred;
//#[macro_use] // TOD: in Rust 2018 you import the macros explicitly.
extern crate shred_derive;

use std::time::{Duration, Instant};

use amethyst::assets::{Handle, Loader, AssetLoaderSystemData};
use amethyst::core::math as na;
use amethyst::core::{Transform, TransformBundle};
use amethyst::ecs::prelude::{Builder, Entity, World, WorldExt};
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode, StringBindings, InputBundle};
use amethyst::prelude::{
    Application, /*Config,*/ GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData,
    StateEvent, Trans,
};
use amethyst::renderer::{Camera, Material, MaterialDefaults, Mesh, RenderingBundle, RenderToWindow, RenderShaded3D};
use amethyst::renderer::rendy::mesh::{PosTex, TexCoord, MeshBuilder};
use amethyst::ui::{/*DrawUi,*/ UiBundle};
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst_rhusics::{setup_3d_arena, time_sync, DefaultPhysicsBundle3};
use cgmath::Point3;
use collision::primitive::{Cuboid, Primitive3};
use collision::Aabb3;
use rhusics_core::CollisionShape;
use rhusics_ecs::physics3d::BodyPose3;

use self::boxes::{
    create_ui, update_ui, BoxSimulationBundle3, Emitter, Graphics, KillRate, ObjectType,
};
use amethyst::utils::application_root_dir;
use amethyst::renderer::types::{MeshData, DefaultBackend};
use amethyst::renderer::rendy::texture::palette::load_from_linear_rgba;
use amethyst::renderer::palette::LinSrgba;

mod boxes;

#[derive(Default)]
pub struct Emitting {
    fps: Option<Entity>,
    num: Option<Entity>,
    collision: Option<Entity>,
}

pub type Shape = CollisionShape<Primitive3<f32>, BodyPose3<f32>, Aabb3<f32>, ObjectType>;

impl SimpleState for Emitting {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.write_resource::<KillRate>().0 = 0.;
        initialise_camera(world);
        let g = Graphics {
            mesh: initialise_mesh(world),
        };

        let (num_display, fps_display, collisions_display) = create_ui(world);
        self.num = Some(num_display);
        self.fps = Some(fps_display);
        self.collision = Some(collisions_display);

        world.insert(g);
        setup_3d_arena(
            Point3::new(-1., -1., -2.),
            Point3::new(1., 1., 0.),
            (
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
                ObjectType::Wall,
            ),
            world,
        );
        initialise_emitters(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Window(ref event)
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) =>
            {
                Trans::Quit
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        time_sync(&data.world);
        update_ui::<Point3<f32>>(
            data.world,
            self.num.unwrap(),
            self.fps.unwrap(),
            self.collision.unwrap(),
        );
        data.data.update(&data.world);
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_3d(1000., 1000.))
        .with(Transform::from(na::Vector3::new(0., 0., 1.)))
        .build();
}

fn initialise_mesh(world: &mut World) -> Handle<Mesh> {
    use genmesh::generators::Cube;
    use genmesh::{MapToVertices, Triangulate, Vertices};
    let vertices = Cube::new()
        .vertex(|v| PosTex {
            position: v.pos.into(),
            tex_coord: TexCoord::from([0.1, 1.0]),
        }).triangulate()
        .vertices()
        .collect::<Vec<_>>();
    let mesh_builder = MeshBuilder::from(vertices);
    let mesh_data = MeshData::from(mesh_builder);
    world
        .read_resource::<Loader>()
        .load_from_data(mesh_data, (), &world.read_resource())
}

fn initialise_material(world: &mut World, r: f32, g: f32, b: f32) -> Handle<Material> {
    let albedo = world.read_resource::<Loader>().load_from_data(
        load_from_linear_rgba(LinSrgba::new(r,g,b,1.)).into(),
        (),
        &world.read_resource(),
    );
    let defaults = world.read_resource::<MaterialDefaults>().0.clone();
    world.exec(|mtl_loader: AssetLoaderSystemData<'_, Material>|{
        mtl_loader.load_from_data(
            Material {
                albedo,
                ..defaults
            },
            ()
        )
    })
}

fn emitter(p: Point3<f32>, d: Duration, material: Handle<Material>) -> Emitter<Point3<f32>> {
    Emitter {
        location: p,
        emission_interval: d,
        last_emit: Instant::now(),
        material,
        emitted: 0,
    }
}

fn initialise_emitters(world: &mut World) {
    let mat = initialise_material(world, 0.3, 1.0, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point3::new(-0.4, 0., -1.),
            Duration::new(0, 50_000_000),
            mat,
        )).build();

    let mat = initialise_material(world, 0.3, 0.0, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point3::new(0.4, 0., -1.),
            Duration::new(0, 75_000_000),
            mat,
        )).build();

    let mat = initialise_material(world, 1.0, 1.0, 1.0);
    world
        .create_entity()
        .with(emitter(
            Point3::new(0., -0.4, -1.),
            Duration::new(0, 100_000_000),
            mat,
        )).build();

    let mat = initialise_material(world, 1.0, 0.3, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point3::new(0., 0.4, -1.),
            Duration::new(0, 25_000_000),
            mat,
        )).build();
}

fn run() -> Result<(), amethyst::Error> {
    amethyst::start_logger(amethyst::LoggerConfig::default());
    // let path = format!(
    //     "{}/../resources/display_config.ron",
    //     env!("CARGO_MANIFEST_DIR")
    // );
    // let config = DisplayConfig::load(&path);
    let app_root = application_root_dir()?;
    let resources = app_root.join("../resources");
    let display_config = resources.join("display_config.ron");

    // let pipe = Pipeline::build().with_stage(
    //     Stage::with_backbuffer()
    //         .clear_target([0., 0., 0., 1.0], 1.0)
    //         .with_pass(DrawFlat::<PosTex>::new())
    //         .with_pass(DrawUi::new()),
    // );

    let game_data = GameDataBuilder::default()
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(DefaultPhysicsBundle3::<ObjectType>::new().with_spatial())?
        .with_bundle(BoxSimulationBundle3::new(Cuboid::new(0.1, 0.1, 0.1).into()))?
        .with_bundle(TransformBundle::new().with_dep(&["sync_system", "emission_system"]))?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?;

    let mut game = Application::new("./", Emitting::default(), game_data)?;

    game.run();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error occurred during game execution: {}", e);
        ::std::process::exit(1);
    }
}
