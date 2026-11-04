use std::collections::HashMap;
use std::fmt;

use crate::evaluate::Value;

#[derive(Debug)]
pub struct Environment<'a> {
    parent: Option<&'a Self>,
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

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    // Assign the given parent to the environment.
    pub fn add_parent(&mut self, parent: &'a Self) {
        self.parent = Some(parent);
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
            let Some(p) = self.parent else {
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

    // Assign a new value to an existing
    // variable. Return an error if the
    // variable is not defined.
    pub fn assign(&mut self, name: &str, value: Value) -> EnvironmentResult<()> {
        let current = self.values.get_mut(name).ok_or_else(|| EnvironmentError {
            message: format!("Undefined variable {}.", name),
        })?;

        *current = value;

        Ok(())
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
        let mut child = Environment::new();

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent.values.insert(
            String::from("somevar"),
            Value::String(String::from("parent")),
        );

        // Connect child and parent environments.
        child.add_parent(&parent);

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
        let mut child = Environment::new();

        // Define the variable in the child
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        child.values.insert(
            String::from("somevar"),
            Value::String(String::from("child")),
        );

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent.values.insert(
            String::from("somevar"),
            Value::String(String::from("parent")),
        );

        // Connect child and parent environments.
        child.add_parent(&parent);

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
        let mut parent = Environment::new();
        let mut child = Environment::new();

        // Define the variable in the grandparent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        grandparent.values.insert(
            String::from("somevar"),
            Value::String(String::from("grandparent")),
        );

        // Connect parent and grandparent environments.
        parent.add_parent(&grandparent);

        // Connect child and parent environments.
        child.add_parent(&parent);

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
        let mut parent = Environment::new();
        let mut child = Environment::new();

        // Define the variable in the parent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        parent.values.insert(
            String::from("somevar"),
            Value::String(String::from("parent")),
        );

        // Define the variable in the grandparent
        // environment without using a method
        // of Environment to avoid cyclic
        // test definitions.
        grandparent.values.insert(
            String::from("somevar"),
            Value::String(String::from("grandparent")),
        );

        // Connect parent and grandparent environments.
        parent.add_parent(&grandparent);

        // Connect child and parent environments.
        child.add_parent(&parent);

        assert_eq!(
            child.get("somevar"),
            Ok(Value::String(String::from("parent")))
        );
    }

    #[test]
    fn test_get_error_with_hierarchy() {
        let parent = Environment::new();
        let mut child = Environment::new();

        // Connect child and parent environments.
        child.add_parent(&parent);

        assert_eq!(
            child.get("somevar"),
            Err(EnvironmentError {
                message: String::from("Undefined variable somevar."),
            })
        );
    }
}
