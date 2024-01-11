use crate::types::CheckServiceResult;
use common::core::errors::{Error, Result};
use std::path::PathBuf;
use std::sync::Arc;
use templatespecification::core::{
    service::TemplateSpecificationService,
    template_engine::{CheckTemplateArgs, TemplateEngine},
};

pub struct CheckServiceArgs {
    pub entry_dir: PathBuf,
}

pub struct CheckService {
    template_specification_service: Arc<TemplateSpecificationService>,
    template_engine: Arc<TemplateEngine>,
}

impl CheckService {
    pub fn new(
        template_specification_service: Arc<TemplateSpecificationService>,
        template_engine: Arc<TemplateEngine>,
    ) -> Self {
        Self {
            template_specification_service,
            template_engine,
        }
    }

    pub async fn check(&self, args: &CheckServiceArgs) -> Result<CheckServiceResult> {
        let path = &args.entry_dir;
        if !path.exists() {
            return Err(Error::new(format!("path {} does not exist", path.display())));
        }

        let path = path.to_owned();
        let template_configuration = self
            .template_specification_service
            .load_template_configuration(Some(path))
            .await?;

        let args = CheckTemplateArgs { template_configuration };
        let res = self.template_engine.check_template(&args).await?;

        Ok(res.into())
    }
}
