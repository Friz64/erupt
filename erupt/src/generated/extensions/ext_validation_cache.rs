#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VALIDATION_CACHE_SPEC_VERSION")]
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VALIDATION_CACHE_EXTENSION_NAME")]
pub const EXT_VALIDATION_CACHE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_validation_cache");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_VALIDATION_CACHE_EXT: *const std::os::raw::c_char = crate::cstr!("vkCreateValidationCacheEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_VALIDATION_CACHE_EXT: *const std::os::raw::c_char = crate::cstr!("vkDestroyValidationCacheEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_VALIDATION_CACHE_DATA_EXT: *const std::os::raw::c_char = crate::cstr!("vkGetValidationCacheDataEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_MERGE_VALIDATION_CACHES_EXT: *const std::os::raw::c_char = crate::cstr!("vkMergeValidationCachesEXT");
crate::non_dispatchable_handle!(ValidationCacheEXT, VALIDATION_CACHE_EXT, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheEXT.html) · Non-dispatchable Handle", "VkValidationCacheEXT");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheCreateFlagsEXT.html) · Bitmask of [`ValidationCacheCreateFlagBitsEXT`]"] # [doc (alias = "VkValidationCacheCreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct ValidationCacheCreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`ValidationCacheCreateFlagsEXT`]"]
#[doc(alias = "VkValidationCacheCreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ValidationCacheCreateFlagBitsEXT(pub u32);
impl ValidationCacheCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ValidationCacheCreateFlagsEXT {
        ValidationCacheCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ValidationCacheCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_validation_cache`]"]
impl crate::vk1_0::StructureType {
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
}
#[doc = "Provided by [`crate::extensions::ext_validation_cache`]"]
impl crate::vk1_0::ObjectType {
    pub const VALIDATION_CACHE_EXT: Self = Self(1000160000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheHeaderVersionEXT.html) · Enum"]
#[doc(alias = "VkValidationCacheHeaderVersionEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);
impl std::fmt::Debug for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ONE_EXT => "ONE_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_validation_cache`]"]
impl crate::extensions::ext_validation_cache::ValidationCacheHeaderVersionEXT {
    pub const ONE_EXT: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateValidationCacheEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_validation_cache: *mut crate::extensions::ext_validation_cache::ValidationCacheEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyValidationCacheEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetValidationCacheDataEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT, p_data_size: *mut usize, p_data: *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergeValidationCachesEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(device: crate::vk1_0::Device, dst_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT, src_cache_count: u32, p_src_caches: *const crate::extensions::ext_validation_cache::ValidationCacheEXT) -> crate::vk1_0::Result;
impl<'a> crate::ExtendableFrom<'a, ShaderModuleValidationCacheCreateInfoEXT> for crate::vk1_0::ShaderModuleCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, ShaderModuleValidationCacheCreateInfoEXTBuilder<'_>> for crate::vk1_0::ShaderModuleCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkValidationCacheCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValidationCacheCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_validation_cache::ValidationCacheCreateFlagsEXT,
    pub initial_data_size: usize,
    pub p_initial_data: *const std::ffi::c_void,
}
impl ValidationCacheCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VALIDATION_CACHE_CREATE_INFO_EXT;
}
impl Default for ValidationCacheCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), initial_data_size: Default::default(), p_initial_data: std::ptr::null() }
    }
}
impl std::fmt::Debug for ValidationCacheCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ValidationCacheCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("initial_data_size", &self.initial_data_size).field("p_initial_data", &self.p_initial_data).finish()
    }
}
impl ValidationCacheCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ValidationCacheCreateInfoEXTBuilder<'a> {
        ValidationCacheCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheCreateInfoEXT.html) · Builder of [`ValidationCacheCreateInfoEXT`]"]
#[repr(transparent)]
pub struct ValidationCacheCreateInfoEXTBuilder<'a>(ValidationCacheCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ValidationCacheCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ValidationCacheCreateInfoEXTBuilder<'a> {
        ValidationCacheCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_validation_cache::ValidationCacheCreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn initial_data_size(mut self, initial_data_size: usize) -> Self {
        self.0.initial_data_size = initial_data_size;
        self
    }
    #[inline]
    pub fn initial_data(mut self, initial_data: *const std::ffi::c_void) -> Self {
        self.0.p_initial_data = initial_data;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> ValidationCacheCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for ValidationCacheCreateInfoEXTBuilder<'a> {
    fn default() -> ValidationCacheCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ValidationCacheCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ValidationCacheCreateInfoEXTBuilder<'a> {
    type Target = ValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ValidationCacheCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkShaderModuleValidationCacheCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
}
impl ShaderModuleValidationCacheCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT;
}
impl Default for ShaderModuleValidationCacheCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), validation_cache: Default::default() }
    }
}
impl std::fmt::Debug for ShaderModuleValidationCacheCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShaderModuleValidationCacheCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("validation_cache", &self.validation_cache).finish()
    }
}
impl ShaderModuleValidationCacheCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
        ShaderModuleValidationCacheCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html) · Builder of [`ShaderModuleValidationCacheCreateInfoEXT`]"]
#[repr(transparent)]
pub struct ShaderModuleValidationCacheCreateInfoEXTBuilder<'a>(ShaderModuleValidationCacheCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
        ShaderModuleValidationCacheCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn validation_cache(mut self, validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT) -> Self {
        self.0.validation_cache = validation_cache as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> ShaderModuleValidationCacheCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    fn default() -> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    type Target = ShaderModuleValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_validation_cache`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateValidationCacheEXT.html) · Function"]
    #[doc(alias = "vkCreateValidationCacheEXT")]
    pub unsafe fn create_validation_cache_ext(&self, create_info: &crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::ext_validation_cache::ValidationCacheEXT> {
        let _function = self.create_validation_cache_ext.expect(crate::NOT_LOADED_MESSAGE);
        let mut validation_cache = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut validation_cache,
        );
        crate::utils::VulkanResult::new(_return, validation_cache)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyValidationCacheEXT.html) · Function"]
    #[doc(alias = "vkDestroyValidationCacheEXT")]
    pub unsafe fn destroy_validation_cache_ext(&self, validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_validation_cache_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(
            self.handle,
            validation_cache as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetValidationCacheDataEXT.html) · Function"]
    #[doc(alias = "vkGetValidationCacheDataEXT")]
    pub unsafe fn get_validation_cache_data_ext(&self, validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT, data_size: *mut usize, data: *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_validation_cache_data_ext.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(self.handle, validation_cache as _, data_size, data);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergeValidationCachesEXT.html) · Function"]
    #[doc(alias = "vkMergeValidationCachesEXT")]
    pub unsafe fn merge_validation_caches_ext(&self, dst_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT, src_caches: &[crate::extensions::ext_validation_cache::ValidationCacheEXT]) -> crate::utils::VulkanResult<()> {
        let _function = self.merge_validation_caches_ext.expect(crate::NOT_LOADED_MESSAGE);
        let src_cache_count = src_caches.len();
        let _return = _function(self.handle, dst_cache as _, src_cache_count as _, src_caches.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
