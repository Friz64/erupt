use erupt::{vk, EntryLoader};

const TITLE: &str = "erupt_examples: version";

fn main() {
    let entry = EntryLoader::new().unwrap();
    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk::version_major(entry.instance_version()),
        vk::version_minor(entry.instance_version()),
        vk::version_patch(entry.instance_version())
    );
}
