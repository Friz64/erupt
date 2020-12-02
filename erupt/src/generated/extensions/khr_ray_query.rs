#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_RAY_QUERY_SPEC_VERSION")]
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_RAY_QUERY_EXTENSION_NAME")]
pub const KHR_RAY_QUERY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_ray_query");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceRayQueryFeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceRayQueryFeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub ray_query: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceRayQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            ray_query: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceRayQueryFeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceRayQueryFeaturesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("ray_query", &(self.ray_query != 0))
            .finish()
    }
}
impl PhysicalDeviceRayQueryFeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
        PhysicalDeviceRayQueryFeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html) · Builder of [`PhysicalDeviceRayQueryFeaturesKHR`](struct.PhysicalDeviceRayQueryFeaturesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceRayQueryFeaturesKHRBuilder<'a>(
    PhysicalDeviceRayQueryFeaturesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
        PhysicalDeviceRayQueryFeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn ray_query(mut self, ray_query: bool) -> Self {
        self.0.ray_query = ray_query as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceRayQueryFeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceRayQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceRayQueryFeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
