use legion::systems::Builder;

pub mod chase;

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    chase::setup_systems(builder)
}
