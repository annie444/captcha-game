use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use std::fmt;

const INVENTORY_WIDTH: f32 = 50.0;
const INVENTORY_HEIGHT: f32 = 25.0;
const INVENTORY_COLOR: Color = Color::srgb(0.0, 255.0, 0.0);
const INVENTORY_Y: f32 = -Y_EXTENT + ((INVENTORY_HEIGHT / 2.0) + 2.0);
const INVENTORY_Z: f32 = 3.0;
const Y_EXTENT: f32 = 250.0;
const CLEANUP_VELOCITY: f32 = 500.0; // pixels per second

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub(crate) struct Supplies(pub(crate) Supply);

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub(crate) enum Supply {
    Food,
    Milk,
    LitterScoop,
    Litter,
    Toy,
}

impl fmt::Display for Supply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Food => write!(f, "food"),
            Self::Milk => write!(f, "milk"),
            Self::Toy => write!(f, "toy"),
            Self::LitterScoop => write!(f, "litter scoop"),
            Self::Litter => write!(f, "litter"),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub(crate) struct Dragging(pub(crate) bool);

#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub(crate) struct OgPos(pub(crate) Vec2);

#[derive(Component, Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub(crate) struct Velocity(pub(crate) f32);

pub(crate) fn make_supplies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Supplies(Supply::Food),
        -2.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Supplies(Supply::Milk),
        -1.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Supplies(Supply::LitterScoop),
        0.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Supplies(Supply::Litter),
        1.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Supplies(Supply::Toy),
        2.0 * (INVENTORY_WIDTH + 2.0),
    );
}

fn add_supply<'a, 'b>(
    commands: &mut Commands<'a, 'b>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    component: Supplies,
    x_position: f32,
) {
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(INVENTORY_WIDTH, INVENTORY_HEIGHT))),
            MeshMaterial2d(materials.add(INVENTORY_COLOR)),
            Transform::from_xyz(x_position, INVENTORY_Y, INVENTORY_Z),
            component,
            Dragging(false),
            OgPos(Vec2::new(x_position, INVENTORY_Y)),
            Velocity(CLEANUP_VELOCITY),
        ))
        .observe(set_click_state)
        .observe(drag_supply)
        .observe(unset_click_state);
}

fn drag_supply(trigger: Trigger<Pointer<Drag>>, mut query: Query<&mut Transform, With<Supplies>>) {
    let transform = query.get_mut(trigger.target()).unwrap();
    let drag = trigger.event();
    let mut translation = transform.map_unchanged(|t| &mut t.translation);
    translation.x += drag.delta.x;
    translation.y -= drag.delta.y;
}

fn set_click_state(
    trigger: Trigger<Pointer<DragStart>>,
    mut query: Query<&mut Dragging, With<Supplies>>,
) {
    let mut item = query.get_mut(trigger.target()).unwrap();
    item.0 = true;
}

fn unset_click_state(
    trigger: Trigger<Pointer<DragEnd>>,
    mut query: Query<&mut Dragging, With<Supplies>>,
) {
    let mut item = query.get_mut(trigger.target()).unwrap();
    item.0 = false;
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct DragQuery {
    pub(crate) transform: Mut<'static, Transform>,
    pub(crate) orig_pos: &'static OgPos,
    pub(crate) dragging: &'static Dragging,
    pub(crate) vel: &'static Velocity, // Pixels per second?
    pub(crate) supply: &'static Supplies,
}

pub(crate) fn move_back(time: Res<Time>, mut query: Query<DragQuery>) {
    for item in query.iter_mut() {
        if item.dragging.0
            || (item.orig_pos.0.x == item.transform.translation.x
                && item.orig_pos.0.y == item.transform.translation.y)
        {
            continue;
        }
        let x_0 = item.orig_pos.0.x;
        let y_0 = item.orig_pos.0.y;
        let x_1 = item.transform.translation.x;
        let y_1 = item.transform.translation.y;

        let total_distance = ((x_1 - x_0).powf(2.0) + (y_1 - y_0).powf(2.0)).sqrt();
        let target_distance = item.vel.0 * time.delta().as_secs_f32();
        let distance_ratio = target_distance / total_distance;

        if total_distance < 3.0 {
            let mut translation = item.transform.map_unchanged(|t| &mut t.translation);
            translation.x = x_0;
            translation.y = y_0;
            continue;
        }

        let x_2 = (1.0 - distance_ratio) * x_1 + (distance_ratio * x_0);
        let y_2 = (1.0 - distance_ratio) * y_1 + (distance_ratio * y_0);
        let mut translation = item.transform.map_unchanged(|t| &mut t.translation);
        translation.x = x_2;
        translation.y = y_2;
    }
}
