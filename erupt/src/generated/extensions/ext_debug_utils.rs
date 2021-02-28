#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEBUG_UTILS_SPEC_VERSION")]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEBUG_UTILS_EXTENSION_NAME")]
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_debug_utils");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_DEBUG_UTILS_OBJECT_NAME_EXT: *const std::os::raw::c_char = crate::cstr!("vkSetDebugUtilsObjectNameEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_DEBUG_UTILS_OBJECT_TAG_EXT: *const std::os::raw::c_char = crate::cstr!("vkSetDebugUtilsObjectTagEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT: *const std::os::raw::c_char = crate::cstr!("vkQueueBeginDebugUtilsLabelEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_END_DEBUG_UTILS_LABEL_EXT: *const std::os::raw::c_char = crate::cstr!("vkQueueEndDebugUtilsLabelEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT: *const std::os::raw::c_char = crate::cstr!("vkQueueInsertDebugUtilsLabelEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_DEBUG_UTILS_LABEL_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdBeginDebugUtilsLabelEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_DEBUG_UTILS_LABEL_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdEndDebugUtilsLabelEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_INSERT_DEBUG_UTILS_LABEL_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdInsertDebugUtilsLabelEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DEBUG_UTILS_MESSENGER_EXT: *const std::os::raw::c_char = crate::cstr!("vkCreateDebugUtilsMessengerEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DEBUG_UTILS_MESSENGER_EXT: *const std::os::raw::c_char = crate::cstr!("vkDestroyDebugUtilsMessengerEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SUBMIT_DEBUG_UTILS_MESSAGE_EXT: *const std::os::raw::c_char = crate::cstr!("vkSubmitDebugUtilsMessageEXT");
crate::non_dispatchable_handle!(
    DebugUtilsMessengerEXT,
    DEBUG_UTILS_MESSENGER_EXT,
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerEXT.html) · Non-dispatchable Handle",
    "VkDebugUtilsMessengerEXT"
);
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html) · Bitmask of [`DebugUtilsMessengerCreateFlagBitsEXT`]"] # [doc (alias = "VkDebugUtilsMessengerCreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct DebugUtilsMessengerCreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DebugUtilsMessengerCreateFlagsEXT`]"]
#[doc(alias = "VkDebugUtilsMessengerCreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DebugUtilsMessengerCreateFlagBitsEXT(pub u32);
impl DebugUtilsMessengerCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DebugUtilsMessengerCreateFlagsEXT {
        DebugUtilsMessengerCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html) · Bitmask of [`DebugUtilsMessengerCallbackDataFlagBitsEXT`]"] # [doc (alias = "VkDebugUtilsMessengerCallbackDataFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct DebugUtilsMessengerCallbackDataFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DebugUtilsMessengerCallbackDataFlagsEXT`]"]
#[doc(alias = "VkDebugUtilsMessengerCallbackDataFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DebugUtilsMessengerCallbackDataFlagBitsEXT(pub u32);
impl DebugUtilsMessengerCallbackDataFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DebugUtilsMessengerCallbackDataFlagsEXT {
        DebugUtilsMessengerCallbackDataFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCallbackDataFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagsEXT.html) · Bitmask of [`DebugUtilsMessageSeverityFlagBitsEXT`]"] # [doc (alias = "VkDebugUtilsMessageSeverityFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct DebugUtilsMessageSeverityFlagsEXT : u32 { const VERBOSE_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: VERBOSE_EXT . 0 ; const INFO_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: INFO_EXT . 0 ; const WARNING_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: WARNING_EXT . 0 ; const ERROR_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: ERROR_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) · Bits enum of [`DebugUtilsMessageSeverityFlagsEXT`]"]
#[doc(alias = "VkDebugUtilsMessageSeverityFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(pub u32);
impl DebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DebugUtilsMessageSeverityFlagsEXT {
        DebugUtilsMessageSeverityFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DebugUtilsMessageSeverityFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VERBOSE_EXT => "VERBOSE_EXT",
            &Self::INFO_EXT => "INFO_EXT",
            &Self::WARNING_EXT => "WARNING_EXT",
            &Self::ERROR_EXT => "ERROR_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_utils`]"]
impl DebugUtilsMessageSeverityFlagBitsEXT {
    pub const VERBOSE_EXT: Self = Self(1);
    pub const INFO_EXT: Self = Self(16);
    pub const WARNING_EXT: Self = Self(256);
    pub const ERROR_EXT: Self = Self(4096);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageTypeFlagsEXT.html) · Bitmask of [`DebugUtilsMessageTypeFlagBitsEXT`]"] # [doc (alias = "VkDebugUtilsMessageTypeFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct DebugUtilsMessageTypeFlagsEXT : u32 { const GENERAL_EXT = DebugUtilsMessageTypeFlagBitsEXT :: GENERAL_EXT . 0 ; const VALIDATION_EXT = DebugUtilsMessageTypeFlagBitsEXT :: VALIDATION_EXT . 0 ; const PERFORMANCE_EXT = DebugUtilsMessageTypeFlagBitsEXT :: PERFORMANCE_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) · Bits enum of [`DebugUtilsMessageTypeFlagsEXT`]"]
#[doc(alias = "VkDebugUtilsMessageTypeFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DebugUtilsMessageTypeFlagBitsEXT(pub u32);
impl DebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DebugUtilsMessageTypeFlagsEXT {
        DebugUtilsMessageTypeFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DebugUtilsMessageTypeFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::GENERAL_EXT => "GENERAL_EXT",
            &Self::VALIDATION_EXT => "VALIDATION_EXT",
            &Self::PERFORMANCE_EXT => "PERFORMANCE_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_utils`]"]
impl DebugUtilsMessageTypeFlagBitsEXT {
    pub const GENERAL_EXT: Self = Self(1);
    pub const VALIDATION_EXT: Self = Self(2);
    pub const PERFORMANCE_EXT: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectNameEXT =
    unsafe extern "system" fn(device: crate::vk1_0::Device, p_name_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectTagEXT =
    unsafe extern "system" fn(device: crate::vk1_0::Device, p_tag_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBeginDebugUtilsLabelEXT = unsafe extern "system" fn(queue: crate::vk1_0::Queue, p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: crate::vk1_0::Queue) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueInsertDebugUtilsLabelEXT = unsafe extern "system" fn(queue: crate::vk1_0::Queue, p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_messenger: *mut crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugUtilsMessengerEXT =
    unsafe extern "system" fn(instance: crate::vk1_0::Instance, messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut std::ffi::c_void,
) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) · Structure"]
#[doc(alias = "VkDebugUtilsObjectNameInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub object_type: crate::vk1_0::ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const std::os::raw::c_char,
}
impl Default for DebugUtilsObjectNameInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: Default::default(),
            p_object_name: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DebugUtilsObjectNameInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugUtilsObjectNameInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("object_type", &self.object_type)
            .field("object_handle", &self.object_handle)
            .field("p_object_name", &self.p_object_name)
            .finish()
    }
}
impl DebugUtilsObjectNameInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugUtilsObjectNameInfoEXTBuilder<'a> {
        DebugUtilsObjectNameInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) · Builder of [`DebugUtilsObjectNameInfoEXT`]"]
#[repr(transparent)]
pub struct DebugUtilsObjectNameInfoEXTBuilder<'a>(DebugUtilsObjectNameInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DebugUtilsObjectNameInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsObjectNameInfoEXTBuilder<'a> {
        DebugUtilsObjectNameInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn object_type(mut self, object_type: crate::vk1_0::ObjectType) -> Self {
        self.0.object_type = object_type as _;
        self
    }
    #[inline]
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.0.object_handle = object_handle as _;
        self
    }
    #[inline]
    pub fn object_name(mut self, object_name: &'a std::ffi::CStr) -> Self {
        self.0.p_object_name = object_name.as_ptr();
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugUtilsObjectNameInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    fn default() -> DebugUtilsObjectNameInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    type Target = DebugUtilsObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html) · Structure"]
#[doc(alias = "VkDebugUtilsObjectTagInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub object_type: crate::vk1_0::ObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const std::ffi::c_void,
}
impl Default for DebugUtilsObjectTagInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: Default::default(),
            tag_name: Default::default(),
            tag_size: Default::default(),
            p_tag: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DebugUtilsObjectTagInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugUtilsObjectTagInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("object_type", &self.object_type)
            .field("object_handle", &self.object_handle)
            .field("tag_name", &self.tag_name)
            .field("tag_size", &self.tag_size)
            .field("p_tag", &self.p_tag)
            .finish()
    }
}
impl DebugUtilsObjectTagInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugUtilsObjectTagInfoEXTBuilder<'a> {
        DebugUtilsObjectTagInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html) · Builder of [`DebugUtilsObjectTagInfoEXT`]"]
#[repr(transparent)]
pub struct DebugUtilsObjectTagInfoEXTBuilder<'a>(DebugUtilsObjectTagInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DebugUtilsObjectTagInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsObjectTagInfoEXTBuilder<'a> {
        DebugUtilsObjectTagInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn object_type(mut self, object_type: crate::vk1_0::ObjectType) -> Self {
        self.0.object_type = object_type as _;
        self
    }
    #[inline]
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.0.object_handle = object_handle as _;
        self
    }
    #[inline]
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.0.tag_name = tag_name as _;
        self
    }
    #[inline]
    pub fn tag_size(mut self, tag_size: usize) -> Self {
        self.0.tag_size = tag_size;
        self
    }
    #[inline]
    pub fn tag(mut self, tag: *const std::ffi::c_void) -> Self {
        self.0.p_tag = tag;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugUtilsObjectTagInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    fn default() -> DebugUtilsObjectTagInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    type Target = DebugUtilsObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsLabelEXT.html) · Structure"]
#[doc(alias = "VkDebugUtilsLabelEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsLabelEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_label_name: *const std::os::raw::c_char,
    pub color: [std::os::raw::c_float; 4],
}
impl Default for DebugUtilsLabelEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_UTILS_LABEL_EXT,
            p_next: std::ptr::null(),
            p_label_name: std::ptr::null(),
            color: unsafe { std::mem::zeroed() },
        }
    }
}
impl std::fmt::Debug for DebugUtilsLabelEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugUtilsLabelEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_label_name", &self.p_label_name)
            .field("color", &self.color)
            .finish()
    }
}
impl DebugUtilsLabelEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugUtilsLabelEXTBuilder<'a> {
        DebugUtilsLabelEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsLabelEXT.html) · Builder of [`DebugUtilsLabelEXT`]"]
#[repr(transparent)]
pub struct DebugUtilsLabelEXTBuilder<'a>(DebugUtilsLabelEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DebugUtilsLabelEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsLabelEXTBuilder<'a> {
        DebugUtilsLabelEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn label_name(mut self, label_name: &'a std::ffi::CStr) -> Self {
        self.0.p_label_name = label_name.as_ptr();
        self
    }
    #[inline]
    pub fn color(mut self, color: [std::os::raw::c_float; 4]) -> Self {
        self.0.color = color as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugUtilsLabelEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugUtilsLabelEXTBuilder<'a> {
    fn default() -> DebugUtilsLabelEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugUtilsLabelEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugUtilsLabelEXTBuilder<'a> {
    type Target = DebugUtilsLabelEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugUtilsLabelEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkDebugUtilsMessengerCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: Option<crate::extensions::ext_debug_utils::PFN_vkDebugUtilsMessengerCallbackEXT>,
    pub p_user_data: *mut std::ffi::c_void,
}
impl Default for DebugUtilsMessengerCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            message_severity: Default::default(),
            message_type: Default::default(),
            pfn_user_callback: Default::default(),
            p_user_data: std::ptr::null_mut(),
        }
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugUtilsMessengerCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("message_severity", &self.message_severity)
            .field("message_type", &self.message_type)
            .field("pfn_user_callback", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_user_callback) })
            .field("p_user_data", &self.p_user_data)
            .finish()
    }
}
impl DebugUtilsMessengerCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        DebugUtilsMessengerCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html) · Builder of [`DebugUtilsMessengerCreateInfoEXT`]"]
#[repr(transparent)]
pub struct DebugUtilsMessengerCreateInfoEXTBuilder<'a>(DebugUtilsMessengerCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        DebugUtilsMessengerCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn message_severity(mut self, message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT) -> Self {
        self.0.message_severity = message_severity as _;
        self
    }
    #[inline]
    pub fn message_type(mut self, message_type: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT) -> Self {
        self.0.message_type = message_type as _;
        self
    }
    #[inline]
    pub fn pfn_user_callback(mut self, pfn_user_callback: Option<crate::extensions::ext_debug_utils::PFN_vkDebugUtilsMessengerCallbackEXT>) -> Self {
        self.0.pfn_user_callback = pfn_user_callback as _;
        self
    }
    #[inline]
    pub fn user_data(mut self, user_data: *mut std::ffi::c_void) -> Self {
        self.0.p_user_data = user_data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugUtilsMessengerCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    fn default() -> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    type Target = DebugUtilsMessengerCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) · Structure"]
#[doc(alias = "VkDebugUtilsMessengerCallbackDataEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const std::os::raw::c_char,
    pub message_id_number: i32,
    pub p_message: *const std::os::raw::c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *const crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
}
impl Default for DebugUtilsMessengerCallbackDataEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            p_message_id_name: std::ptr::null(),
            message_id_number: Default::default(),
            p_message: std::ptr::null(),
            queue_label_count: Default::default(),
            p_queue_labels: std::ptr::null(),
            cmd_buf_label_count: Default::default(),
            p_cmd_buf_labels: std::ptr::null(),
            object_count: Default::default(),
            p_objects: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCallbackDataEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugUtilsMessengerCallbackDataEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("p_message_id_name", &self.p_message_id_name)
            .field("message_id_number", &self.message_id_number)
            .field("p_message", &self.p_message)
            .field("queue_label_count", &self.queue_label_count)
            .field("p_queue_labels", &self.p_queue_labels)
            .field("cmd_buf_label_count", &self.cmd_buf_label_count)
            .field("p_cmd_buf_labels", &self.p_cmd_buf_labels)
            .field("object_count", &self.object_count)
            .field("p_objects", &self.p_objects)
            .finish()
    }
}
impl DebugUtilsMessengerCallbackDataEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        DebugUtilsMessengerCallbackDataEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) · Builder of [`DebugUtilsMessengerCallbackDataEXT`]"]
#[repr(transparent)]
pub struct DebugUtilsMessengerCallbackDataEXTBuilder<'a>(DebugUtilsMessengerCallbackDataEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        DebugUtilsMessengerCallbackDataEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn message_id_name(mut self, message_id_name: &'a std::ffi::CStr) -> Self {
        self.0.p_message_id_name = message_id_name.as_ptr();
        self
    }
    #[inline]
    pub fn message_id_number(mut self, message_id_number: i32) -> Self {
        self.0.message_id_number = message_id_number as _;
        self
    }
    #[inline]
    pub fn message(mut self, message: &'a std::ffi::CStr) -> Self {
        self.0.p_message = message.as_ptr();
        self
    }
    #[inline]
    pub fn queue_labels(mut self, queue_labels: &'a [crate::extensions::ext_debug_utils::DebugUtilsLabelEXTBuilder]) -> Self {
        self.0.p_queue_labels = queue_labels.as_ptr() as _;
        self.0.queue_label_count = queue_labels.len() as _;
        self
    }
    #[inline]
    pub fn cmd_buf_labels(mut self, cmd_buf_labels: &'a [crate::extensions::ext_debug_utils::DebugUtilsLabelEXTBuilder]) -> Self {
        self.0.p_cmd_buf_labels = cmd_buf_labels.as_ptr() as _;
        self.0.cmd_buf_label_count = cmd_buf_labels.len() as _;
        self
    }
    #[inline]
    pub fn objects(mut self, objects: &'a [crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXTBuilder]) -> Self {
        self.0.p_objects = objects.as_ptr() as _;
        self.0.object_count = objects.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugUtilsMessengerCallbackDataEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    fn default() -> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    type Target = DebugUtilsMessengerCallbackDataEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_utils`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) · Function"]
    #[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
    pub unsafe fn set_debug_utils_object_name_ext(&self, name_info: &crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT) -> crate::utils::VulkanResult<()> {
        let _function = self.set_debug_utils_object_name_ext.expect("`set_debug_utils_object_name_ext` is not loaded");
        let _return = _function(self.handle, name_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) · Function"]
    #[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
    pub unsafe fn set_debug_utils_object_tag_ext(&self, tag_info: &crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT) -> crate::utils::VulkanResult<()> {
        let _function = self.set_debug_utils_object_tag_ext.expect("`set_debug_utils_object_tag_ext` is not loaded");
        let _return = _function(self.handle, tag_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) · Function"]
    #[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
    pub unsafe fn queue_begin_debug_utils_label_ext(&self, queue: crate::vk1_0::Queue, label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> () {
        let _function = self.queue_begin_debug_utils_label_ext.expect("`queue_begin_debug_utils_label_ext` is not loaded");
        let _return = _function(queue as _, label_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) · Function"]
    #[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: crate::vk1_0::Queue) -> () {
        let _function = self.queue_end_debug_utils_label_ext.expect("`queue_end_debug_utils_label_ext` is not loaded");
        let _return = _function(queue as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) · Function"]
    #[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
    pub unsafe fn queue_insert_debug_utils_label_ext(&self, queue: crate::vk1_0::Queue, label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> () {
        let _function = self.queue_insert_debug_utils_label_ext.expect("`queue_insert_debug_utils_label_ext` is not loaded");
        let _return = _function(queue as _, label_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) · Function"]
    #[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
    pub unsafe fn cmd_begin_debug_utils_label_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> () {
        let _function = self.cmd_begin_debug_utils_label_ext.expect("`cmd_begin_debug_utils_label_ext` is not loaded");
        let _return = _function(command_buffer as _, label_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) · Function"]
    #[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: crate::vk1_0::CommandBuffer) -> () {
        let _function = self.cmd_end_debug_utils_label_ext.expect("`cmd_end_debug_utils_label_ext` is not loaded");
        let _return = _function(command_buffer as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) · Function"]
    #[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
    pub unsafe fn cmd_insert_debug_utils_label_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT) -> () {
        let _function = self.cmd_insert_debug_utils_label_ext.expect("`cmd_insert_debug_utils_label_ext` is not loaded");
        let _return = _function(command_buffer as _, label_info as _);
        ()
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_utils`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) · Function"]
    #[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        create_info: &crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        messenger: Option<crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT>,
    ) -> crate::utils::VulkanResult<crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT> {
        let _function = self.create_debug_utils_messenger_ext.expect("`create_debug_utils_messenger_ext` is not loaded");
        let mut messenger = match messenger {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut messenger,
        );
        crate::utils::VulkanResult::new(_return, messenger)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) · Function"]
    #[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
    pub unsafe fn destroy_debug_utils_messenger_ext(&self, messenger: Option<crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_debug_utils_messenger_ext.expect("`destroy_debug_utils_messenger_ext` is not loaded");
        let _return = _function(
            self.handle,
            match messenger {
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
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) · Function"]
    #[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
        callback_data: &crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    ) -> () {
        let _function = self.submit_debug_utils_message_ext.expect("`submit_debug_utils_message_ext` is not loaded");
        let _return = _function(self.handle, message_severity as _, message_types as _, callback_data as _);
        ()
    }
}
