# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_external_fence_fd.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_fence_fd");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceFdKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_import_fence_fd_info: *const crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceFdKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_get_fd_info: *const crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrExternalFenceFdDeviceLoaderExt`](trait.KhrExternalFenceFdDeviceLoaderExt.html)"]
pub struct KhrExternalFenceFdDeviceCommands {
    pub import_fence_fd_khr: PFN_vkImportFenceFdKHR,
    pub get_fence_fd_khr: PFN_vkGetFenceFdKHR,
}
impl KhrExternalFenceFdDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrExternalFenceFdDeviceCommands> {
        unsafe {
            Some(KhrExternalFenceFdDeviceCommands {
                import_fence_fd_khr: std::mem::transmute(loader.symbol("vkImportFenceFdKHR")?),
                get_fence_fd_khr: std::mem::transmute(loader.symbol("vkGetFenceFdKHR")?),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrExternalFenceFdDeviceCommands`](struct.KhrExternalFenceFdDeviceCommands.html)"]
pub trait KhrExternalFenceFdDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceFdKHR.html) · Device Command"]
    unsafe fn import_fence_fd_khr(
        &self,
        import_fence_fd_info: &crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceFdKHR.html) · Device Command"]
    unsafe fn get_fence_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int>;
}
impl KhrExternalFenceFdDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkImportFenceFdKHR.html) · Device Command"]
    unsafe fn import_fence_fd_khr(
        &self,
        import_fence_fd_info: &crate::extensions::khr_external_fence_fd::ImportFenceFdInfoKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_external_fence_fd
            .as_ref()
            .expect("`khr_external_fence_fd` not loaded")
            .import_fence_fd_khr;
        let _val = function(self.handle, import_fence_fd_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceFdKHR.html) · Device Command"]
    unsafe fn get_fence_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_fence_fd::FenceGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int> {
        let function = self
            .khr_external_fence_fd
            .as_ref()
            .expect("`khr_external_fence_fd` not loaded")
            .get_fence_fd_khr;
        let mut fd = fd.unwrap_or_else(|| Default::default());
        let _val = function(self.handle, get_fd_info, &mut fd);
        crate::utils::VulkanResult::new(_val, fd)
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportFenceFdInfoKHR.html) · Structure"]
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
impl ImportFenceFdInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> ImportFenceFdInfoKHRBuilder<'a> {
        ImportFenceFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ImportFenceFdInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ImportFenceFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fence", &self.fence)
            .field("flags", &self.flags)
            .field("handle_type", &self.handle_type)
            .field("fd", &self.fd)
            .finish()
    }
}
impl Default for ImportFenceFdInfoKHR {
    fn default() -> ImportFenceFdInfoKHR {
        ImportFenceFdInfoKHR {
            s_type: crate::vk1_0::StructureType::IMPORT_FENCE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ImportFenceFdInfoKHR`](struct.ImportFenceFdInfoKHR.html)"]
#[repr(transparent)]
pub struct ImportFenceFdInfoKHRBuilder<'a>(ImportFenceFdInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImportFenceFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportFenceFdInfoKHRBuilder<'a> {
        ImportFenceFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_1::FenceImportFlags) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fd(mut self, fd: std::os::raw::c_int) -> Self {
        self.0.fd = fd;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ImportFenceFdInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for ImportFenceFdInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FenceGetFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub fence: crate::vk1_0::Fence,
    pub handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
}
impl FenceGetFdInfoKHR {
    #[inline]
    pub fn builder<'a>(self) -> FenceGetFdInfoKHRBuilder<'a> {
        FenceGetFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for FenceGetFdInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("FenceGetFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("fence", &self.fence)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl Default for FenceGetFdInfoKHR {
    fn default() -> FenceGetFdInfoKHR {
        FenceGetFdInfoKHR {
            s_type: crate::vk1_0::StructureType::FENCE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`FenceGetFdInfoKHR`](struct.FenceGetFdInfoKHR.html)"]
#[repr(transparent)]
pub struct FenceGetFdInfoKHRBuilder<'a>(FenceGetFdInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> FenceGetFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> FenceGetFdInfoKHRBuilder<'a> {
        FenceGetFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fence(mut self, fence: crate::vk1_0::Fence) -> Self {
        self.0.fence = fence;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalFenceHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> FenceGetFdInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for FenceGetFdInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
