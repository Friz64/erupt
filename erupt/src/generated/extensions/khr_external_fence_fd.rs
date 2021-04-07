#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_external_fence_fd");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_FENCE_FD_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetFenceFdKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_IMPORT_FENCE_FD_KHR: *const std::os::raw::c_char = crate::cstr!("vkImportFenceFdKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceFdKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_get_fd_info: *const crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR, p_fd: *mut std::os::raw::c_int) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceFdKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_import_fence_fd_info: *const crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceFdInfoKHR.html) · Structure"]
#[doc(alias = "VkImportFenceFdInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportFenceFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fence: crate::vk1_0::Fence,
    pub flags: crate::vk1_1::FenceImportFlags,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    pub fd: std::os::raw::c_int,
}
impl Default for ImportFenceFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMPORT_FENCE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImportFenceFdInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportFenceFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fence", &self.fence)
            .field("flags", &self.flags)
            .field("handle_type", &self.handle_type)
            .field("fd", &self.fd)
            .finish()
    }
}
impl ImportFenceFdInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportFenceFdInfoKHRBuilder<'a> {
        ImportFenceFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceFdInfoKHR.html) · Builder of [`ImportFenceFdInfoKHR`]"]
#[repr(transparent)]
pub struct ImportFenceFdInfoKHRBuilder<'a>(ImportFenceFdInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImportFenceFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportFenceFdInfoKHRBuilder<'a> {
        ImportFenceFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::FenceImportFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    pub fn fd(mut self, fd: std::os::raw::c_int) -> Self {
        self.0.fd = fd as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportFenceFdInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ImportFenceFdInfoKHRBuilder<'a> {
    fn default() -> ImportFenceFdInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportFenceFdInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportFenceFdInfoKHRBuilder<'a> {
    type Target = ImportFenceFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportFenceFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetFdInfoKHR.html) · Structure"]
#[doc(alias = "VkFenceGetFdInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FenceGetFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fence: crate::vk1_0::Fence,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
}
impl Default for FenceGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::FENCE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for FenceGetFdInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FenceGetFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fence", &self.fence)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl FenceGetFdInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> FenceGetFdInfoKHRBuilder<'a> {
        FenceGetFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceGetFdInfoKHR.html) · Builder of [`FenceGetFdInfoKHR`]"]
#[repr(transparent)]
pub struct FenceGetFdInfoKHRBuilder<'a>(FenceGetFdInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> FenceGetFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> FenceGetFdInfoKHRBuilder<'a> {
        FenceGetFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence as _;
        self
    }
    #[inline]
    pub fn handle_type(mut self, handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FenceGetFdInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for FenceGetFdInfoKHRBuilder<'a> {
    fn default() -> FenceGetFdInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FenceGetFdInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FenceGetFdInfoKHRBuilder<'a> {
    type Target = FenceGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FenceGetFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_external_fence_fd`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceFdKHR.html) · Function"]
    #[doc(alias = "vkGetFenceFdKHR")]
    pub unsafe fn get_fence_fd_khr(&self, get_fd_info: &crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR) -> crate::utils::VulkanResult<std::os::raw::c_int> {
        let _function = self.get_fence_fd_khr.expect("`get_fence_fd_khr` is not loaded");
        let mut fd = Default::default();
        let _return = _function(self.handle, get_fd_info as _, &mut fd);
        crate::utils::VulkanResult::new(_return, fd)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceFdKHR.html) · Function"]
    #[doc(alias = "vkImportFenceFdKHR")]
    pub unsafe fn import_fence_fd_khr(&self, import_fence_fd_info: &crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.import_fence_fd_khr.expect("`import_fence_fd_khr` is not loaded");
        let _return = _function(self.handle, import_fence_fd_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
