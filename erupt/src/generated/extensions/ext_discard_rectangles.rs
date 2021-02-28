#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION")]
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME")]
pub const EXT_DISCARD_RECTANGLES_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_discard_rectangles");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DISCARD_RECTANGLE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetDiscardRectangleEXT");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html) · Bitmask of [`PipelineDiscardRectangleStateCreateFlagBitsEXT`]"] # [doc (alias = "VkPipelineDiscardRectangleStateCreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct PipelineDiscardRectangleStateCreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineDiscardRectangleStateCreateFlagsEXT`]"]
#[doc(alias = "VkPipelineDiscardRectangleStateCreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineDiscardRectangleStateCreateFlagBitsEXT(pub u32);
impl PipelineDiscardRectangleStateCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineDiscardRectangleStateCreateFlagsEXT {
        PipelineDiscardRectangleStateCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineDiscardRectangleStateCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDiscardRectangleModeEXT.html) · Enum"]
#[doc(alias = "VkDiscardRectangleModeEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DiscardRectangleModeEXT(pub i32);
impl std::fmt::Debug for DiscardRectangleModeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INCLUSIVE_EXT => "INCLUSIVE_EXT",
            &Self::EXCLUSIVE_EXT => "EXCLUSIVE_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_discard_rectangles`]"]
impl DiscardRectangleModeEXT {
    pub const INCLUSIVE_EXT: Self = Self(0);
    pub const EXCLUSIVE_EXT: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleEXT =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, first_discard_rectangle: u32, discard_rectangle_count: u32, p_discard_rectangles: *const crate::vk1_0::Rect2D) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceDiscardRectanglePropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_discard_rectangles: u32,
}
impl Default for PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_discard_rectangles: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceDiscardRectanglePropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_discard_rectangles", &self.max_discard_rectangles)
            .finish()
    }
}
impl PhysicalDeviceDiscardRectanglePropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
        PhysicalDeviceDiscardRectanglePropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html) · Builder of [`PhysicalDeviceDiscardRectanglePropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a>(PhysicalDeviceDiscardRectanglePropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
        PhysicalDeviceDiscardRectanglePropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_discard_rectangles(mut self, max_discard_rectangles: u32) -> Self {
        self.0.max_discard_rectangles = max_discard_rectangles as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceDiscardRectanglePropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceDiscardRectanglePropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineDiscardRectangleStateCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: crate::extensions::ext_discard_rectangles::DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const crate::vk1_0::Rect2D,
}
impl Default for PipelineDiscardRectangleStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            discard_rectangle_mode: Default::default(),
            discard_rectangle_count: Default::default(),
            p_discard_rectangles: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for PipelineDiscardRectangleStateCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineDiscardRectangleStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("discard_rectangle_mode", &self.discard_rectangle_mode)
            .field("discard_rectangle_count", &self.discard_rectangle_count)
            .field("p_discard_rectangles", &self.p_discard_rectangles)
            .finish()
    }
}
impl PipelineDiscardRectangleStateCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
        PipelineDiscardRectangleStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateInfoEXT.html) · Builder of [`PipelineDiscardRectangleStateCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a>(PipelineDiscardRectangleStateCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
        PipelineDiscardRectangleStateCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn discard_rectangle_mode(mut self, discard_rectangle_mode: crate::extensions::ext_discard_rectangles::DiscardRectangleModeEXT) -> Self {
        self.0.discard_rectangle_mode = discard_rectangle_mode as _;
        self
    }
    #[inline]
    pub fn discard_rectangles(mut self, discard_rectangles: &'a [crate::vk1_0::Rect2DBuilder]) -> Self {
        self.0.p_discard_rectangles = discard_rectangles.as_ptr() as _;
        self.0.discard_rectangle_count = discard_rectangles.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineDiscardRectangleStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineDiscardRectangleStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_discard_rectangles`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) · Function"]
    #[doc(alias = "vkCmdSetDiscardRectangleEXT")]
    pub unsafe fn cmd_set_discard_rectangle_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, first_discard_rectangle: u32, discard_rectangles: &[crate::vk1_0::Rect2DBuilder]) -> () {
        let _function = self.cmd_set_discard_rectangle_ext.expect("`cmd_set_discard_rectangle_ext` is not loaded");
        let discard_rectangle_count = discard_rectangles.len();
        let _return = _function(command_buffer as _, first_discard_rectangle as _, discard_rectangle_count as _, discard_rectangles.as_ptr() as _);
        ()
    }
}
