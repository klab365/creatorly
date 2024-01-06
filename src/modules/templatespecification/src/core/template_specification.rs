#[derive(Debug, Clone, PartialEq)]
/// The template specification. It contains the questions, which are asked.
pub struct TemplateSpecification {
    /// Represents a list of questions for a template specification.
    pub placeholders: Vec<TemplateSpecificationItem>,
}

impl TemplateSpecification {
    pub const PREFIX: &'static str = "creatorly";

    pub fn new() -> Self {
        Self {
            placeholders: Vec::new(),
        }
    }
}

impl Default for TemplateSpecification {
    fn default() -> Self {
        TemplateSpecification::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents an item in the template specification.
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

    /// Returns the template key.
    pub fn get_template_key(&self) -> &String {
        &self.template_key
    }

    /// Returns the answer.
    pub fn get_answer(&self) -> &String {
        &self.answer
    }

    /// Sets the answer.
    pub fn set_answer(&mut self, answer: String) {
        self.answer = answer;
    }

    /// Returns the item.
    pub fn get_item(&self) -> &TemplateSpecificationItemType {
        &self.item
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
        let mut sut = TemplateSpecification {
            placeholders: Vec::new(),
        };
        sut.placeholders.push(TemplateSpecificationItem {
            template_key: "project_name".to_string(),
            item: TemplateSpecificationItemType::SingleChoice("DemoBoilerplate".to_string()),
            answer: "DemoBoilerplate".to_string(),
        });

        // assert
        assert_eq!(sut.placeholders.len(), 1);
        assert_eq!(sut.placeholders[0].template_key, "project_name");
        assert_eq!(
            sut.placeholders[0].get_single_choice(),
            Some(&String::from("DemoBoilerplate"))
        );
        assert_eq!(sut.placeholders[0].answer, "DemoBoilerplate");
    }

    #[test]
    fn test_multiple_choice() {
        // arrange & act
        let mut sut = TemplateSpecification {
            placeholders: Vec::new(),
        };
        sut.placeholders.push(TemplateSpecificationItem {
            template_key: "licence".to_string(),
            item: TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "GPL".to_string()]),
            answer: "MIT".to_string(),
        });

        // assert
        assert_eq!(sut.placeholders.len(), 1);
        assert_eq!(sut.placeholders[0].template_key, "licence");
        assert_eq!(sut.placeholders[0].answer, "MIT");
        assert_eq!(
            sut.placeholders[0].get_multiple_choice(),
            Some(&vec![String::from("MIT"), String::from("GPL")])
        );
    }

    #[test]
    fn test_multiple_choice_should_be_invalid() {
        // arrange & act
        let mut sut = TemplateSpecification {
            placeholders: Vec::new(),
        };
        sut.placeholders.push(TemplateSpecificationItem {
            template_key: "licence".to_string(),
            item: TemplateSpecificationItemType::MultipleChoice(vec!["MIT".to_string(), "GPL".to_string()]),
            answer: "Apache".to_string(),
        });

        // assert
        assert_eq!(sut.placeholders.len(), 1);
        assert_eq!(sut.placeholders[0].template_key, "licence");
        assert_eq!(sut.placeholders[0].answer, "Apache");
        assert_eq!(
            sut.placeholders[0].get_multiple_choice(),
            Some(&vec![String::from("MIT"), String::from("GPL")])
        );
    }
}
