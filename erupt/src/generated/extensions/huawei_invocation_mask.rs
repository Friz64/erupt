#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION")]
pub const HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME")]
pub const HUAWEI_INVOCATION_MASK_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_HUAWEI_invocation_mask");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_INVOCATION_MASK_HUAWEI: *const std::os::raw::c_char = crate::cstr!("vkCmdBindInvocationMaskHUAWEI");
#[doc = "Provided by [`crate::extensions::huawei_invocation_mask`]"]
impl crate::extensions::khr_synchronization2::AccessFlagBits2KHR {
    pub const INVOCATION_MASK_READ_HUAWEI: Self = Self(549755813888);
}
#[doc = "Provided by [`crate::extensions::huawei_invocation_mask`]"]
impl crate::extensions::khr_synchronization2::PipelineStageFlagBits2KHR {
    pub const INVOCATION_MASK_HUAWEI: Self = Self(1099511627776);
}
#[doc = "Provided by [`crate::extensions::huawei_invocation_mask`]"]
impl crate::vk1_0::ImageUsageFlagBits {
    pub const INVOCATION_MASK_HUAWEI: Self = Self(262144);
}
#[doc = "Provided by [`crate::extensions::huawei_invocation_mask`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, image_view: crate::vk1_0::ImageView, image_layout: crate::vk1_0::ImageLayout) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInvocationMaskFeaturesHUAWEI> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInvocationMaskFeaturesHUAWEI> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceInvocationMaskFeaturesHUAWEI")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub invocation_mask: crate::vk1_0::Bool32,
}
impl PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI;
}
impl Default for PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), invocation_mask: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceInvocationMaskFeaturesHUAWEI").field("s_type", &self.s_type).field("p_next", &self.p_next).field("invocation_mask", &(self.invocation_mask != 0)).finish()
    }
}
impl PhysicalDeviceInvocationMaskFeaturesHUAWEI {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
        PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceInvocationMaskFeaturesHUAWEI.html) · Builder of [`PhysicalDeviceInvocationMaskFeaturesHUAWEI`]"]
#[repr(transparent)]
pub struct PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a>(PhysicalDeviceInvocationMaskFeaturesHUAWEI, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
        PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn invocation_mask(mut self, invocation_mask: bool) -> Self {
        self.0.invocation_mask = invocation_mask as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceInvocationMaskFeaturesHUAWEI {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
    fn default() -> PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
    type Target = PhysicalDeviceInvocationMaskFeaturesHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceInvocationMaskFeaturesHUAWEIBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::huawei_invocation_mask`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindInvocationMaskHUAWEI.html) · Function"]
    #[doc(alias = "vkCmdBindInvocationMaskHUAWEI")]
    pub unsafe fn cmd_bind_invocation_mask_huawei(&self, command_buffer: crate::vk1_0::CommandBuffer, image_view: crate::vk1_0::ImageView, image_layout: crate::vk1_0::ImageLayout) -> () {
        let _function = self.cmd_bind_invocation_mask_huawei.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, image_view as _, image_layout as _);
        ()
    }
}
