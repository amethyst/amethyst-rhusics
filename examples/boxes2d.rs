
use std::time::{Duration, Instant};

use amethyst::assets::{Handle, Loader};
use amethyst::core::nalgebra as na;
use amethyst::core::{Transform, TransformBundle};
use amethyst::ecs::prelude::{Builder, Entity, World};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::{
    Application, Config, GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData,
    StateEvent, Trans,
};
use amethyst::renderer::{
    Camera, DisplayConfig, DrawFlat, Material, MaterialDefaults, Mesh, Pipeline, PosTex,
    RenderBundle, Stage, VirtualKeyCode,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::fps_counter::FPSCounterBundle;
use amethyst_rhusics::{setup_2d_arena, time_sync, DefaultPhysicsBundle2};
use cgmath::Point2;
use collision::primitive::{Primitive2, Rectangle};
use collision::Aabb2;
use rhusics_core::CollisionShape;
use rhusics_ecs::physics2d::BodyPose2;

use crate::boxes::{
    create_ui, update_ui, BoxSimulationBundle2, Emitter, Graphics, KillRate, ObjectType,
};

mod boxes;

#[derive(Default)]
pub struct Emitting {
    fps: Option<Entity>,
    num: Option<Entity>,
    collision: Option<Entity>,
}

pub type Shape = CollisionShape<Primitive2<f32>, BodyPose2<f32>, Aabb2<f32>, ObjectType>;

impl SimpleState for Emitting {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.write_resource::<KillRate>().0 = 0.01;
        initialise_camera(world);
        let g = Graphics {
            mesh: initialise_mesh(world),
        };
        let (num_display, fps_display, collisions_display) = create_ui(world);
        self.num = Some(num_display);
        self.fps = Some(fps_display);
        self.collision = Some(collisions_display);

        world.add_resource(g);
        setup_2d_arena(
            Point2::new(-1., -1.),
            Point2::new(1., 1.),
            (
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
        update_ui::<Point2<f32>>(
            data.world,
            self.num.unwrap(),
            self.fps.unwrap(),
            self.collision.unwrap(),
        );
        data.data.update(data.world);
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    world
        .create_entity()
        .with(Camera::standard_2d())
        .with(Transform::from(na::Vector3::new(0., 0., 5.)))
        .build();
}

fn initialise_mesh(world: &mut World) -> Handle<Mesh> {
    use genmesh::generators::Cube;
    use genmesh::{MapToVertices, Triangulate, Vertices};
    let vertices = Cube::new()
        .vertex(|v| PosTex {
            position: v.pos.into(),
            tex_coord: na::Vector2::new(0.1, 0.1),
        }).triangulate()
        .vertices()
        .collect::<Vec<_>>();
    world
        .read_resource::<Loader>()
        .load_from_data(vertices.into(), (), &world.read_resource())
}

fn initialise_material(world: &mut World, r: f32, g: f32, b: f32) -> Material {
    let albedo = world.read_resource::<Loader>().load_from_data(
        [r, g, b, 1.0].into(),
        (),
        &world.read_resource(),
    );
    Material {
        albedo,
        ..world.read_resource::<MaterialDefaults>().0.clone()
    }
}

fn emitter(p: Point2<f32>, d: Duration, material: Material) -> Emitter<Point2<f32>> {
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
            Point2::new(-0.4, 0.),
            Duration::new(0, 50_000_000),
            mat,
        )).build();

    let mat = initialise_material(world, 0.3, 0.0, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point2::new(0.4, 0.),
            Duration::new(0, 75_000_000),
            mat,
        )).build();

    let mat = initialise_material(world, 1.0, 1.0, 1.0);
    world
        .create_entity()
        .with(emitter(
            Point2::new(0., -0.4),
            Duration::new(0, 100_000_000),
            mat,
        )).build();

    let mat = initialise_material(world, 1.0, 0.3, 0.3);
    world
        .create_entity()
        .with(emitter(
            Point2::new(0., 0.4),
            Duration::new(1, 25_000_000),
            mat,
        )).build();
}

fn run() -> Result<(), amethyst::Error> {
    amethyst::start_logger(amethyst::LoggerConfig::default());
    let path = format!(
        "{}/../resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new())
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(FPSCounterBundle::default())?
        .with_bundle(DefaultPhysicsBundle2::<ObjectType>::new().with_spatial())?
        .with_bundle(BoxSimulationBundle2::new(Rectangle::new(0.1, 0.1).into()))?
        .with_bundle(TransformBundle::new().with_dep(&["sync_system", "emission_system"]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

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
