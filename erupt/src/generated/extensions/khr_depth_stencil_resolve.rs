# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_depth_stencil_resolve.html)\n\n## Extends\n- [`ResolveModeFlagBits`](../../vk1_2/struct.ResolveModeFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_depth_stencil_resolve");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescriptionDepthStencilResolveKHR.html) · Alias"]
pub type SubpassDescriptionDepthStencilResolveKHR =
    crate::vk1_2::SubpassDescriptionDepthStencilResolve;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDepthStencilResolvePropertiesKHR.html) · Alias"]
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR =
    crate::vk1_2::PhysicalDeviceDepthStencilResolveProperties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveModeFlagsKHR.html) · Alias"]
pub type ResolveModeFlagsKHR = crate::vk1_2::ResolveModeFlags;
