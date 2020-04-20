# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_conditional_rendering.html)\n\n## Extends\n- [`AccessFlagBits`](../../vk1_0/struct.AccessFlagBits.html)\n- [`BufferUsageFlagBits`](../../vk1_0/struct.BufferUsageFlagBits.html)\n- [`PipelineStageFlagBits`](../../vk1_0/struct.PipelineStageFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_conditional_rendering");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn ( command_buffer : crate :: vk1_0 :: CommandBuffer , p_conditional_rendering_begin : * const crate :: extensions :: ext_conditional_rendering :: ConditionalRenderingBeginInfoEXT , ) -> std :: ffi :: c_void ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtConditionalRenderingDeviceLoaderExt`](trait.ExtConditionalRenderingDeviceLoaderExt.html)"]
pub struct ExtConditionalRenderingDeviceCommands {
    pub cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    pub cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
}
impl ExtConditionalRenderingDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtConditionalRenderingDeviceCommands> {
        unsafe {
            Some(ExtConditionalRenderingDeviceCommands {
                cmd_begin_conditional_rendering_ext: std::mem::transmute(
                    loader.symbol("vkCmdBeginConditionalRenderingEXT")?,
                ),
                cmd_end_conditional_rendering_ext: std::mem::transmute(
                    loader.symbol("vkCmdEndConditionalRenderingEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtConditionalRenderingDeviceCommands`](struct.ExtConditionalRenderingDeviceCommands.html)"]
pub trait ExtConditionalRenderingDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) · Device Command"]
    unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        conditional_rendering_begin : & crate :: extensions :: ext_conditional_rendering :: ConditionalRenderingBeginInfoEXT,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) · Device Command"]
    unsafe fn cmd_end_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
    ) -> ();
}
impl ExtConditionalRenderingDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html) · Device Command"]
    unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        conditional_rendering_begin : & crate :: extensions :: ext_conditional_rendering :: ConditionalRenderingBeginInfoEXT,
    ) -> () {
        let function = self
            .ext_conditional_rendering
            .as_ref()
            .expect("`ext_conditional_rendering` not loaded")
            .cmd_begin_conditional_rendering_ext;
        let _val = function(command_buffer, conditional_rendering_begin);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndConditionalRenderingEXT.html) · Device Command"]
    unsafe fn cmd_end_conditional_rendering_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
    ) -> () {
        let function = self
            .ext_conditional_rendering
            .as_ref()
            .expect("`ext_conditional_rendering` not loaded")
            .cmd_end_conditional_rendering_ext;
        let _val = function(command_buffer);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
    pub flags: crate::extensions::ext_conditional_rendering::ConditionalRenderingFlagsEXT,
}
impl ConditionalRenderingBeginInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> ConditionalRenderingBeginInfoEXTBuilder<'a> {
        ConditionalRenderingBeginInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ConditionalRenderingBeginInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ConditionalRenderingBeginInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer", &self.buffer)
            .field("offset", &self.offset)
            .field("flags", &self.flags)
            .finish()
    }
}
impl Default for ConditionalRenderingBeginInfoEXT {
    fn default() -> ConditionalRenderingBeginInfoEXT {
        ConditionalRenderingBeginInfoEXT {
            s_type: crate::vk1_0::StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT,
            p_next: std::ptr::null(),
            buffer: Default::default(),
            offset: Default::default(),
            flags: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html) · Builder of [`ConditionalRenderingBeginInfoEXT`](struct.ConditionalRenderingBeginInfoEXT.html)"]
#[repr(transparent)]
pub struct ConditionalRenderingBeginInfoEXTBuilder<'a>(
    ConditionalRenderingBeginInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ConditionalRenderingBeginInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ConditionalRenderingBeginInfoEXTBuilder<'a> {
        ConditionalRenderingBeginInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_conditional_rendering::ConditionalRenderingFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ConditionalRenderingBeginInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ConditionalRenderingBeginInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html) · Flag Bits of [`ConditionalRenderingFlagsEXT`](struct.ConditionalRenderingFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ConditionalRenderingFlagBitsEXT(pub u32);
impl ConditionalRenderingFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ConditionalRenderingFlagsEXT {
        ConditionalRenderingFlagsEXT::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::ext_conditional_rendering`](../../extensions/ext_conditional_rendering/index.html)"]
impl ConditionalRenderingFlagBitsEXT {
    pub const INVERTED_EXT: Self = Self(0x00000001);
}
impl std::fmt::Debug for ConditionalRenderingFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::INVERTED_EXT => "INVERTED_EXT",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConditionalRenderingFlagsEXT.html) · Flags of [`ConditionalRenderingFlagBitsEXT`](struct.ConditionalRenderingFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ConditionalRenderingFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const INVERTED_EXT = ConditionalRenderingFlagBitsEXT :: INVERTED_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub conditional_rendering: crate::vk1_0::Bool32,
    pub inherited_conditional_rendering: crate::vk1_0::Bool32,
}
impl PhysicalDeviceConditionalRenderingFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
        PhysicalDeviceConditionalRenderingFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceConditionalRenderingFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("conditional_rendering", &(self.conditional_rendering != 0))
            .field(
                "inherited_conditional_rendering",
                &(self.inherited_conditional_rendering != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceConditionalRenderingFeaturesEXT {
    fn default() -> PhysicalDeviceConditionalRenderingFeaturesEXT {
        PhysicalDeviceConditionalRenderingFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            conditional_rendering: Default::default(),
            inherited_conditional_rendering: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceConditionalRenderingFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceConditionalRenderingFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html) · Builder of [`PhysicalDeviceConditionalRenderingFeaturesEXT`](struct.PhysicalDeviceConditionalRenderingFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a>(
    PhysicalDeviceConditionalRenderingFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
        PhysicalDeviceConditionalRenderingFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conditional_rendering(mut self, conditional_rendering: bool) -> Self {
        self.0.conditional_rendering = conditional_rendering as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn inherited_conditional_rendering(
        mut self,
        inherited_conditional_rendering: bool,
    ) -> Self {
        self.0.inherited_conditional_rendering = inherited_conditional_rendering as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceConditionalRenderingFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub conditional_rendering_enable: crate::vk1_0::Bool32,
}
impl CommandBufferInheritanceConditionalRenderingInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
        CommandBufferInheritanceConditionalRenderingInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CommandBufferInheritanceConditionalRenderingInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "conditional_rendering_enable",
                &(self.conditional_rendering_enable != 0),
            )
            .finish()
    }
}
impl Default for CommandBufferInheritanceConditionalRenderingInfoEXT {
    fn default() -> CommandBufferInheritanceConditionalRenderingInfoEXT {
        CommandBufferInheritanceConditionalRenderingInfoEXT { s_type : crate :: vk1_0 :: StructureType :: COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT , p_next : std :: ptr :: null ( ) , conditional_rendering_enable : Default :: default ( ) , }
    }
}
impl crate::ExtendableBy<CommandBufferInheritanceConditionalRenderingInfoEXT>
    for crate::vk1_0::CommandBufferInheritanceInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html) · Builder of [`CommandBufferInheritanceConditionalRenderingInfoEXT`](struct.CommandBufferInheritanceConditionalRenderingInfoEXT.html)"]
#[repr(transparent)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a>(
    CommandBufferInheritanceConditionalRenderingInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
        CommandBufferInheritanceConditionalRenderingInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conditional_rendering_enable(mut self, conditional_rendering_enable: bool) -> Self {
        self.0.conditional_rendering_enable = conditional_rendering_enable as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CommandBufferInheritanceConditionalRenderingInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
