#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION")]
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME")]
pub const EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_conditional_rendering");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_CONDITIONAL_RENDERING_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdBeginConditionalRenderingEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_CONDITIONAL_RENDERING_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdEndConditionalRenderingEXT");
#[doc = "Provided by [`crate::extensions::ext_conditional_rendering`]"]
impl crate::vk1_0::BufferUsageFlagBits {
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(512);
}
#[doc = "Provided by [`crate::extensions::ext_conditional_rendering`]"]
impl crate::vk1_0::AccessFlagBits {
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576);
}
#[doc = "Provided by [`crate::extensions::ext_conditional_rendering`]"]
impl crate::vk1_0::StructureType {
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1000081000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1000081001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
}
#[doc = "Provided by [`crate::extensions::ext_conditional_rendering`]"]
impl crate::vk1_0::PipelineStageFlagBits {
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(262144);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingFlagsEXT.html) · Bitmask of [`ConditionalRenderingFlagBitsEXT`]"] # [doc (alias = "VkConditionalRenderingFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct ConditionalRenderingFlagsEXT : u32 { const INVERTED_EXT = ConditionalRenderingFlagBitsEXT :: INVERTED_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) · Bits enum of [`ConditionalRenderingFlagsEXT`]"]
#[doc(alias = "VkConditionalRenderingFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ConditionalRenderingFlagBitsEXT(pub u32);
impl ConditionalRenderingFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ConditionalRenderingFlagsEXT {
        ConditionalRenderingFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ConditionalRenderingFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INVERTED_EXT => "INVERTED_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_conditional_rendering`]"]
impl crate::extensions::ext_conditional_rendering::ConditionalRenderingFlagBitsEXT {
    pub const INVERTED_EXT: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_conditional_rendering_begin: *const crate::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndConditionalRenderingEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> ();
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceConditionalRenderingFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, CommandBufferInheritanceConditionalRenderingInfoEXT> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'_>> for crate::vk1_0::CommandBufferInheritanceInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceConditionalRenderingFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) · Structure"]
#[doc(alias = "VkConditionalRenderingBeginInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
    pub flags: crate::extensions::ext_conditional_rendering::ConditionalRenderingFlagsEXT,
}
impl Default for ConditionalRenderingBeginInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT, p_next: std::ptr::null(), buffer: Default::default(), offset: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for ConditionalRenderingBeginInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConditionalRenderingBeginInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("buffer", &self.buffer).field("offset", &self.offset).field("flags", &self.flags).finish()
    }
}
impl ConditionalRenderingBeginInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ConditionalRenderingBeginInfoEXTBuilder<'a> {
        ConditionalRenderingBeginInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) · Builder of [`ConditionalRenderingBeginInfoEXT`]"]
#[repr(transparent)]
pub struct ConditionalRenderingBeginInfoEXTBuilder<'a>(ConditionalRenderingBeginInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ConditionalRenderingBeginInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ConditionalRenderingBeginInfoEXTBuilder<'a> {
        ConditionalRenderingBeginInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_conditional_rendering::ConditionalRenderingFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ConditionalRenderingBeginInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for ConditionalRenderingBeginInfoEXTBuilder<'a> {
    fn default() -> ConditionalRenderingBeginInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ConditionalRenderingBeginInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ConditionalRenderingBeginInfoEXTBuilder<'a> {
    type Target = ConditionalRenderingBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ConditionalRenderingBeginInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) · Structure"]
#[doc(alias = "VkCommandBufferInheritanceConditionalRenderingInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub conditional_rendering_enable: crate::vk1_0::Bool32,
}
impl Default for CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT, p_next: std::ptr::null(), conditional_rendering_enable: Default::default() }
    }
}
impl std::fmt::Debug for CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferInheritanceConditionalRenderingInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("conditional_rendering_enable", &(self.conditional_rendering_enable != 0)).finish()
    }
}
impl CommandBufferInheritanceConditionalRenderingInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
        CommandBufferInheritanceConditionalRenderingInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) · Builder of [`CommandBufferInheritanceConditionalRenderingInfoEXT`]"]
#[repr(transparent)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a>(CommandBufferInheritanceConditionalRenderingInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
        CommandBufferInheritanceConditionalRenderingInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn conditional_rendering_enable(mut self, conditional_rendering_enable: bool) -> Self {
        self.0.conditional_rendering_enable = conditional_rendering_enable as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandBufferInheritanceConditionalRenderingInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    fn default() -> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    type Target = CommandBufferInheritanceConditionalRenderingInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceConditionalRenderingFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub conditional_rendering: crate::vk1_0::Bool32,
    pub inherited_conditional_rendering: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT, p_next: std::ptr::null_mut(), conditional_rendering: Default::default(), inherited_conditional_rendering: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceConditionalRenderingFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("conditional_rendering", &(self.conditional_rendering != 0)).field("inherited_conditional_rendering", &(self.inherited_conditional_rendering != 0)).finish()
    }
}
impl PhysicalDeviceConditionalRenderingFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
        PhysicalDeviceConditionalRenderingFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) · Builder of [`PhysicalDeviceConditionalRenderingFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a>(PhysicalDeviceConditionalRenderingFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
        PhysicalDeviceConditionalRenderingFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn conditional_rendering(mut self, conditional_rendering: bool) -> Self {
        self.0.conditional_rendering = conditional_rendering as _;
        self
    }
    #[inline]
    pub fn inherited_conditional_rendering(mut self, inherited_conditional_rendering: bool) -> Self {
        self.0.inherited_conditional_rendering = inherited_conditional_rendering as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceConditionalRenderingFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceConditionalRenderingFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_conditional_rendering`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) · Function"]
    #[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
    pub unsafe fn cmd_begin_conditional_rendering_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, conditional_rendering_begin: &crate::extensions::ext_conditional_rendering::ConditionalRenderingBeginInfoEXT) -> () {
        let _function = self.cmd_begin_conditional_rendering_ext.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, conditional_rendering_begin as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) · Function"]
    #[doc(alias = "vkCmdEndConditionalRenderingEXT")]
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: crate::vk1_0::CommandBuffer) -> () {
        let _function = self.cmd_end_conditional_rendering_ext.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _);
        ()
    }
}
