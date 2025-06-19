use crate::person::Person;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a resource that can be used in a project. A resource can be either a material or personnel.
pub enum Resource {
    /// Represents a material resource that can be used in a project.
    Material(Material),
    /// Represents a personnel resource. Personnel is usually a resource that can complete tasks
    Personnel {
        /// Information about the person.
        person: Person,
        /// Hourly rate of the person.
        hourly_rate: Option<u16>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a material resource that can be used in a project.
/// It can be either consumable or non-consumable.
pub enum Material {
    /// A consumable resource is a material that needs to be resupplied after use.
    Consumable(Consumable),
    /// A non-consumable resource is a material that does not need to be resupplied after use.
    NonConsumable(NonConsumable),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Represents a consumable material resource that can be used in a project.
pub struct Consumable {
    /// Name of the consumable material.
    name: String,
    /// Available quantity of the consumable material.
    quantity: Option<u16>,
    /// Cost per unit of the consumable material used.
    cost_per_unit: Option<u16>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Represents a non-consumable material resource that can be used in a project.
pub struct NonConsumable {
    /// Name of the non-consumable material.
    name: String,
    /// Available quantity of the non-consumable material.
    quantity: Option<u16>,
    /// Some non consumable materials can have a hourly rate. For example, due to energy consumption.
    hourly_rate: Option<u16>,
}

impl Consumable {
    /// Creates a new consumable material resource.
    pub fn new(name: impl Into<String>) -> Self {
        Consumable {
            name: name.into(),
            quantity: None,
            cost_per_unit: None,
        }
    }

    /// Returns the name of the consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::Consumable;
    ///
    /// let consumable = Consumable::new("Steel".to_owned());
    /// assert_eq!(consumable.name(), "Steel");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the quantity of the consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::Consumable;
    ///
    /// let consumable = Consumable::new("Steel".to_owned());
    /// assert_eq!(consumable.quantity(), None);
    /// ```
    pub fn quantity(&self) -> Option<u16> {
        self.quantity
    }

    /// Returns the cost per unit of the consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::Consumable;
    ///
    /// let consumable = Consumable::new("Steel".to_owned());
    /// assert_eq!(consumable.cost_per_unit(), None);
    /// ```
    pub fn cost_per_unit(&self) -> Option<u16> {
        self.cost_per_unit
    }
}

impl NonConsumable {
    /// Creates a new non-consumable material resource.
    pub fn new(name: impl Into<String>) -> Self {
        NonConsumable {
            name: name.into(),
            quantity: None,
            hourly_rate: None,
        }
    }

    /// Returns the name of the non-consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::NonConsumable;
    ///
    /// let non_consumable = NonConsumable::new("Steel".to_owned());
    /// assert_eq!(non_consumable.name(), "Steel");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the quantity of the non-consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::NonConsumable;
    ///
    /// let non_consumable = NonConsumable::new("Steel".to_owned());
    /// assert_eq!(non_consumable.quantity(), None);
    /// ```
    pub fn quantity(&self) -> Option<u16> {
        self.quantity
    }

    /// Returns the hourly rate of the non-consumable material.
    /// # Example
    /// ```
    /// use planter_core::resources::NonConsumable;
    ///
    /// let non_consumable = NonConsumable::new("Steel".to_owned());
    /// assert_eq!(non_consumable.hourly_rate(), None);
    /// ```
    pub fn hourly_rate(&self) -> Option<u16> {
        self.hourly_rate
    }
}
