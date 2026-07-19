use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{Context, bail};
use bon::Builder;
use chrono::{DateTime, Utc};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    resources::{Material, Resource},
    stakeholders::Stakeholder,
    task::Task,
};

#[derive(Debug, Default, Builder)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[builder(on(String, into))]
/// Represents a project with a name and a list of resources.
pub struct Project {
    /// The name of the project.
    name: String,
    /// The description of the project.
    description: Option<String>,
    /// The start date of the project.
    start_date: Option<DateTime<Utc>>,
    /// The tasks associated with the project.
    #[builder(default)]
    tasks: HashMap<Uuid, Task>,
    /// Insertion order of tasks.
    #[builder(default)]
    task_order: Vec<Uuid>,
    /// Successor relationships for the time-relationship DAG.
    #[builder(default)]
    succ: HashMap<Uuid, Vec<(Uuid, TimeRelationship)>>,
    /// Predecessor relationships for the time-relationship DAG.
    #[builder(default)]
    pred: HashMap<Uuid, Vec<(Uuid, TimeRelationship)>>,
    /// Subtask tree: parent -> children.
    #[builder(default)]
    children: HashMap<Uuid, Vec<Uuid>>,
    /// Subtask tree: child -> parent.
    #[builder(default)]
    parent_of: HashMap<Uuid, Uuid>,
    /// The list of resources associated with the project.
    #[builder(default)]
    resources: Vec<Resource>,
    /// The list of stakeholders associated with the project.
    #[builder(default)]
    stakeholders: Vec<Stakeholder>,
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The predecessor - successor relationship between tasks.
pub enum TimeRelationship {
    /// The predecessor has to start for the successor to finish.
    StartToFinish,
    /// The predecessor has to finish for the successor to finish.
    FinishToFinish,
    #[default]
    /// The predecessor has to finish for the successor to start.
    FinishToStart,
    /// The predecessor has to start for the successor to start.
    StartToStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The direction of a relationship update.
pub enum RelDir {
    /// Update the predecessors of a task.
    Predecessors,
    /// Update the successors of a task.
    Successors,
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
    /// let project = Project::new("World domination");
    /// assert_eq!(project.name(), "World domination");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Returns the name of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    ///
    /// let project = Project::new("World domination");
    /// assert_eq!(project.name(), "World domination");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the description of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    ///
    /// let project = Project::new("World domination");
    /// assert_eq!(project.description(), None);
    /// ```
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Adds a task to the project and returns its stable [`Uuid`].
    ///
    /// # Arguments
    ///
    /// * `task` - The task to add to the project.
    ///
    /// # Returns
    ///
    /// The stable [`Uuid`] assigned to the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let id = project.add_task(Task::new("Become world leader"));
    /// assert_eq!(project.tasks().count(), 1);
    /// ```
    pub fn add_task(&mut self, task: Task) -> Uuid {
        let id = task.id();
        self.tasks.insert(id, task);
        self.task_order.push(id);
        id
    }

    /// Inserts a new task as a sibling right before `sibling_id` in the task order.
    /// If the sibling has a parent, the new task becomes a child of the same parent.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let a = project.add_task(Task::new("Build an army"));
    /// let b = project.add_task(Task::new("Train troops"));
    /// let c = project.add_sibling_before(Task::new("Gather allies"), b);
    ///
    /// let ids: Vec<_> = project.tasks().map(|t| t.id()).collect();
    /// assert_eq!(ids, vec![a, c, b]);
    /// ```
    pub fn add_sibling_before(&mut self, task: Task, sibling_id: Uuid) -> Uuid {
        let id = task.id();
        self.tasks.insert(id, task);
        if let Some(pos) = self.task_order.iter().position(|&t| t == sibling_id) {
            self.task_order.insert(pos, id);
        } else {
            self.task_order.push(id);
        }
        if let Some(&parent_id) = self.parent_of.get(&sibling_id) {
            self.parent_of.insert(id, parent_id);
            self.children.entry(parent_id).or_default().push(id);
        }
        id
    }

    /// Inserts a new task as a sibling right after `sibling_id` in the task order.
    /// If the sibling has a parent, the new task becomes a child of the same parent.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let a = project.add_task(Task::new("Build an army"));
    /// let b = project.add_task(Task::new("Train troops"));
    /// let c = project.add_sibling_after(Task::new("Gather allies"), a);
    ///
    /// let ids: Vec<_> = project.tasks().map(|t| t.id()).collect();
    /// assert_eq!(ids, vec![a, c, b]);
    /// ```
    pub fn add_sibling_after(&mut self, task: Task, sibling_id: Uuid) -> Uuid {
        let id = task.id();
        self.tasks.insert(id, task);
        if let Some(pos) = self.task_order.iter().position(|&t| t == sibling_id) {
            self.task_order.insert(pos + 1, id);
        } else {
            self.task_order.push(id);
        }
        if let Some(&parent_id) = self.parent_of.get(&sibling_id) {
            self.parent_of.insert(id, parent_id);
            self.children.entry(parent_id).or_default().push(id);
        }
        id
    }

    /// Deletes a task and all references to it from the project.
    ///
    /// # Arguments
    ///
    /// * `id` - The [`Uuid`] of the task to remove.
    ///
    /// # Errors
    /// Returns an error if the task doesn't exist.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let id = project.add_task(Task::new("Become world leader"));
    /// assert_eq!(project.tasks().count(), 1);
    /// assert!(project.rm_task(id).is_ok());
    /// assert_eq!(project.tasks().count(), 0);
    /// ```
    pub fn rm_task(&mut self, id: Uuid) -> anyhow::Result<Task> {
        let task = self
            .tasks
            .remove(&id)
            .context("Tried removing a non existing task")?;
        self.task_order.retain(|t| *t != id);

        // Remove all time relationships involving this task.
        for (succ, _) in self.succ.remove(&id).into_iter().flatten() {
            if let Some(preds) = self.pred.get_mut(&succ) {
                preds.retain(|(p, _)| *p != id);
            }
        }
        for (pred_, _) in self.pred.remove(&id).into_iter().flatten() {
            if let Some(succs) = self.succ.get_mut(&pred_) {
                succs.retain(|(s, _)| *s != id);
            }
        }

        // Remove subtask relationships.
        if let Some(parent) = self.parent_of.remove(&id)
            && let Some(children) = self.children.get_mut(&parent)
        {
            children.retain(|c| *c != id);
        }
        self.children.remove(&id);

        Ok(task)
    }

    /// Gets a reference to the task with the given [`Uuid`].
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let id = project.add_task(Task::new("Become world leader"));
    /// assert_eq!(project.task(id).unwrap().name(), "Become world leader");
    /// ```
    #[must_use]
    pub fn task(&self, id: Uuid) -> Option<&Task> {
        self.tasks.get(&id)
    }

    /// Gets a mutable reference to the task with the given [`Uuid`].
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let id = project.add_task(Task::new("Become world leader"));
    /// let task = project.task_mut(id).unwrap();
    /// assert_eq!(task.name(), "Become world leader");
    ///
    /// task.edit_name("Become world's biggest loser");
    /// assert_eq!(task.name(), "Become world's biggest loser")
    /// ```
    #[must_use]
    pub fn task_mut(&mut self, id: Uuid) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    /// Returns the tasks of the project in insertion order.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_task(Task::new("Become world leader"));
    /// assert_eq!(project.tasks().count(), 1);
    /// ```
    pub fn tasks(&self) -> impl Iterator<Item = &Task> {
        self.task_order.iter().filter_map(|id| self.tasks.get(id))
    }

    /// Returns a mutable iterator over the tasks.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_task(Task::new("Become world leader"));
    /// assert_eq!(project.tasks_mut().count(), 1);
    /// ```
    pub fn tasks_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        self.tasks.values_mut()
    }

    /// Adds a relationship between tasks, where one is the predecessor and the other a successor.
    ///
    /// # Arguments
    ///
    /// * `predecessor` - The [`Uuid`] of the predecessor task.
    /// * `successor` - The [`Uuid`] of the successor task.
    /// * `kind` - The type of relationship.
    ///
    /// # Errors
    /// Returns an error if either task doesn't exist, if the relationship would create a cycle,
    /// or if the relationship already exists.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TimeRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let pred = project.add_task(Task::new("Get rich"));
    /// let succ = project.add_task(Task::new("Become world leader"));
    /// project.add_time_relationship(pred, succ, TimeRelationship::default());
    ///
    /// assert_eq!(project.successors(pred).next().unwrap().name(), "Become world leader")
    /// ```
    pub fn add_time_relationship(
        &mut self,
        predecessor: Uuid,
        successor: Uuid,
        kind: TimeRelationship,
    ) -> anyhow::Result<()> {
        if !self.tasks.contains_key(&predecessor) || !self.tasks.contains_key(&successor) {
            bail!("Task not found");
        }

        if self
            .succ
            .get(&predecessor)
            .map(|e| e.iter().any(|(s, _)| *s == successor))
            .unwrap_or(false)
        {
            bail!("Relationship between tasks already exists");
        }

        if self.would_cycle(successor, predecessor) {
            bail!("A cycle was detected between tasks {predecessor} and {successor}");
        }

        self.add_one_edge(predecessor, successor, kind);
        Ok(())
    }

    /// Removes a relationship between tasks.
    ///
    /// # Arguments
    ///
    /// * `predecessor` - The [`Uuid`] of the predecessor task.
    /// * `successor` - The [`Uuid`] of the successor task.
    ///
    /// # Errors
    /// Returns an error if no relationship exists between the tasks.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TimeRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let pred = project.add_task(Task::new("Get rich"));
    /// let succ = project.add_task(Task::new("Become world leader"));
    /// project.add_time_relationship(pred, succ, TimeRelationship::default());
    /// project.rm_time_relationship(pred, succ).unwrap();
    ///
    /// assert_eq!(project.successors(pred).count(), 0);
    /// ```
    pub fn rm_time_relationship(
        &mut self,
        predecessor: Uuid,
        successor: Uuid,
    ) -> anyhow::Result<()> {
        let exists = self
            .succ
            .get(&predecessor)
            .map(|e| e.iter().any(|(s, _)| *s == successor))
            .unwrap_or(false);
        if !exists {
            bail!("Tried to remove a relationship that doesn't exist");
        }
        self.remove_one_edge(predecessor, successor);
        Ok(())
    }

    /// Gets the successors of a given task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TimeRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let pred = project.add_task(Task::new("Get rich"));
    /// let succ = project.add_task(Task::new("Become world leader"));
    /// project.add_time_relationship(pred, succ, TimeRelationship::default());
    ///
    /// assert_eq!(project.successors(pred).next().unwrap().name(), "Become world leader")
    /// ```
    pub fn successors(&self, id: Uuid) -> impl Iterator<Item = &Task> {
        self.succ
            .get(&id)
            .into_iter()
            .flatten()
            .filter_map(move |(succ_id, _)| self.tasks.get(succ_id))
    }

    /// Gets the [`Uuid`]s of all successors for a given task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TimeRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let pred = project.add_task(Task::new("Get rich"));
    /// let succ = project.add_task(Task::new("Become world leader"));
    /// project.add_time_relationship(pred, succ, TimeRelationship::default());
    ///
    /// assert_eq!(project.successors_ids(pred).next().unwrap(), succ)
    /// ```
    pub fn successors_ids(&self, id: Uuid) -> impl Iterator<Item = Uuid> {
        self.succ
            .get(&id)
            .into_iter()
            .flatten()
            .map(|(succ_id, _)| *succ_id)
    }

    /// Gets the predecessors of a given task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TimeRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let pred = project.add_task(Task::new("Get rich"));
    /// let succ = project.add_task(Task::new("Become world leader"));
    /// project.add_time_relationship(pred, succ, TimeRelationship::default());
    ///
    /// assert_eq!(project.predecessors(succ).next().unwrap().name(), "Get rich")
    /// ```
    pub fn predecessors(&self, id: Uuid) -> impl Iterator<Item = &Task> {
        self.pred
            .get(&id)
            .into_iter()
            .flatten()
            .filter_map(move |(pred_id, _)| self.tasks.get(pred_id))
    }

    /// Gets the [`Uuid`]s of all predecessors for a given task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, TimeRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let pred = project.add_task(Task::new("Get rich"));
    /// let succ = project.add_task(Task::new("Become world leader"));
    /// project.add_time_relationship(pred, succ, TimeRelationship::default());
    ///
    /// assert_eq!(project.predecessors_ids(succ).next().unwrap(), pred)
    /// ```
    pub fn predecessors_ids(&self, id: Uuid) -> impl Iterator<Item = Uuid> {
        self.pred
            .get(&id)
            .into_iter()
            .flatten()
            .map(|(pred_id, _)| *pred_id)
    }

    /// Sets the predecessors or successors of a task to exactly the given set of tasks.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The [`Uuid`] of the task whose relationships need updating.
    /// * `ids` - The tasks to set as predecessors or successors.
    /// * `dir` - Whether to update predecessors or successors.
    /// * `kind` - The type of time relationship.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Any task doesn't exist.
    /// * The update would create a cycle.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::{Project, RelDir, TimeRelationship}, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let id0 = project.add_task(Task::new("Become world leader"));
    /// let id1 = project.add_task(Task::new("Get rich"));
    /// let id2 = project.add_task(Task::new("Be evil"));
    ///
    /// project.update_relationships(id2, &[id0, id1], RelDir::Predecessors, TimeRelationship::FinishToStart).unwrap();
    /// assert_eq!(project.predecessors(id2).count(), 2);
    /// ```
    pub fn update_relationships(
        &mut self,
        task_id: Uuid,
        ids: &[Uuid],
        dir: RelDir,
        kind: TimeRelationship,
    ) -> anyhow::Result<()> {
        if !self.tasks.contains_key(&task_id) {
            bail!("Task {task_id} doesn't exist");
        }
        for &id in ids {
            if !self.tasks.contains_key(&id) {
                bail!("Task {id} doesn't exist");
            }
        }

        let old: HashSet<Uuid> = match dir {
            RelDir::Predecessors => self.predecessors_ids(task_id).collect(),
            RelDir::Successors => self.successors_ids(task_id).collect(),
        };
        let new: HashSet<Uuid> = ids.iter().copied().collect();

        let to_add: Vec<Uuid> = ids.iter().filter(|i| !old.contains(i)).copied().collect();
        let to_remove: Vec<Uuid> = old.iter().filter(|i| !new.contains(i)).copied().collect();

        let mut added = Vec::new();
        for &i in &to_add {
            let (pred, succ) = match dir {
                RelDir::Predecessors => (i, task_id),
                RelDir::Successors => (task_id, i),
            };
            if self.is_ancestor(pred, succ) || self.is_ancestor(succ, pred) {
                for &(p, s) in &added {
                    self.remove_one_edge(p, s);
                }
                bail!(
                    "Cannot add a predecessor/successor relationship between an ancestor and a descendant"
                );
            }
            if self.would_cycle(succ, pred) {
                for &(p, s) in &added {
                    self.remove_one_edge(p, s);
                }
                bail!("A cycle was detected between tasks {i} and {task_id}");
            }
            self.add_one_edge(pred, succ, kind);
            added.push((pred, succ));
        }

        for &i in &to_remove {
            let (pred, succ) = match dir {
                RelDir::Predecessors => (i, task_id),
                RelDir::Successors => (task_id, i),
            };
            self.remove_one_edge(pred, succ);
        }

        Ok(())
    }

    /// Moves `id` right after `after_id` in the global task order, affecting display order.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let a = project.add_task(Task::new("Build an army"));
    /// let b = project.add_task(Task::new("Train troops"));
    /// let c = project.add_task(Task::new("Gather allies"));
    /// project.move_task_after(c, a);
    ///
    /// let ids: Vec<_> = project.tasks().map(|t| t.id()).collect();
    /// assert_eq!(ids, vec![a, c, b]);
    /// ```
    pub fn move_task_after(&mut self, id: Uuid, after_id: Uuid) {
        let id_pos = self.task_order.iter().position(|&t| t == id);
        let after_pos = self.task_order.iter().position(|&t| t == after_id);
        if let (Some(ip), Some(ap)) = (id_pos, after_pos) {
            self.task_order.remove(ip);
            let insert_at = if ap > ip { ap } else { ap + 1 };
            self.task_order.insert(insert_at, id);
        }
    }

    /// Adds a subtask to a given task, marking the child as a component of the parent.
    /// The parent task is completed when all children are completed.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The [`Uuid`] of the parent task.
    /// * `child_id` - The [`Uuid`] of the child subtask.
    ///
    /// # Errors
    ///
    /// Returns an error if either task doesn't exist.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let parent = project.add_task(Task::new("Build a house"));
    /// let child1 = project.add_task(Task::new("Lay foundations"));
    /// let child2 = project.add_task(Task::new("Build roof"));
    ///
    /// project.add_subtask(parent, child1).unwrap();
    /// project.add_subtask(parent, child2).unwrap();
    /// assert_eq!(project.subtasks(parent).count(), 2);
    /// ```
    pub fn add_subtask(&mut self, parent_id: Uuid, child_id: Uuid) -> anyhow::Result<()> {
        if !self.tasks.contains_key(&parent_id) || !self.tasks.contains_key(&child_id) {
            bail!("Task not found");
        }
        if parent_id == child_id {
            bail!("A task cannot be a subtask of itself");
        }
        // No-op if already a child of this parent.
        if self.parent_of.get(&child_id) == Some(&parent_id) {
            return Ok(());
        }
        // Reject if parent is already a descendant of child (cycle).
        let mut current = parent_id;
        while let Some(&ancestor) = self.parent_of.get(&current) {
            if ancestor == child_id {
                bail!("Cannot make a task a subtask of one of its own descendants");
            }
            current = ancestor;
        }
        // Remove from any existing parent first.
        if let Some(old_parent) = self.parent_of.remove(&child_id)
            && let Some(children) = self.children.get_mut(&old_parent)
        {
            children.retain(|c| *c != child_id);
            if children.is_empty() {
                self.children.remove(&old_parent);
            }
        }
        self.children.entry(parent_id).or_default().push(child_id);
        self.parent_of.insert(child_id, parent_id);
        Ok(())
    }

    /// Removes a subtask relationship, promoting the child back to a top-level task.
    ///
    /// # Errors
    ///
    /// Returns an error if the task is not a subtask.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let army = project.add_task(Task::new("Build an army"));
    /// let supplies = project.add_task(Task::new("Gather supplies"));
    /// project.add_subtask(army, supplies).unwrap();
    /// assert!(project.task_parent(supplies).is_some());
    ///
    /// project.remove_subtask(supplies).unwrap();
    /// assert!(project.task_parent(supplies).is_none());
    /// ```
    pub fn remove_subtask(&mut self, child_id: Uuid) -> anyhow::Result<()> {
        let parent = self
            .parent_of
            .remove(&child_id)
            .context("Task is not a subtask")?;
        if let Some(children) = self.children.get_mut(&parent) {
            children.retain(|c| *c != child_id);
            if children.is_empty() {
                self.children.remove(&parent);
            }
        }
        Ok(())
    }

    /// Returns the parent [`Uuid`] of a subtask, or `None` if the task is at root level.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let army = project.add_task(Task::new("Build an army"));
    /// let supplies = project.add_task(Task::new("Gather supplies"));
    ///
    /// assert!(project.task_parent(supplies).is_none());
    ///
    /// project.add_subtask(army, supplies).unwrap();
    /// assert_eq!(project.task_parent(supplies), Some(army));
    /// ```
    pub fn task_parent(&self, child_id: Uuid) -> Option<Uuid> {
        self.parent_of.get(&child_id).copied()
    }

    /// Gets the [`Uuid`]s of all subtasks of the given task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let parent = project.add_task(Task::new("Build a house"));
    /// let child = project.add_task(Task::new("Lay foundations"));
    /// assert_eq!(project.subtasks(parent).count(), 0);
    ///
    /// project.add_subtask(parent, child).unwrap();
    /// assert_eq!(project.subtasks(parent).count(), 1);
    /// ```
    pub fn subtasks(&self, parent_id: Uuid) -> impl Iterator<Item = Uuid> + '_ {
        self.children.get(&parent_id).into_iter().flatten().copied()
    }

    /// Expands the parent's start/finish dates to encompass all children.
    /// Only ever expands outward — never contracts.
    /// Has no effect if no child has start/finish dates.
    ///
    /// # Errors
    ///
    /// Returns an error if the parent task doesn't exist.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Utc;
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination");
    /// let army = project.add_task(Task::new("Build an army"));
    /// let supplies = project.add_task(Task::new("Gather supplies"));
    /// project.add_subtask(army, supplies).unwrap();
    ///
    /// let now = Utc::now();
    /// project.task_mut(supplies).unwrap().edit_start(now).unwrap();
    /// project.sync_parent_dates(army).unwrap();
    ///
    /// assert_eq!(project.task(army).unwrap().start(), Some(now));
    /// ```
    pub fn sync_parent_dates(&mut self, parent_id: Uuid) -> anyhow::Result<()> {
        let earliest_start = self
            .subtasks(parent_id)
            .filter_map(|child_id| self.task(child_id).and_then(|t| t.start()))
            .min();
        let latest_finish = self
            .subtasks(parent_id)
            .filter_map(|child_id| self.task(child_id).and_then(|t| t.finish()))
            .max();

        if earliest_start.is_none() && latest_finish.is_none() {
            return Ok(());
        }

        let parent = self.task_mut(parent_id).context("Parent task not found")?;
        if let Some(start) = earliest_start
            && parent.start().is_none_or(|ps| start < ps)
        {
            let _ = parent.edit_start(start);
        }
        if let Some(finish) = latest_finish
            && parent.finish().is_none_or(|pf| finish > pf)
        {
            let _ = parent.edit_finish(finish);
        }
        Ok(())
    }

    /// Returns the start date of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    /// use chrono::Utc;
    ///
    /// let start_date = Utc::now();
    /// let project = Project::builder().name("World domination").start_date(start_date).build();
    /// assert_eq!(project.start_date(), Some(start_date));
    /// ```
    #[must_use]
    pub const fn start_date(&self) -> Option<DateTime<Utc>> {
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
    /// use planter_core::{resources::Resource, project::Project, person::Person};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_resource(Resource::Personnel {
    ///     person: Person::new("Sebastiano", "Giordano").unwrap(),
    ///     hourly_rate: None,
    /// });
    /// assert_eq!(project.resources().len(), 1);
    /// ```
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    /// Get a reference to a resource used in the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::Resource, project::Project, person::Person};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_resource(Resource::Personnel {
    ///     person: Person::new("Sebastiano", "Giordano").unwrap(),
    ///     hourly_rate: None,
    /// });
    ///
    /// assert!(project.resource(0).is_some());
    /// ```
    #[must_use]
    pub fn resource(&self, index: usize) -> Option<&Resource> {
        self.resources.get(index)
    }

    /// Remove a resource from the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::Resource, project::Project, person::Person};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_resource(Resource::Personnel {
    ///     person: Person::new("Sebastiano", "Giordano").unwrap(),
    ///     hourly_rate: None,
    /// });
    ///
    /// assert!(project.resource(0).is_some());
    /// project.rm_resource(0);
    /// assert!(project.resource(0).is_none());
    /// assert!(project.rm_resource(0).is_none());
    /// ```
    #[must_use]
    pub fn rm_resource(&mut self, index: usize) -> Option<Resource> {
        if index < self.resources.len() {
            Some(self.resources.remove(index))
        } else {
            None
        }
    }

    /// Get a mutable reference to a resource used in the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::Resource, project::Project, person::Person};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_resource(Resource::Personnel {
    ///     person: Person::new("Sebastiano", "Giordano").unwrap(),
    ///     hourly_rate: None,
    /// });
    ///
    /// let resource = project.resource_mut(0).unwrap();
    /// match resource {
    ///   Resource::Material(_) => panic!(),
    ///   Resource::Personnel {
    ///     person,
    ///     ..
    ///   } => {
    ///     person.update_first_name("Alessandro");
    ///     assert_eq!(person.first_name(), "Alessandro");
    ///   }
    /// }
    /// ```
    #[must_use]
    pub fn resource_mut(&mut self, index: usize) -> Option<&mut Resource> {
        self.resources.get_mut(index)
    }

    /// Returns a reference to the list of resources associated with the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::{Resource, Material, NonConsumable}, project::Project};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_resource(Resource::Material(Material::NonConsumable(
    ///    NonConsumable::new("Crowbar"),
    /// )));
    /// assert_eq!(project.resources().len(), 1);
    /// ```
    #[must_use]
    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    /// Converts a resource into a `Consumable`, if that's possible.
    ///
    /// # Arguments
    ///
    /// * resource_index - The index of a `Material`, that's not a `Consumable`
    ///
    /// # Errors
    ///
    /// * `ResourceConversionError::ResourceNotFound` - If a resource with the specified index does not exist.
    /// * `ResourceConversionError::ConversionNotPossible` - When trying to convert a resource that's not a `Material`.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::{Resource, Material, NonConsumable}, project::Project};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_resource(Resource::Material(Material::NonConsumable(
    ///    NonConsumable::new("Crowbar"),
    /// )));
    /// assert!(project.res_into_consumable(0).is_ok());
    /// ```
    pub fn res_into_consumable(
        &mut self,
        resource_index: usize,
    ) -> Result<(), ResourceConversionError> {
        self.convert_resource(resource_index, true)
    }

    /// Converts a resource into a `NonConsumable`, if that's possible.
    ///
    /// # Arguments
    ///
    /// * resource_index - The index of a `Material`, that's not a `NonConsumable`
    ///
    /// # Errors
    ///
    /// * `ResourceConversionError::ResourceNotFound` - If a resource with the specified index does not exist.
    /// * `ResourceConversionError::ConversionNotPossible` - When trying to convert a resource that's not a `Material`.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::{Resource, Material, Consumable}, project::Project};
    ///
    /// let mut project = Project::new("World domination");
    /// project.add_resource(Resource::Material(Material::Consumable(
    ///    Consumable::new("Stimpack"),
    /// )));
    /// assert!(project.res_into_nonconsumable(0).is_ok());
    /// ```
    pub fn res_into_nonconsumable(
        &mut self,
        resource_index: usize,
    ) -> Result<(), ResourceConversionError> {
        self.convert_resource(resource_index, false)
    }

    fn convert_resource(
        &mut self,
        resource_index: usize,
        to_consumable: bool,
    ) -> Result<(), ResourceConversionError> {
        let res = self
            .resources
            .get_mut(resource_index)
            .ok_or(ResourceConversionError::ResourceNotFound)?;
        let replacement = match res {
            Resource::Material(Material::NonConsumable(nc)) if to_consumable => {
                Some(Resource::Material(Material::Consumable(nc.clone().into())))
            }
            Resource::Material(Material::Consumable(c)) if !to_consumable => Some(
                Resource::Material(Material::NonConsumable(c.clone().into())),
            ),
            Resource::Material(Material::Consumable(_))
            | Resource::Material(Material::NonConsumable(_)) => None,
            _ => return Err(ResourceConversionError::ConversionNotPossible),
        };
        if let Some(r) = replacement {
            *res = r;
        }
        Ok(())
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
    /// use planter_core::{stakeholders::Stakeholder, project::Project, person::Person};
    ///
    /// let mut project = Project::new("World domination");
    /// let person = Person::new("Margherita", "Hack").unwrap();
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
    /// use planter_core::{stakeholders::Stakeholder, project::Project, person::Person};
    ///
    /// let mut project = Project::new("World domination");
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// project.add_stakeholder(Stakeholder::Individual {
    ///   person,
    ///   description: None,
    /// });
    /// assert_eq!(project.stakeholders().len(), 1);
    /// ```
    #[must_use]
    pub fn stakeholders(&self) -> &[Stakeholder] {
        &self.stakeholders
    }

    /// Removes a stakeholder from the project by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the stakeholder to remove.
    ///
    /// # Returns
    ///
    /// The removed stakeholder, or `None` if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{stakeholders::Stakeholder, project::Project, person::Person};
    ///
    /// let mut project = Project::new("World domination");
    /// let person = Person::new("Margherita", "Hack").unwrap();
    /// project.add_stakeholder(Stakeholder::Individual { person, description: None });
    /// assert_eq!(project.stakeholders().len(), 1);
    /// let removed = project.rm_stakeholder(0);
    /// assert!(removed.is_some());
    /// assert_eq!(project.stakeholders().len(), 0);
    /// ```
    #[must_use]
    pub fn rm_stakeholder(&mut self, index: usize) -> Option<Stakeholder> {
        if index < self.stakeholders.len() {
            Some(self.stakeholders.remove(index))
        } else {
            None
        }
    }

    /// Returns `true` if `ancestor_id` is an ancestor of `descendant_id` in the task tree.
    fn is_ancestor(&self, ancestor_id: Uuid, descendant_id: Uuid) -> bool {
        let mut seen = HashSet::new();
        let mut current = descendant_id;
        while let Some(parent) = self.task_parent(current) {
            if !seen.insert(parent) {
                break;
            }
            if parent == ancestor_id {
                return true;
            }
            current = parent;
        }
        false
    }

    /// BFS from `from` following successors. Returns `true` if `to` is reachable.
    fn would_cycle(&self, from: Uuid, to: Uuid) -> bool {
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(from);
        while let Some(v) = q.pop_front() {
            if v == to {
                return true;
            }
            if seen.insert(v)
                && let Some(succs) = self.succ.get(&v)
            {
                for (s, _) in succs {
                    q.push_back(*s);
                }
            }
        }
        false
    }

    fn add_one_edge(&mut self, pred: Uuid, succ: Uuid, kind: TimeRelationship) {
        self.succ.entry(pred).or_default().push((succ, kind));
        self.pred.entry(succ).or_default().push((pred, kind));
    }

    fn remove_one_edge(&mut self, pred: Uuid, succ: Uuid) {
        if let Some(entries) = self.succ.get_mut(&pred) {
            entries.retain(|(s, _)| *s != succ);
            if entries.is_empty() {
                self.succ.remove(&pred);
            }
        }
        if let Some(entries) = self.pred.get_mut(&succ) {
            entries.retain(|(p, _)| *p != pred);
            if entries.is_empty() {
                self.pred.remove(&succ);
            }
        }
    }
}

/// Represents an error that can occur when trying to convert `Material` resources variants to another variant.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ResourceConversionError {
    /// Used when trying to convert a resource with an index out of bounds.
    #[error("The resource with the specified index wasn't found")]
    ResourceNotFound,
    /// Used when trying to convert a resource that's not a material, for example personnel.
    #[error("Tried to convert a resource that's not a material")]
    ConversionNotPossible,
}

#[cfg(test)]
/// Utilities to test `[Project]`
pub mod test_utils {
    use proptest::{collection, prelude::*};

    use crate::task::{Task, test_utils::task_strategy};

    use super::{Project, RelDir, TimeRelationship};

    const MAX_TASKS: usize = 100;
    const MIN_TASKS: usize = 5;

    /// Generate a random amount of randomly generated `[Tasks]`.
    pub fn tasks_strategy() -> impl Strategy<Value = Vec<Task>> {
        collection::vec(task_strategy(), MIN_TASKS..MAX_TASKS)
    }

    /// Generate a random `[Project]` with a linear chain of time relationships.
    pub fn project_graph_strategy() -> impl Strategy<Value = Project> {
        (".*", tasks_strategy()).prop_map(|(n, tasks)| {
            let mut project = Project::builder().name(n).build();
            let mut ids = Vec::new();
            for task in tasks {
                ids.push(project.add_task(task));
            }

            let mut previous = None;
            for &current in &ids {
                if let Some(prev) = previous {
                    project
                        .update_relationships(
                            prev,
                            &[current],
                            RelDir::Successors,
                            TimeRelationship::FinishToStart,
                        )
                        .unwrap();
                }
                previous = Some(current);
            }
            project
        })
    }

    /// Generate a random `[Project]` with no time relationships.
    pub fn project_strategy() -> impl Strategy<Value = Project> {
        (".*", tasks_strategy()).prop_map(|(n, tasks)| {
            let mut project = Project::builder().name(n).build();
            for task in tasks {
                project.add_task(task);
            }
            project
        })
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rand::{RngExt, rng};

    use chrono::Utc;
    use uuid::Uuid;

    use crate::{
        person::Person,
        project::{
            Project, RelDir, ResourceConversionError, TimeRelationship,
            test_utils::{project_graph_strategy, project_strategy},
        },
        resources::{Consumable, Material, NonConsumable, Resource},
        stakeholders::Stakeholder,
        task::Task,
    };

    fn task_ids(project: &Project) -> Vec<Uuid> {
        project.tasks().map(|t| t.id()).collect()
    }

    proptest! {
        #[test]
        fn update_relationships_predecessor_rejects_circular_graphs(mut project in project_graph_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let last = *ids.last().unwrap();
            assert!(project.update_relationships(ids[0], &[last], RelDir::Predecessors, TimeRelationship::FinishToStart).is_err());
        }

        #[test]
        fn update_relationships_rejects_circular_graphs(mut project in project_graph_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let last = *ids.last().unwrap();
            assert!(project.update_relationships(last, &[ids[0]], RelDir::Successors, TimeRelationship::FinishToStart).is_err());
        }

        #[test]
        fn update_relationships_rejects_non_existent_ids(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.is_empty() { return Ok(()); }
            let fake = Uuid::new_v4();
            assert!(project.update_relationships(ids[0], &[fake], RelDir::Predecessors, TimeRelationship::FinishToStart).is_err());
            assert!(project.update_relationships(ids[0], &[fake], RelDir::Successors, TimeRelationship::FinishToStart).is_err());
        }

        #[test]
        fn update_relationships_predecessor_removes_them_if_input_is_empty(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rng();
            let idx1 = rng.random_range(0..ids.len());
            let mut idx2 = idx1;
            while idx2 == idx1 {
                idx2 = rng.random_range(0..ids.len());
            }

            project.update_relationships(ids[idx1], &[ids[idx2]], RelDir::Predecessors, TimeRelationship::FinishToStart).unwrap();
            project.update_relationships(ids[idx1], &[], RelDir::Predecessors, TimeRelationship::FinishToStart).unwrap();

            assert_eq!(project.predecessors(ids[idx1]).count(), 0);
        }

        #[test]
        fn update_relationships_predecessor_removes_ids_not_present_in_input(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 3 { return Ok(()); }
            let mut rng = rng();
            let idx1 = rng.random_range(0..ids.len());
            let mut idx2 = idx1;
            let mut idx3 = idx1;
            while idx2 == idx1 {
                idx2 = rng.random_range(0..ids.len());
            }
            while idx3 == idx1 || idx3 == idx2 {
                idx3 = rng.random_range(0..ids.len());
            }

            project.update_relationships(ids[idx1], &[ids[idx2], ids[idx3]], RelDir::Predecessors, TimeRelationship::FinishToStart).unwrap();
            project.update_relationships(ids[idx1], &[ids[idx2]], RelDir::Predecessors, TimeRelationship::FinishToStart).unwrap();

            let mut predecessors = project.predecessors(ids[idx1]);
            assert_eq!(predecessors.next().map(|t| t.name()), project.task(ids[idx2]).map(|t| t.name()));
            assert!(predecessors.next().is_none());
        }

        #[test]
        fn update_relationships_predecessor_works(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rng();
            let idx1 = rng.random_range(0..ids.len());
            let mut idx2 = idx1;
            while idx2 == idx1 {
                idx2 = rng.random_range(0..ids.len());
            }

            project.update_relationships(ids[idx1], &[ids[idx2]], RelDir::Predecessors, TimeRelationship::FinishToStart).unwrap();

            assert_eq!(project.predecessors(ids[idx1]).count(), 1);
            assert_eq!(
                project.predecessors(ids[idx1]).next().map(|t| t.name()),
                project.task(ids[idx2]).map(|t| t.name())
            );
        }

        #[test]
        fn update_relationships_works(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rng();
            let idx1 = rng.random_range(0..ids.len());
            let mut idx2 = idx1;
            while idx2 == idx1 {
                idx2 = rng.random_range(0..ids.len());
            }

            project.update_relationships(ids[idx1], &[ids[idx2]], RelDir::Successors, TimeRelationship::FinishToStart).unwrap();

            let mut successors = project.successors(ids[idx1]);
            assert_eq!(successors.next().map(|t| t.name()), project.task(ids[idx2]).map(|t| t.name()));
            assert!(successors.next().is_none());
        }

        #[test]
        fn update_relationships_removes_them_if_input_is_empty(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rng();
            let idx1 = rng.random_range(0..ids.len());
            let mut idx2 = idx1;
            while idx2 == idx1 {
                idx2 = rng.random_range(0..ids.len());
            }

            project.update_relationships(ids[idx1], &[ids[idx2]], RelDir::Successors, TimeRelationship::FinishToStart).unwrap();
            project.update_relationships(ids[idx1], &[], RelDir::Successors, TimeRelationship::FinishToStart).unwrap();

            assert_eq!(project.successors(ids[idx1]).count(), 0);
        }

        #[test]
        fn update_relationships_removes_ids_not_present_in_input(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 3 { return Ok(()); }
            let mut rng = rng();
            let idx1 = rng.random_range(0..ids.len());
            let mut idx2 = idx1;
            let mut idx3 = idx1;
            while idx2 == idx1 {
                idx2 = rng.random_range(0..ids.len());
            }
            while idx3 == idx1 || idx3 == idx2 {
                idx3 = rng.random_range(0..ids.len());
            }

            project.update_relationships(ids[idx1], &[ids[idx2], ids[idx3]], RelDir::Successors, TimeRelationship::FinishToStart).unwrap();
            project.update_relationships(ids[idx1], &[ids[idx2]], RelDir::Successors, TimeRelationship::FinishToStart).unwrap();

            let mut successors = project.successors(ids[idx1]);
            assert_eq!(successors.next().map(|t| t.name()), project.task(ids[idx2]).map(|t| t.name()));
            assert!(successors.next().is_none());
        }
    }

    #[test]
    fn update_relationships_rolls_back_partial_additions_on_cycle() {
        let mut project = Project::new("test");
        let a = project.add_task(Task::new("A"));
        let b = project.add_task(Task::new("B"));
        let c = project.add_task(Task::new("C"));
        let d = project.add_task(Task::new("D"));

        project
            .add_time_relationship(a, b, TimeRelationship::FinishToStart)
            .unwrap();
        project
            .add_time_relationship(b, c, TimeRelationship::FinishToStart)
            .unwrap();
        project
            .add_time_relationship(c, d, TimeRelationship::FinishToStart)
            .unwrap();

        let old_preds: Vec<Uuid> = project.predecessors_ids(c).collect();
        assert_eq!(old_preds, vec![b]);

        let result = project.update_relationships(
            c,
            &[a, d],
            RelDir::Predecessors,
            TimeRelationship::FinishToStart,
        );
        assert!(result.is_err());

        let preds: Vec<Uuid> = project.predecessors_ids(c).collect();
        assert_eq!(
            preds,
            vec![b],
            "predecessors should be unchanged after rollback"
        );
        assert!(
            !project.predecessors_ids(c).any(|i| i == a),
            "partially-added edge a→c should have been rolled back"
        );
    }

    #[test]
    fn update_relationships_handles_overlap() {
        let mut project = Project::new("test");
        let a = project.add_task(Task::new("A"));
        let b = project.add_task(Task::new("B"));
        let c = project.add_task(Task::new("C"));
        let d = project.add_task(Task::new("D"));

        project
            .update_relationships(
                c,
                &[a, b],
                RelDir::Predecessors,
                TimeRelationship::FinishToStart,
            )
            .unwrap();
        let preds: Vec<Uuid> = project.predecessors_ids(c).collect();
        assert!(preds.contains(&a), "should contain a, got {preds:?}");
        assert!(preds.contains(&b), "should contain b, got {preds:?}");

        project
            .update_relationships(
                c,
                &[b, d],
                RelDir::Predecessors,
                TimeRelationship::FinishToStart,
            )
            .unwrap();
        let preds: Vec<Uuid> = project.predecessors_ids(c).collect();
        assert!(preds.contains(&b));
        assert!(preds.contains(&d));
        assert!(!preds.contains(&a));
    }

    #[test]
    fn res_into_consumable_returns_the_correct_errors() {
        let mut project = Project::new("World domination");

        project.add_resource(Resource::Personnel {
            person: Person::new("Sebastiano", "Giordano").unwrap(),
            hourly_rate: None,
        });

        assert_eq!(
            project.res_into_consumable(0),
            Err(ResourceConversionError::ConversionNotPossible)
        );

        assert_eq!(
            project.res_into_consumable(1),
            Err(ResourceConversionError::ResourceNotFound)
        );

        project.add_resource(Resource::Material(Material::Consumable(Consumable::new(
            "Stimpack",
        ))));

        assert!(project.res_into_consumable(1).is_ok());

        if let Resource::Material(Material::Consumable(_)) = project.resources()[1] {
        } else {
            panic!("It changed the resource type");
        }

        project.add_resource(Resource::Material(Material::NonConsumable(
            NonConsumable::new("Crowbar"),
        )));
        project.res_into_consumable(2).unwrap();
        if let Resource::Material(Material::Consumable(_)) = project.resources()[2] {
        } else {
            panic!("It didn't change the resource type");
        }
    }

    #[test]
    fn res_into_nonconsumable_returns_the_correct_errors() {
        let mut project = Project::new("World domination");

        project.add_resource(Resource::Personnel {
            person: Person::new("Sebastiano", "Giordano").unwrap(),
            hourly_rate: None,
        });

        assert_eq!(
            project.res_into_nonconsumable(0),
            Err(ResourceConversionError::ConversionNotPossible)
        );

        assert_eq!(
            project.res_into_nonconsumable(1),
            Err(ResourceConversionError::ResourceNotFound)
        );

        project.add_resource(Resource::Material(Material::NonConsumable(
            NonConsumable::new("Crowbar"),
        )));

        assert!(project.res_into_nonconsumable(1).is_ok());

        if let Resource::Material(Material::NonConsumable(_)) = project.resources()[1] {
        } else {
            panic!("It changed the resource type");
        }

        project.add_resource(Resource::Material(Material::Consumable(Consumable::new(
            "Stimpack",
        ))));
        project.res_into_nonconsumable(2).unwrap();
        if let Resource::Material(Material::NonConsumable(_)) = project.resources()[2] {
        } else {
            panic!("It didn't change the resource type");
        }
    }

    fn name_strategy() -> impl Strategy<Value = String> {
        r"[a-zA-Z0-9]{1,30}"
    }

    proptest! {
        #[test]
        fn task_add_rm_lifecycle(mut project in project_strategy()) {
            let initial_count = project.tasks().count();
            let id = project.add_task(Task::new("new task"));
            assert_eq!(project.tasks().count(), initial_count + 1);
            project.rm_task(id).unwrap();
            assert_eq!(project.tasks().count(), initial_count);
            for task in project.tasks() {
                assert_ne!(task.name(), "new task");
            }
        }

        #[test]
        fn rm_task_cleans_subtask_relationships(name in name_strategy()) {
            let mut project = Project::new(name);
            let parent = project.add_task(Task::new("parent"));
            let child = project.add_task(Task::new("child"));
            project.add_subtask(parent, child).unwrap();
            assert_eq!(project.subtasks(parent).collect::<Vec<_>>(), vec![child]);
            project.rm_task(parent).unwrap();
            assert!(project.subtasks(parent).next().is_none());
        }

        #[test]
        fn resource_add_rm_lifecycle(name in name_strategy()) {
            let mut project = Project::new(name);
            assert_eq!(project.resources().len(), 0);
            project.add_resource(Resource::Material(Material::new("widget")));
            assert_eq!(project.resources().len(), 1);
            project.add_resource(Resource::Material(Material::new("gadget")));
            assert_eq!(project.resources().len(), 2);
            let removed = project.rm_resource(0).unwrap();
            assert!(matches!(removed, Resource::Material(ref m) if m.name() == "widget"));
            assert_eq!(project.resources().len(), 1);
            assert!(matches!(project.resources()[0], Resource::Material(ref m) if m.name() == "gadget"));
        }

        #[test]
        fn stakeholder_add_increases_count(name in name_strategy(), first in "[a-zA-Z]{1,50}", last in "[a-zA-Z]{1,50}") {
            let mut project = Project::new(name);
            let p = Person::new(&first, &last).unwrap();
            project.add_stakeholder(Stakeholder::Individual { person: p, description: None });
            assert_eq!(project.stakeholders().len(), 1);
        }

        #[test]
        fn rm_stakeholder_removes_and_returns(name in name_strategy(), first in "[a-zA-Z]{1,50}", last in "[a-zA-Z]{1,50}") {
            let mut project = Project::new(name);
            let p = Person::new(&first, &last).unwrap();
            project.add_stakeholder(Stakeholder::Individual { person: p.clone(), description: None });
            assert_eq!(project.stakeholders().len(), 1);
            let removed = project.rm_stakeholder(0);
            assert!(removed.is_some());
            assert_eq!(project.stakeholders().len(), 0);
            assert!(project.rm_stakeholder(0).is_none());
        }

        #[test]
        fn add_time_relationship_works(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rand::rng();
            let p = rng.random_range(0..ids.len());
            let mut s = p;
            while s == p {
                s = rng.random_range(0..ids.len());
            }

            project.add_time_relationship(ids[p], ids[s], TimeRelationship::FinishToStart).unwrap();
            let succs: Vec<_> = project.successors_ids(ids[p]).collect();
            assert!(succs.contains(&ids[s]), "successors({}) should contain {}", ids[p], ids[s]);
            let preds: Vec<_> = project.predecessors_ids(ids[s]).collect();
            assert!(preds.contains(&ids[p]), "predecessors({}) should contain {}", ids[s], ids[p]);
        }

        #[test]
        fn add_time_relationship_rejects_duplicate(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rand::rng();
            let p = rng.random_range(0..ids.len());
            let mut s = p;
            while s == p {
                s = rng.random_range(0..ids.len());
            }

            project.add_time_relationship(ids[p], ids[s], TimeRelationship::FinishToStart).unwrap();
            assert!(
                project.add_time_relationship(ids[p], ids[s], TimeRelationship::FinishToStart).is_err(),
                "duplicate edge should be rejected"
            );
        }

        #[test]
        fn rm_time_relationship_works(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rand::rng();
            let p = rng.random_range(0..ids.len());
            let mut s = p;
            while s == p {
                s = rng.random_range(0..ids.len());
            }

            project.add_time_relationship(ids[p], ids[s], TimeRelationship::FinishToStart).unwrap();
            project.rm_time_relationship(ids[p], ids[s]).unwrap();
            let succs: Vec<_> = project.successors_ids(ids[p]).collect();
            assert!(!succs.contains(&ids[s]), "successors({}) should not contain {}", ids[p], ids[s]);
        }

        #[test]
        fn add_subtask_works(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rand::rng();
            let p = rng.random_range(0..ids.len());
            let mut c = p;
            while c == p {
                c = rng.random_range(0..ids.len());
            }

            project.add_subtask(ids[p], ids[c]).unwrap();
            assert!(project.subtasks(ids[p]).any(|s| s == ids[c]), "subtasks({}) should contain {}", ids[p], ids[c]);
        }

        #[test]
        fn res_into_consumable_preserves_fields(res_name in name_strategy(), qty in 1u16..1000u16, cost in 1u16..1000u16) {
            let mut project = Project::new("project");
            let mut m = Material::NonConsumable(NonConsumable::new(res_name.clone()));
            m.update_quantity(qty);
            m.update_cost_per_unit(cost);
            project.add_resource(Resource::Material(m));
            project.res_into_consumable(0).unwrap();
            if let Resource::Material(ref m) = project.resources()[0] {
                assert_eq!(m.name(), res_name);
                assert_eq!(m.quantity(), Some(qty));
                assert_eq!(m.cost_per_unit(), Some(cost));
            } else {
                panic!("Expected Material");
            }
        }

        #[test]
        fn res_into_nonconsumable_preserves_fields(res_name in name_strategy(), qty in 1u16..1000u16, cost in 1u16..1000u16) {
            let mut project = Project::new("project");
            let mut m = Material::Consumable(Consumable::new(res_name.clone()));
            m.update_quantity(qty);
            m.update_cost_per_unit(cost);
            project.add_resource(Resource::Material(m));
            project.res_into_nonconsumable(0).unwrap();
            if let Resource::Material(ref m) = project.resources()[0] {
                assert_eq!(m.name(), res_name);
                assert_eq!(m.quantity(), Some(qty));
                assert_eq!(m.cost_per_unit(), Some(cost));
            } else {
                panic!("Expected Material");
            }
        }

        #[test]
        fn add_subtask_rejects_invalid_ids(name in name_strategy()) {
            let mut project = Project::new(name);
            let task = project.add_task(Task::new("only task"));
            let fake = Uuid::new_v4();
            assert!(project.subtasks(fake).next().is_none());
            assert!(project.add_subtask(fake, task).is_err());
            assert!(project.add_subtask(task, fake).is_err());
        }

        #[test]
        fn rm_time_relationship_rejects_invalid_ids(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.is_empty() { return Ok(()); }
            let fake = Uuid::new_v4();
            assert!(project.rm_time_relationship(ids[0], fake).is_err());
            assert!(project.rm_time_relationship(fake, ids[0]).is_err());
        }

        #[test]
        fn rm_task_rejects_invalid_id(name in name_strategy()) {
            let mut project = Project::new(name);
            let fake = Uuid::new_v4();
            assert!(project.rm_task(fake).is_err());
        }

        #[test]
        fn res_into_consumable_on_personnel_returns_error(name in name_strategy()) {
            let mut project = Project::new(name);
            let p = Person::new("test", "person").unwrap();
            project.add_resource(Resource::Personnel { person: p, hourly_rate: None });
            assert_eq!(
                project.res_into_consumable(0),
                Err(ResourceConversionError::ConversionNotPossible)
            );
        }

        #[test]
        fn res_into_nonconsumable_on_personnel_returns_error(name in name_strategy()) {
            let mut project = Project::new(name);
            let p = Person::new("test", "person").unwrap();
            project.add_resource(Resource::Personnel { person: p, hourly_rate: None });
            assert_eq!(
                project.res_into_nonconsumable(0),
                Err(ResourceConversionError::ConversionNotPossible)
            );
        }
    }

    #[test]
    fn add_time_relationship_rejects_invalid_ids() {
        let mut project = Project::new("test");
        let task = project.add_task(Task::new("task"));
        let fake = Uuid::new_v4();
        assert!(
            project
                .add_time_relationship(fake, task, TimeRelationship::FinishToStart)
                .is_err()
        );
        assert!(
            project
                .add_time_relationship(task, fake, TimeRelationship::FinishToStart)
                .is_err()
        );
    }

    #[test]
    fn add_sibling_before_falls_back_to_end_when_sibling_not_found() {
        let mut project = Project::new("World domination");
        let a = project.add_task(Task::new("Build an army"));
        let fake = Uuid::new_v4();
        let b = project.add_sibling_before(Task::new("Train troops"), fake);

        let ids: Vec<Uuid> = project.tasks().map(|t| t.id()).collect();
        assert_eq!(ids, vec![a, b]);
    }

    #[test]
    fn move_task_after_is_noop_for_nonexistent_ids() {
        let mut project = Project::new("World domination");
        let a = project.add_task(Task::new("Build an army"));
        let b = project.add_task(Task::new("Train troops"));
        let fake = Uuid::new_v4();

        project.move_task_after(fake, a);
        assert_eq!(
            project.tasks().map(|t| t.id()).collect::<Vec<_>>(),
            vec![a, b]
        );

        project.move_task_after(a, fake);
        assert_eq!(
            project.tasks().map(|t| t.id()).collect::<Vec<_>>(),
            vec![a, b]
        );
    }

    #[test]
    fn remove_subtask_rejects_non_subtask() {
        let mut project = Project::new("World domination");
        let task = project.add_task(Task::new("Do something"));
        assert!(project.remove_subtask(task).is_err());
    }

    proptest! {
        #[test]
        fn add_sibling_before_inserts_correctly(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rand::rng();
            let idx = rng.random_range(0..ids.len());

            let sibling_id = ids[idx];
            let new_id = project.add_sibling_before(Task::new("Minion"), sibling_id);

            let ordered_ids: Vec<Uuid> = project.tasks().map(|t| t.id()).collect();
            let new_pos = ordered_ids.iter().position(|&id| id == new_id).unwrap();
            let sibling_pos = ordered_ids.iter().position(|&id| id == sibling_id).unwrap();
            assert_eq!(new_pos, sibling_pos - 1);
        }

        #[test]
        fn add_sibling_after_inserts_correctly(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rand::rng();
            let idx = rng.random_range(0..ids.len());

            let sibling_id = ids[idx];
            let new_id = project.add_sibling_after(Task::new("Minion"), sibling_id);

            let ordered_ids: Vec<Uuid> = project.tasks().map(|t| t.id()).collect();
            let new_pos = ordered_ids.iter().position(|&id| id == new_id).unwrap();
            let sibling_pos = ordered_ids.iter().position(|&id| id == sibling_id).unwrap();
            assert_eq!(new_pos, sibling_pos + 1);
        }

        #[test]
        fn add_sibling_inherits_parent(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 3 { return Ok(()); }
            let mut rng = rand::rng();
            let parent_idx = rng.random_range(0..ids.len());
            let child_idx = rng.random_range(0..ids.len());
            if parent_idx == child_idx { return Ok(()); }

            project.add_subtask(ids[parent_idx], ids[child_idx]).unwrap();
            let new_id = project.add_sibling_before(Task::new("Minion"), ids[child_idx]);

            assert_eq!(project.task_parent(new_id), Some(ids[parent_idx]));
            let children: Vec<Uuid> = project.subtasks(ids[parent_idx]).collect();
            assert!(children.contains(&new_id));
        }

        #[test]
        fn move_task_after_reorders_correctly(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 3 { return Ok(()); }
            let mut rng = rand::rng();
            let idx = rng.random_range(0..ids.len());
            let mut after_idx = rng.random_range(0..ids.len());
            while after_idx == idx {
                after_idx = rng.random_range(0..ids.len());
            }

            project.move_task_after(ids[idx], ids[after_idx]);
            let new_ids: Vec<Uuid> = project.tasks().map(|t| t.id()).collect();

            let task_pos = new_ids.iter().position(|&id| id == ids[idx]).unwrap();
            let after_pos = new_ids.iter().position(|&id| id == ids[after_idx]).unwrap();
            assert_eq!(task_pos, after_pos + 1);
        }

        #[test]
        fn remove_subtask_promotes_to_top_level(mut project in project_strategy()) {
            let ids = task_ids(&project);
            if ids.len() < 2 { return Ok(()); }
            let mut rng = rand::rng();
            let parent_idx = rng.random_range(0..ids.len());
            let child_idx = rng.random_range(0..ids.len());
            if parent_idx == child_idx { return Ok(()); }

            project.add_subtask(ids[parent_idx], ids[child_idx]).unwrap();
            assert!(project.task_parent(ids[child_idx]).is_some());
            project.remove_subtask(ids[child_idx]).unwrap();

            assert!(project.task_parent(ids[child_idx]).is_none());
            assert!(project.subtasks(ids[parent_idx]).next().is_none());
        }

        #[test]
        fn sync_parent_dates_expands_to_children(
            mut project in project_strategy(),
            start_offset in 0..1_000_000i64,
            finish_offset in 0..1_000_000i64,
        ) {
            let ids = task_ids(&project);
            if ids.len() < 3 { return Ok(()); }
            let mut rng = rand::rng();
            let parent_idx = rng.random_range(0..ids.len());
            let child1_idx = rng.random_range(0..ids.len());
            let child2_idx = rng.random_range(0..ids.len());
            if child1_idx == parent_idx || child2_idx == parent_idx || child1_idx == child2_idx {
                return Ok(());
            }

            project.add_subtask(ids[parent_idx], ids[child1_idx]).unwrap();
            project.add_subtask(ids[parent_idx], ids[child2_idx]).unwrap();

            let now = Utc::now();
            let child1_start = now - chrono::Duration::milliseconds(start_offset);
            let child2_finish = now + chrono::Duration::milliseconds(finish_offset);
            project.task_mut(ids[child1_idx]).unwrap().edit_start(child1_start).unwrap();
            project.task_mut(ids[child2_idx]).unwrap().edit_finish(child2_finish).unwrap();

            project.sync_parent_dates(ids[parent_idx]).unwrap();

            assert_eq!(project.task(ids[parent_idx]).unwrap().start(), Some(child1_start));
            assert_eq!(project.task(ids[parent_idx]).unwrap().finish(), Some(child2_finish));
        }
    }

    #[test]
    fn sync_parent_dates_noop_when_children_have_no_dates() {
        let mut project = Project::new("World domination");
        let army = project.add_task(Task::new("Build an army"));
        let supplies = project.add_task(Task::new("Gather supplies"));
        project.add_subtask(army, supplies).unwrap();

        let now = Utc::now();
        project.task_mut(army).unwrap().edit_start(now).unwrap();

        project.sync_parent_dates(army).unwrap();
        assert_eq!(project.task(army).unwrap().start(), Some(now));
        assert!(project.task(army).unwrap().finish().is_none());
    }
}
