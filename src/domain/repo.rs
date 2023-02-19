#[derive(Debug, Clone)]
pub struct Repo {
    name: String,
    description: String,
    url: String,
    repo_type: String,
}

impl Repo {
    pub fn new(name: String, description: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("name is empty".to_string());
        }

        Ok(Self {
            name: name,
            description: description,
            url: "".to_string(),
            repo_type: "".to_string(),
        })
    }
}

mod tests {
    use super::*;

    #[test]
    fn should_create_the_expected_repo() {
        let repo = Repo::new("repo".to_string(), "test description".to_string()).unwrap();

        assert_eq!(repo.name, "repo");
        assert_eq!(repo.description, "test description");
        assert_eq!(repo.url, "");
        assert_eq!(repo.repo_type, "");
    }
}
