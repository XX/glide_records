use bevy::{
    app::App,
    ecs::{
        query::With,
        system::{Commands, IntoSystem, Query},
    },
};

struct Person;

struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".into()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".into()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".into()));
}

fn hello_world() {
    println!("hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}
