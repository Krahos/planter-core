use chrono::{DateTime, Utc};

use crate::{resources::Resource, stakeholders::Stakeholder, task::Task};

/// Represents a project with a name and a list of resources.
pub struct Project {
    /// The name of the project.
    name: String,
    /// The description of the project.
    description: Option<String>,
    /// The start date of the project.
    start_date: Option<DateTime<Utc>>,
    /// The list of tasks associated with the project.
    tasks: Vec<Task>,
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
    /// let project = Project::new("World domination".to_string());
    /// assert_eq!(project.name(), "World domination");
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            start_date: None,
            resources: Vec::new(),
            stakeholders: Vec::new(),
            tasks: Vec::new(),
        }
    }

    /// Returns the name of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::project::Project;
    ///
    /// let project = Project::new("World domination".to_string());
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
    /// let project = Project::new("World domination".to_string()).with_description("This is a project".to_string());
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
    /// let project = Project::new("World domination".to_string()).with_start_date(start_date);
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
    /// let project = Project::new("World domination".to_string());
    /// assert_eq!(project.description(), None);
    /// ```
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
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
    /// let mut project = Project::new("World domination".to_string());
    /// assert_eq!(project.tasks().len(), 0);
    /// project.add_task(Task::new("Become world leader".to_string()));
    /// assert_eq!(project.tasks().len(), 1);
    /// ```
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Returns the tasks of the project.
    ///
    /// # Example
    ///
    /// ```
    /// use planter_core::{project::Project, task::Task};
    ///
    /// let mut project = Project::new("World domination".to_string());
    /// project.add_task(Task::new("Become world leader".to_string()));
    /// assert_eq!(project.tasks().len(), 1);
    /// ```
    pub fn tasks(&self) -> &Vec<Task> {
        &self.tasks
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
    /// let project = Project::new("World domination".to_string()).with_start_date(start_date);
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
    /// let mut project = Project::new("World domination".to_string());
    /// project.add_resource(Resource::Personnel {
    ///     person: Person::new(Name::parse("Sebastiano".to_string(), "Giordano".to_string()).unwrap()),
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
    /// let mut project = Project::new("World domination".to_string());
    /// project.add_resource(Resource::Material(Material::NonConsumable(
    ///    NonConsumable::new("Crowbar".to_string()),
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
    /// let mut project = Project::new("World domination".to_string());
    /// let person = Person::new(Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap());
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
    /// let mut project = Project::new("World domination".to_string());
    /// let person = Person::new(Name::parse("Margherita".to_string(), "Hack".to_string()).unwrap());
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
