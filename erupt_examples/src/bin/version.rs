use erupt::CoreLoader;

const TITLE: &str = "erupt_examples: version";

fn main() {
    let mut core = CoreLoader::new().unwrap();

    let api_version = core.instance_version();
    println!(
        "{} - Vulkan {}.{}.{}",
        TITLE,
        erupt::version_major(api_version),
        erupt::version_minor(api_version),
        erupt::version_patch(api_version)
    );
}
