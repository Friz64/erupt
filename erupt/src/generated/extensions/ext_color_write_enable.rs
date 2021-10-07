#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION")]
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME")]
pub const EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_color_write_enable");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_COLOR_WRITE_ENABLE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetColorWriteEnableEXT");
#[doc = "Provided by [`crate::extensions::ext_color_write_enable`]"]
impl crate::vk1_0::DynamicState {
    pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1000381000);
}
#[doc = "Provided by [`crate::extensions::ext_color_write_enable`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetColorWriteEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, attachment_count: u32, p_color_write_enables: *const crate::vk1_0::Bool32) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceColorWriteEnableFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineColorWriteCreateInfoEXT> for crate::vk1_0::PipelineColorBlendStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineColorWriteCreateInfoEXTBuilder<'_>> for crate::vk1_0::PipelineColorBlendStateCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceColorWriteEnableFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceColorWriteEnableFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub color_write_enable: crate::vk1_0::Bool32,
}
impl PhysicalDeviceColorWriteEnableFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT;
}
impl Default for PhysicalDeviceColorWriteEnableFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), color_write_enable: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceColorWriteEnableFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceColorWriteEnableFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("color_write_enable", &(self.color_write_enable != 0)).finish()
    }
}
impl PhysicalDeviceColorWriteEnableFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
        PhysicalDeviceColorWriteEnableFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceColorWriteEnableFeaturesEXT.html) · Builder of [`PhysicalDeviceColorWriteEnableFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a>(PhysicalDeviceColorWriteEnableFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
        PhysicalDeviceColorWriteEnableFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn color_write_enable(mut self, color_write_enable: bool) -> Self {
        self.0.color_write_enable = color_write_enable as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceColorWriteEnableFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceColorWriteEnableFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceColorWriteEnableFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineColorWriteCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineColorWriteCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub attachment_count: u32,
    pub p_color_write_enables: *const crate::vk1_0::Bool32,
}
impl PipelineColorWriteCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT;
}
impl Default for PipelineColorWriteCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), attachment_count: Default::default(), p_color_write_enables: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineColorWriteCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineColorWriteCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("attachment_count", &self.attachment_count).field("p_color_write_enables", &self.p_color_write_enables).finish()
    }
}
impl PipelineColorWriteCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineColorWriteCreateInfoEXTBuilder<'a> {
        PipelineColorWriteCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorWriteCreateInfoEXT.html) · Builder of [`PipelineColorWriteCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineColorWriteCreateInfoEXTBuilder<'a>(PipelineColorWriteCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineColorWriteCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineColorWriteCreateInfoEXTBuilder<'a> {
        PipelineColorWriteCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn color_write_enables(mut self, color_write_enables: &'a [crate::vk1_0::Bool32]) -> Self {
        self.0.p_color_write_enables = color_write_enables.as_ptr() as _;
        self.0.attachment_count = color_write_enables.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineColorWriteCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineColorWriteCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineColorWriteCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineColorWriteCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineColorWriteCreateInfoEXTBuilder<'a> {
    type Target = PipelineColorWriteCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineColorWriteCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_color_write_enable`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetColorWriteEnableEXT.html) · Function"]
    #[doc(alias = "vkCmdSetColorWriteEnableEXT")]
    pub unsafe fn cmd_set_color_write_enable_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, color_write_enables: &[crate::vk1_0::Bool32]) -> () {
        let _function = self.cmd_set_color_write_enable_ext.expect(crate::NOT_LOADED_MESSAGE);
        let attachment_count = color_write_enables.len();
        let _return = _function(command_buffer as _, attachment_count as _, color_write_enables.as_ptr() as _);
        ()
    }
}
