use amethyst::ecs::prelude::{Entity, Join, World};
use amethyst::assets::Loader;
use amethyst::ui::{Anchor, Anchored, TtfFormat, UiText, UiTransform};
use amethyst::core::{Parent, Time};
use amethyst::utils::fps_counter::FPSCounter;
use super::{Collisions, Emitter};

pub fn create_ui(world: &mut World) -> (Entity, Entity, Entity) {
    let font = world.read_resource::<Loader>().load(
        "examples/resources/font/square.ttf",
        TtfFormat,
        (),
        (),
        &world.read_resource(),
    );
    let num_display = world
        .create_entity()
        .with(UiTransform::new(
            "num".to_string(),
            100.,
            25.,
            1.,
            200.,
            50.,
            0,
        ))
        .with(UiText::new(
            font.clone(),
            "N/A".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            25.,
        ))
        .with(Anchored::new(Anchor::TopLeft))
        .build();

    let fps_display = world
        .create_entity()
        .with(UiTransform::new(
            "fps".to_string(),
            100.,
            0.,
            1.,
            200.,
            50.,
            0,
        ))
        .with(UiText::new(
            font.clone(),
            "N/A".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            25.,
        ))
        .with(Parent {
            entity: num_display,
        })
        .with(Anchored::new(Anchor::BottomLeft))
        .build();

    let collisions_display = world
        .create_entity()
        .with(UiTransform::new(
            "collisions".to_string(),
            100.,
            0.,
            1.,
            200.,
            50.,
            0,
        ))
        .with(UiText::new(
            font.clone(),
            "N/A".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            25.,
        ))
        .with(Parent {
            entity: fps_display,
        })
        .with(Anchored::new(Anchor::BottomLeft))
        .build();
    (num_display, fps_display, collisions_display)
}

pub fn update_ui<P>(
    world: &mut World,
    num_entity: Entity,
    fps_entity: Entity,
    collision_entity: Entity,
) where
    P: Send + Sync + 'static,
{
    let frame_number = world.read_resource::<Time>().frame_number();
    let fps = world.read_resource::<FPSCounter>().sampled_fps();

    if frame_number % 20 == 0 {
        if let Some(fps_display) = world.write_storage::<UiText>().get_mut(fps_entity) {
            fps_display.text = format!("FPS: {:.*}", 2, fps);
        }
        let emitted: u64 = (&world.read_storage::<Emitter<P>>())
            .join()
            .map(|e| e.emitted)
            .sum();
        if let Some(num_display) = world.write_storage::<UiText>().get_mut(num_entity) {
            num_display.text = format!("Num: {:.*}", 2, emitted);
        }
        let collision = world.read_resource::<Collisions>().0;
        if let Some(collision_display) = world.write_storage::<UiText>().get_mut(collision_entity) {
            collision_display.text = format!("Coll: {:.*}", 2, collision);
        }
    }
}
