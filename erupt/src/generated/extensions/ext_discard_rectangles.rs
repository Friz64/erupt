# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_discard_rectangles.html)\n\n## Extends\n- [`DynamicState`](../../vk1_0/struct.DynamicState.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DISCARD_RECTANGLES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_discard_rectangles");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    p_discard_rectangles: *const crate::vk1_0::Rect2D,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtDiscardRectanglesDeviceLoaderExt`](trait.ExtDiscardRectanglesDeviceLoaderExt.html)"]
pub struct ExtDiscardRectanglesDeviceCommands {
    pub cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
}
impl ExtDiscardRectanglesDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtDiscardRectanglesDeviceCommands> {
        unsafe {
            Some(ExtDiscardRectanglesDeviceCommands {
                cmd_set_discard_rectangle_ext: std::mem::transmute(
                    loader.symbol("vkCmdSetDiscardRectangleEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtDiscardRectanglesDeviceCommands`](struct.ExtDiscardRectanglesDeviceCommands.html)"]
pub trait ExtDiscardRectanglesDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) · Device Command"]
    unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangles: &[crate::vk1_0::Rect2DBuilder],
    ) -> ();
}
impl ExtDiscardRectanglesDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDiscardRectangleEXT.html) · Device Command"]
    unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangles: &[crate::vk1_0::Rect2DBuilder],
    ) -> () {
        let function = self
            .ext_discard_rectangles
            .as_ref()
            .expect("`ext_discard_rectangles` not loaded")
            .cmd_set_discard_rectangle_ext;
        let discard_rectangle_count = discard_rectangles.len() as _;
        let _val = function(
            command_buffer,
            first_discard_rectangle,
            discard_rectangle_count,
            discard_rectangles.as_ptr() as _,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceDiscardRectanglePropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_discard_rectangles: u32,
}
impl PhysicalDeviceDiscardRectanglePropertiesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceDiscardRectanglePropertiesEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
        PhysicalDeviceDiscardRectanglePropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceDiscardRectanglePropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_discard_rectangles", &self.max_discard_rectangles)
            .finish()
    }
}
impl Default for PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn default() -> PhysicalDeviceDiscardRectanglePropertiesEXT {
        PhysicalDeviceDiscardRectanglePropertiesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_discard_rectangles: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceDiscardRectanglePropertiesEXT::extend`](struct.PhysicalDeviceDiscardRectanglePropertiesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceDiscardRectanglePropertiesEXT {}
impl ExtendableByPhysicalDeviceDiscardRectanglePropertiesEXT
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceDiscardRectanglePropertiesEXT`](struct.PhysicalDeviceDiscardRectanglePropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a>(
    PhysicalDeviceDiscardRectanglePropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
        PhysicalDeviceDiscardRectanglePropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_discard_rectangles(mut self, max_discard_rectangles: u32) -> Self {
        self.0.max_discard_rectangles = max_discard_rectangles;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceDiscardRectanglePropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceDiscardRectanglePropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags:
        crate::extensions::ext_discard_rectangles::PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: crate::extensions::ext_discard_rectangles::DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const crate::vk1_0::Rect2D,
}
impl PipelineDiscardRectangleStateCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineDiscardRectangleStateCreateInfoEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
        PipelineDiscardRectangleStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineDiscardRectangleStateCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineDiscardRectangleStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("discard_rectangle_mode", &self.discard_rectangle_mode)
            .field("discard_rectangle_count", &self.discard_rectangle_count)
            .field("p_discard_rectangles", &self.p_discard_rectangles)
            .finish()
    }
}
impl Default for PipelineDiscardRectangleStateCreateInfoEXT {
    fn default() -> PipelineDiscardRectangleStateCreateInfoEXT {
        PipelineDiscardRectangleStateCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            discard_rectangle_mode: Default::default(),
            discard_rectangle_count: Default::default(),
            p_discard_rectangles: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`PipelineDiscardRectangleStateCreateInfoEXT::extend`](struct.PipelineDiscardRectangleStateCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByPipelineDiscardRectangleStateCreateInfoEXT {}
impl ExtendableByPipelineDiscardRectangleStateCreateInfoEXT
    for crate::vk1_0::GraphicsPipelineCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineDiscardRectangleStateCreateInfoEXT`](struct.PipelineDiscardRectangleStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a>(
    PipelineDiscardRectangleStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
        PipelineDiscardRectangleStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: ext_discard_rectangles :: PipelineDiscardRectangleStateCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn discard_rectangle_mode(
        mut self,
        discard_rectangle_mode: crate::extensions::ext_discard_rectangles::DiscardRectangleModeEXT,
    ) -> Self {
        self.0.discard_rectangle_mode = discard_rectangle_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn discard_rectangles(
        mut self,
        discard_rectangles: &'a [crate::vk1_0::Rect2DBuilder],
    ) -> Self {
        self.0.discard_rectangle_count = discard_rectangles.len() as _;
        self.0.p_discard_rectangles = discard_rectangles.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineDiscardRectangleStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineDiscardRectangleStateCreateFlagsEXT`](struct.PipelineDiscardRectangleStateCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDiscardRectangleStateCreateFlagsEXT.html) · Flags of [`PipelineDiscardRectangleStateCreateFlagBitsEXT`](struct.PipelineDiscardRectangleStateCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineDiscardRectangleStateCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDiscardRectangleModeEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DiscardRectangleModeEXT(pub i32);
#[doc = "[Part of `extensions::ext_discard_rectangles`](../../extensions/ext_discard_rectangles/index.html)"]
impl DiscardRectangleModeEXT {
    pub const INCLUSIVE_EXT: Self = Self(0);
    pub const EXCLUSIVE_EXT: Self = Self(1);
}
impl std::fmt::Debug for DiscardRectangleModeEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::INCLUSIVE_EXT => "INCLUSIVE_EXT",
            &Self::EXCLUSIVE_EXT => "EXCLUSIVE_EXT",
            _ => "Unknown enum variant",
        })
    }
}
