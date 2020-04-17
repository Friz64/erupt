# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_win32_keyed_mutex.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_win32_keyed_mutex");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWin32KeyedMutexAcquireReleaseInfoKHR.html) · Structure"]
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
impl Win32KeyedMutexAcquireReleaseInfoKHR {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByWin32KeyedMutexAcquireReleaseInfoKHR,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
        Win32KeyedMutexAcquireReleaseInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for Win32KeyedMutexAcquireReleaseInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Win32KeyedMutexAcquireReleaseInfoKHR")
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
impl Default for Win32KeyedMutexAcquireReleaseInfoKHR {
    fn default() -> Win32KeyedMutexAcquireReleaseInfoKHR {
        Win32KeyedMutexAcquireReleaseInfoKHR {
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
#[doc = "Used by [`Win32KeyedMutexAcquireReleaseInfoKHR::extend`](struct.Win32KeyedMutexAcquireReleaseInfoKHR.html#method.extend)"]
pub trait ExtendableByWin32KeyedMutexAcquireReleaseInfoKHR {}
impl ExtendableByWin32KeyedMutexAcquireReleaseInfoKHR for crate::vk1_0::SubmitInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`Win32KeyedMutexAcquireReleaseInfoKHR`](struct.Win32KeyedMutexAcquireReleaseInfoKHR.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn acquire_syncs(mut self, acquire_syncs: &'a [crate::vk1_0::DeviceMemory]) -> Self {
        self.0.acquire_count = acquire_syncs.len() as _;
        self.0.p_acquire_syncs = acquire_syncs.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acquire_keys(mut self, acquire_keys: &'a [u64]) -> Self {
        self.0.acquire_count = acquire_keys.len() as _;
        self.0.p_acquire_keys = acquire_keys.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn acquire_timeouts(mut self, acquire_timeouts: &'a [u32]) -> Self {
        self.0.acquire_count = acquire_timeouts.len() as _;
        self.0.p_acquire_timeouts = acquire_timeouts.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn release_syncs(mut self, release_syncs: &'a [crate::vk1_0::DeviceMemory]) -> Self {
        self.0.release_count = release_syncs.len() as _;
        self.0.p_release_syncs = release_syncs.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn release_keys(mut self, release_keys: &'a [u64]) -> Self {
        self.0.release_count = release_keys.len() as _;
        self.0.p_release_keys = release_keys.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> Win32KeyedMutexAcquireReleaseInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
