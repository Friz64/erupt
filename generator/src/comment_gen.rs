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
                let part = part
                    .map(|part| format!("#{}", part))
                    .unwrap_or(String::new());

                format!(
                    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/\
                        {}.{}-extensions/man/html/{}.html{})",
                    self.version.0, self.version.1, name, part,
                )
            })
            .unwrap_or("<s>Vulkan Manual Page</s>".into());

        format!("{} · {}", man_page, description)
    }

    pub fn link<I, T>(
        &self,
        comment_location: &Origin,
        target_location: &Origin,
        target_item: I,
        target_title: T,
    ) -> String
    where
        I: Display,
        T: Display,
    {
        let redirection = comment_location.doc_path(target_location);
        format!("[`{}`]({}/{}.html)", target_title, redirection, target_item)
    }

    pub fn provided_by(&self, comment_location: &Origin, target_location: &Origin) -> String {
        format!(
            "Provided by {}",
            self.link(
                comment_location,
                target_location,
                "index",
                target_location.module_path_pretty()
            )
        )
    }
}
