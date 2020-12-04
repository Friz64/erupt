#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NVX_multiview_per_view_attributes");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub per_view_position_all_components: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn default() -> Self {
        Self { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX , p_next : std :: ptr :: null_mut () , per_view_position_all_components : Default :: default () }
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "per_view_position_all_components",
                &(self.per_view_position_all_components != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    #[inline]
    pub fn into_builder<'a>(
        self,
    ) -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
        PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html) · Builder of [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`]"]
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
    #[inline]
    pub fn per_view_position_all_components(
        mut self,
        per_view_position_all_components: bool,
    ) -> Self {
        self.0.per_view_position_all_components = per_view_position_all_components as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
        self.0
    }
}
impl<'a> std::default::Default
    for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a>
{
    fn default() -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
