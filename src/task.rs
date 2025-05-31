use crate::{duration::PositiveDuration, resources::Resource};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// A task is a unit of work that can be completed by a person or a group of people.
/// It can be assigned resources and can have a start, finish, and duration.
pub struct Task {
    /// The name of the task.
    name: String,
    /// The description of the task.
    description: String,
    /// Whether the task is completed.
    completed: bool,
    /// The start time of the task.
    start: Option<DateTime<Utc>>,
    /// The finish time of the task.
    finish: Option<DateTime<Utc>>,
    /// The duration of the task.
    duration: Option<PositiveDuration>,
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
    /// let task = Task::new("Become world leader".to_owned());
    /// assert_eq!(task.name(), "Become world leader");
    /// ```
    pub fn new(name: String) -> Self {
        Task {
            name,
            description: String::new(),
            completed: false,
            start: None,
            finish: None,
            duration: None,
            resources: Vec::new(),
        }
    }

    /// Edits the start time of the task.
    /// If a duration is already set, the finish time will be updated accordingly.
    /// If there is a finish time set, but not a duration, the duration will be updated accordingly.
    /// The finish time will be pushed ahead if the start time is after the finish time.
    ///
    /// # Arguments
    ///
    /// * `start` - The new start time of the task.
    ///
    /// # Panics
    ///
    /// Panics if start and end times are too far apart see [`duration`] for details.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc, Duration};
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// let start_time = Utc::now();
    /// task.edit_start(start_time);
    /// assert_eq!(task.start().unwrap(), start_time);
    /// ```
    #[allow(clippy::expect_used)]
    pub fn edit_start(&mut self, start: DateTime<Utc>) {
        self.start = Some(start);

        if let Some(duration) = self.duration {
            self.finish = Some(start + *duration);
        }

        if let Some(finish) = self.finish {
            if finish < start {
                self.finish = Some(start);
            }
            if self.duration().is_none() {
                let duration = finish - start;
                self.duration = Some(
                    duration
                        .try_into()
                        .expect("Start time and end time were too far apart"),
                );
            }
        }
    }

    /// Returns the start time of the task. It's None by default.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc};
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert!(task.start().is_none());
    ///
    /// let start_time = Utc::now();
    /// task.edit_start(start_time);
    /// assert_eq!(task.start().unwrap(), start_time);
    /// ```
    pub fn start(&self) -> Option<DateTime<Utc>> {
        self.start
    }

    /// Edits the finish time of the task.
    /// If there is a start time already set, duration will be updated accordingly.
    /// Start time will be pushed back if it's after the finish time.
    ///
    /// # Arguments
    ///
    /// * `finish` - The new finish time of the task.
    ///
    /// # Panics
    ///
    /// Panics if start and end times are too far apart see [`duration`] for details.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc};
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert!(task.start().is_none());
    ///
    /// let mut finish_time = Utc::now();
    /// task.edit_finish(finish_time);
    /// assert_eq!(task.finish().unwrap(), finish_time);
    /// ```
    #[allow(clippy::expect_used)]
    pub fn edit_finish(&mut self, finish: DateTime<Utc>) {
        self.finish = Some(finish);

        if let Some(start) = self.start() {
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
                    .expect("Start time and end time were too far apart"),
            );
        }
    }

    /// Returns the finish time of the task. It's None by default.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::{Utc};
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert!(task.finish().is_none());
    /// let finish_time = Utc::now();
    /// task.edit_finish(finish_time);
    /// assert_eq!(task.finish().unwrap(), finish_time);
    /// ```
    pub fn finish(&self) -> Option<DateTime<Utc>> {
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
    /// use planter_core::{task::Task, duration::PositiveDuration};
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// task.edit_duration(Duration::minutes(30).try_into().unwrap());
    /// assert!(task.duration().is_some());
    /// assert_eq!(task.duration().unwrap(), Duration::minutes(30).try_into().unwrap());
    /// ```
    pub fn edit_duration(&mut self, duration: PositiveDuration) {
        self.duration = Some(duration);

        if let Some(start) = self.start() {
            let finish = start + *duration;
            self.finish = Some(finish);
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
    /// let mut task = Task::new("Become world leader".to_owned());
    /// let resource = Resource::Material(Material::NonConsumable(
    ///   NonConsumable::new("Crowbar".to_owned()),
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
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert!(task.resources().is_empty());
    /// let resource = Resource::Material(Material::NonConsumable(
    ///   NonConsumable::new("Crowbar".to_owned()),
    /// ));
    /// task.add_resource(resource);
    /// assert_eq!(task.resources().len(), 1);
    /// ```
    pub fn resources(&self) -> &[Resource] {
        &self.resources
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
    /// let mut task = Task::new("Become world leader".to_owned());
    /// task.edit_name("Become world boss".to_owned());
    /// assert_eq!(task.name(), "Become world boss");
    /// ```
    pub fn edit_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns the name of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert_eq!(task.name(), "Become world leader");
    /// ```
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
    /// let mut task = Task::new("Become world leader".to_owned());
    /// task.edit_description("Description".to_owned());
    /// assert_eq!(task.description(), "Description");
    /// ```
    pub fn edit_description(&mut self, description: String) {
        self.description = description;
    }

    /// Returns the description of the task.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// task.edit_description("Description".to_owned());
    /// assert_eq!(task.description(), "Description");
    /// ```
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Whether the task is completed. It's false by default.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert!(!task.completed());
    /// task.toggle_completed();
    /// assert!(task.completed());
    /// ```
    pub fn completed(&self) -> bool {
        self.completed
    }

    /// Marks the task as completed.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::task::Task;
    ///
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert!(!task.completed());
    /// task.toggle_completed();
    /// assert!(task.completed());
    /// task.toggle_completed();
    /// assert!(!task.completed());
    /// ```
    pub fn toggle_completed(&mut self) {
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
    /// let mut task = Task::new("Become world leader".to_owned());
    /// assert!(task.duration().is_none());
    ///
    /// task.edit_duration(Duration::hours(1).try_into().unwrap());
    /// assert!(task.duration().unwrap() == Duration::hours(1).try_into().unwrap());
    /// ```
    pub fn duration(&self) -> Option<PositiveDuration> {
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

    use crate::duration::MAX_DURATION;

    use super::*;

    proptest! {
        #[test]
        fn duration_is_properly_set_when_adding_start_and_finish_time(milliseconds in 0..MAX_DURATION) {
            let start = Utc::now();
            let finish = start + Duration::milliseconds(milliseconds);
            let mut task = Task::new("World domination".to_owned());

            task.edit_start(start);
            task.edit_finish(finish);

            assert!(task.duration().unwrap() == Duration::milliseconds(milliseconds).try_into().unwrap());
        }

        #[test]
        fn task_times_stay_none_when_adding_duration(milliseconds in 0..MAX_DURATION) {
            let mut task = Task::new("World domination".to_owned());

            let duration = Duration::milliseconds(milliseconds).try_into().unwrap();
            task.edit_duration(duration);
            assert!(task.finish().is_none());
            assert!(task.start().is_none());
        }

        #[test]
        fn finish_time_is_properly_set_when_adding_duration(milliseconds in 0..MAX_DURATION) {
            let start = Utc::now();
            let mut task = Task::new("World domination".to_owned());

            task.edit_start(start);
            let duration = Duration::milliseconds(milliseconds).try_into().unwrap();
            task.edit_duration(duration);
            assert!(task.finish().unwrap() == start + *duration);
        }

        #[test]
        fn finish_time_is_properly_pushed_ahead_when_adding_duration(milliseconds in 0..MAX_DURATION) {
            let start = Utc::now();
            let finish = start + Duration::milliseconds(milliseconds);
            let mut task = Task::new("World domination".to_owned());

            task.edit_start(start);
            task.edit_finish(finish);

            let duration = Duration::milliseconds(milliseconds + 1).try_into().unwrap();
            task.edit_duration(duration);
            assert!(task.finish().unwrap() == start + *duration);
        }

        #[test]
        fn start_time_is_properly_pushed_back_when_adding_earlier_end_time(milliseconds in 0..MAX_DURATION) {
            let start = Utc::now();
            let finish = start - Duration::milliseconds(milliseconds);
            let mut task = Task::new("World domination".to_owned());

            task.edit_start(start);
            task.edit_finish(finish);

            assert!(task.start().unwrap() == task.finish().unwrap());
        }
    }
}
