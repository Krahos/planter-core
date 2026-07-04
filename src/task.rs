use crate::{duration::NonNegativeDuration, resources::Resource};
use anyhow::Context;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
/// A task is a unit of work that can be completed by a person or a group of people.
/// It can be assigned resources and can have a start, finish, and duration.
pub struct Task {
    /// The stable identifier of the task.
    id: Uuid,
    /// The name of the task.
    name: String,
    /// The description of the task.
    description: Option<String>,
    /// Whether the task is completed.
    completed: bool,
    /// The start time of the task.
    start: Option<DateTime<Utc>>,
    /// The finish time of the task.
    finish: Option<DateTime<Utc>>,
    /// The duration of the task.
    duration: Option<NonNegativeDuration>,
    /// The resources assigned to the task.
    resources: Vec<Resource>,
}

impl Task {
    /// Creates a new task with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the task.
    ///
    /// # Returns
    ///
    /// A new task with the given name.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let task = Task::new("Become world leader");
    /// assert_eq!(task.name(), "Become world leader");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Task {
            id: Uuid::new_v4(),
            name: name.into(),
            description: None,
            completed: false,
            start: None,
            finish: None,
            duration: None,
            resources: Vec::new(),
        }
    }

    /// Returns the stable identifier of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let task = Task::new("Become world leader");
    /// let id = task.id();
    /// ```
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    /// Edits the start time of the task.
    /// If a finish time is already set, the duration is recalculated to `finish - start`.
    /// If the new start time is after the finish time, the finish time is pushed
    /// ahead to match, resulting in a zero duration.
    ///
    /// # Arguments
    ///
    /// * `start` - The new start time of the task.
    ///
    /// # Errors
    ///
    /// Returns an error if the task has a finish date and the start date passed
    /// as parameter is too far from that.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Utc;
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// let start_time = Utc::now();
    /// task.edit_start(start_time).unwrap();
    /// assert_eq!(task.start().unwrap(), start_time);
    /// ```
    pub fn edit_start(&mut self, start: DateTime<Utc>) -> anyhow::Result<()> {
        self.start = Some(start);

        if let Some(finish) = self.finish {
            let finish = if finish < start { start } else { finish };
            self.finish = Some(finish);
            self.duration = Some(
                (finish - start)
                    .try_into()
                    .context("Start and finish times were too far apart")?,
            );
        } else if let Some(duration) = self.duration {
            self.finish = Some(start + *duration);
        }
        Ok(())
    }

    /// Returns the start time of the task. It's None by default.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Utc;
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert!(task.start().is_none());
    ///
    /// let start_time = Utc::now();
    /// task.edit_start(start_time).unwrap();
    /// assert_eq!(task.start().unwrap(), start_time);
    /// ```
    #[must_use]
    pub const fn start(&self) -> Option<DateTime<Utc>> {
        self.start
    }

    /// Edits the finish time of the task.
    /// If a start time is already set, the duration is recalculated to `finish - start`.
    /// If the new finish time is before the start time, the start time is pushed
    /// back to match, resulting in a zero duration.
    ///
    /// # Arguments
    ///
    /// * `finish` - The new finish time of the task.
    ///
    /// # Errors
    ///
    /// Returns an error if the task has a start date and the finish date passed
    /// as parameter is too far from that.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Utc;
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert!(task.start().is_none());
    ///
    /// let mut finish_time = Utc::now();
    /// task.edit_finish(finish_time).unwrap();
    /// assert_eq!(task.finish().unwrap(), finish_time);
    /// ```
    pub fn edit_finish(&mut self, finish: DateTime<Utc>) -> anyhow::Result<()> {
        self.finish = Some(finish);

        if let Some(start) = self.start {
            let start = if finish < start {
                self.start = Some(finish);
                finish
            } else {
                start
            };
            let duration = finish - start;
            self.duration = Some(
                duration
                    .try_into()
                    .context("Start time and finish time were too far apart")?,
            );
        } else if let Some(duration) = self.duration {
            self.start = Some(finish - *duration);
        }
        Ok(())
    }

    /// Returns the finish time of the task. It's None by default.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Utc;
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert!(task.finish().is_none());
    /// let finish_time = Utc::now();
    /// task.edit_finish(finish_time).unwrap();
    /// assert_eq!(task.finish().unwrap(), finish_time);
    /// ```
    #[must_use]
    pub const fn finish(&self) -> Option<DateTime<Utc>> {
        self.finish
    }

    /// Edits the duration of the task. If the task has a start time, finish time will be updated accordingly.
    ///
    /// # Arguments
    ///
    /// * `duration` - The new duration of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, Duration};
    /// use planter_core::{task::Task, duration::NonNegativeDuration};
    ///
    /// let mut task = Task::new("Become world leader");
    /// task.edit_duration(Duration::minutes(30).try_into().unwrap());
    /// assert!(task.duration().is_some());
    /// assert_eq!(task.duration().unwrap(), Duration::minutes(30).try_into().unwrap());
    /// ```
    pub fn edit_duration(&mut self, duration: NonNegativeDuration) {
        self.duration = Some(duration);

        if let Some(start) = self.start() {
            let finish = start + *duration;
            self.finish = Some(finish);
        } else if let Some(finish) = self.finish() {
            self.start = Some(finish - *duration);
        }
    }

    /// Adds a [`Resource`] to the task.
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to add to the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::{Resource, Material, NonConsumable}, task::Task};
    ///
    /// let mut task = Task::new("Become world leader");
    /// let resource = Resource::Material(Material::NonConsumable(
    ///   NonConsumable::new("Crowbar"),
    /// ));
    /// task.add_resource(resource);
    ///
    /// assert_eq!(task.resources().len(), 1);
    /// ```
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }

    /// Returns the list of [`Resource`] assigned to the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    /// use planter_core::resources::{Resource, Material, NonConsumable};
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert!(task.resources().is_empty());
    /// let resource = Resource::Material(Material::NonConsumable(
    ///   NonConsumable::new("Crowbar"),
    /// ));
    /// task.add_resource(resource);
    /// assert_eq!(task.resources().len(), 1);
    /// ```
    #[must_use]
    pub fn resources(&self) -> &[Resource] {
        &self.resources
    }

    /// Removes a [`Resource`] from the task by index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the resource to remove.
    ///
    /// # Returns
    ///
    /// The removed resource, or `None` if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{resources::{Resource, Material, NonConsumable}, task::Task};
    ///
    /// let mut task = Task::new("Become world leader");
    /// let resource = Resource::Material(Material::NonConsumable(
    ///   NonConsumable::new("Crowbar"),
    /// ));
    /// task.add_resource(resource);
    /// assert_eq!(task.resources().len(), 1);
    ///
    /// let removed = task.rm_resource(0);
    /// assert!(removed.is_some());
    /// assert_eq!(task.resources().len(), 0);
    /// ```
    #[must_use]
    pub fn rm_resource(&mut self, index: usize) -> Option<Resource> {
        if index < self.resources.len() {
            Some(self.resources.remove(index))
        } else {
            None
        }
    }

    /// Edits the name of the task.
    ///
    /// # Arguments
    ///
    /// * `name` - The new name of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// task.edit_name("Become world boss");
    /// assert_eq!(task.name(), "Become world boss");
    /// ```
    pub fn edit_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Returns the name of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert_eq!(task.name(), "Become world leader");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Edits the description of the task.
    ///
    /// # Arguments
    ///
    /// * `description` - The new description of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// task.edit_description("Description");
    /// assert_eq!(task.description(), Some("Description"));
    /// ```
    pub fn edit_description(&mut self, description: impl Into<String>) {
        self.description = Some(description.into());
    }

    /// Clears the description of the task, setting it to `None`.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// task.edit_description("Description");
    /// assert_eq!(task.description(), Some("Description"));
    ///
    /// task.clear_description();
    /// assert!(task.description().is_none());
    /// ```
    pub fn clear_description(&mut self) {
        self.description = None;
    }

    /// Returns the description of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// task.edit_description("Description");
    /// assert_eq!(task.description(), Some("Description"));
    /// ```
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Whether the task is completed. It's false by default.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert!(!task.completed());
    /// task.toggle_completed();
    /// assert!(task.completed());
    /// ```
    #[must_use]
    pub const fn completed(&self) -> bool {
        self.completed
    }

    /// Marks the task as completed.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert!(!task.completed());
    /// task.toggle_completed();
    /// assert!(task.completed());
    /// task.toggle_completed();
    /// assert!(!task.completed());
    /// ```
    pub const fn toggle_completed(&mut self) {
        self.completed = !self.completed;
    }

    /// Returns the duration of the task. It's None by default.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, Duration};
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader");
    /// assert!(task.duration().is_none());
    ///
    /// task.edit_duration(Duration::hours(1).try_into().unwrap());
    /// assert!(task.duration().unwrap() == Duration::hours(1).try_into().unwrap());
    /// ```
    #[must_use]
    pub const fn duration(&self) -> Option<NonNegativeDuration> {
        self.duration
    }
}

#[cfg(test)]
/// Utilities to test Tasks.
pub mod test_utils {
    use proptest::prelude::*;

    use super::Task;

    /// Generates an empty task with a random name.
    pub fn task_strategy() -> impl Strategy<Value = Task> {
        ".*".prop_map(Task::new)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use proptest::prelude::*;

    use crate::resources::{Material, Resource};
    use crate::task::test_utils::task_strategy;

    use super::*;

    const MAX_TEST_MS: i64 = 1_000_000;

    proptest! {
        #[test]
        fn duration_is_properly_set_when_adding_start_and_finish_time(milliseconds in 0..MAX_TEST_MS) {
            let start = Utc::now();
            let finish = start + Duration::milliseconds(milliseconds);
            let mut task = Task::new("World domination");

            task.edit_start(start).unwrap();
            task.edit_finish(finish).unwrap();

            assert!(task.duration().unwrap() == Duration::milliseconds(milliseconds).try_into().unwrap());
        }

        #[test]
        fn task_times_stay_none_when_adding_duration(milliseconds in 0..MAX_TEST_MS) {
            let mut task = Task::new("World domination");

            let duration = Duration::milliseconds(milliseconds).try_into().unwrap();
            task.edit_duration(duration);
            assert!(task.finish().is_none());
            assert!(task.start().is_none());
        }

        #[test]
        fn finish_time_is_properly_set_when_adding_duration(milliseconds in 0..MAX_TEST_MS) {
            let start = Utc::now();
            let mut task = Task::new("World domination");

            task.edit_start(start).unwrap();
            let duration = Duration::milliseconds(milliseconds).try_into().unwrap();
            task.edit_duration(duration);
            assert!(task.finish().unwrap() == start + *duration);
        }

        #[test]
        fn finish_time_is_properly_pushed_ahead_when_adding_duration(milliseconds in 0..MAX_TEST_MS) {
            let start = Utc::now();
            let finish = start + Duration::milliseconds(milliseconds);
            let mut task = Task::new("World domination");

            task.edit_start(start).unwrap();
            task.edit_finish(finish).unwrap();

            let duration = Duration::milliseconds(milliseconds + 1).try_into().unwrap();
            task.edit_duration(duration);
            assert!(task.finish().unwrap() == start + *duration);
        }


        #[test]
        fn start_time_is_properly_pushed_back_when_adding_earlier_finish_time(milliseconds in 0..MAX_TEST_MS) {
            let start = Utc::now();
            let finish = start - Duration::milliseconds(milliseconds);
            let mut task = Task::new("World domination");

            task.edit_start(start).unwrap();
            task.edit_finish(finish).unwrap();

            assert!(task.start().unwrap() == task.finish().unwrap());
        }
    }

    #[test]
    fn edit_start_clamps_finish_when_start_after_finish() {
        let finish = Utc::now();
        let start = finish + Duration::milliseconds(1);
        let mut task = Task::new("World domination");

        task.edit_finish(finish).unwrap();
        task.edit_start(start).unwrap();

        assert_eq!(task.finish(), Some(start));
        assert_eq!(
            task.duration(),
            Some(Duration::milliseconds(0).try_into().unwrap())
        );
    }

    proptest! {
        #[test]
        fn toggle_completed_is_self_inverse(mut task in task_strategy()) {
            let original = task.completed();
            task.toggle_completed();
            assert_eq!(task.completed(), !original);
            task.toggle_completed();
            assert_eq!(task.completed(), original);
        }

        #[test]
        fn add_resource_increments_count(mut task in task_strategy()) {
            let count = task.resources().len();
            task.add_resource(Resource::Material(Material::new("test")));
            assert_eq!(task.resources().len(), count + 1);
        }

        #[test]
        fn edit_name_roundtrip(mut task in task_strategy(), name in ".*") {
            task.edit_name(&name);
            assert_eq!(task.name(), &name);
        }

        #[test]
        fn start_without_finish_or_duration(start_millis in 0..MAX_TEST_MS) {
            let start = Utc::now() + chrono::Duration::milliseconds(start_millis);
            let mut task = Task::new("test");
            task.edit_start(start).unwrap();
            assert!(task.finish().is_none());
            assert!(task.duration().is_none());
        }

        #[test]
        fn edit_start_and_duration_sets_finish(milliseconds in 0..MAX_TEST_MS) {
            let start = Utc::now();
            let mut task = Task::new("test");
            task.edit_start(start).unwrap();
            let duration = chrono::Duration::milliseconds(milliseconds).try_into().unwrap();
            task.edit_duration(duration);
            assert_eq!(task.finish(), Some(start + *duration));
        }

        #[test]
        fn clear_description_sets_to_none(mut task in task_strategy(), desc in ".*") {
            task.edit_description(&desc);
            assert_eq!(task.description(), Some(desc.as_str()));
            task.clear_description();
            assert!(task.description().is_none());
        }

        #[test]
        fn rm_resource_works_correctly(mut task in task_strategy()) {
            assert!(task.rm_resource(0).is_none());
            task.add_resource(Resource::Material(Material::new("a")));
            task.add_resource(Resource::Material(Material::new("b")));
            assert_eq!(task.resources().len(), 2);
            let removed = task.rm_resource(0);
            assert!(removed.is_some());
            assert_eq!(task.resources().len(), 1);
            assert!(task.rm_resource(1).is_none());
        }

        #[test]
        fn edit_start_with_duration_infers_finish(milliseconds in 0..MAX_TEST_MS) {
            let start = Utc::now();
            let duration = chrono::Duration::milliseconds(milliseconds).try_into().unwrap();
            let mut task = Task::new("World domination");
            task.edit_duration(duration);
            task.edit_start(start).unwrap();
            assert_eq!(task.finish(), Some(start + *duration));
        }

        #[test]
        fn edit_finish_with_duration_infers_start(milliseconds in 0..MAX_TEST_MS) {
            let finish = Utc::now();
            let duration = chrono::Duration::milliseconds(milliseconds).try_into().unwrap();
            let mut task = Task::new("World domination");
            task.edit_duration(duration);
            task.edit_finish(finish).unwrap();
            assert_eq!(task.start(), Some(finish - *duration));
        }

        #[test]
        fn edit_duration_with_finish_infers_start(milliseconds in 0..MAX_TEST_MS) {
            let finish = Utc::now();
            let duration = chrono::Duration::milliseconds(milliseconds).try_into().unwrap();
            let mut task = Task::new("World domination");
            task.edit_finish(finish).unwrap();
            task.edit_duration(duration);
            assert_eq!(task.start(), Some(finish - *duration));
        }
    }
}
