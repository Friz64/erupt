use crate::origin::Origin;
use std::fmt::Display;

pub struct DocCommentGen {
    version: (u32, u32),
}

impl DocCommentGen {
    pub fn new(version: (u32, u32)) -> DocCommentGen {
        DocCommentGen { version }
    }

    pub fn def<T>(&self, full_name: Option<&str>, description: T, part: Option<&str>) -> String
    where
        T: Display,
    {
        let man_page = full_name
            .map(|name| {
                let part = part.map(|part| format!("#{}", part)).unwrap_or_default();
                format!(
                    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/\
                        {}.{}-extensions/man/html/{}.html{})",
                    self.version.0, self.version.1, name, part,
                )
            })
            .unwrap_or_else(|| "<s>Vulkan Manual Page</s>".into());

        format!("{} · {}", man_page, description)
    }

    pub fn provided_by(&self, target_location: &Origin) -> String {
        format!(
            "Provided by [`crate::{}`]",
            target_location.module_path_pretty()
        )
    }
}
