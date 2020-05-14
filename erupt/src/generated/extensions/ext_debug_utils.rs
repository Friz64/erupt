# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_debug_utils.html)\n\n## Extends\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DEBUG_UTILS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_debug_utils");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_name_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_tag_info: *const crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
    queue: crate::vk1_0::Queue,
    p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueEndDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: crate::vk1_0::Queue) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
    queue: crate::vk1_0::Queue,
    p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndDebugUtilsLabelEXT =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_label_info: *const crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    p_create_info: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_messenger: *mut crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: crate::vk1_0::Instance,
    message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
) -> std::ffi::c_void;
#[doc = "Provides Instance Commands for [`ExtDebugUtilsInstanceLoaderExt`](trait.ExtDebugUtilsInstanceLoaderExt.html)"]
pub struct ExtDebugUtilsInstanceCommands {
    pub create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
    pub destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
    pub submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
}
impl ExtDebugUtilsInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtDebugUtilsInstanceCommands> {
        unsafe {
            Some(ExtDebugUtilsInstanceCommands {
                create_debug_utils_messenger_ext: std::mem::transmute(
                    loader.symbol("vkCreateDebugUtilsMessengerEXT")?,
                ),
                destroy_debug_utils_messenger_ext: std::mem::transmute(
                    loader.symbol("vkDestroyDebugUtilsMessengerEXT")?,
                ),
                submit_debug_utils_message_ext: std::mem::transmute(
                    loader.symbol("vkSubmitDebugUtilsMessageEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtDebugUtilsInstanceCommands`](struct.ExtDebugUtilsInstanceCommands.html)"]
pub trait ExtDebugUtilsInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) · Instance Command"]
    unsafe fn create_debug_utils_messenger_ext(
        &self,
        create_info: &crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        messenger: Option<crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT>,
    ) -> crate::utils::VulkanResult<crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) · Instance Command"]
    unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) · Instance Command"]
    unsafe fn submit_debug_utils_message_ext(
        &self,
        message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
        callback_data: &crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    ) -> ();
}
impl ExtDebugUtilsInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html) · Instance Command"]
    unsafe fn create_debug_utils_messenger_ext(
        &self,
        create_info: &crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        messenger: Option<crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT>,
    ) -> crate::utils::VulkanResult<crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT>
    {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .create_debug_utils_messenger_ext;
        let mut messenger = messenger.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut messenger,
        );
        crate::utils::VulkanResult::new(_val, messenger)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html) · Instance Command"]
    unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: crate::extensions::ext_debug_utils::DebugUtilsMessengerEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .destroy_debug_utils_messenger_ext;
        let _val = function(
            self.handle,
            messenger,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html) · Instance Command"]
    unsafe fn submit_debug_utils_message_ext(
        &self,
        message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
        callback_data: &crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    ) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .submit_debug_utils_message_ext;
        let _val = function(self.handle, message_severity, message_types, callback_data);
        ()
    }
}
#[doc = "Provides Device Commands for [`ExtDebugUtilsDeviceLoaderExt`](trait.ExtDebugUtilsDeviceLoaderExt.html)"]
pub struct ExtDebugUtilsDeviceCommands {
    pub set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
    pub set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
    pub queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
    pub queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
    pub queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
    pub cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
    pub cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
    pub cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
}
impl ExtDebugUtilsDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtDebugUtilsDeviceCommands> {
        unsafe {
            Some(ExtDebugUtilsDeviceCommands {
                set_debug_utils_object_name_ext: std::mem::transmute(
                    loader.symbol("vkSetDebugUtilsObjectNameEXT")?,
                ),
                set_debug_utils_object_tag_ext: std::mem::transmute(
                    loader.symbol("vkSetDebugUtilsObjectTagEXT")?,
                ),
                queue_begin_debug_utils_label_ext: std::mem::transmute(
                    loader.symbol("vkQueueBeginDebugUtilsLabelEXT")?,
                ),
                queue_end_debug_utils_label_ext: std::mem::transmute(
                    loader.symbol("vkQueueEndDebugUtilsLabelEXT")?,
                ),
                queue_insert_debug_utils_label_ext: std::mem::transmute(
                    loader.symbol("vkQueueInsertDebugUtilsLabelEXT")?,
                ),
                cmd_begin_debug_utils_label_ext: std::mem::transmute(
                    loader.symbol("vkCmdBeginDebugUtilsLabelEXT")?,
                ),
                cmd_end_debug_utils_label_ext: std::mem::transmute(
                    loader.symbol("vkCmdEndDebugUtilsLabelEXT")?,
                ),
                cmd_insert_debug_utils_label_ext: std::mem::transmute(
                    loader.symbol("vkCmdInsertDebugUtilsLabelEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtDebugUtilsDeviceCommands`](struct.ExtDebugUtilsDeviceCommands.html)"]
pub trait ExtDebugUtilsDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) · Device Command"]
    unsafe fn set_debug_utils_object_name_ext(
        &self,
        name_info: &crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) · Device Command"]
    unsafe fn set_debug_utils_object_tag_ext(
        &self,
        tag_info: &crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: crate::vk1_0::Queue,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn queue_end_debug_utils_label_ext(&self, queue: crate::vk1_0::Queue) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: crate::vk1_0::Queue,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn cmd_end_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> ();
}
impl ExtDebugUtilsDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html) · Device Command"]
    unsafe fn set_debug_utils_object_name_ext(
        &self,
        name_info: &crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .set_debug_utils_object_name_ext;
        let _val = function(self.handle, name_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html) · Device Command"]
    unsafe fn set_debug_utils_object_tag_ext(
        &self,
        tag_info: &crate::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .set_debug_utils_object_tag_ext;
        let _val = function(self.handle, tag_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: crate::vk1_0::Queue,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .queue_begin_debug_utils_label_ext;
        let _val = function(queue, label_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn queue_end_debug_utils_label_ext(&self, queue: crate::vk1_0::Queue) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .queue_end_debug_utils_label_ext;
        let _val = function(queue);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: crate::vk1_0::Queue,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .queue_insert_debug_utils_label_ext;
        let _val = function(queue, label_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .cmd_begin_debug_utils_label_ext;
        let _val = function(command_buffer, label_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn cmd_end_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
    ) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .cmd_end_debug_utils_label_ext;
        let _val = function(command_buffer);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html) · Device Command"]
    unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        label_info: &crate::extensions::ext_debug_utils::DebugUtilsLabelEXT,
    ) -> () {
        let function = self
            .ext_debug_utils
            .as_ref()
            .expect("`ext_debug_utils` not loaded")
            .cmd_insert_debug_utils_label_ext;
        let _val = function(command_buffer, label_info);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub object_type: crate::vk1_0::ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const std::os::raw::c_char,
}
impl DebugUtilsObjectNameInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DebugUtilsObjectNameInfoEXTBuilder<'a> {
        DebugUtilsObjectNameInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugUtilsObjectNameInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugUtilsObjectNameInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("object_type", &self.object_type)
            .field("object_handle", &self.object_handle)
            .field("p_object_name", &self.p_object_name)
            .finish()
    }
}
impl Default for DebugUtilsObjectNameInfoEXT {
    fn default() -> DebugUtilsObjectNameInfoEXT {
        DebugUtilsObjectNameInfoEXT {
            s_type: crate::vk1_0::StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object_handle: Default::default(),
            p_object_name: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) · Builder of [`DebugUtilsObjectNameInfoEXT`](struct.DebugUtilsObjectNameInfoEXT.html)"]
#[repr(transparent)]
pub struct DebugUtilsObjectNameInfoEXTBuilder<'a>(
    DebugUtilsObjectNameInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DebugUtilsObjectNameInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsObjectNameInfoEXTBuilder<'a> {
        DebugUtilsObjectNameInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn object_type(mut self, object_type: crate::vk1_0::ObjectType) -> Self {
        self.0.object_type = object_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.0.object_handle = object_handle;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn object_name(mut self, object_name: &'a std::ffi::CStr) -> Self {
        self.0.p_object_name = object_name.as_ptr();
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DebugUtilsObjectNameInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
impl DebugUtilsObjectTagInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DebugUtilsObjectTagInfoEXTBuilder<'a> {
        DebugUtilsObjectTagInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugUtilsObjectTagInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugUtilsObjectTagInfoEXT")
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
impl Default for DebugUtilsObjectTagInfoEXT {
    fn default() -> DebugUtilsObjectTagInfoEXT {
        DebugUtilsObjectTagInfoEXT {
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
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html) · Builder of [`DebugUtilsObjectTagInfoEXT`](struct.DebugUtilsObjectTagInfoEXT.html)"]
#[repr(transparent)]
pub struct DebugUtilsObjectTagInfoEXTBuilder<'a>(
    DebugUtilsObjectTagInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DebugUtilsObjectTagInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsObjectTagInfoEXTBuilder<'a> {
        DebugUtilsObjectTagInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn object_type(mut self, object_type: crate::vk1_0::ObjectType) -> Self {
        self.0.object_type = object_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.0.object_handle = object_handle;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.0.tag_name = tag_name;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn tag(mut self, tag: &'a [u8]) -> Self {
        self.0.tag_size = tag.len() as _;
        self.0.p_tag = tag.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DebugUtilsObjectTagInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsLabelEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_label_name: *const std::os::raw::c_char,
    pub color: [f32; 4],
}
impl DebugUtilsLabelEXT {
    #[inline]
    pub fn builder<'a>(self) -> DebugUtilsLabelEXTBuilder<'a> {
        DebugUtilsLabelEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugUtilsLabelEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugUtilsLabelEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_label_name", &self.p_label_name)
            .field("color", &self.color)
            .finish()
    }
}
impl Default for DebugUtilsLabelEXT {
    fn default() -> DebugUtilsLabelEXT {
        DebugUtilsLabelEXT {
            s_type: crate::vk1_0::StructureType::DEBUG_UTILS_LABEL_EXT,
            p_next: std::ptr::null(),
            p_label_name: std::ptr::null(),
            color: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsLabelEXT.html) · Builder of [`DebugUtilsLabelEXT`](struct.DebugUtilsLabelEXT.html)"]
#[repr(transparent)]
pub struct DebugUtilsLabelEXTBuilder<'a>(DebugUtilsLabelEXT, std::marker::PhantomData<&'a ()>);
impl<'a> DebugUtilsLabelEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsLabelEXTBuilder<'a> {
        DebugUtilsLabelEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn label_name(mut self, label_name: &'a std::ffi::CStr) -> Self {
        self.0.p_label_name = label_name.as_ptr();
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.0.color = color;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DebugUtilsLabelEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugUtilsLabelEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback:
        Option<crate::extensions::ext_debug_utils::PFN_vkDebugUtilsMessengerCallbackEXT>,
    pub p_user_data: *mut std::ffi::c_void,
}
impl DebugUtilsMessengerCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    #[doc = "# Safety"]
    #[doc = "Make sure you don't drop `self` before it is used by the pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: crate::ExtendableBy<Self>,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        DebugUtilsMessengerCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugUtilsMessengerCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("message_severity", &self.message_severity)
            .field("message_type", &self.message_type)
            .field("pfn_user_callback", &unsafe {
                std::mem::transmute::<_, *mut ()>(self.pfn_user_callback)
            })
            .field("p_user_data", &self.p_user_data)
            .finish()
    }
}
impl Default for DebugUtilsMessengerCreateInfoEXT {
    fn default() -> DebugUtilsMessengerCreateInfoEXT {
        DebugUtilsMessengerCreateInfoEXT {
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
impl crate::ExtendableBy<DebugUtilsMessengerCreateInfoEXT> for crate::vk1_0::InstanceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateInfoEXT.html) · Builder of [`DebugUtilsMessengerCreateInfoEXT`](struct.DebugUtilsMessengerCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct DebugUtilsMessengerCreateInfoEXTBuilder<'a>(
    DebugUtilsMessengerCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        DebugUtilsMessengerCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_debug_utils::DebugUtilsMessengerCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn message_severity(
        mut self,
        message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagsEXT,
    ) -> Self {
        self.0.message_severity = message_severity;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn message_type(
        mut self,
        message_type: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    ) -> Self {
        self.0.message_type = message_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pfn_user_callback(
        mut self,
        pfn_user_callback: Option<
            crate::extensions::ext_debug_utils::PFN_vkDebugUtilsMessengerCallbackEXT,
        >,
    ) -> Self {
        self.0.pfn_user_callback = pfn_user_callback;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn user_data(mut self, user_data: &'a mut std::ffi::c_void) -> Self {
        self.0.p_user_data = user_data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DebugUtilsMessengerCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`DebugUtilsMessengerCreateFlagsEXT`](struct.DebugUtilsMessengerCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCreateFlagsEXT.html) · Flags of [`DebugUtilsMessengerCreateFlagBitsEXT`](struct.DebugUtilsMessengerCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DebugUtilsMessengerCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagBitsEXT.html) · Flag Bits of [`DebugUtilsMessageSeverityFlagsEXT`](struct.DebugUtilsMessageSeverityFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(pub u32);
impl DebugUtilsMessageSeverityFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DebugUtilsMessageSeverityFlagsEXT {
        DebugUtilsMessageSeverityFlagsEXT::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::ext_debug_utils`](../../extensions/ext_debug_utils/index.html)"]
impl DebugUtilsMessageSeverityFlagBitsEXT {
    pub const VERBOSE_EXT: Self = Self(0x00000001);
    pub const INFO_EXT: Self = Self(0x00000010);
    pub const WARNING_EXT: Self = Self(0x00000100);
    pub const ERROR_EXT: Self = Self(0x00001000);
}
impl std::fmt::Debug for DebugUtilsMessageSeverityFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::VERBOSE_EXT => "VERBOSE_EXT",
            &Self::INFO_EXT => "INFO_EXT",
            &Self::WARNING_EXT => "WARNING_EXT",
            &Self::ERROR_EXT => "ERROR_EXT",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageSeverityFlagsEXT.html) · Flags of [`DebugUtilsMessageSeverityFlagBitsEXT`](struct.DebugUtilsMessageSeverityFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DebugUtilsMessageSeverityFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const VERBOSE_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: VERBOSE_EXT . 0 ; const INFO_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: INFO_EXT . 0 ; const WARNING_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: WARNING_EXT . 0 ; const ERROR_EXT = DebugUtilsMessageSeverityFlagBitsEXT :: ERROR_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageTypeFlagBitsEXT.html) · Flag Bits of [`DebugUtilsMessageTypeFlagsEXT`](struct.DebugUtilsMessageTypeFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugUtilsMessageTypeFlagBitsEXT(pub u32);
impl DebugUtilsMessageTypeFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DebugUtilsMessageTypeFlagsEXT {
        DebugUtilsMessageTypeFlagsEXT::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::ext_debug_utils`](../../extensions/ext_debug_utils/index.html)"]
impl DebugUtilsMessageTypeFlagBitsEXT {
    pub const GENERAL_EXT: Self = Self(0x00000001);
    pub const VALIDATION_EXT: Self = Self(0x00000002);
    pub const PERFORMANCE_EXT: Self = Self(0x00000004);
}
impl std::fmt::Debug for DebugUtilsMessageTypeFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::GENERAL_EXT => "GENERAL_EXT",
            &Self::VALIDATION_EXT => "VALIDATION_EXT",
            &Self::PERFORMANCE_EXT => "PERFORMANCE_EXT",
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessageTypeFlagsEXT.html) · Flags of [`DebugUtilsMessageTypeFlagBitsEXT`](struct.DebugUtilsMessageTypeFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DebugUtilsMessageTypeFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; const GENERAL_EXT = DebugUtilsMessageTypeFlagBitsEXT :: GENERAL_EXT . 0 ; const VALIDATION_EXT = DebugUtilsMessageTypeFlagBitsEXT :: VALIDATION_EXT . 0 ; const PERFORMANCE_EXT = DebugUtilsMessageTypeFlagBitsEXT :: PERFORMANCE_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkDebugUtilsMessengerCallbackEXT.html) · Application defined function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: crate::extensions::ext_debug_utils::DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: crate::extensions::ext_debug_utils::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut std::ffi::c_void,
) -> crate::vk1_0::Bool32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) · Structure"]
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
impl DebugUtilsMessengerCallbackDataEXT {
    #[inline]
    pub fn builder<'a>(self) -> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        DebugUtilsMessengerCallbackDataEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugUtilsMessengerCallbackDataEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugUtilsMessengerCallbackDataEXT")
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
impl Default for DebugUtilsMessengerCallbackDataEXT {
    fn default() -> DebugUtilsMessengerCallbackDataEXT {
        DebugUtilsMessengerCallbackDataEXT {
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
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataEXT.html) · Builder of [`DebugUtilsMessengerCallbackDataEXT`](struct.DebugUtilsMessengerCallbackDataEXT.html)"]
#[repr(transparent)]
pub struct DebugUtilsMessengerCallbackDataEXTBuilder<'a>(
    DebugUtilsMessengerCallbackDataEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        DebugUtilsMessengerCallbackDataEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn message_id_name(mut self, message_id_name: &'a std::ffi::CStr) -> Self {
        self.0.p_message_id_name = message_id_name.as_ptr();
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn message_id_number(mut self, message_id_number: i32) -> Self {
        self.0.message_id_number = message_id_number;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn message(mut self, message: &'a std::ffi::CStr) -> Self {
        self.0.p_message = message.as_ptr();
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn queue_labels(
        mut self,
        queue_labels: &'a [crate::extensions::ext_debug_utils::DebugUtilsLabelEXTBuilder],
    ) -> Self {
        self.0.queue_label_count = queue_labels.len() as _;
        self.0.p_queue_labels = queue_labels.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn cmd_buf_labels(
        mut self,
        cmd_buf_labels: &'a [crate::extensions::ext_debug_utils::DebugUtilsLabelEXTBuilder],
    ) -> Self {
        self.0.cmd_buf_label_count = cmd_buf_labels.len() as _;
        self.0.p_cmd_buf_labels = cmd_buf_labels.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn objects(
        mut self,
        objects: &'a [crate::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXTBuilder],
    ) -> Self {
        self.0.object_count = objects.len() as _;
        self.0.p_objects = objects.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DebugUtilsMessengerCallbackDataEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
crate :: non_dispatchable_handle ! ( DebugUtilsMessengerEXT , DEBUG_UTILS_MESSENGER_EXT , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerEXT.html) · Non-dispatchable Handle" ) ;
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`DebugUtilsMessengerCallbackDataFlagsEXT`](struct.DebugUtilsMessengerCallbackDataFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
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
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugUtilsMessengerCallbackDataFlagsEXT.html) · Flags of [`DebugUtilsMessengerCallbackDataFlagBitsEXT`](struct.DebugUtilsMessengerCallbackDataFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct DebugUtilsMessengerCallbackDataFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
