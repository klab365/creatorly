use super::template_configuration::TemplateConfiguration;
use common::core::errors::{Error, Result};

type ValidateFunction = fn(&TemplateConfiguration) -> Result<()>;

// list of validation funcitons
const VALIDATION_FUNCTIONS: [ValidateFunction; 1] = [have_empty_file_list];

/// Validates the template configuration.
/// It checks if the configuration is valid.
/// It returns an error if the configuration is invalid.
///
/// The validate functions are defined in the VALIDATION_FUNCTIONS array.
pub fn validate_template_configuration(template: &TemplateConfiguration) -> Result<()> {
    for validation_function in VALIDATION_FUNCTIONS.iter() {
        validation_function(template)?;
    }

    Ok(())
}

fn have_empty_file_list(template: &TemplateConfiguration) -> Result<()> {
    for item in &template.templates {
        if item.file_list.is_empty() {
            return Err(Error::new(format!(
                "It has no files on path: {}",
                item.root_path.display()
            )));
        }
    }

    Ok(())
}
