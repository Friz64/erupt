# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_validation_cache.html)\n\n## Extends\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VALIDATION_CACHE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_validation_cache");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateValidationCacheEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_validation_cache: *mut crate::extensions::ext_validation_cache::ValidationCacheEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyValidationCacheEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergeValidationCachesEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    dst_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
    src_cache_count: u32,
    p_src_caches: *const crate::extensions::ext_validation_cache::ValidationCacheEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetValidationCacheDataEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
    p_data_size: *mut usize,
    p_data: *mut std::ffi::c_void,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`ExtValidationCacheDeviceLoaderExt`](trait.ExtValidationCacheDeviceLoaderExt.html)"]
pub struct ExtValidationCacheDeviceCommands {
    pub create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
    pub destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
    pub merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
    pub get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
}
impl ExtValidationCacheDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtValidationCacheDeviceCommands> {
        unsafe {
            Some(ExtValidationCacheDeviceCommands {
                create_validation_cache_ext: std::mem::transmute(
                    loader.symbol("vkCreateValidationCacheEXT")?,
                ),
                destroy_validation_cache_ext: std::mem::transmute(
                    loader.symbol("vkDestroyValidationCacheEXT")?,
                ),
                merge_validation_caches_ext: std::mem::transmute(
                    loader.symbol("vkMergeValidationCachesEXT")?,
                ),
                get_validation_cache_data_ext: std::mem::transmute(
                    loader.symbol("vkGetValidationCacheDataEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtValidationCacheDeviceCommands`](struct.ExtValidationCacheDeviceCommands.html)"]
pub trait ExtValidationCacheDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateValidationCacheEXT.html) · Device Command"]
    unsafe fn create_validation_cache_ext(
        &self,
        create_info: &crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        validation_cache: Option<crate::extensions::ext_validation_cache::ValidationCacheEXT>,
    ) -> crate::utils::VulkanResult<crate::extensions::ext_validation_cache::ValidationCacheEXT>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyValidationCacheEXT.html) · Device Command"]
    unsafe fn destroy_validation_cache_ext(
        &self,
        validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergeValidationCachesEXT.html) · Device Command"]
    unsafe fn merge_validation_caches_ext(
        &self,
        dst_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        src_caches: &[crate::extensions::ext_validation_cache::ValidationCacheEXT],
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetValidationCacheDataEXT.html) · Device Command"]
    unsafe fn get_validation_cache_data_ext(
        &self,
        validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        data_size: *mut usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
}
impl ExtValidationCacheDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateValidationCacheEXT.html) · Device Command"]
    unsafe fn create_validation_cache_ext(
        &self,
        create_info: &crate::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        validation_cache: Option<crate::extensions::ext_validation_cache::ValidationCacheEXT>,
    ) -> crate::utils::VulkanResult<crate::extensions::ext_validation_cache::ValidationCacheEXT>
    {
        let function = self
            .ext_validation_cache
            .as_ref()
            .expect("`ext_validation_cache` not loaded")
            .create_validation_cache_ext;
        let mut validation_cache = validation_cache.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut validation_cache,
        );
        crate::utils::VulkanResult::new(_val, validation_cache)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyValidationCacheEXT.html) · Device Command"]
    unsafe fn destroy_validation_cache_ext(
        &self,
        validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .ext_validation_cache
            .as_ref()
            .expect("`ext_validation_cache` not loaded")
            .destroy_validation_cache_ext;
        let _val = function(
            self.handle,
            validation_cache,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergeValidationCachesEXT.html) · Device Command"]
    unsafe fn merge_validation_caches_ext(
        &self,
        dst_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        src_caches: &[crate::extensions::ext_validation_cache::ValidationCacheEXT],
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_validation_cache
            .as_ref()
            .expect("`ext_validation_cache` not loaded")
            .merge_validation_caches_ext;
        let src_cache_count = src_caches.len() as _;
        let _val = function(
            self.handle,
            dst_cache,
            src_cache_count,
            src_caches.as_ptr() as _,
        );
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetValidationCacheDataEXT.html) · Device Command"]
    unsafe fn get_validation_cache_data_ext(
        &self,
        validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
        data_size: *mut usize,
        data: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_validation_cache
            .as_ref()
            .expect("`ext_validation_cache` not loaded")
            .get_validation_cache_data_ext;
        let _val = function(self.handle, validation_cache, data_size, data);
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheCreateInfoEXT.html) · Structure"]
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
    #[inline]
    pub fn builder<'a>(self) -> ValidationCacheCreateInfoEXTBuilder<'a> {
        ValidationCacheCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ValidationCacheCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ValidationCacheCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("initial_data_size", &self.initial_data_size)
            .field("p_initial_data", &self.p_initial_data)
            .finish()
    }
}
impl Default for ValidationCacheCreateInfoEXT {
    fn default() -> ValidationCacheCreateInfoEXT {
        ValidationCacheCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            initial_data_size: Default::default(),
            p_initial_data: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ValidationCacheCreateInfoEXT`](struct.ValidationCacheCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct ValidationCacheCreateInfoEXTBuilder<'a>(
    ValidationCacheCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ValidationCacheCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ValidationCacheCreateInfoEXTBuilder<'a> {
        ValidationCacheCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_validation_cache::ValidationCacheCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn initial_data(mut self, initial_data: &'a [std::ffi::c_void]) -> Self {
        self.0.initial_data_size = initial_data.len() as _;
        self.0.p_initial_data = initial_data.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ValidationCacheCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ValidationCacheCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`ValidationCacheCreateFlagsEXT`](struct.ValidationCacheCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(Unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheCreateFlagsEXT.html) · Flags of [`ValidationCacheCreateFlagBitsEXT`](struct.ValidationCacheCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct ValidationCacheCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
crate :: non_dispatchable_handle ! ( ValidationCacheEXT , VALIDATION_CACHE_EXT , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheEXT.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
}
impl ShaderModuleValidationCacheCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByShaderModuleValidationCacheCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
        ShaderModuleValidationCacheCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ShaderModuleValidationCacheCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ShaderModuleValidationCacheCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("validation_cache", &self.validation_cache)
            .finish()
    }
}
impl Default for ShaderModuleValidationCacheCreateInfoEXT {
    fn default() -> ShaderModuleValidationCacheCreateInfoEXT {
        ShaderModuleValidationCacheCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            validation_cache: Default::default(),
        }
    }
}
#[doc = "Used by [`ShaderModuleValidationCacheCreateInfoEXT::extend`](struct.ShaderModuleValidationCacheCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByShaderModuleValidationCacheCreateInfoEXT {}
impl ExtendableByShaderModuleValidationCacheCreateInfoEXT for crate::vk1_0::ShaderModuleCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ShaderModuleValidationCacheCreateInfoEXT`](struct.ShaderModuleValidationCacheCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct ShaderModuleValidationCacheCreateInfoEXTBuilder<'a>(
    ShaderModuleValidationCacheCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
        ShaderModuleValidationCacheCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn validation_cache(
        mut self,
        validation_cache: crate::extensions::ext_validation_cache::ValidationCacheEXT,
    ) -> Self {
        self.0.validation_cache = validation_cache;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ShaderModuleValidationCacheCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCacheHeaderVersionEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);
#[doc = "[Part of `extensions::ext_validation_cache`](../../extensions/ext_validation_cache/index.html)"]
impl ValidationCacheHeaderVersionEXT {
    pub const ONE_EXT: Self = Self(1);
}
impl std::fmt::Debug for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ONE_EXT => "ONE_EXT",
            _ => "(Unknown)",
        })
    }
}
