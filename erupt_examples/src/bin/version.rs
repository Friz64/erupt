use erupt::{vk, EntryLoader};

const TITLE: &str = "erupt_examples: version";

fn main() {
    let entry = EntryLoader::new().unwrap();
    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk::api_version_major(entry.instance_version()),
        vk::api_version_minor(entry.instance_version()),
        vk::api_version_patch(entry.instance_version())
    );
}
