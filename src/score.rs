use amethyst::{
    assets::Loader,
    ecs::{Builder, Entity, World},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

#[derive(Default)]
pub struct ScoreBoard {
    pub left: u32,
    pub right: u32,
}

pub struct ScoreText {
    pub left: Entity,
    pub right: Entity,
}

impl ScoreBoard {}

pub fn init_entities(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "resources/square.ttf",
        TtfFormat,
        (),
        (),
        &world.read_resource(),
    );

    let left_pos = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
        0,
    );
    let right_pos = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        50.,
        -50.,
        1.,
        200.,
        50.,
        0,
    );

    let left = world
        .create_entity()
        .with(left_pos)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    let right = world
        .create_entity()
        .with(right_pos)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();

    world.add_resource(ScoreText { left, right });
}
