use std::collections::HashMap;
use std::fmt;

use crate::evaluate::Value;

// The Environment derives Default
// in order to use std::mem::take
// which leaves an empty environment in place
// of the one that has been moved out.
#[derive(Debug, Default)]
pub struct Environment {
    parent: Option<Box<Self>>,
    values: HashMap<String, Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnvironmentError {
    pub message: String,
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type EnvironmentResult<T> = Result<T, EnvironmentError>;

impl Environment {
    // Create a new environment
    // without a parent.
    pub fn new() -> Self {
        Self::default()
    }

    // Create a new environment that
    // takes ownership of the parent.
    pub fn new_with_parent(parent: Self) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            values: HashMap::new(),
        }
    }

    // Consume the environment and give
    // back ownership of the parent.
    pub fn restore_parent(self) -> Option<Self> {
        self.parent.map(|p| *p)
    }

    // Retrieve the value of a variable
    // from the environment. Return an
    // error if the variable is not
    // available (undefined).
    pub fn get(&self, name: &str) -> EnvironmentResult<Value> {
        // Use the internal value storage
        // to retrieve the value of the key.
        let value = self.values.get(name).cloned();

        // Extract the actual value if
        // present, otherwise query the
        // parent environment.
        // If no value is present and there
        // is no parent environment,
        // return an error.
        let Some(v) = value else {
            // Find the parent environment.
            let Some(p) = &self.parent else {
                // There is no parent environment.
                return Err(EnvironmentError {
                    message: format!("Undefined variable {}.", name),
                });
            };

            // Return the value from the
            // parent environment.
            return p.get(name);
        };

        Ok(v)
    }

    // Create a variable with the given
    // value in the environment.
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    // Assign a new value to an existing variable.
    // If the variable doesn't exist in the current
    // environment it tries to assign it in the
    // parent environment.
    // If the variable doesn't exist and there
    // is no parent environment it returns
    // an error.
    pub fn assign(&mut self, name: &str, value: Value) -> EnvironmentResult<()> {
        if let Some(current) = self.values.get_mut(name) {
            *current = value;
            return Ok(());
        }

        // Find the parent environment.
        let Some(p) = &mut self.parent else {
            // There is no parent environment.
            return Err(EnvironmentError {
                message: format!("Undefined variable {}.", name),
            });
        };

        // Assign the value in the
        // parent environment.
        p.assign(name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test that a newly created environment
    // is configured correctly.
    fn test_new() {
        let environment = Environment::new();

        assert_eq!(environment.values.len(), 0)
    }

    #[test]
    // Test that the method `get` returns
    // the correct error when the variable
    // doesn't exist.
    fn test_get_error() {
        let environment = Environment::new();

        assert_eq!(
            environment.get("somevar"),
            Err(EnvironmentError {
                message: String::from("Undefined variable somevar."),
            })
        );
    }

    #[test]
    // Test that the method `get` works
    // correctly when used to retrieve the
    // value of an existing variable.
    fn test_get_variable() {
        let mut environment = Environment::new();

        // Define the variable without using a
        // method of Environment to avoid cyclic
        // test definitions.
        environment
            .values
            .insert(String::from("somevar"), Value::Boolean(true));

        assert_eq!(environment.get("somevar"), Ok(Value::Boolean(true)));
    }

    #[test]
    // Test that the method `define` works
    // correctly when used to create a
    // variable that doesn't exist yet.
    fn test_define_non_existing_variable() {
        let mut environment = Environment::new();

        environment.define(String::from("somevar"), Value::Boolean(true));

        // Check that the variable is present
        // without using a method of Environment
        // to avoid cyclic test definitions.
        assert_eq!(
            environment.values.get("somevar"),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    // Test that the method `define` works
    // correctly when used to create a
    // variable that already exists.
    fn test_define_existing_variable() {
        let mut environment = Environment::new();

        // Define the variable without using a
        // method of Environment to avoid cyclic
        // test definitions.
        environment
            .values
            .insert(String::from("somevar"), Value::Boolean(false));

        environment.define(String::from("somevar"), Value::Boolean(true));

        // Check that the variable is present
        // without using a method of Environment
        // to avoid cyclic test definitions.
        assert_eq!(
            environment.values.get("somevar"),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    // Test that the method `assign` works
    // correctly when used to assign a
    // value to a variable that already exists.
    fn test_assign_variable() {
        let mut environment = Environment::new();

        // Define the variable without using a
        // method of Environment to avoid cyclic
        // test definitions.
        environment
            .values
            .insert(String::from("somevar"), Value::Boolean(true));

        assert_eq!(environment.assign("somevar", Value::Number(42.0)), Ok(()));

        // Check the new value without using
        // a method of Environment to avoid
        // cyclic test definitions.
        assert_eq!(
            environment.values.get("somevar"),
            Some(&Value::Number(42.0))
        );
    }

    #[test]
    // Test that the method `assign` returns
    // the correct error when used to assign a
    // value to a variable that doesn't exist.
    fn test_assign_error() {
        let mut environment = Environment::new();

        assert_eq!(
            environment.assign("somevar", Value::Boolean(true)),
            Err(EnvironmentError {
                message: String::from("Undefined variable somevar."),
            })
        );

        // Check that the failed assignment
        // didn't define the variable.
        assert_eq!(environment.values.get("somevar"), None);
    }

    #[test]
    // Test that the method `get` works
    // correctly when used to retrieve the
    // value of a variable defined in the
    // parent environment and not in the
    // child one.
    fn test_get_variable_from_parent() {
        let mut parent = Environment::new();

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent.values.insert(
            String::from("somevar"),
            Value::String(String::from("parent")),
        );

        // Create a child environment
        // connected with the parent.
        let child = Environment::new_with_parent(parent);

        assert_eq!(
            child.get("somevar"),
            Ok(Value::String(String::from("parent")))
        );
    }

    #[test]
    // Test that the method `get` works
    // correctly when used to retrieve the
    // value of a variable defined both in
    // the child environment and in the
    // parent one.
    fn test_get_variable_from_parent_shadow() {
        let mut parent = Environment::new();

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent.values.insert(
            String::from("somevar"),
            Value::String(String::from("parent")),
        );

        // Create a child environment
        // connected with the parent.
        let mut child = Environment::new_with_parent(parent);

        // Define the variable in the child
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        child.values.insert(
            String::from("somevar"),
            Value::String(String::from("child")),
        );

        assert_eq!(
            child.get("somevar"),
            Ok(Value::String(String::from("child")))
        );
    }

    #[test]
    // Test that the method `get` works
    // correctly when used to retrieve the
    // value of a variable defined in the
    // grandparent environment, not in the
    // child one, not in the parent one.
    fn test_get_variable_from_grandparent() {
        let mut grandparent = Environment::new();

        // Define the variable in the grandparent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        grandparent.values.insert(
            String::from("somevar"),
            Value::String(String::from("grandparent")),
        );

        // Create a parent environment
        // connected with the grandparent.
        let parent = Environment::new_with_parent(grandparent);

        // Create a child environment
        // connected with the parent.
        let child = Environment::new_with_parent(parent);

        assert_eq!(
            child.get("somevar"),
            Ok(Value::String(String::from("grandparent")))
        );
    }

    #[test]
    // Test that the method `get` works
    // correctly when used to retrieve the
    // value of a variable defined in the
    // grandparent environment and in the
    // parent one, but not in the child one.
    fn test_get_variable_from_grandparent_shadow() {
        let mut grandparent = Environment::new();

        // Define the variable in the grandparent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        grandparent.values.insert(
            String::from("somevar"),
            Value::String(String::from("grandparent")),
        );

        // Create a parent environment
        // connected with the grandparent.
        let mut parent = Environment::new_with_parent(grandparent);

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent.values.insert(
            String::from("somevar"),
            Value::String(String::from("parent")),
        );

        // Create a child environment
        // connected with the parent.
        let child = Environment::new_with_parent(parent);

        assert_eq!(
            child.get("somevar"),
            Ok(Value::String(String::from("parent")))
        );
    }

    #[test]
    fn test_get_error_with_hierarchy() {
        let parent = Environment::new();

        // Create a child environment
        // connected with the parent.
        let child = Environment::new_with_parent(parent);

        assert_eq!(
            child.get("somevar"),
            Err(EnvironmentError {
                message: String::from("Undefined variable somevar."),
            })
        );
    }

    #[test]
    // Test that the method `assign` works
    // correctly when used to assign a value
    // to a variable defined in the parent
    // environment and not in the child one.
    fn test_assign_variable_in_parent() {
        let mut parent = Environment::new();

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent
            .values
            .insert(String::from("somevar"), Value::Boolean(true));

        // Create a child environment
        // connected with the parent.
        let mut child = Environment::new_with_parent(parent);

        assert_eq!(child.assign("somevar", Value::Number(42.0)), Ok(()));

        // Check that the assignment didn't
        // define the variable in the child.
        assert_eq!(child.values.get("somevar"), None);

        // Get the parent back and check
        // the new value without using
        // a method of Environment to avoid
        // cyclic test definitions.
        let parent = child.restore_parent().unwrap();

        assert_eq!(parent.values.get("somevar"), Some(&Value::Number(42.0)));
    }

    #[test]
    // Test that the method `assign` returns
    // the correct error when used to assign a
    // value to a variable that doesn't exist
    // in any environment of the hierarchy.
    fn test_assign_error_with_hierarchy() {
        let parent = Environment::new();

        // Create a child environment
        // connected with the parent.
        let mut child = Environment::new_with_parent(parent);

        assert_eq!(
            child.assign("somevar", Value::Boolean(true)),
            Err(EnvironmentError {
                message: String::from("Undefined variable somevar."),
            })
        );
    }

    #[test]
    // Test that the method `restore_parent`
    // gives back the parent environment
    // and drops the child one.
    fn test_restore_parent() {
        let mut parent = Environment::new();

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent
            .values
            .insert(String::from("somevar"), Value::Boolean(true));

        // Create a child environment
        // connected with the parent.
        let mut child = Environment::new_with_parent(parent);

        // Define a variable in the child
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        child
            .values
            .insert(String::from("childvar"), Value::Boolean(false));

        let parent = child.restore_parent().unwrap();

        assert_eq!(parent.values.get("somevar"), Some(&Value::Boolean(true)));
        assert_eq!(parent.values.get("childvar"), None);
    }

    #[test]
    // Test that the method `restore_parent`
    // returns None if there is no parent.
    fn test_restore_parent_no_parent() {
        let environment = Environment::new();

        assert!(environment.restore_parent().is_none());
    }
}
