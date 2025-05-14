use std::collections::HashSet;

use anyhow::{Context, bail};
use daggy::{
    Dag,
    petgraph::{
        Direction,
        csr::IndexType,
        visit::{IntoNeighborsDirected, IntoNodeIdentifiers},
    },
};

use crate::task::Task;

#[derive(Debug, Default)]
/// Contains all the Tasks within a given project
pub struct Tasks {
    tasks: Dag<Task, TimeRelationship, usize>,

    subtask_relationships: Vec<SubtaskRelationship>,
}

#[derive(Debug, Default, Clone, Copy)]
/// A given task, might be composed of different subtasks.
pub struct SubtaskRelationship {
    task: usize,
    subtask: usize,
}

#[derive(Debug, Default, Clone, Copy)]
/// The predecessor - successor relationship between tasks.
pub enum TimeRelationship {
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

impl Tasks {
    /// Constructs a new empty `[Tasks]`.
    pub fn new() -> Self {
        Self {
            tasks: Dag::new(),
            subtask_relationships: Vec::new(),
        }
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
        self.add_task(task);
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
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let task_vec = vec![Task::new("Become world leader".to_owned()), Task::new("Get rich".to_owned())];
    /// let tasks = Tasks::new().with_tasks(task_vec);
    /// assert_eq!(tasks.tasks().count(), 2);
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
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// assert_eq!(tasks.tasks().count(), 0);
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(tasks.tasks().count(), 1);
    /// ```
    pub fn add_task(&mut self, task: Task) {
        self.tasks.add_node(task);
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
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(tasks.tasks().count(), 1);
    /// assert!(tasks.rm_task(0).is_ok());
    /// assert_eq!(tasks.tasks().count(), 0);
    /// ```
    pub fn rm_task(&mut self, i: usize) -> anyhow::Result<()> {
        self.tasks
            .remove_node(i.into())
            .context("Tried removing a non existing node from Tasks")?;
        Ok(())
    }

    /// Returns an iterator over the tasks contained here.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(tasks.tasks().count(), 1);
    /// ```
    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.tasks.raw_nodes().iter().map(|node| &node.weight)
    }

    /// Returns a mutable reference to the tasks of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(tasks.tasks().count(), 1);
    /// ```
    pub fn tasks_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        self.tasks.node_weights_mut()
    }

    /// Gets a reference to the task with the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to identify the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let mut project = Tasks::new();
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// assert_eq!(project.task(0).unwrap().name(), "Become world leader".to_owned());
    /// ```
    pub fn task(&self, index: usize) -> Option<&Task> {
        self.tasks.node_weight(index.into())
    }

    /// Gets a mutable reference to the task with the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index to identify the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// let task = tasks.task_mut(0).unwrap();
    /// assert_eq!(task.name(), "Become world leader");
    ///
    /// task.edit_name("Become world's biggest loser".to_owned());
    /// assert_eq!(task.name(), "Become world's biggest loser".to_owned())
    /// ```
    pub fn task_mut(&mut self, index: usize) -> Option<&mut Task> {
        self.tasks.node_weight_mut(index.into())
    }

    /// Adds a relationship betwen tasks, where one is the predecessor and the other one a successor.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::Tasks, tasks::TimeRelationship, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Get rich".to_owned()));
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// tasks.add_relationship(0, 1, TimeRelationship::default());
    ///
    /// assert_eq!(tasks.successors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn add_relationship(
        &mut self,
        predecessor_index: usize,
        successor_index: usize,
        kind: TimeRelationship,
    ) -> anyhow::Result<()> {
        self.tasks
            .update_edge(predecessor_index.into(), successor_index.into(), kind)
            .context("Tried to add a relationship between non existing nodes")?;
        anyhow::Ok(())
    }

    /// Removes a relationship betwen tasks, where one is the predecessor and the other one a successor.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::{Tasks, TimeRelationship}, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Get rich".to_owned()));
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// tasks.add_relationship(0, 1, TimeRelationship::default());
    /// tasks.remove_relationship(0, 1);
    ///
    /// assert_eq!(tasks.successors(0).count(), 0);
    /// ```
    pub fn remove_relationship(
        &mut self,
        predecessor_index: usize,
        successor_index: usize,
    ) -> anyhow::Result<()> {
        let edge_index = self
            .tasks
            .find_edge(predecessor_index.into(), successor_index.into())
            .context(
                "Tried to remove a relationship that doesn't exist or between non existing nodes",
            )?;

        self.tasks.remove_edge(edge_index);
        anyhow::Ok(())
    }

    /// Gets the list of successors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::{Tasks, TimeRelationship}, task::Task};
    ///
    /// let mut project = Tasks::new();
    /// project.add_task(Task::new("Get rich".to_owned()));
    /// project.add_task(Task::new("Become world leader".to_owned()));
    /// project.add_relationship(0, 1, TimeRelationship::default());
    ///
    /// assert_eq!(project.successors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn successors(&self, node_index: usize) -> impl Iterator<Item = &Task> {
        self.tasks
            .neighbors_directed(node_index.into(), Direction::Outgoing)
            .map(|index| &self.tasks[index])
    }

    /// Gets the indices of all successors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::{Tasks, TimeRelationship}, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Get rich".to_owned()));
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// tasks.add_relationship(0, 1, TimeRelationship::default());
    ///
    /// assert_eq!(tasks.successors_indices(0).next().unwrap(), 1)
    /// ```
    pub fn successors_indices(&self, node_index: usize) -> impl Iterator<Item = usize> {
        self.tasks
            .neighbors_directed(node_index.index().into(), Direction::Outgoing)
            .map(|index| index.index())
    }

    /// Gets the list of predecessors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::{Tasks, TimeRelationship}, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Get rich".to_owned()));
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// tasks.add_relationship(1, 0, TimeRelationship::default());
    ///
    /// assert_eq!(tasks.predecessors(0).next().unwrap().name(), "Become world leader".to_owned())
    /// ```
    pub fn predecessors(&self, node_index: usize) -> impl Iterator<Item = &Task> {
        self.tasks
            .neighbors_directed(node_index.into(), Direction::Incoming)
            .map(|index| &self.tasks[index])
    }

    /// Gets the indices of all predecessors for a given node.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{tasks::{Tasks, TimeRelationship}, task::Task};
    ///
    /// let mut tasks = Tasks::new();
    /// tasks.add_task(Task::new("Get rich".to_owned()));
    /// tasks.add_task(Task::new("Become world leader".to_owned()));
    /// tasks.add_relationship(1, 0, TimeRelationship::default());
    ///
    /// assert_eq!(tasks.predecessors_indices(0).next().unwrap(), 1)
    /// ```
    pub fn predecessors_indices(&self, node_index: usize) -> impl Iterator<Item = usize> {
        self.tasks
            .neighbors_directed(node_index.index().into(), Direction::Incoming)
            .map(|index| index.index())
    }

    /// Updates the tasks by making sure the predecessors for the task with
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
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let task_vec = vec![
    ///      Task::new("Become world leader".to_owned()),
    ///      Task::new("Get rich".to_owned()),
    ///      Task::new("Be evil".to_owned())
    /// ];
    /// let mut tasks = Tasks::new().with_tasks(task_vec);
    ///
    /// tasks.update_predecessors(2, &[0, 1]);
    /// assert_eq!(tasks.predecessors(2).count(), 2);
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
                .add_edge(i.into(), task_index.into(), TimeRelationship::FinishToStart)
                .context(format!(
                    "A cycle was detected between tasks {i} and {task_index}"
                ))?;
        }

        // Remove all predecessors.
        for i in self
            .predecessors_indices(task_index)
            .collect::<Vec<usize>>()
        {
            self.remove_relationship(i, task_index)
                .expect("It should have been possible to remove a predecessor. This is a bug.");
        }
        // Update predecessors.
        for &i in predecessors_indices {
            self.tasks
                .add_edge(i.into(), task_index.into(), TimeRelationship::FinishToStart)
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

    /// Updates the tasks by making sure the successors for the task with
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
    /// use planter_core::{tasks::Tasks, task::Task};
    ///
    /// let task_vec = vec![
    ///      Task::new("Become world leader".to_owned()),
    ///      Task::new("Get rich".to_owned()),
    ///      Task::new("Be evil".to_owned())
    /// ];
    /// let mut tasks = Tasks::new().with_tasks(task_vec);
    ///
    /// tasks.update_successors(0, &[1, 2]);
    /// assert_eq!(tasks.successors(0).count(), 2);
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
                .add_edge(task_index.into(), i.into(), TimeRelationship::FinishToStart)
                .context(format!(
                    "A cycle was detected between tasks {i} and {task_index}"
                ))?;
        }

        // Remove all successors.
        for i in self.successors_indices(task_index).collect::<Vec<usize>>() {
            self.remove_relationship(task_index, i)
                .expect("It should have been possible to remove a predecessor. This is a bug.");
        }
        // Update successors.
        for &i in successors_indices {
            self.tasks
                .add_edge( task_index.into(), i.into(), TimeRelationship::FinishToStart)
                .context("This shouldn't have happened because the data structure was just checked for cycles.")?;
        }
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.tasks.node_count()
    }
}
