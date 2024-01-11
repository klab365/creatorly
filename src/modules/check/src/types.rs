use templatespecification::core::template_engine::CheckTemplateResult;

#[derive(Default)]
pub struct CheckServiceResult {
    pub issues: Vec<String>,
}

impl CheckServiceResult {
    pub fn new() -> Self {
        Self { issues: vec![] }
    }

    pub fn add_issue(&mut self, issue: String) {
        self.issues.push(issue);
    }

    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty()
    }
}

impl From<CheckTemplateResult> for CheckServiceResult {
    fn from(result: CheckTemplateResult) -> Self {
        let mut service_result = Self::new();

        for issue in result.issues.iter() {
            service_result.add_issue(issue.clone());
        }

        service_result
    }
}
