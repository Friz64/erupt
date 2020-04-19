use erupt::CoreLoader;

fn main() {
    let mut core = CoreLoader::new().unwrap();

    let api_version = core.instance_version();
    println!(
        "erupt-examples: version - Vulkan {}.{}.{}",
        erupt::version_major(api_version),
        erupt::version_minor(api_version),
        erupt::version_patch(api_version)
    );
}
