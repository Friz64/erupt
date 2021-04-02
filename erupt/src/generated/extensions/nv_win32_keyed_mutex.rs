#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION")]
pub const NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
pub const NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_win32_keyed_mutex");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html) · Structure"]
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoNV")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const crate::vk1_0::DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeout_milliseconds: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const crate::vk1_0::DeviceMemory,
    pub p_release_keys: *const u64,
}
impl Default for Win32KeyedMutexAcquireReleaseInfoNV {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
            p_next: std::ptr::null(),
            acquire_count: Default::default(),
            p_acquire_syncs: std::ptr::null(),
            p_acquire_keys: std::ptr::null(),
            p_acquire_timeout_milliseconds: std::ptr::null(),
            release_count: Default::default(),
            p_release_syncs: std::ptr::null(),
            p_release_keys: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for Win32KeyedMutexAcquireReleaseInfoNV {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Win32KeyedMutexAcquireReleaseInfoNV")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("acquire_count", &self.acquire_count)
            .field("p_acquire_syncs", &self.p_acquire_syncs)
            .field("p_acquire_keys", &self.p_acquire_keys)
            .field("p_acquire_timeout_milliseconds", &self.p_acquire_timeout_milliseconds)
            .field("release_count", &self.release_count)
            .field("p_release_syncs", &self.p_release_syncs)
            .field("p_release_keys", &self.p_release_keys)
            .finish()
    }
}
impl Win32KeyedMutexAcquireReleaseInfoNV {
    #[inline]
    pub fn into_builder<'a>(self) -> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
        Win32KeyedMutexAcquireReleaseInfoNVBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoNV.html) · Builder of [`Win32KeyedMutexAcquireReleaseInfoNV`]"]
#[repr(transparent)]
pub struct Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a>(Win32KeyedMutexAcquireReleaseInfoNV, std::marker::PhantomData<&'a ()>);
impl<'a> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    #[inline]
    pub fn new() -> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
        Win32KeyedMutexAcquireReleaseInfoNVBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn acquire_syncs(mut self, acquire_syncs: &'a [crate::vk1_0::DeviceMemory]) -> Self {
        self.0.p_acquire_syncs = acquire_syncs.as_ptr() as _;
        self.0.acquire_count = acquire_syncs.len() as _;
        self
    }
    #[inline]
    pub fn acquire_keys(mut self, acquire_keys: &'a [u64]) -> Self {
        self.0.p_acquire_keys = acquire_keys.as_ptr() as _;
        self.0.acquire_count = acquire_keys.len() as _;
        self
    }
    #[inline]
    pub fn acquire_timeout_milliseconds(mut self, acquire_timeout_milliseconds: &'a [u32]) -> Self {
        self.0.p_acquire_timeout_milliseconds = acquire_timeout_milliseconds.as_ptr() as _;
        self.0.acquire_count = acquire_timeout_milliseconds.len() as _;
        self
    }
    #[inline]
    pub fn release_syncs(mut self, release_syncs: &'a [crate::vk1_0::DeviceMemory]) -> Self {
        self.0.p_release_syncs = release_syncs.as_ptr() as _;
        self.0.release_count = release_syncs.len() as _;
        self
    }
    #[inline]
    pub fn release_keys(mut self, release_keys: &'a [u64]) -> Self {
        self.0.p_release_keys = release_keys.as_ptr() as _;
        self.0.release_count = release_keys.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Win32KeyedMutexAcquireReleaseInfoNV {
        self.0
    }
}
impl<'a> std::default::Default for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    fn default() -> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    type Target = Win32KeyedMutexAcquireReleaseInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl crate::Repr<Win32KeyedMutexAcquireReleaseInfoNV> for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'_> {}
