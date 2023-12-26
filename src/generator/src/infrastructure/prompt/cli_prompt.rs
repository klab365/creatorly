use crate::core::interfaces::Prompt;
use std::io::{self, Write};
use templatespecification::core::template_specification::{TemplateSpecificationItem, TemplateSpecificationItemType};

pub struct CliPrompt {}

impl CliPrompt {
    const DEFAULT_INDEX_MULTIPLE_CHOICE: usize = 1;
    const ICON_QUESTION: &'static str = "  ❓";
}

impl Prompt for CliPrompt {
    fn get_answer(&self, template_specification_item: &mut TemplateSpecificationItem) {
        match &template_specification_item.get_item() {
            TemplateSpecificationItemType::SingleChoice(choice) => {
                print!(
                    "{} {} ({}): ",
                    CliPrompt::ICON_QUESTION,
                    template_specification_item.get_template_key(),
                    choice
                );
                io::stdout().flush().unwrap();

                let mut answer = String::new();
                let stdin = io::stdin();
                stdin.read_line(&mut answer).expect("issue to read line");
                let cleaned_answer = answer
                    .trim()
                    .strip_suffix("\r\n")
                    .or(answer.trim().strip_prefix('\n'))
                    .unwrap_or(answer.trim())
                    .to_string();

                if cleaned_answer.is_empty() {
                    template_specification_item.set_answer(choice.to_string());
                    return;
                }

                template_specification_item.set_answer(cleaned_answer);
            }
            TemplateSpecificationItemType::MultipleChoice(choices) => {
                println!(
                    "{} {} (1):",
                    CliPrompt::ICON_QUESTION,
                    template_specification_item.get_template_key()
                );
                for (index, choice) in choices.iter().enumerate() {
                    println!("  {} {}", index + 1, choice);
                }

                let mut answer = String::new();
                let stdin = io::stdin();
                stdin.read_line(&mut answer).expect("issue to read line");
                let index = answer
                    .trim()
                    .parse::<usize>()
                    .unwrap_or(CliPrompt::DEFAULT_INDEX_MULTIPLE_CHOICE);

                if index - 1 > choices.len() {
                    println!("index doesn't exist");
                    return;
                }

                template_specification_item.set_answer(choices[index - 1].clone());
            }
        }
    }
}
