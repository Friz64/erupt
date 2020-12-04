#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION")]
pub const KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
pub const KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_win32_keyed_mutex");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html) · Structure"]
#[doc(alias = "VkWin32KeyedMutexAcquireReleaseInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const crate::vk1_0::DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeouts: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const crate::vk1_0::DeviceMemory,
    pub p_release_keys: *const u64,
}
impl Default for Win32KeyedMutexAcquireReleaseInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
            p_next: std::ptr::null(),
            acquire_count: Default::default(),
            p_acquire_syncs: std::ptr::null(),
            p_acquire_keys: std::ptr::null(),
            p_acquire_timeouts: std::ptr::null(),
            release_count: Default::default(),
            p_release_syncs: std::ptr::null(),
            p_release_keys: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for Win32KeyedMutexAcquireReleaseInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Win32KeyedMutexAcquireReleaseInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("acquire_count", &self.acquire_count)
            .field("p_acquire_syncs", &self.p_acquire_syncs)
            .field("p_acquire_keys", &self.p_acquire_keys)
            .field("p_acquire_timeouts", &self.p_acquire_timeouts)
            .field("release_count", &self.release_count)
            .field("p_release_syncs", &self.p_release_syncs)
            .field("p_release_keys", &self.p_release_keys)
            .finish()
    }
}
impl Win32KeyedMutexAcquireReleaseInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
        Win32KeyedMutexAcquireReleaseInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html) · Builder of [`Win32KeyedMutexAcquireReleaseInfoKHR`]"]
#[repr(transparent)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a>(
    Win32KeyedMutexAcquireReleaseInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
        Win32KeyedMutexAcquireReleaseInfoKHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn acquire_timeouts(mut self, acquire_timeouts: &'a [u32]) -> Self {
        self.0.p_acquire_timeouts = acquire_timeouts.as_ptr() as _;
        self.0.acquire_count = acquire_timeouts.len() as _;
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
    pub fn build(self) -> Win32KeyedMutexAcquireReleaseInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    fn default() -> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    type Target = Win32KeyedMutexAcquireReleaseInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
