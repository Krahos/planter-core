//! This is a library with types and behaviour inspired by the PMBOK Guide 7th edition.

/// A duration is a unit of time that represents the amount of time required to complete a task.
pub mod duration;
/// A person can either be a resource, a team member or a stakeholder.
pub mod person;
/// A project is a set of activities required to transform ideas into reality.
/// It is composed by tasks, team, stakeholders, resources and so on.
/// It is usually divided into different phases:
///     - Feasibility, where the project is evaluated to understand if the project is valid, and the organization is capable of delivering the expected outcome.
///     - Design. Planning and analysis lead to the design of the project deliverable that will be developed.
///     - Build. Construction of the deliverable with integrated quality assurance activities is conducted.
///     - Test. Final quality review and inspection of deliverables are carried out before transition, go-live, or acceptance by the customer.
///     - Deploy. Project deliverables are put into use and transitional activities required for sustainment, benefits realization, and organizational change management are completed.
///     - Close. The project is closed, project knowledge and artifacts are archived, project team members are released, and contracts are closed.
/// Depending on the kind of project (adaptive, predictive, hybrid), these phases can occur one after the other sequentially or in a loop.
/// In a predictive approach, each phase comes right after the previous one. In an adaptive approach, there are blocks of planning, design, build that go one after the other, until the project is done. At the end of each iteration (sprint), there is feedback on the work done, and a prioritisation of the backlog.
pub mod project;
/// A resource is a material required in the project to carry out tasks and provide deliverables. Resources have a cost, the sum of which, will concur with the total cost of the project. Project team members can also be considered resources.
pub mod resources;
/// A stakeholder is a person or organization that has an interest in the project. Stakeholders have a level of influence and a level of interest in the project.
pub mod stakeholders;
/// A task is a unit of work that needs to be completed in order to achieve the project's objectives. Tasks have a duration, a start date, an end date, and a status.
pub mod task;
/// Tasks, within a project, are have various dependencies between them:
///     - Time relationships (Start to finish, Finish to start, ...).
///     - Subtask relationships (EG: TaskA has Task1 and Task2 as subtasks).
/// These relationships are covered here.
pub mod tasks;
