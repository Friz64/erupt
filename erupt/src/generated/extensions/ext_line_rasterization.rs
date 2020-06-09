# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_line_rasterization.html)\n\n## Extends\n- [`DynamicState`](../../vk1_0/struct.DynamicState.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_LINE_RASTERIZATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_line_rasterization");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineStippleEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtLineRasterizationDeviceLoaderExt`](trait.ExtLineRasterizationDeviceLoaderExt.html)"]
pub struct ExtLineRasterizationDeviceCommands {
    pub cmd_set_line_stipple_ext: Option<PFN_vkCmdSetLineStippleEXT>,
}
impl ExtLineRasterizationDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtLineRasterizationDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtLineRasterizationDeviceCommands {
                cmd_set_line_stipple_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCmdSetLineStippleEXT");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
#[inline]
fn device_commands(loader: &crate::DeviceLoader) -> &ExtLineRasterizationDeviceCommands {
    loader
        .ext_line_rasterization
        .as_ref()
        .expect("`ext_line_rasterization` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtLineRasterizationDeviceCommands`](struct.ExtLineRasterizationDeviceCommands.html)"]
pub trait ExtLineRasterizationDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineStippleEXT.html) · Device Command"]
    unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) -> ();
}
impl ExtLineRasterizationDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineStippleEXT.html) · Device Command"]
    unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) -> () {
        let function = device_commands(self)
            .cmd_set_line_stipple_ext
            .as_ref()
            .expect("`cmd_set_line_stipple_ext` not available");
        let _val = function(command_buffer, line_stipple_factor, line_stipple_pattern);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub rectangular_lines: crate::vk1_0::Bool32,
    pub bresenham_lines: crate::vk1_0::Bool32,
    pub smooth_lines: crate::vk1_0::Bool32,
    pub stippled_rectangular_lines: crate::vk1_0::Bool32,
    pub stippled_bresenham_lines: crate::vk1_0::Bool32,
    pub stippled_smooth_lines: crate::vk1_0::Bool32,
}
impl PhysicalDeviceLineRasterizationFeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceLineRasterizationFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceLineRasterizationFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("rectangular_lines", &(self.rectangular_lines != 0))
            .field("bresenham_lines", &(self.bresenham_lines != 0))
            .field("smooth_lines", &(self.smooth_lines != 0))
            .field(
                "stippled_rectangular_lines",
                &(self.stippled_rectangular_lines != 0),
            )
            .field(
                "stippled_bresenham_lines",
                &(self.stippled_bresenham_lines != 0),
            )
            .field("stippled_smooth_lines", &(self.stippled_smooth_lines != 0))
            .finish()
    }
}
impl Default for PhysicalDeviceLineRasterizationFeaturesEXT {
    fn default() -> PhysicalDeviceLineRasterizationFeaturesEXT {
        PhysicalDeviceLineRasterizationFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            rectangular_lines: Default::default(),
            bresenham_lines: Default::default(),
            smooth_lines: Default::default(),
            stippled_rectangular_lines: Default::default(),
            stippled_bresenham_lines: Default::default(),
            stippled_smooth_lines: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceLineRasterizationFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceLineRasterizationFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html) · Builder of [`PhysicalDeviceLineRasterizationFeaturesEXT`](struct.PhysicalDeviceLineRasterizationFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a>(
    PhysicalDeviceLineRasterizationFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn rectangular_lines(mut self, rectangular_lines: bool) -> Self {
        self.0.rectangular_lines = rectangular_lines as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn bresenham_lines(mut self, bresenham_lines: bool) -> Self {
        self.0.bresenham_lines = bresenham_lines as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn smooth_lines(mut self, smooth_lines: bool) -> Self {
        self.0.smooth_lines = smooth_lines as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
        self.0.stippled_rectangular_lines = stippled_rectangular_lines as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
        self.0.stippled_bresenham_lines = stippled_bresenham_lines as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
        self.0.stippled_smooth_lines = stippled_smooth_lines as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceLineRasterizationFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceLineRasterizationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub line_sub_pixel_precision_bits: u32,
}
impl PhysicalDeviceLineRasterizationPropertiesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceLineRasterizationPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceLineRasterizationPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "line_sub_pixel_precision_bits",
                &self.line_sub_pixel_precision_bits,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceLineRasterizationPropertiesEXT {
    fn default() -> PhysicalDeviceLineRasterizationPropertiesEXT {
        PhysicalDeviceLineRasterizationPropertiesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            line_sub_pixel_precision_bits: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceLineRasterizationPropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html) · Builder of [`PhysicalDeviceLineRasterizationPropertiesEXT`](struct.PhysicalDeviceLineRasterizationPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a>(
    PhysicalDeviceLineRasterizationPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
        PhysicalDeviceLineRasterizationPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn line_sub_pixel_precision_bits(mut self, line_sub_pixel_precision_bits: u32) -> Self {
        self.0.line_sub_pixel_precision_bits = line_sub_pixel_precision_bits;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceLineRasterizationPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceLineRasterizationPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceLineRasterizationPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationLineStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub line_rasterization_mode:
        crate::extensions::ext_line_rasterization::LineRasterizationModeEXT,
    pub stippled_line_enable: crate::vk1_0::Bool32,
    pub line_stipple_factor: u32,
    pub line_stipple_pattern: u16,
}
impl PipelineRasterizationLineStateCreateInfoEXT {
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
    pub fn builder<'a>(self) -> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationLineStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineRasterizationLineStateCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineRasterizationLineStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("line_rasterization_mode", &self.line_rasterization_mode)
            .field("stippled_line_enable", &(self.stippled_line_enable != 0))
            .field("line_stipple_factor", &self.line_stipple_factor)
            .field("line_stipple_pattern", &self.line_stipple_pattern)
            .finish()
    }
}
impl Default for PipelineRasterizationLineStateCreateInfoEXT {
    fn default() -> PipelineRasterizationLineStateCreateInfoEXT {
        PipelineRasterizationLineStateCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            line_rasterization_mode: Default::default(),
            stippled_line_enable: Default::default(),
            line_stipple_factor: Default::default(),
            line_stipple_pattern: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PipelineRasterizationLineStateCreateInfoEXT>
    for crate::vk1_0::PipelineRasterizationStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html) · Builder of [`PipelineRasterizationLineStateCreateInfoEXT`](struct.PipelineRasterizationLineStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineRasterizationLineStateCreateInfoEXTBuilder<'a>(
    PipelineRasterizationLineStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationLineStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn line_rasterization_mode(
        mut self,
        line_rasterization_mode : crate :: extensions :: ext_line_rasterization :: LineRasterizationModeEXT,
    ) -> Self {
        self.0.line_rasterization_mode = line_rasterization_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn stippled_line_enable(mut self, stippled_line_enable: bool) -> Self {
        self.0.stippled_line_enable = stippled_line_enable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn line_stipple_factor(mut self, line_stipple_factor: u32) -> Self {
        self.0.line_stipple_factor = line_stipple_factor;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn line_stipple_pattern(mut self, line_stipple_pattern: u16) -> Self {
        self.0.line_stipple_pattern = line_stipple_pattern;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineRasterizationLineStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineRasterizationLineStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLineRasterizationModeEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct LineRasterizationModeEXT(pub i32);
#[doc = "[Part of `extensions::ext_line_rasterization`](../../extensions/ext_line_rasterization/index.html)"]
impl LineRasterizationModeEXT {
    pub const DEFAULT_EXT: Self = Self(0);
    pub const RECTANGULAR_EXT: Self = Self(1);
    pub const BRESENHAM_EXT: Self = Self(2);
    pub const RECTANGULAR_SMOOTH_EXT: Self = Self(3);
}
impl std::fmt::Debug for LineRasterizationModeEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DEFAULT_EXT => "DEFAULT_EXT",
            &Self::RECTANGULAR_EXT => "RECTANGULAR_EXT",
            &Self::BRESENHAM_EXT => "BRESENHAM_EXT",
            &Self::RECTANGULAR_SMOOTH_EXT => "RECTANGULAR_SMOOTH_EXT",
            _ => "(unknown)",
        })
    }
}
