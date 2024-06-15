use crate::files::{FileManagerError, S3FileManager};
use std::collections::HashMap;

pub struct Template(String);

impl From<String> for Template {
    fn from(content: String) -> Self {
        Template(content)
    }
}

pub struct TemplateManager {
    file_manager: S3FileManager,
    templates: HashMap<String, Template>,
}

impl TemplateManager {
    pub fn new(file_manager: S3FileManager) -> Self {
        TemplateManager {
            file_manager,
            templates: HashMap::new(),
        }
    }

    pub async fn get_template(
        &mut self,
        key: impl Into<String>,
    ) -> Result<Template, FileManagerError> {
        let key = key.into();
        if let Some(template) = self.templates.get(&key) {
            let template = Template(template.0.clone());
            return Ok(template);
        }
        let content = self.file_manager.get_file_content(key.clone()).await?;
        self.templates
            .insert(key.clone(), Template(content.clone()));
        Ok(content.into())
    }
}
