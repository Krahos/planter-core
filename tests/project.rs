//! Module containing tests for the `Project` struct.

use anyhow::Context;
use chrono::Utc;
use planter_core::{
    person::Person,
    project::{Project, TimeRelationship},
    resources::{Consumable, Material, NonConsumable, Resource},
    stakeholders::Stakeholder,
    task::Task,
};

#[test]
/// The standard workflow when creating a project involves initializing it with
/// a name, optionally a description, and a start date. The project is kept
/// mutable and the user can add/remove tasks, resources, stakeholders, and
/// other relevant information.
fn test_project() -> anyhow::Result<()> {
    // Initialize a project with a name, description, and start date.
    let start_date = Utc::now();
    let mut project = Project::builder()
        .name("World domination")
        .description("My second attempt to conquer the world with a crowbar and a stimpack")
        .start_date(start_date)
        .build();

    // Add tasks to the project.
    project.add_task(Task::new("Find a crowbar"));
    project.add_task(Task::new("Find a stimpack"));
    project.add_task(Task::new("Open a proprietary software house"));
    project.add_task(Task::new("Prey on free software projects"));
    project.add_task(Task::new("Profit"));
    assert_eq!(project.tasks().count(), 5);

    // Add subtask relatonships to the project.
    project.add_subtask(1, 2);
    project.add_subtask(1, 3);

    assert_eq!(project.subtasks(1).len(), 2);

    // Add time relationships between tasks of the project.
    project
        .add_time_relationship(0, 1, TimeRelationship::StartToFinish)
        .context("Tasks don't exist or circular dependencies detected")?;

    project
        .add_time_relationship(0, 4, TimeRelationship::StartToFinish)
        .context("Tasks don't exist or circular dependencies detected")?;

    // Add a non consumable material to the project
    project.add_resource(Resource::Material(Material::NonConsumable(
        NonConsumable::new("Crowbar"),
    )));
    // Add a consumable material to the project
    project.add_resource(Resource::Material(Material::Consumable(Consumable::new(
        "Stimpack",
    ))));

    // Add a personnel resource to the project
    project.add_resource(Resource::Personnel {
        person: Person::new("Sebastiano", "Giordano").context("Failed to parse a name.")?,
        hourly_rate: None,
    });
    assert_eq!(project.resources().len(), 3);

    // Add stakeholders to the project
    let person = Person::new("Margherita", "Hack").context("Failed to parse a name")?;
    project.add_stakeholder(Stakeholder::Individual {
        person,
        description: Some("She could try to stop me".to_owned()),
    });
    project.add_stakeholder(Stakeholder::Organization {
        name: "Acme".to_owned(),
        description: Some("They might decide to buy me more stimpacks".to_owned()),
    });
    assert_eq!(project.stakeholders().len(), 2);

    Ok(())
}
