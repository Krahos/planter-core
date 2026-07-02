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
    let start_date = Utc::now();
    let mut project = Project::builder()
        .name("World domination")
        .description("My second attempt to conquer the world with a crowbar and a stimpack")
        .start_date(start_date)
        .build();

    let crowbar_id = project.add_task(Task::new("Find a crowbar"));
    let stimpack_id = project.add_task(Task::new("Find a stimpack"));
    let software_id = project.add_task(Task::new("Open a proprietary software house"));
    let prey_id = project.add_task(Task::new("Prey on free software projects"));
    let profit_id = project.add_task(Task::new("Profit"));
    assert_eq!(project.tasks().count(), 5);

    project
        .add_subtask(stimpack_id, software_id)
        .context("Failed to add subtask")?;
    project
        .add_subtask(stimpack_id, prey_id)
        .context("Failed to add subtask")?;

    assert_eq!(project.subtasks(stimpack_id).count(), 2);

    project
        .add_time_relationship(crowbar_id, stimpack_id, TimeRelationship::StartToFinish)
        .context("Tasks don't exist or circular dependencies detected")?;

    project
        .add_time_relationship(crowbar_id, profit_id, TimeRelationship::StartToFinish)
        .context("Tasks don't exist or circular dependencies detected")?;

    let mut crowbar_mat = Material::Consumable(Consumable::new("Crowbar"));
    crowbar_mat.update_quantity(5);
    crowbar_mat.update_cost_per_unit(150);
    project.add_resource(Resource::Material(crowbar_mat));

    project
        .res_into_nonconsumable(0)
        .context("Failed to convert consumable into non consumable")?;
    if let Resource::Material(ref m) = project.resources()[0] {
        assert_eq!(m.name(), "Crowbar");
        assert_eq!(m.quantity(), Some(5));
        assert_eq!(m.cost_per_unit(), Some(150));
    } else {
        panic!("Expected Material after conversion");
    }

    project.add_resource(Resource::Material(Material::NonConsumable(
        NonConsumable::new("Stimpack"),
    )));

    project
        .res_into_consumable(0)
        .context("Failed to convert non consumable into consumable")?;
    if let Resource::Material(ref m) = project.resources()[0] {
        assert_eq!(m.name(), "Crowbar");
        assert_eq!(m.quantity(), Some(5));
        assert_eq!(m.cost_per_unit(), Some(150));
    } else {
        panic!("Expected Material after conversion");
    }

    project.add_resource(Resource::Personnel {
        person: Person::new("Sebastiano", "Giordano").context("Failed to parse a name.")?,
        hourly_rate: None,
    });
    assert_eq!(project.resources().len(), 3);

    let _ = project.rm_resource(1);
    assert_eq!(project.resources().len(), 2);

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
