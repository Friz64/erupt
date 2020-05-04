# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_custom_border_color.html)\n\n## Extends\n- [`BorderColor`](../../vk1_0/struct.BorderColor.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_custom_border_color");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub custom_border_color: crate::vk1_0::ClearColorValue,
    pub format: crate::vk1_0::Format,
}
impl SamplerCustomBorderColorCreateInfoEXT {
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
    pub fn builder<'a>(self) -> SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
        SamplerCustomBorderColorCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SamplerCustomBorderColorCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SamplerCustomBorderColorCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("custom_border_color", &self.custom_border_color)
            .field("format", &self.format)
            .finish()
    }
}
impl Default for SamplerCustomBorderColorCreateInfoEXT {
    fn default() -> SamplerCustomBorderColorCreateInfoEXT {
        SamplerCustomBorderColorCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            custom_border_color: Default::default(),
            format: Default::default(),
        }
    }
}
impl crate::ExtendableBy<SamplerCustomBorderColorCreateInfoEXT>
    for crate::vk1_0::SamplerCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html) · Builder of [`SamplerCustomBorderColorCreateInfoEXT`](struct.SamplerCustomBorderColorCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct SamplerCustomBorderColorCreateInfoEXTBuilder<'a>(
    SamplerCustomBorderColorCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
        SamplerCustomBorderColorCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn custom_border_color(
        mut self,
        custom_border_color: crate::vk1_0::ClearColorValue,
    ) -> Self {
        self.0.custom_border_color = custom_border_color;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SamplerCustomBorderColorCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    type Target = SamplerCustomBorderColorCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerCustomBorderColorCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_custom_border_color_samplers: u32,
}
impl PhysicalDeviceCustomBorderColorPropertiesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceCustomBorderColorPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_custom_border_color_samplers",
                &self.max_custom_border_color_samplers,
            )
            .finish()
    }
}
impl Default for PhysicalDeviceCustomBorderColorPropertiesEXT {
    fn default() -> PhysicalDeviceCustomBorderColorPropertiesEXT {
        PhysicalDeviceCustomBorderColorPropertiesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_custom_border_color_samplers: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceCustomBorderColorPropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html) · Builder of [`PhysicalDeviceCustomBorderColorPropertiesEXT`](struct.PhysicalDeviceCustomBorderColorPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a>(
    PhysicalDeviceCustomBorderColorPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_custom_border_color_samplers(
        mut self,
        max_custom_border_color_samplers: u32,
    ) -> Self {
        self.0.max_custom_border_color_samplers = max_custom_border_color_samplers;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceCustomBorderColorPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceCustomBorderColorPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCustomBorderColorPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub custom_border_colors: crate::vk1_0::Bool32,
    pub custom_border_color_without_format: crate::vk1_0::Bool32,
}
impl PhysicalDeviceCustomBorderColorFeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceCustomBorderColorFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("custom_border_colors", &(self.custom_border_colors != 0))
            .field(
                "custom_border_color_without_format",
                &(self.custom_border_color_without_format != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceCustomBorderColorFeaturesEXT {
    fn default() -> PhysicalDeviceCustomBorderColorFeaturesEXT {
        PhysicalDeviceCustomBorderColorFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            custom_border_colors: Default::default(),
            custom_border_color_without_format: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceCustomBorderColorFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceCustomBorderColorFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) · Builder of [`PhysicalDeviceCustomBorderColorFeaturesEXT`](struct.PhysicalDeviceCustomBorderColorFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a>(
    PhysicalDeviceCustomBorderColorFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
        PhysicalDeviceCustomBorderColorFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn custom_border_colors(mut self, custom_border_colors: bool) -> Self {
        self.0.custom_border_colors = custom_border_colors as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn custom_border_color_without_format(
        mut self,
        custom_border_color_without_format: bool,
    ) -> Self {
        self.0.custom_border_color_without_format = custom_border_color_without_format as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceCustomBorderColorFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceCustomBorderColorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceCustomBorderColorFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
