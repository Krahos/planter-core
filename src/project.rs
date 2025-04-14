use std::collections::HashSet;

use anyhow::{Context, bail};
use chrono::{DateTime, Utc};
use daggy::{
    Dag,
    petgraph::{
        Direction,
        visit::{IntoNeighborsDirected, IntoNodeIdentifiers},
    },
};

use crate::{resources::Resource, stakeholders::Stakeholder, task::Task};

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
    tasks: Dag<Task, TaskRelationship, usize>,
    /// The list of resources associated with the project.
    resources: Vec<Resource>,
    /// The list of stakeholders associated with the project.
    stakeholders: Vec<Stakeholder>,
}

#[derive(Debug, Default, Clone, Copy)]
/// The predecessor - successor relationship between tasks can be of different kinds.
pub enum TaskRelationship {
    /// The predecessor has to start for the successor to finish.
    StartToFinish,
    /// The predecessor has to start for the successor to finish.
    FinishToFinish,
    #[default]
    /// The predecessor has to finish for the successor to start.
    FinishToStart,
    /// The predecessor has to start for the successor to finish.
    StartToStart,
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
            tasks: Dag::new(),
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
        self.tasks.add_node(task);
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
        self.tasks.add_node(task);
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
        self.tasks.node_weight(index.into())
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
        self.tasks.node_weight_mut(index.into())
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
        self.tasks.raw_nodes().iter().map(|node| &node.weight)
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
        self.tasks.node_weights_mut()
    }

    /// Adds a relationship betwen tasks, where one is the predecessor and the other one a successor.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TaskRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(0, 1, TaskRelationship::default());
    ///
    /// assert_eq!(project.successors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn add_relationship(
        &mut self,
        predecessor_index: usize,
        successor_index: usize,
        kind: TaskRelationship,
    ) -> anyhow::Result<()> {
        self.tasks
            .update_edge(predecessor_index.into(), successor_index.into(), kind)
            .context("Tried to add a relationship between non existing nodes")?;
        anyhow::Ok(())
    }

    /// Gets the list of successors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TaskRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(0, 1, TaskRelationship::default());
    ///
    /// assert_eq!(project.successors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn successors(&self, node_index: usize) -> impl Iterator<Item = &Task> {
        self.tasks
            .neighbors_directed(node_index.into(), Direction::Outgoing)
            .map(|index| &self.tasks[index])
    }

    /// Gets the list of predecessors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TaskRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_owned());
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(1, 0, TaskRelationship::default());
    ///
    /// assert_eq!(project.predecessors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn predecessors(&self, node_index: usize) -> impl Iterator<Item = &Task> {
        self.tasks
            .neighbors_directed(node_index.into(), Direction::Incoming)
            .map(|index| &self.tasks[index])
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
        self.validate_indices(task_index, predecessors_indices)?;

        // Update predecessors in a cloned data structure for tasks.
        // If this gives an error, the actual data structure won't be polluted.
        // TODO: benchmark and see if there is a better way to do this without cloning.
        let mut tasks_clone = self.tasks.clone();
        for &i in predecessors_indices {
            tasks_clone
                .add_edge(i.into(), task_index.into(), TaskRelationship::FinishToStart)
                .context(format!(
                    "A cycle was detected between tasks {i} and {task_index}"
                ))?;
        }

        // Update predecessors.
        for &i in predecessors_indices {
            self.tasks
                .add_edge(i.into(), task_index.into(), TaskRelationship::FinishToStart)
                .context("This shouldn't have happened because the data structure was just checked for cycles.")?;
        }
        Ok(())
    }

    /// Checks that all the tasks with indices passed as parameters actually exist in the project.
    fn validate_indices(&self, task_index: usize, related_indices: &[usize]) -> anyhow::Result<()> {
        let graph_edges: HashSet<usize> =
            self.tasks.node_identifiers().map(|i| i.index()).collect();
        // Make sure all the listed predecessors exist within the graph.
        if !related_indices.iter().all(|i| graph_edges.contains(i)) {
            bail!("Some index in the predecessors list doesn't exist in the graph");
        }

        // Make sure the task index exists withing the graph.
        if !graph_edges.contains(&task_index) {
            bail!(format!(
                "Task index {task_index} doesn't exist in the graph"
            ));
        }

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
        self.validate_indices(task_index, successors_indices)?;

        // Update successors in a cloned data structure for tasks.
        // If this gives an error, the actual data structure won't be polluted.
        // TODO: benchmark and see if there is a better way to do this without cloning.
        let mut tasks_clone = self.tasks.clone();
        for &i in successors_indices {
            tasks_clone
                .add_edge(task_index.into(), i.into(), TaskRelationship::FinishToStart)
                .context(format!(
                    "A cycle was detected between tasks {i} and {task_index}"
                ))?;
        }

        // Update successors.
        for &i in successors_indices {
            self.tasks
                .add_edge( task_index.into(), i.into(), TaskRelationship::FinishToStart)
                .context("This shouldn't have happened because the data structure was just checked for cycles.")?;
        }
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

    use super::{Project, TaskRelationship};

    const MAX_TASKS: usize = 100;

    /// Generates a random task relationship kind.
    pub fn task_relationship_strategy() -> impl Strategy<Value = TaskRelationship> {
        prop_oneof![
            Just(TaskRelationship::StartToFinish),
            Just(TaskRelationship::StartToStart),
            Just(TaskRelationship::FinishToFinish),
            Just(TaskRelationship::FinishToStart),
        ]
    }

    /// Generate a random amount of randomly generated `[Tasks]`.
    pub fn tasks_strategy() -> impl Strategy<Value = Vec<Task>> {
        collection::vec(task_strategy(), 2..MAX_TASKS)
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
        fn update_predecessors_works(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.node_count());
            let mut task_index2 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.node_count());
            }

            project.update_predecessors(task_index1, &[task_index2]).unwrap();

            let mut predecessors =project.predecessors(task_index1);
            assert_eq!(predecessors.next(), project.task(task_index2));
            assert!(predecessors.next().is_none());
        }

        #[test]
        fn update_successors_works(mut project in project_strategy()) {
            let mut rng = rng();
            let task_index1 = rng.random_range(0..project.tasks.node_count());
            let mut task_index2 = task_index1;

            while task_index2 == task_index1 {
                task_index2 = rng.random_range(0..project.tasks.node_count());
            }

            project.update_successors(task_index1, &[task_index2]).unwrap();

            let mut successors = project.successors(task_index1);
            assert_eq!(successors.next(), project.task(task_index2));
            assert!(successors.next().is_none());
        }
    }
}
