use bevy::ecs::query::QueryData;
use bevy::prelude::*;

const INVENTORY_WIDTH: f32 = 50.0;
const INVENTORY_HEIGHT: f32 = 25.0;
const INVENTORY_COLOR: Color = Color::srgb(0.0, 255.0, 0.0);
const INVENTORY_Y: f32 = -Y_EXTENT + ((INVENTORY_HEIGHT / 2.0) + 2.0);
const INVENTORY_Z: f32 = 3.0;
const Y_EXTENT: f32 = 250.0;
const CLEANUP_VELOCITY: f32 = 500.0; // pixels per second

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Food;

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Milk;

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct LitterScoop;

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Litter;

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Toy;

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Supplies;

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Dragging(pub bool);

#[derive(Component, Copy, Clone, PartialEq, Default, Debug)]
pub struct OgPos(pub Vec2);

#[derive(Component, Copy, Clone, PartialEq, PartialOrd, Default, Debug)]
pub struct Velocity(pub f32);

pub fn make_supplies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Food,
        -2.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Milk,
        -1.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        LitterScoop,
        0.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Litter,
        1.0 * (INVENTORY_WIDTH + 2.0),
    );
    add_supply(
        &mut commands,
        &mut meshes,
        &mut materials,
        Toy,
        2.0 * (INVENTORY_WIDTH + 2.0),
    );
}

fn add_supply<'a, 'b, C: Component + Clone + Copy>(
    commands: &mut Commands<'a, 'b>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    component: C,
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

fn drag_supply(trigger: Trigger<Pointer<Drag>>, mut query: Query<&mut Transform>) {
    let transform = query.get_mut(trigger.target()).unwrap();
    let drag = trigger.event();
    let mut translation = transform.map_unchanged(|t| &mut t.translation);
    translation.x += drag.delta.x;
    translation.y -= drag.delta.y;
}

fn set_click_state(trigger: Trigger<Pointer<DragStart>>, mut query: Query<&mut Dragging>) {
    let mut item = query.get_mut(trigger.target()).unwrap();
    item.0 = true;
}

fn unset_click_state(trigger: Trigger<Pointer<DragEnd>>, mut query: Query<&mut Dragging>) {
    let mut item = query.get_mut(trigger.target()).unwrap();
    item.0 = false;
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct DragQuery {
    pub transform: Mut<'static, Transform>,
    pub orig_pos: &'static OgPos,
    pub dragging: &'static Dragging,
    pub vel: &'static Velocity, // Pixels per second?
}

pub fn move_back(time: Res<Time>, mut query: Query<DragQuery>) {
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
        println!("Putting supplies back");

        let total_distance = ((x_1 - x_0).powf(2.0) + (y_1 - y_0).powf(2.0)).sqrt();
        let target_distance = item.vel.0 * time.delta().as_secs_f32();
        let distance_ratio = target_distance / total_distance;

        println!(
            "Distance ratio is {distance_ratio}. The target distance is {target_distance}, and the total distance is {total_distance}."
        );

        if total_distance < 3.0 {
            let mut translation = item.transform.map_unchanged(|t| &mut t.translation);
            println!("Moving to the end");
            translation.x = x_0;
            translation.y = y_0;
            continue;
        }

        let x_2 = (1.0 - distance_ratio) * x_1 + (distance_ratio * x_0);
        let y_2 = (1.0 - distance_ratio) * y_1 + (distance_ratio * y_0);
        println!("Moving to {x_2} and {y_2}");
        let mut translation = item.transform.map_unchanged(|t| &mut t.translation);
        translation.x = x_2;
        translation.y = y_2;
    }
}
