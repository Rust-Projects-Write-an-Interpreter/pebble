use std::collections::HashMap;
use std::fmt;

use crate::evaluate::Value;

#[derive(Debug)]
pub struct Environment {
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
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    // Retrieve the value of a variable
    // from the environment. Return an
    // error if the variable is not
    // available (undefined).
    pub fn get(&self, name: &str) -> EnvironmentResult<Value> {
        self.values
            .get(name)
            .cloned()
            .ok_or_else(|| EnvironmentError {
                message: format!("Undefined variable {}.", name),
            })
    }

    // Create a variable with the given
    // value in the environment.
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
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
}
