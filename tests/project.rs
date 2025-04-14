//! Module containing tests for the `Project` struct.

#![allow(clippy::unwrap_used)]

use chrono::Utc;
use planter_core::{
    person::{Name, Person},
    project::{Project, TaskRelationship},
    resources::{Consumable, Material, NonConsumable, Resource},
    stakeholders::Stakeholder,
    task::Task,
};

#[test]
/// The standard workflow when creating a project involves initializing it with a name, optionally a description, and a start date.
/// The project is kept mutable and the user can add/remove tasks, resources, stakeholders, and other relevant information.
fn test_project() {
    // Initialize a project with a name, description, and start date.
    let start_date = Utc::now();
    let mut project = Project::new("World domination".to_string())
        .with_description(
            "My second attempt to conquer the world with a crowbar and a stimpack".to_string(),
        )
        .with_start_date(start_date);

    // Add tasks to the project
    project.add_task(Task::new("Become world leader".to_string()));
    project.add_task(Task::new("Profit".to_string()));
    assert_eq!(project.tasks().count(), 2);

    // Add relationships between tasks of the project.
    project
        .add_relationship(0, 1, TaskRelationship::StartToFinish)
        .expect("Tasks to exist and circular dependencies not present");

    // Add a non consumable material to the project
    project.add_resource(Resource::Material(Material::NonConsumable(
        NonConsumable::new("Crowbar".to_string()),
    )));
    // Add a consumable material to the project
    project.add_resource(Resource::Material(Material::Consumable(Consumable::new(
        "Stimpack".to_string(),
    ))));

    // Add a personnel resource to the project
    project.add_resource(Resource::Personnel {
        person: Person::new(Name::parse("Sebastiano".to_string(), "Giordano".to_string()).unwrap()),
        hourly_rate: None,
    });
    assert_eq!(project.resources().len(), 3);

    // Add stakeholders to the project
    let person = Person::new(Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap());
    project.add_stakeholder(Stakeholder::Individual {
        person,
        description: Some("She could try to stop me".to_string()),
    });
    project.add_stakeholder(Stakeholder::Organization {
        name: "Acme".to_string(),
        description: Some("They might decide to buy me more stimpacks".to_string()),
    });
    assert_eq!(project.stakeholders().len(), 2);
}
