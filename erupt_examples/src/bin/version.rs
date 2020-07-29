use erupt::{vk1_0, EntryLoader};

const TITLE: &str = "erupt_examples: version";

fn main() {
    let entry = EntryLoader::new().unwrap();
    println!(
        "{} - Vulkan Instance {}.{}.{}",
        TITLE,
        vk1_0::version_major(entry.instance_version()),
        vk1_0::version_minor(entry.instance_version()),
        vk1_0::version_patch(entry.instance_version())
    );
}
