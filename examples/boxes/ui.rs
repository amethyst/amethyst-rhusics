use super::{Collisions, Emitter};
use amethyst::assets::Loader;
use amethyst::core::{Parent, Time};
use amethyst::ecs::prelude::{Builder, Entity, Join, World, WorldExt};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::utils::fps_counter::FpsCounter;

#[allow(unused)]
pub fn create_ui(world: &mut World) -> (Entity, Entity, Entity) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let num_display = world
        .create_entity()
        .with(UiTransform::new(
            "num".to_string(),
            Anchor::TopLeft,
            Anchor::Middle,
            100.,
            25.,
            1.,
            200.,
            50.,
        )).with(UiText::new(
            font.clone(),
            "N/A".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            25.,
        )).build();

    let fps_display = world
        .create_entity()
        .with(UiTransform::new(
            "fps".to_string(),
            Anchor::BottomLeft,
            Anchor::Middle,
            100.,
            0.,
            1.,
            200.,
            50.,
        )).with(UiText::new(
            font.clone(),
            "N/A".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            25.,
        )).with(Parent {
            entity: num_display,
        }).build();

    let collisions_display = world
        .create_entity()
        .with(UiTransform::new(
            "collisions".to_string(),
            Anchor::BottomLeft,
            Anchor::Middle,
            100.,
            0.,
            1.,
            200.,
            50.,
        )).with(UiText::new(
            font.clone(),
            "N/A".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            25.,
        )).with(Parent {
            entity: fps_display,
        }).build();
    (num_display, fps_display, collisions_display)
}

#[allow(unused)]
pub fn update_ui<P>(
    world: &mut World,
    num_entity: Entity,
    fps_entity: Entity,
    collision_entity: Entity,
) where
    P: Send + Sync + 'static,
{
    let frame_number = world.read_resource::<Time>().frame_number();
    let fps = world.read_resource::<FpsCounter>().sampled_fps();

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
