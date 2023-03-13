/// The template specification. It contains the questions, which are asked.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateSpecification {
    pub questions: Vec<TemplateSpecificationItem>,
}

impl TemplateSpecification {
    pub fn new() -> Self {
        Self { questions: Vec::new() }
    }

    /// Returns true, if all questions are valid. Otherwise false.
    pub fn is_valid(&self) -> bool {
        self.questions.iter().all(|q| q.is_valid())
    }
}

impl Default for TemplateSpecification {
    fn default() -> Self {
        TemplateSpecification::new()
    }
}

/// A template specification item. It contains the template key, the type and the answer.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateSpecificationItem {
    pub item: TemplateSpecificationItemType,
    pub template_key: String,
    pub answer: String,
}

impl TemplateSpecificationItem {
    pub fn new(template_key: String, item: TemplateSpecificationItemType) -> Self {
        Self {
            item,
            template_key,
            answer: String::new(),
        }
    }

    /// checks if the answer is valid
    pub fn is_valid(&self) -> bool {
        match &self.item {
            TemplateSpecificationItemType::SingleChoice(_) => true,
            TemplateSpecificationItemType::MultipleChoice(answers) => answers.contains(&self.answer),
        }
    }

    /// Returns the single choice, if the item is a single choice. Otherwise None.
    pub fn get_single_choice(&self) -> Option<&String> {
        match &self.item {
            TemplateSpecificationItemType::SingleChoice(answer) => Some(answer),
            _ => None,
        }
    }

    /// Returns the multiple choice, if the item is a multiple choice. Otherwise None.
    pub fn get_multiple_choice(&self) -> Option<&Vec<String>> {
        match &self.item {
            TemplateSpecificationItemType::MultipleChoice(answers) => Some(answers),
            _ => None,
        }
    }
}

/// The type of the template specification item.
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateSpecificationItemType {
    /// A single choice item.
    SingleChoice(String),

    /// A multiple choice item.
    MultipleChoice(Vec<String>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_choice() {
        // arrange & act
        let mut sut = TemplateSpecification { questions: Vec::new() };
        sut.questions.push(TemplateSpecificationItem {
            template_key: "project_name".to_string(),
            item: TemplateSpecificationItemType::SingleChoice("DemoBoilerplate".to_string()),
            answer: "DemoBoilerplate".to_string(),
        });

        // assert
        assert_eq!(sut.questions.len(), 1);
        assert_eq!(sut.questions[0].template_key, "project_name");
        assert_eq!(sut.questions[0].get_single_choice(), Some(&String::from("DemoBoilerplate")));
        assert_eq!(sut.questions[0].answer, "DemoBoilerplate");
        assert_eq!(sut.is_valid(), true);
    }

    #[test]
    fn test_multiple_choice() {
        // arrange & act
        let mut sut = TemplateSpecification { questions: Vec::new() };
        sut.questions.push(TemplateSpecificationItem {
            template_key: "licence".to_string(),
            item: TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "GPL".to_string()]),
            answer: "MIT".to_string(),
        });

        // assert
        assert_eq!(sut.questions.len(), 1);
        assert_eq!(sut.questions[0].template_key, "licence");
        assert_eq!(sut.questions[0].answer, "MIT");
        assert_eq!(sut.questions[0].get_multiple_choice(), Some(&vec![String::from("MIT"), String::from("GPL")]));
        assert_eq!(sut.is_valid(), true);
    }

    #[test]
    fn test_multiple_choice_should_be_invalid() {
        // arrange & act
        let mut sut = TemplateSpecification { questions: Vec::new() };
        sut.questions.push(TemplateSpecificationItem {
            template_key: "licence".to_string(),
            item: TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "GPL".to_string()]),
            answer: "Apache".to_string(),
        });

        // assert
        assert_eq!(sut.questions.len(), 1);
        assert_eq!(sut.questions[0].template_key, "licence");
        assert_eq!(sut.questions[0].answer, "Apache");
        assert_eq!(sut.questions[0].get_multiple_choice(), Some(&vec![String::from("MIT"), String::from("GPL")]));
        assert_eq!(sut.is_valid(), false);
    }
}
