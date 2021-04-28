#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEBUG_REPORT_SPEC_VERSION")]
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEBUG_REPORT_EXTENSION_NAME")]
pub const EXT_DEBUG_REPORT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_debug_report");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DEBUG_REPORT_CALLBACK_EXT: *const std::os::raw::c_char = crate::cstr!("vkCreateDebugReportCallbackEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DEBUG_REPORT_CALLBACK_EXT: *const std::os::raw::c_char = crate::cstr!("vkDestroyDebugReportCallbackEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DEBUG_REPORT_MESSAGE_EXT: *const std::os::raw::c_char = crate::cstr!("vkDebugReportMessageEXT");
crate::non_dispatchable_handle!(DebugReportCallbackEXT, DEBUG_REPORT_CALLBACK_EXT, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportCallbackEXT.html) · Non-dispatchable Handle", "VkDebugReportCallbackEXT");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportFlagsEXT.html) · Bitmask of [`DebugReportFlagBitsEXT`]"] # [doc (alias = "VkDebugReportFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct DebugReportFlagsEXT : u32 { const INFORMATION_EXT = DebugReportFlagBitsEXT :: INFORMATION_EXT . 0 ; const WARNING_EXT = DebugReportFlagBitsEXT :: WARNING_EXT . 0 ; const PERFORMANCE_WARNING_EXT = DebugReportFlagBitsEXT :: PERFORMANCE_WARNING_EXT . 0 ; const ERROR_EXT = DebugReportFlagBitsEXT :: ERROR_EXT . 0 ; const DEBUG_EXT = DebugReportFlagBitsEXT :: DEBUG_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportFlagBitsEXT.html) · Bits enum of [`DebugReportFlagsEXT`]"]
#[doc(alias = "VkDebugReportFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DebugReportFlagBitsEXT(pub u32);
impl DebugReportFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DebugReportFlagsEXT {
        DebugReportFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DebugReportFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INFORMATION_EXT => "INFORMATION_EXT",
            &Self::WARNING_EXT => "WARNING_EXT",
            &Self::PERFORMANCE_WARNING_EXT => "PERFORMANCE_WARNING_EXT",
            &Self::ERROR_EXT => "ERROR_EXT",
            &Self::DEBUG_EXT => "DEBUG_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_report`]"]
impl DebugReportFlagBitsEXT {
    pub const INFORMATION_EXT: Self = Self(1);
    pub const WARNING_EXT: Self = Self(2);
    pub const PERFORMANCE_WARNING_EXT: Self = Self(4);
    pub const ERROR_EXT: Self = Self(8);
    pub const DEBUG_EXT: Self = Self(16);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportObjectTypeEXT.html) · Enum"]
#[doc(alias = "VkDebugReportObjectTypeEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DebugReportObjectTypeEXT(pub i32);
impl std::fmt::Debug for DebugReportObjectTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNKNOWN_EXT => "UNKNOWN_EXT",
            &Self::INSTANCE_EXT => "INSTANCE_EXT",
            &Self::PHYSICAL_DEVICE_EXT => "PHYSICAL_DEVICE_EXT",
            &Self::DEVICE_EXT => "DEVICE_EXT",
            &Self::QUEUE_EXT => "QUEUE_EXT",
            &Self::SEMAPHORE_EXT => "SEMAPHORE_EXT",
            &Self::COMMAND_BUFFER_EXT => "COMMAND_BUFFER_EXT",
            &Self::FENCE_EXT => "FENCE_EXT",
            &Self::DEVICE_MEMORY_EXT => "DEVICE_MEMORY_EXT",
            &Self::BUFFER_EXT => "BUFFER_EXT",
            &Self::IMAGE_EXT => "IMAGE_EXT",
            &Self::EVENT_EXT => "EVENT_EXT",
            &Self::QUERY_POOL_EXT => "QUERY_POOL_EXT",
            &Self::BUFFER_VIEW_EXT => "BUFFER_VIEW_EXT",
            &Self::IMAGE_VIEW_EXT => "IMAGE_VIEW_EXT",
            &Self::SHADER_MODULE_EXT => "SHADER_MODULE_EXT",
            &Self::PIPELINE_CACHE_EXT => "PIPELINE_CACHE_EXT",
            &Self::PIPELINE_LAYOUT_EXT => "PIPELINE_LAYOUT_EXT",
            &Self::RENDER_PASS_EXT => "RENDER_PASS_EXT",
            &Self::PIPELINE_EXT => "PIPELINE_EXT",
            &Self::DESCRIPTOR_SET_LAYOUT_EXT => "DESCRIPTOR_SET_LAYOUT_EXT",
            &Self::SAMPLER_EXT => "SAMPLER_EXT",
            &Self::DESCRIPTOR_POOL_EXT => "DESCRIPTOR_POOL_EXT",
            &Self::DESCRIPTOR_SET_EXT => "DESCRIPTOR_SET_EXT",
            &Self::FRAMEBUFFER_EXT => "FRAMEBUFFER_EXT",
            &Self::COMMAND_POOL_EXT => "COMMAND_POOL_EXT",
            &Self::SURFACE_KHR_EXT => "SURFACE_KHR_EXT",
            &Self::SWAPCHAIN_KHR_EXT => "SWAPCHAIN_KHR_EXT",
            &Self::DEBUG_REPORT_CALLBACK_EXT_EXT => "DEBUG_REPORT_CALLBACK_EXT_EXT",
            &Self::DISPLAY_KHR_EXT => "DISPLAY_KHR_EXT",
            &Self::DISPLAY_MODE_KHR_EXT => "DISPLAY_MODE_KHR_EXT",
            &Self::VALIDATION_CACHE_EXT_EXT => "VALIDATION_CACHE_EXT_EXT",
            &Self::SAMPLER_YCBCR_CONVERSION_EXT => "SAMPLER_YCBCR_CONVERSION_EXT",
            &Self::DESCRIPTOR_UPDATE_TEMPLATE_EXT => "DESCRIPTOR_UPDATE_TEMPLATE_EXT",
            &Self::ACCELERATION_STRUCTURE_KHR_EXT => "ACCELERATION_STRUCTURE_KHR_EXT",
            &Self::ACCELERATION_STRUCTURE_NV_EXT => "ACCELERATION_STRUCTURE_NV_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_report`]"]
impl DebugReportObjectTypeEXT {
    pub const UNKNOWN_EXT: Self = Self(0);
    pub const INSTANCE_EXT: Self = Self(1);
    pub const PHYSICAL_DEVICE_EXT: Self = Self(2);
    pub const DEVICE_EXT: Self = Self(3);
    pub const QUEUE_EXT: Self = Self(4);
    pub const SEMAPHORE_EXT: Self = Self(5);
    pub const COMMAND_BUFFER_EXT: Self = Self(6);
    pub const FENCE_EXT: Self = Self(7);
    pub const DEVICE_MEMORY_EXT: Self = Self(8);
    pub const BUFFER_EXT: Self = Self(9);
    pub const IMAGE_EXT: Self = Self(10);
    pub const EVENT_EXT: Self = Self(11);
    pub const QUERY_POOL_EXT: Self = Self(12);
    pub const BUFFER_VIEW_EXT: Self = Self(13);
    pub const IMAGE_VIEW_EXT: Self = Self(14);
    pub const SHADER_MODULE_EXT: Self = Self(15);
    pub const PIPELINE_CACHE_EXT: Self = Self(16);
    pub const PIPELINE_LAYOUT_EXT: Self = Self(17);
    pub const RENDER_PASS_EXT: Self = Self(18);
    pub const PIPELINE_EXT: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT_EXT: Self = Self(20);
    pub const SAMPLER_EXT: Self = Self(21);
    pub const DESCRIPTOR_POOL_EXT: Self = Self(22);
    pub const DESCRIPTOR_SET_EXT: Self = Self(23);
    pub const FRAMEBUFFER_EXT: Self = Self(24);
    pub const COMMAND_POOL_EXT: Self = Self(25);
    pub const SURFACE_KHR_EXT: Self = Self(26);
    pub const SWAPCHAIN_KHR_EXT: Self = Self(27);
    pub const DEBUG_REPORT_CALLBACK_EXT_EXT: Self = Self(28);
    pub const DISPLAY_KHR_EXT: Self = Self(29);
    pub const DISPLAY_MODE_KHR_EXT: Self = Self(30);
    pub const VALIDATION_CACHE_EXT_EXT: Self = Self(33);
    pub const SAMPLER_YCBCR_CONVERSION_EXT: Self = Self(1000156000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_EXT: Self = Self(1000085000);
    pub const DEBUG_REPORT_EXT: Self = Self::DEBUG_REPORT_CALLBACK_EXT_EXT;
    pub const VALIDATION_CACHE_EXT: Self = Self::VALIDATION_CACHE_EXT_EXT;
}
#[doc = "Provided by [`crate::extensions::khr_acceleration_structure`]"]
impl DebugReportObjectTypeEXT {
    pub const ACCELERATION_STRUCTURE_KHR_EXT: Self = Self(1000150000);
}
#[doc = "Provided by [`crate::extensions::nv_ray_tracing`]"]
impl DebugReportObjectTypeEXT {
    pub const ACCELERATION_STRUCTURE_NV_EXT: Self = Self(1000165000);
}
#[doc = "Provided by [`crate::extensions::khr_descriptor_update_template`]"]
impl DebugReportObjectTypeEXT {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE_EXT;
}
#[doc = "Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]"]
impl DebugReportObjectTypeEXT {
    pub const SAMPLER_YCBCR_CONVERSION_KHR_EXT: Self = Self::SAMPLER_YCBCR_CONVERSION_EXT;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugReportCallbackEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_create_info: *const crate::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_callback: *mut crate::extensions::ext_debug_report::DebugReportCallbackEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugReportCallbackEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(instance: crate::vk1_0::Instance, callback: crate::extensions::ext_debug_report::DebugReportCallbackEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugReportMessageEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(instance: crate::vk1_0::Instance, flags: crate::extensions::ext_debug_report::DebugReportFlagsEXT, object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT, object: u64, location: usize, message_code: i32, p_layer_prefix: *const std::os::raw::c_char, p_message: *const std::os::raw::c_char) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkDebugReportCallbackEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(flags: crate::extensions::ext_debug_report::DebugReportFlagsEXT, object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT, object: u64, location: usize, message_code: i32, p_layer_prefix: *const std::os::raw::c_char, p_message: *const std::os::raw::c_char, p_user_data: *mut std::ffi::c_void) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkDebugReportCallbackCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_debug_report::DebugReportFlagsEXT,
    pub pfn_callback: Option<crate::extensions::ext_debug_report::PFN_vkDebugReportCallbackEXT>,
    pub p_user_data: *mut std::ffi::c_void,
}
impl Default for DebugReportCallbackCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT, p_next: std::ptr::null(), flags: Default::default(), pfn_callback: Default::default(), p_user_data: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for DebugReportCallbackCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugReportCallbackCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("pfn_callback", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_callback) }).field("p_user_data", &self.p_user_data).finish()
    }
}
impl DebugReportCallbackCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugReportCallbackCreateInfoEXTBuilder<'a> {
        DebugReportCallbackCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugReportCallbackCreateInfoEXT.html) · Builder of [`DebugReportCallbackCreateInfoEXT`]"]
#[repr(transparent)]
pub struct DebugReportCallbackCreateInfoEXTBuilder<'a>(DebugReportCallbackCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DebugReportCallbackCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugReportCallbackCreateInfoEXTBuilder<'a> {
        DebugReportCallbackCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_debug_report::DebugReportFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn pfn_callback(mut self, pfn_callback: Option<crate::extensions::ext_debug_report::PFN_vkDebugReportCallbackEXT>) -> Self {
        self.0.pfn_callback = pfn_callback as _;
        self
    }
    #[inline]
    pub fn user_data(mut self, user_data: *mut std::ffi::c_void) -> Self {
        self.0.p_user_data = user_data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugReportCallbackCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugReportCallbackCreateInfoEXTBuilder<'a> {
    fn default() -> DebugReportCallbackCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugReportCallbackCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugReportCallbackCreateInfoEXTBuilder<'a> {
    type Target = DebugReportCallbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugReportCallbackCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_report`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugReportCallbackEXT.html) · Function"]
    #[doc(alias = "vkCreateDebugReportCallbackEXT")]
    pub unsafe fn create_debug_report_callback_ext(&self, create_info: &crate::extensions::ext_debug_report::DebugReportCallbackCreateInfoEXT, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::ext_debug_report::DebugReportCallbackEXT> {
        let _function = self.create_debug_report_callback_ext.expect("`create_debug_report_callback_ext` is not loaded");
        let mut callback = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut callback,
        );
        crate::utils::VulkanResult::new(_return, callback)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugReportCallbackEXT.html) · Function"]
    #[doc(alias = "vkDestroyDebugReportCallbackEXT")]
    pub unsafe fn destroy_debug_report_callback_ext(&self, callback: Option<crate::extensions::ext_debug_report::DebugReportCallbackEXT>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_debug_report_callback_ext.expect("`destroy_debug_report_callback_ext` is not loaded");
        let _return = _function(
            self.handle,
            match callback {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugReportMessageEXT.html) · Function"]
    #[doc(alias = "vkDebugReportMessageEXT")]
    pub unsafe fn debug_report_message_ext(&self, flags: crate::extensions::ext_debug_report::DebugReportFlagsEXT, object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT, object: u64, location: usize, message_code: i32, layer_prefix: Option<&std::ffi::CStr>, message: Option<&std::ffi::CStr>) -> () {
        let _function = self.debug_report_message_ext.expect("`debug_report_message_ext` is not loaded");
        let _return = _function(
            self.handle,
            flags as _,
            object_type as _,
            object as _,
            location as _,
            message_code as _,
            match layer_prefix {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
            match message {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
        );
        ()
    }
}
