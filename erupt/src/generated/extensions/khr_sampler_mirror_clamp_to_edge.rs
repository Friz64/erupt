#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_sampler_mirror_clamp_to_edge");
#[doc = "Provided by [`crate::extensions::khr_sampler_mirror_clamp_to_edge`]"]
impl crate::vk1_0::SamplerAddressMode {
    pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self::MIRROR_CLAMP_TO_EDGE;
}
