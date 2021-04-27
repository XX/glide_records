use bevy::{
    app::{AppBuilder, Plugin},
    core::{Time, Timer},
    ecs::{
        query::With,
        system::{Commands, IntoSystem, Query, Res, ResMut},
    },
};

struct Person;

struct Name(String);

struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".into()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".into()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".into()));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
