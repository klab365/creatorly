mod application;
mod domain;

use crate::application::create::service::CreateProjectInput;
use crate::application::create::service::Service as CreateProjectService;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
