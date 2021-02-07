use bevy::{app::AppExit, prelude::*};
use rand::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InitPlugin)
        .run();
}

// COMPONENTS
struct Ship {
    name: String,
}

struct Health {
    value: usize,
}

struct Shield(usize);

// RESOURCES

#[derive(Default)]
struct GameState {
    alive_ships: usize,
}

// PLUGINS
pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<GameState>()
            .add_startup_system(startup_system.system())
            .add_system(health_check_system.system())
            .add_system(drop_ship_system.system())
            .add_system(game_over_system.system());
    }
}

// SYSTEMS

fn health_check_system(game_state: Res<GameState>, mut query: Query<(&Ship, &Shield, &mut Health)>) {
    if game_state.alive_ships == 1 {
        return
    }

    println!("=====Battle!======");
    for (ship, shield, mut health) in query.iter_mut() {
        let got_damage = random::<bool>();
        let protected_by_shield = shield.0 >= thread_rng().gen_range(1..=10);

        if got_damage && !protected_by_shield {
            health.value -= 1;
            println!("Ship {} received a damage! Their health is: {}",
                ship.name, health.value);
        } else {
            println!("Ship {} didn't receive any damage. The health is {}",
                ship.name, health.value);
        }
    }
}

fn drop_ship_system(
    commands: &mut Commands,
    mut game_state: ResMut<GameState>,
    query: Query<(Entity, &Ship, &Health)>,
) {
    let mut alive_ships: usize = 0;
    for (ent, ship, health) in query.iter() {
        if health.value <= 0 {
            println!("+++ Ship {} has explosed! +++", ship.name);
            commands.despawn(ent);
        } else {
            alive_ships += 1;
        }
    }

    game_state.alive_ships = alive_ships;
}

fn game_over_system(
    query: Query<(&Ship, &Health)>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    let mut alive_count = 0;
    let mut alive_ship: Option<(&Ship, i32)> = None;
    for (ship, _) in query.iter() {
        alive_count += 1;
        alive_ship = Some((ship, alive_count));
    }

    match alive_ship {
        Some((ship, 1)) => {
            println!("*** Ship {} won the battle! ***", ship.name);
            app_exit_events.send(AppExit);
        },
        Some((_, _)) => (),
        None => {
            println!("All dead!\n");
            app_exit_events.send(AppExit);
        },
    }
}

fn startup_system(commands: &mut Commands, mut game_state: ResMut<GameState>) {
    commands.spawn_batch(vec![
        (
            Ship {
                name: "Mad Sheep".to_string(),
            },
            Health { value: 5 },
            Shield(8),
        ),
        (
            Ship {
                name: "Strong Viking".to_string(),
            },
            Health { value: 8 },
            Shield(3),
        ),
    ]);

    println!("\nStart!");
    game_state.alive_ships = 2;
}