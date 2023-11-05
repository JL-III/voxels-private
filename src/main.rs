use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PeoplePlugin)
    .run()
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system(print_names)
        .add_system(people_with_jobs);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
    Person {
        name: "Alex".to_string(),
    },
    Employed { job: Job::Doctor },
    ));
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn(Person {
        name: "Charlie".to_string(),
    });
    commands.spawn(Person {
        name: "David".to_string(),
    });
    commands.spawn(Person {
        name: "Ellen".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name)
    }
}

pub fn people_with_jobs(
    person_query: Query<&Person, With<Employed>>
) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    FireFighter,
    Lawyer,
}