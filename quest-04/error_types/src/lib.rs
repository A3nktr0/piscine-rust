pub use chrono::{DateTime, NaiveDate, Utc};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut validated_fields = Vec::new();

        if self.first_name.is_empty() {
            return Err(FormError::new(
                "first_name".to_string(),
                self.first_name.clone(),
                "No user name".to_string(),
            ));
        } else {
            validated_fields.push("Valid first name");
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password".to_string(),
                self.password.clone(),
                "At least 8 characters".to_string(),
            ));
        }

        let mut password_validator = (0, 0, 0);
        for c in self.password.chars() {
            if c.is_numeric() {
                password_validator.0 += 1;
            } else if c.is_alphabetic() {
                password_validator.1 += 1;
            } else if !c.is_alphanumeric() {
                password_validator.2 += 1;
            }
        }

        if password_validator.0 == 0 || password_validator.1 == 0 || password_validator.2 == 0 {
            return Err(FormError::new("password".to_string(), self.password.clone(), "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string()));
        } else {
            validated_fields.push("Valid password");
        }

        Ok(validated_fields)
    }
}
