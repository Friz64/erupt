# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_pipeline_library.html)\n\n## Extends\n- [`PipelineCreateFlagBits`](../../vk1_0/struct.PipelineCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PIPELINE_LIBRARY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_pipeline_library");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub library_count: u32,
    pub p_libraries: *const crate::vk1_0::Pipeline,
}
impl PipelineLibraryCreateInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> PipelineLibraryCreateInfoKHRBuilder<'a> {
        PipelineLibraryCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineLibraryCreateInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineLibraryCreateInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("library_count", &self.library_count)
            .field("p_libraries", &self.p_libraries)
            .finish()
    }
}
impl Default for PipelineLibraryCreateInfoKHR {
    fn default() -> PipelineLibraryCreateInfoKHR {
        PipelineLibraryCreateInfoKHR {
            s_type: crate::vk1_0::StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR,
            p_next: std::ptr::null(),
            library_count: Default::default(),
            p_libraries: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html) · Builder of [`PipelineLibraryCreateInfoKHR`](struct.PipelineLibraryCreateInfoKHR.html)"]
#[repr(transparent)]
pub struct PipelineLibraryCreateInfoKHRBuilder<'a>(
    PipelineLibraryCreateInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineLibraryCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineLibraryCreateInfoKHRBuilder<'a> {
        PipelineLibraryCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn libraries(mut self, libraries: &'a [crate::vk1_0::Pipeline]) -> Self {
        self.0.library_count = libraries.len() as _;
        self.0.p_libraries = libraries.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineLibraryCreateInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineLibraryCreateInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineLibraryCreateInfoKHRBuilder<'a> {
    type Target = PipelineLibraryCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineLibraryCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
