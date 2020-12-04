#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEBUG_MARKER_SPEC_VERSION")]
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_DEBUG_MARKER_EXTENSION_NAME")]
pub const EXT_DEBUG_MARKER_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_debug_marker");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_DEBUG_MARKER_SET_OBJECT_NAME_EXT")]
pub const FN_DEBUG_MARKER_SET_OBJECT_NAME_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkDebugMarkerSetObjectNameEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_DEBUG_MARKER_SET_OBJECT_TAG_EXT")]
pub const FN_DEBUG_MARKER_SET_OBJECT_TAG_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkDebugMarkerSetObjectTagEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_CMD_DEBUG_MARKER_BEGIN_EXT")]
pub const FN_CMD_DEBUG_MARKER_BEGIN_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDebugMarkerBeginEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_CMD_DEBUG_MARKER_END_EXT")]
pub const FN_CMD_DEBUG_MARKER_END_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDebugMarkerEndEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "FN_CMD_DEBUG_MARKER_INSERT_EXT")]
pub const FN_CMD_DEBUG_MARKER_INSERT_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdDebugMarkerInsertEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_name_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_tag_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerEndEXT =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html) · Structure"]
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    pub object: u64,
    pub p_object_name: *const std::os::raw::c_char,
}
impl Default for DebugMarkerObjectNameInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object: Default::default(),
            p_object_name: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DebugMarkerObjectNameInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugMarkerObjectNameInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("object_type", &self.object_type)
            .field("object", &self.object)
            .field("p_object_name", &self.p_object_name)
            .finish()
    }
}
impl DebugMarkerObjectNameInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugMarkerObjectNameInfoEXTBuilder<'a> {
        DebugMarkerObjectNameInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html) · Builder of [`DebugMarkerObjectNameInfoEXT`]"]
#[repr(transparent)]
pub struct DebugMarkerObjectNameInfoEXTBuilder<'a>(
    DebugMarkerObjectNameInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DebugMarkerObjectNameInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugMarkerObjectNameInfoEXTBuilder<'a> {
        DebugMarkerObjectNameInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn object_type(
        mut self,
        object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    ) -> Self {
        self.0.object_type = object_type as _;
        self
    }
    #[inline]
    pub fn object(mut self, object: u64) -> Self {
        self.0.object = object as _;
        self
    }
    #[inline]
    pub fn object_name(mut self, object_name: &'a std::ffi::CStr) -> Self {
        self.0.p_object_name = object_name.as_ptr();
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugMarkerObjectNameInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    fn default() -> DebugMarkerObjectNameInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    type Target = DebugMarkerObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html) · Structure"]
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    pub object: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const std::ffi::c_void,
}
impl Default for DebugMarkerObjectTagInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object: Default::default(),
            tag_name: Default::default(),
            tag_size: Default::default(),
            p_tag: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for DebugMarkerObjectTagInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugMarkerObjectTagInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("object_type", &self.object_type)
            .field("object", &self.object)
            .field("tag_name", &self.tag_name)
            .field("tag_size", &self.tag_size)
            .field("p_tag", &self.p_tag)
            .finish()
    }
}
impl DebugMarkerObjectTagInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugMarkerObjectTagInfoEXTBuilder<'a> {
        DebugMarkerObjectTagInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html) · Builder of [`DebugMarkerObjectTagInfoEXT`]"]
#[repr(transparent)]
pub struct DebugMarkerObjectTagInfoEXTBuilder<'a>(
    DebugMarkerObjectTagInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DebugMarkerObjectTagInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugMarkerObjectTagInfoEXTBuilder<'a> {
        DebugMarkerObjectTagInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn object_type(
        mut self,
        object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    ) -> Self {
        self.0.object_type = object_type as _;
        self
    }
    #[inline]
    pub fn object(mut self, object: u64) -> Self {
        self.0.object = object as _;
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
    pub fn build(self) -> DebugMarkerObjectTagInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    fn default() -> DebugMarkerObjectTagInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    type Target = DebugMarkerObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html) · Structure"]
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_marker_name: *const std::os::raw::c_char,
    pub color: [std::os::raw::c_float; 4],
}
impl Default for DebugMarkerMarkerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEBUG_MARKER_MARKER_INFO_EXT,
            p_next: std::ptr::null(),
            p_marker_name: std::ptr::null(),
            color: unsafe { std::mem::zeroed() },
        }
    }
}
impl std::fmt::Debug for DebugMarkerMarkerInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DebugMarkerMarkerInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_marker_name", &self.p_marker_name)
            .field("color", &self.color)
            .finish()
    }
}
impl DebugMarkerMarkerInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> DebugMarkerMarkerInfoEXTBuilder<'a> {
        DebugMarkerMarkerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html) · Builder of [`DebugMarkerMarkerInfoEXT`]"]
#[repr(transparent)]
pub struct DebugMarkerMarkerInfoEXTBuilder<'a>(
    DebugMarkerMarkerInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DebugMarkerMarkerInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DebugMarkerMarkerInfoEXTBuilder<'a> {
        DebugMarkerMarkerInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn marker_name(mut self, marker_name: &'a std::ffi::CStr) -> Self {
        self.0.p_marker_name = marker_name.as_ptr();
        self
    }
    #[inline]
    pub fn color(mut self, color: [std::os::raw::c_float; 4]) -> Self {
        self.0.color = color as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DebugMarkerMarkerInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for DebugMarkerMarkerInfoEXTBuilder<'a> {
    fn default() -> DebugMarkerMarkerInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DebugMarkerMarkerInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DebugMarkerMarkerInfoEXTBuilder<'a> {
    type Target = DebugMarkerMarkerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DebugMarkerMarkerInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_debug_marker`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) · Function"]
    #[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        name_info: &crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .debug_marker_set_object_name_ext
            .expect("`debug_marker_set_object_name_ext` is not loaded");
        let _return = _function(self.handle, name_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) · Function"]
    #[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        tag_info: &crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .debug_marker_set_object_tag_ext
            .expect("`debug_marker_set_object_tag_ext` is not loaded");
        let _return = _function(self.handle, tag_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) · Function"]
    #[doc(alias = "vkCmdDebugMarkerBeginEXT")]
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) -> () {
        let _function = self
            .cmd_debug_marker_begin_ext
            .expect("`cmd_debug_marker_begin_ext` is not loaded");
        let _return = _function(command_buffer as _, marker_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html) · Function"]
    #[doc(alias = "vkCmdDebugMarkerEndEXT")]
    pub unsafe fn cmd_debug_marker_end_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
    ) -> () {
        let _function = self
            .cmd_debug_marker_end_ext
            .expect("`cmd_debug_marker_end_ext` is not loaded");
        let _return = _function(command_buffer as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) · Function"]
    #[doc(alias = "vkCmdDebugMarkerInsertEXT")]
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) -> () {
        let _function = self
            .cmd_debug_marker_insert_ext
            .expect("`cmd_debug_marker_insert_ext` is not loaded");
        let _return = _function(command_buffer as _, marker_info as _);
        ()
    }
}
