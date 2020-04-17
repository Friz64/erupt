# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_NVX_multiview_per_view_attributes.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)\n- [`SubpassDescriptionFlagBits`](../../vk1_0/struct.SubpassDescriptionFlagBits.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NVX_multiview_per_view_attributes");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub per_view_position_all_components: crate::vk1_0::Bool32,
}
impl PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
        PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "per_view_position_all_components",
                &(self.per_view_position_all_components != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn default() -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
        PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX , p_next : std :: ptr :: null_mut ( ) , per_view_position_all_components : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX::extend`](struct.PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {}
impl ExtendableByPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`](struct.PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a>(
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
        PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn per_view_position_all_components(
        mut self,
        per_view_position_all_components: bool,
    ) -> Self {
        self.0.per_view_position_all_components = per_view_position_all_components as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
    type Target = PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
