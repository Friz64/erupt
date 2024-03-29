// DO NOT EDIT: @generated by erupt's generator
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;
///<s>Vulkan Manual Page</s> · Constant
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!(
    "VK_EXT_acquire_drm_display"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_ACQUIRE_DRM_DISPLAY_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkAcquireDrmDisplayEXT"
);
///<s>Vulkan Manual Page</s> · Constant
pub const FN_GET_DRM_DISPLAY_EXT: *const std::os::raw::c_char = crate::cstr!(
    "vkGetDrmDisplayEXT"
);
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html) · Function
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    drm_fd: i32,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html) · Function
#[allow(non_camel_case_types)]
pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
///Provided by [`crate::extensions::ext_acquire_drm_display`]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html) · Function
    #[doc(alias = "vkAcquireDrmDisplayEXT")]
    pub unsafe fn acquire_drm_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        drm_fd: i32,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self.acquire_drm_display_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(physical_device as _, drm_fd as _, display as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    ///[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html) · Function
    #[doc(alias = "vkGetDrmDisplayEXT")]
    pub unsafe fn get_drm_display_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_display::DisplayKHR> {
        let _function = self.get_drm_display_ext.expect(crate::NOT_LOADED_MESSAGE);
        let mut display = Default::default();
        let _return = _function(
            physical_device as _,
            drm_fd as _,
            connector_id as _,
            &mut display,
        );
        crate::utils::VulkanResult::new(_return, display)
    }
}
