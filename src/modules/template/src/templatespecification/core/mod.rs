use std::path::PathBuf;

pub mod interfaces;
pub mod service;
pub mod template_configuration;
pub mod template_engine;
pub mod template_specification;

mod validate_template;

/// Sorts the paths by their directory structure.
fn sort_by_directory_structure(paths: &mut [PathBuf]) {
    paths.sort_by(|a, b| {
        let a_depth = a.components().count();
        let b_depth = b.components().count();

        if a_depth == b_depth {
            a.cmp(b)
        } else {
            a_depth.cmp(&b_depth)
        }
    });
}
