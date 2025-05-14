use anyhow::Context;
use chrono::{DateTime, Utc};

use crate::{
    resources::Resource,
    stakeholders::Stakeholder,
    task::Task,
    tasks::{Tasks, TimeRelationship},
};

#[derive(Debug, Default)]
/// Represents a project with a name and a list of resources.
pub struct Project {
    /// The name of the project.
    name: String,
    /// The description of the project.
    description: Option<String>,
    /// The start date of the project.
    start_date: Option<DateTime<Utc>>,
    /// The tasks associated with the project.
    tasks: Tasks,
    /// The list of resources associated with the project.
    resources: Vec<Resource>,
    /// The list of stakeholders associated with the project.
    stakeholders: Vec<Stakeholder>,
}

impl Project {
    /// Creates a new project with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project.
    ///
    /// # Returns
    ///
    /// A new `Project` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    ///
    /// let project = Project::new("World domination".to_owned());
    /// assert_eq!(project.name(), "World domination");
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            start_date: None,
            resources: Vec::new(),
            stakeholders: Vec::new(),
            tasks: Tasks::new(),
        }
    }

    /// Returns the name of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    ///
    /// let project = Project::new("World domination".to_owned());
    /// assert_eq!(project.name(), "World domination");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Adds a description to the project.
    ///
    /// # Arguments
    ///
    /// * `description` - The description to add to the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    ///
    /// let project = Project::new("World domination".to_owned()).with_description("This is a project".to_owned());
    /// assert_eq!(project.description(), Some("This is a project"));
    /// ```
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Adds a start date to the project.
    ///
    /// # Arguments
    ///
    /// * `start_date` - The start date to add to the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    /// use chrono::Utc;
    ///
    /// let start_date = Utc::now();
    /// let project = Project::new("World domination".to_owned()).with_start_date(start_date);
    /// assert_eq!(project.start_date(), Some(start_date));
    /// ```
    pub fn with_start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// Returns the description of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    ///
    /// let project = Project::new("World domination".to_owned());
    /// assert_eq!(project.description(), None);
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Adds a task to the project.
    ///
    /// # Arguments
    ///
    /// * `task` - An already created task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let task = Task::new("Become world leader".to_owned());
    /// let project = Project::new("World domination".to_owned()).with_task(task);
    ///
    /// assert_eq!(project.task(0).unwrap().name(), "Become world leader");
    ///
    /// ```
    pub fn with_task(mut self, task: Task) -> Self {
        self.tasks.add_task(task);
        self
    }

    /// Adds a collection of tasks to the project.
    ///
    /// # Arguments
    ///
    /// * `start_date` - The start date to add to the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let tasks = vec![Task::new("Become world leader".to_owned()), Task::new("Get rich".to_owned())];
    /// let project = Project::new("World domination".to_owned()).with_tasks(tasks);
    /// assert_eq!(project.tasks().count(), 2);
    /// ```
    pub fn with_tasks(self, tasks: impl IntoIterator<Item = Task>) -> Self {
        tasks.into_iter().fold(self, Self::with_task)
    }

    /// Adds a task to the project.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to add to the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// assert_eq!(project.tasks().count(), 0);
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(project.tasks().count(), 1);
    /// ```
    pub fn add_task(&mut self, task: Task) {
        self.tasks.add_task(task);
    }

    /// Deletes a task and all references to it from the project.
    ///
    /// # Arguments
    ///
    /// * `i` - The index of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(project.tasks().count(), 1);
    /// assert!(project.rm_task(0).is_ok());
    /// assert_eq!(project.tasks().count(), 0);
    /// ```
    pub fn rm_task(&mut self, i: usize) -> anyhow::Result<()> {
        self.tasks
            .rm_task(i)
            .context("Tried removing a non existing node from Dag")?;
        Ok(())
    }

    /// Gets  a reference to the task with the given index from the project.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to identify the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(project.task(0).unwrap().name(), "Become world leader".to_owned());
    /// ```
    pub fn task(&self, index: usize) -> Option<&Task> {
        self.tasks.task(index)
    }

    /// Gets a mutable reference to the task with the given index from the project.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to identify the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// let task = project.task_mut(0).unwrap();
    /// assert_eq!(task.name(), "Become world leader");
    ///
    /// task.edit_name("Become world's biggest loser".to_owned());
    /// assert_eq!(task.name(), "Become world's biggest loser".to_owned())
    /// ```
    pub fn task_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.tasks.task_mut(index)
    }

    /// Returns the tasks of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(project.tasks().count(), 1);
    /// ```
    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.tasks()
    }

    /// Returns a mutable reference to the tasks of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(project.tasks().count(), 1);
    /// ```
    pub fn tasks_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        self.tasks.tasks_mut()
    }

    /// Adds a relationship betwen tasks, where one is the predecessor and the other one a successor.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project}, tasks::TimeRelationship, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(0, 1, TimeRelationship::default());
    ///
    /// assert_eq!(project.successors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn add_relationship(
        &mut self,
        predecessor_index: usize,
        successor_index: usize,
        kind: TimeRelationship,
    ) -> anyhow::Result<()> {
        self.tasks
            .add_relationship(predecessor_index, successor_index, kind)
            .context("Tried to add a relationship between non existing nodes")?;
        anyhow::Ok(())
    }

    /// Removes a relationship betwen tasks, where one is the predecessor and the other one a successor.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, tasks::TimeRelationship, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(0, 1, TimeRelationship::default());
    /// project.remove_relationship(0, 1);
    ///
    /// assert_eq!(project.successors(0).count(), 0);
    /// ```
    pub fn remove_relationship(
        &mut self,
        predecessor_index: usize,
        successor_index: usize,
    ) -> anyhow::Result<()> {
        self.tasks
            .remove_relationship(predecessor_index, successor_index)
            .context("Failed to remove relationship from project")?;

        anyhow::Ok(())
    }

    /// Gets the list of successors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, tasks::TimeRelationship, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(0, 1, TimeRelationship::default());
    ///
    /// assert_eq!(project.successors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn successors(&self, node_index: usize) -> impl Iterator<Item = &Task> {
        self.tasks.successors(node_index)
    }

    /// Gets the indices of all successors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, tasks::TimeRelationship, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(0, 1, TimeRelationship::default());
    ///
    /// assert_eq!(project.successors_indices(0).next().unwrap(), 1)
    /// ```
    pub fn successors_indices(&self, node_index: usize) -> impl Iterator<Item = usize> {
        self.tasks.successors_indices(node_index)
    }

    /// Gets the list of predecessors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, tasks::TimeRelationship, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(1, 0, TimeRelationship::default());
    ///
    /// assert_eq!(project.predecessors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn predecessors(&self, node_index: usize) -> impl Iterator<Item = &Task> {
        self.tasks.predecessors(node_index)
    }

    /// Gets the indices of all predecessors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, tasks::TimeRelationship, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(1, 0, TimeRelationship::default());
    ///
    /// assert_eq!(project.predecessors_indices(0).next().unwrap(), 1)
    /// ```
    pub fn predecessors_indices(&self, node_index: usize) -> impl Iterator<Item = usize> {
        self.tasks.predecessors_indices(node_index)
    }

    /// Updates the project by making sure the predecessors for the task with
    /// index `node_index` are exactly the ones listed in `predecessors_indices`
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index whose predecessors need to be updated.
    /// * `predecessors_indices` - The indices of the predecessors.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let tasks = vec![
    ///      Task::new("Become world leader".to_owned()),
    ///      Task::new("Get rich".to_owned()),
    ///      Task::new("Be evil".to_owned())
    /// ];
    /// let mut project = Project::new("World domination".to_owned()).with_tasks(tasks);
    ///
    /// project.update_predecessors(2, &[0, 1]);
    /// assert_eq!(project.predecessors(2).count(), 2);
    pub fn update_predecessors(
        &mut self,
        task_index: usize,
        predecessors_indices: &[usize],
    ) -> anyhow::Result<()> {
        self.tasks
            .update_predecessors(task_index, predecessors_indices)
            .context("Failed to update predecessors within the project")?;
        Ok(())
    }

    /// Updates the project by making sure the successors for the task with
    /// index `node_index` are exactly the ones listed in `successors_indices`
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index whose successors need to be updated.
    /// * `successors_indices` - The indices of the successors.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let tasks = vec![
    ///      Task::new("Become world leader".to_owned()),
    ///      Task::new("Get rich".to_owned()),
    ///      Task::new("Be evil".to_owned())
    /// ];
    /// let mut project = Project::new("World domination".to_owned()).with_tasks(tasks);
    ///
    /// project.update_successors(0, &[1, 2]);
    /// assert_eq!(project.successors(0).count(), 2);
    pub fn update_successors(
        &mut self,
        task_index: usize,
        successors_indices: &[usize],
    ) -> anyhow::Result<()> {
        self.tasks
            .update_successors(task_index, successors_indices)
            .context("Failed to update successors in the project")?;
        Ok(())
    }

    /// Returns the start date of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project};
    /// use chrono::Utc;
    ///
    /// let start_date = Utc::now();
    /// let project = Project::new("World domination".to_owned()).with_start_date(start_date);
    /// assert_eq!(project.start_date(), Some(start_date));
    /// ```
    pub fn start_date(&self) -> Option<DateTime<Utc>> {
        self.start_date
    }

    /// Adds a resource to the project.
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to add to the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::Resource, project::Project, person::{Person, Name}};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_resource(Resource::Personnel {
    ///     person: Person::new(Name::parse("Sebastiano".to_owned(), "Giordano".to_owned()).unwrap()),
    ///     hourly_rate: None,
    /// });
    /// assert_eq!(project.resources().len(), 1);
    /// ```
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    /// Returns a reference to the list of resources associated with the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::{Resource, Material, NonConsumable}, project::Project};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_resource(Resource::Material(Material::NonConsumable(
    ///    NonConsumable::new("Crowbar".to_owned()),
    /// )));
    /// assert_eq!(project.resources().len(), 1);
    /// ```
    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    /// Adds a stakeholder to the project.
    ///
    /// # Arguments
    ///
    /// * `stakeholder` - The stakeholder to add to the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{stakeholders::Stakeholder, project::Project, person::{Person, Name}};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// let person = Person::new(Name::parse("Margherita".to_owned(), "Hack".to_owned()).unwrap());
    /// project.add_stakeholder(Stakeholder::Individual {
    ///   person,
    ///   description: None,
    /// });
    /// assert_eq!(project.stakeholders().len(), 1);
    /// ```
    pub fn add_stakeholder(&mut self, stakeholder: Stakeholder) {
        self.stakeholders.push(stakeholder);
    }

    /// Returns a reference to the list of stakeholders associated with the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{stakeholders::Stakeholder, project::Project, person::{Person, Name}};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// let person = Person::new(Name::parse("Margherita".to_owned(), "Hack".to_owned()).unwrap());
    /// project.add_stakeholder(Stakeholder::Individual {
    ///   person,
    ///   description: None,
    /// });
    /// assert_eq!(project.stakeholders().len(), 1);
    /// ```
    pub fn stakeholders(&self) -> &[Stakeholder] {
        &self.stakeholders
    }
}

#[cfg(test)]
/// Utilities to test `[Project]`
pub mod test_utils {
    use proptest::{collection, prelude::*};

    use crate::task::{Task, test_utils::task_strategy};

    use super::{Project, TimeRelationship};

    const MAX_TASKS: usize = 100;
    const MIN_TASKS: usize = 5;

    /// Generates a random task relationship kind.
    pub fn task_relationship_strategy() -> impl Strategy<Value = TimeRelationship> {
        prop_oneof![
            Just(TimeRelationship::StartToFinish),
            Just(TimeRelationship::StartToStart),
            Just(TimeRelationship::FinishToFinish),
            Just(TimeRelationship::FinishToStart),
        ]
    }

    /// Generate a random amount of randomly generated `[Tasks]`, between `[MIN_TASKS]` and `[MAX_TASKS]`.
    pub fn tasks_strategy() -> impl Strategy<Value = Vec<Task>> {
        collection::vec(task_strategy(), MIN_TASKS..MAX_TASKS)
    }

    /// Generate a random `[Project]` where every node is connected to the previous one.
    pub fn project_graph_strategy() -> impl Strategy<Value = Project> {
        (".*", tasks_strategy()).prop_map(|(n, tasks)| {
            let indices = 0..tasks.len();
            let mut project = Project::new(n).with_tasks(tasks);

            let mut previous = None;
            indices.for_each(|current| {
                if let Some(prev) = previous {
                    project.update_successors(prev, &[current]).unwrap();
                }
                previous = Some(current);
            });
            project
        })
    }

    /// Generate a random `[Project]` with `[Task]`s, but no predecessors/successors relationships.
    pub fn project_strategy() -> impl Strategy<Value = Project> {
        (".*", tasks_strategy()).prop_map(|(n, tasks)| Project::new(n).with_tasks(tasks))
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rand::{Rng, rng};

    use crate::project::test_utils::{project_graph_strategy, project_strategy};
    proptest! {
        #[test]
        fn update_predecessors_rejects_circular_graphs(mut project in project_graph_strategy()) {
            assert!(project.update_predecessors(0, &[project.tasks().count() - 1]).is_err());
        }

        #[test]
        fn update_successors_rejects_circular_graphs(mut project in project_graph_strategy()) {
            assert!(project.update_successors(project.tasks().count() - 1, &[0] ).is_err());
        }

        #[test]
        fn update_predecessors_rejects_non_existent_indices(mut project in project_strategy()) {
            let count: usize = project.tasks().count();

            assert!(project.update_predecessors(0, &[count]).is_err())
        }

        #[test]
        fn update_successors_rejects_non_existent_indices(mut project in project_strategy()) {
            let count: usize = project.tasks().count();

            assert!(project.update_successors(0, &[count]).is_err())
        }

        #[test]
        fn update_predecessors_removes_them_if_input_is_empty(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.count());
            let mut task_index2 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.count());
            }

            project.update_predecessors(task_index1, &[task_index2]).unwrap();
            project.update_predecessors(task_index1, &[]).unwrap();

            assert_eq!(project.predecessors(task_index1).count(), 0);
        }

        #[test]
        fn update_predecessors_removes_indices_not_present_in_input(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.count());
            let mut task_index2 = task_index1;
            let mut task_index3 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.count());
            }
            while task_index3 == task_index1 || task_index3 == task_index2 {
                task_index3 = rng.random_range(0..project.tasks.count());
            }

            project.update_predecessors(task_index1, &[task_index2, task_index3]).unwrap();
            project.update_predecessors(task_index1, &[task_index2]).unwrap();

            let mut predecessors = project.predecessors(task_index1);
            assert_eq!(predecessors.next(), project.task(task_index2));
            assert!(predecessors.next().is_none());
        }

        #[test]
        fn update_predecessors_works(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.count());
            let mut task_index2 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.count());
            }

            project.update_predecessors(task_index1, &[task_index2]).unwrap();

            let mut predecessors = project.predecessors(task_index1);
            assert_eq!(project.predecessors(task_index1).count(), 1);
            assert_eq!(predecessors.next(), project.task(task_index2));
        }

        #[test]
        fn update_successors_works(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.count());
            let mut task_index2 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.count());
            }

            project.update_successors(task_index1, &[task_index2]).unwrap();

            let mut successors = project.successors(task_index1);
            assert_eq!(successors.next(), project.task(task_index2));
            assert!(successors.next().is_none());
        }

        #[test]
        fn update_successors_removes_them_if_input_is_empty(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.count());
            let mut task_index2 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.count());
            }

            project.update_successors(task_index1, &[task_index2]).unwrap();
            project.update_successors(task_index1, &[]).unwrap();

            assert_eq!(project.successors(task_index1).count(), 0);
        }

        #[test]
        fn update_successors_removes_indices_not_present_in_input(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.count());
            let mut task_index2 = task_index1;
            let mut task_index3 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.count());
            }
            while task_index3 == task_index1 || task_index3 == task_index2 {
                task_index3 = rng.random_range(0..project.tasks.count());
            }

            project.update_successors(task_index1, &[task_index2, task_index3]).unwrap();
            project.update_successors(task_index1, &[task_index2]).unwrap();

            let mut successors = project.successors(task_index1);
            assert_eq!(successors.next(), project.task(task_index2));
            assert!(successors.next().is_none());
        }
    }
}
