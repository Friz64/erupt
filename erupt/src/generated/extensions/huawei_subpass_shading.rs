#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION")]
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 0;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME")]
pub const HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_HUAWEI_subpass_shading");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_SUBPASS_SHADING_MAX_WORKGROUP_SIZE_HUAWEI: *const std::os::raw::c_char = crate::cstr!("vkGetSubpassShadingMaxWorkgroupSizeHUAWEI");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SUBPASS_SHADING_HUAWEI: *const std::os::raw::c_char = crate::cstr!("vkCmdSubpassShadingHUAWEI");
#[doc = "Provided by [`crate::extensions::huawei_subpass_shading`]"]
impl crate::extensions::khr_synchronization2::PipelineStageFlagBits2KHR {
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(549755813888);
}
#[doc = "Provided by [`crate::extensions::huawei_subpass_shading`]"]
impl crate::vk1_0::PipelineBindPoint {
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1000369003);
}
#[doc = "Provided by [`crate::extensions::huawei_subpass_shading`]"]
impl crate::vk1_0::ShaderStageFlagBits {
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(16384);
}
#[doc = "Provided by [`crate::extensions::huawei_subpass_shading`]"]
impl crate::vk1_0::StructureType {
    pub const SUBPASSS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSubpassShadingMaxWorkgroupSizeHUAWEI.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(renderpass: crate::vk1_0::RenderPass, p_max_workgroup_size: *mut crate::vk1_0::Extent2D) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSubpassShadingHUAWEI.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> ();
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceSubpassShadingFeaturesHUAWEI> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, SubpassShadingPipelineCreateInfoHUAWEI> for crate::vk1_0::ComputePipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, SubpassShadingPipelineCreateInfoHUAWEIBuilder<'_>> for crate::vk1_0::ComputePipelineCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceSubpassShadingFeaturesHUAWEI> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceSubpassShadingPropertiesHUAWEI> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html) · Structure"]
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassShadingPipelineCreateInfoHUAWEI {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub render_pass: crate::vk1_0::RenderPass,
    pub subpass: u32,
}
impl Default for SubpassShadingPipelineCreateInfoHUAWEI {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::SUBPASSS_SHADING_PIPELINE_CREATE_INFO_HUAWEI, p_next: std::ptr::null_mut(), render_pass: Default::default(), subpass: Default::default() }
    }
}
impl std::fmt::Debug for SubpassShadingPipelineCreateInfoHUAWEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubpassShadingPipelineCreateInfoHUAWEI").field("s_type", &self.s_type).field("p_next", &self.p_next).field("render_pass", &self.render_pass).field("subpass", &self.subpass).finish()
    }
}
impl SubpassShadingPipelineCreateInfoHUAWEI {
    #[inline]
    pub fn into_builder<'a>(self) -> SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
        SubpassShadingPipelineCreateInfoHUAWEIBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html) · Builder of [`SubpassShadingPipelineCreateInfoHUAWEI`]"]
#[repr(transparent)]
pub struct SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a>(SubpassShadingPipelineCreateInfoHUAWEI, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
        SubpassShadingPipelineCreateInfoHUAWEIBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn render_pass(mut self, render_pass: crate::vk1_0::RenderPass) -> Self {
        self.0.render_pass = render_pass as _;
        self
    }
    #[inline]
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.0.subpass = subpass as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SubpassShadingPipelineCreateInfoHUAWEI {
        self.0
    }
}
impl<'a> std::default::Default for SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
    fn default() -> SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
    type Target = SubpassShadingPipelineCreateInfoHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassShadingPipelineCreateInfoHUAWEIBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
impl Default for PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI, p_next: std::ptr::null_mut(), max_subpass_shading_workgroup_size_aspect_ratio: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSubpassShadingPropertiesHUAWEI").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_subpass_shading_workgroup_size_aspect_ratio", &self.max_subpass_shading_workgroup_size_aspect_ratio).finish()
    }
}
impl PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
        PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html) · Builder of [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a>(PhysicalDeviceSubpassShadingPropertiesHUAWEI, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
        PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_subpass_shading_workgroup_size_aspect_ratio(mut self, max_subpass_shading_workgroup_size_aspect_ratio: u32) -> Self {
        self.0.max_subpass_shading_workgroup_size_aspect_ratio = max_subpass_shading_workgroup_size_aspect_ratio as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSubpassShadingPropertiesHUAWEI {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
    fn default() -> PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
    type Target = PhysicalDeviceSubpassShadingPropertiesHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubpassShadingPropertiesHUAWEIBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub subpass_shading: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI, p_next: std::ptr::null_mut(), subpass_shading: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSubpassShadingFeaturesHUAWEI").field("s_type", &self.s_type).field("p_next", &self.p_next).field("subpass_shading", &(self.subpass_shading != 0)).finish()
    }
}
impl PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
        PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html) · Builder of [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a>(PhysicalDeviceSubpassShadingFeaturesHUAWEI, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
        PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn subpass_shading(mut self, subpass_shading: bool) -> Self {
        self.0.subpass_shading = subpass_shading as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSubpassShadingFeaturesHUAWEI {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
    fn default() -> PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
    type Target = PhysicalDeviceSubpassShadingFeaturesHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubpassShadingFeaturesHUAWEIBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::huawei_subpass_shading`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSubpassShadingMaxWorkgroupSizeHUAWEI.html) · Function"]
    #[doc(alias = "vkGetSubpassShadingMaxWorkgroupSizeHUAWEI")]
    pub unsafe fn get_subpass_shading_max_workgroup_size_huawei(&self, renderpass: crate::vk1_0::RenderPass) -> crate::utils::VulkanResult<crate::vk1_0::Extent2D> {
        let _function = self.get_subpass_shading_max_workgroup_size_huawei.expect(crate::NOT_LOADED_MESSAGE);
        let mut max_workgroup_size = Default::default();
        let _return = _function(renderpass as _, &mut max_workgroup_size);
        crate::utils::VulkanResult::new(_return, max_workgroup_size)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSubpassShadingHUAWEI.html) · Function"]
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    pub unsafe fn cmd_subpass_shading_huawei(&self, command_buffer: crate::vk1_0::CommandBuffer) -> () {
        let _function = self.cmd_subpass_shading_huawei.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _);
        ()
    }
}
