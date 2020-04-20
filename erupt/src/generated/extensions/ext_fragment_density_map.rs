# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_fragment_density_map.html)\n\n## Extends\n- [`AccessFlagBits`](../../vk1_0/struct.AccessFlagBits.html)\n- [`FormatFeatureFlagBits`](../../vk1_0/struct.FormatFeatureFlagBits.html)\n- [`ImageCreateFlagBits`](../../vk1_0/struct.ImageCreateFlagBits.html)\n- [`ImageLayout`](../../vk1_0/struct.ImageLayout.html)\n- [`ImageUsageFlagBits`](../../vk1_0/struct.ImageUsageFlagBits.html)\n- [`ImageViewCreateFlagBits`](../../vk1_0/struct.ImageViewCreateFlagBits.html)\n- [`PipelineStageFlagBits`](../../vk1_0/struct.PipelineStageFlagBits.html)\n- [`SamplerCreateFlagBits`](../../vk1_0/struct.SamplerCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_fragment_density_map");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub fragment_density_map: crate::vk1_0::Bool32,
    pub fragment_density_map_dynamic: crate::vk1_0::Bool32,
    pub fragment_density_map_non_subsampled_images: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentDensityMapFeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceFragmentDensityMapFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fragment_density_map", &(self.fragment_density_map != 0))
            .field(
                "fragment_density_map_dynamic",
                &(self.fragment_density_map_dynamic != 0),
            )
            .field(
                "fragment_density_map_non_subsampled_images",
                &(self.fragment_density_map_non_subsampled_images != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceFragmentDensityMapFeaturesEXT {
    fn default() -> PhysicalDeviceFragmentDensityMapFeaturesEXT {
        PhysicalDeviceFragmentDensityMapFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            fragment_density_map: Default::default(),
            fragment_density_map_dynamic: Default::default(),
            fragment_density_map_non_subsampled_images: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceFragmentDensityMapFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDeviceFragmentDensityMapFeaturesEXT>
    for crate::vk1_0::DeviceCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) · Builder of [`PhysicalDeviceFragmentDensityMapFeaturesEXT`](struct.PhysicalDeviceFragmentDensityMapFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a>(
    PhysicalDeviceFragmentDensityMapFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_density_map(mut self, fragment_density_map: bool) -> Self {
        self.0.fragment_density_map = fragment_density_map as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_density_map_dynamic(mut self, fragment_density_map_dynamic: bool) -> Self {
        self.0.fragment_density_map_dynamic = fragment_density_map_dynamic as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_density_map_non_subsampled_images(
        mut self,
        fragment_density_map_non_subsampled_images: bool,
    ) -> Self {
        self.0.fragment_density_map_non_subsampled_images =
            fragment_density_map_non_subsampled_images as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceFragmentDensityMapFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMapFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentDensityMapFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_fragment_density_texel_size: crate::vk1_0::Extent2D,
    pub max_fragment_density_texel_size: crate::vk1_0::Extent2D,
    pub fragment_density_invocations: crate::vk1_0::Bool32,
}
impl PhysicalDeviceFragmentDensityMapPropertiesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceFragmentDensityMapPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "min_fragment_density_texel_size",
                &self.min_fragment_density_texel_size,
            )
            .field(
                "max_fragment_density_texel_size",
                &self.max_fragment_density_texel_size,
            )
            .field(
                "fragment_density_invocations",
                &(self.fragment_density_invocations != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceFragmentDensityMapPropertiesEXT {
    fn default() -> PhysicalDeviceFragmentDensityMapPropertiesEXT {
        PhysicalDeviceFragmentDensityMapPropertiesEXT {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            min_fragment_density_texel_size: Default::default(),
            max_fragment_density_texel_size: Default::default(),
            fragment_density_invocations: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDeviceFragmentDensityMapPropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) · Builder of [`PhysicalDeviceFragmentDensityMapPropertiesEXT`](struct.PhysicalDeviceFragmentDensityMapPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a>(
    PhysicalDeviceFragmentDensityMapPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
        PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn min_fragment_density_texel_size(
        mut self,
        min_fragment_density_texel_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.min_fragment_density_texel_size = min_fragment_density_texel_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_fragment_density_texel_size(
        mut self,
        max_fragment_density_texel_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.max_fragment_density_texel_size = max_fragment_density_texel_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_density_invocations(mut self, fragment_density_invocations: bool) -> Self {
        self.0.fragment_density_invocations = fragment_density_invocations as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceFragmentDensityMapPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceFragmentDensityMapPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFragmentDensityMapPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fragment_density_map_attachment: crate::vk1_0::AttachmentReference,
}
impl RenderPassFragmentDensityMapCreateInfoEXT {
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
    pub fn builder<'a>(self) -> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
        RenderPassFragmentDensityMapCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RenderPassFragmentDensityMapCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderPassFragmentDensityMapCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "fragment_density_map_attachment",
                &self.fragment_density_map_attachment,
            )
            .finish()
    }
}
impl Default for RenderPassFragmentDensityMapCreateInfoEXT {
    fn default() -> RenderPassFragmentDensityMapCreateInfoEXT {
        RenderPassFragmentDensityMapCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            fragment_density_map_attachment: Default::default(),
        }
    }
}
impl crate::ExtendableBy<RenderPassFragmentDensityMapCreateInfoEXT>
    for crate::vk1_0::RenderPassCreateInfo
{
}
impl crate::ExtendableBy<RenderPassFragmentDensityMapCreateInfoEXT>
    for crate::vk1_2::RenderPassCreateInfo2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) · Builder of [`RenderPassFragmentDensityMapCreateInfoEXT`](struct.RenderPassFragmentDensityMapCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a>(
    RenderPassFragmentDensityMapCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
        RenderPassFragmentDensityMapCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fragment_density_map_attachment(
        mut self,
        fragment_density_map_attachment: crate::vk1_0::AttachmentReference,
    ) -> Self {
        self.0.fragment_density_map_attachment = fragment_density_map_attachment;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RenderPassFragmentDensityMapCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    type Target = RenderPassFragmentDensityMapCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
