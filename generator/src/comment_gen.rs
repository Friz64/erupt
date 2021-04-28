use crate::origin::Origin;
use std::fmt::Display;

pub struct DocCommentGen {
    version: (u32, u32, u32),
}

impl DocCommentGen {
    pub fn new(version: (u32, u32, u32)) -> DocCommentGen {
        DocCommentGen { version }
    }

    pub fn def<T>(&self, full_name: Option<&str>, description: T, origin: Option<&Origin>) -> String
    where
        T: Display,
    {
        let man_page = if matches!(origin, Some(Origin::External { .. })) || full_name.is_none() {
            "<s>Vulkan Manual Page</s>".into()
        } else {
            format!(
                "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/\
                    {}.{}-extensions/man/html/{}.html)",
                self.version.0,
                self.version.1,
                full_name.unwrap(),
            )
        };

        format!("{} · {}", man_page, description)
    }

    pub fn provided_by(&self, target_location: &Origin) -> String {
        format!(
            "Provided by [`crate::{}`]",
            target_location.module_path_pretty()
        )
    }

    pub fn major_version(&self) -> u32 {
        self.version.0
    }

    pub fn minor_version(&self) -> u32 {
        self.version.1
    }

    pub fn patch_version(&self) -> u32 {
        self.version.2
    }
}
