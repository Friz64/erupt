# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_debug_marker.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_DEBUG_MARKER_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_debug_marker");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_tag_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_name_info: *const crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerEndEXT =
    unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_marker_info: *const crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtDebugMarkerDeviceLoaderExt`](trait.ExtDebugMarkerDeviceLoaderExt.html)"]
pub struct ExtDebugMarkerDeviceCommands {
    pub debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    pub debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    pub cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    pub cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    pub cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
}
impl ExtDebugMarkerDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtDebugMarkerDeviceCommands> {
        unsafe {
            Some(ExtDebugMarkerDeviceCommands {
                debug_marker_set_object_tag_ext: std::mem::transmute(
                    loader.symbol("vkDebugMarkerSetObjectTagEXT")?,
                ),
                debug_marker_set_object_name_ext: std::mem::transmute(
                    loader.symbol("vkDebugMarkerSetObjectNameEXT")?,
                ),
                cmd_debug_marker_begin_ext: std::mem::transmute(
                    loader.symbol("vkCmdDebugMarkerBeginEXT")?,
                ),
                cmd_debug_marker_end_ext: std::mem::transmute(
                    loader.symbol("vkCmdDebugMarkerEndEXT")?,
                ),
                cmd_debug_marker_insert_ext: std::mem::transmute(
                    loader.symbol("vkCmdDebugMarkerInsertEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtDebugMarkerDeviceCommands`](struct.ExtDebugMarkerDeviceCommands.html)"]
pub trait ExtDebugMarkerDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) · Device Command"]
    unsafe fn debug_marker_set_object_tag_ext(
        &self,
        tag_info: &crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) · Device Command"]
    unsafe fn debug_marker_set_object_name_ext(
        &self,
        name_info: &crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) · Device Command"]
    unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html) · Device Command"]
    unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: crate::vk1_0::CommandBuffer) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) · Device Command"]
    unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) -> ();
}
impl ExtDebugMarkerDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectTagEXT.html) · Device Command"]
    unsafe fn debug_marker_set_object_tag_ext(
        &self,
        tag_info: &crate::extensions::ext_debug_marker::DebugMarkerObjectTagInfoEXT,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_debug_marker
            .as_ref()
            .expect("`ext_debug_marker` not loaded")
            .debug_marker_set_object_tag_ext;
        let _val = function(self.handle, tag_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html) · Device Command"]
    unsafe fn debug_marker_set_object_name_ext(
        &self,
        name_info: &crate::extensions::ext_debug_marker::DebugMarkerObjectNameInfoEXT,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .ext_debug_marker
            .as_ref()
            .expect("`ext_debug_marker` not loaded")
            .debug_marker_set_object_name_ext;
        let _val = function(self.handle, name_info);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html) · Device Command"]
    unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) -> () {
        let function = self
            .ext_debug_marker
            .as_ref()
            .expect("`ext_debug_marker` not loaded")
            .cmd_debug_marker_begin_ext;
        let _val = function(command_buffer, marker_info);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html) · Device Command"]
    unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: crate::vk1_0::CommandBuffer) -> () {
        let function = self
            .ext_debug_marker
            .as_ref()
            .expect("`ext_debug_marker` not loaded")
            .cmd_debug_marker_end_ext;
        let _val = function(command_buffer);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) · Device Command"]
    unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        marker_info: &crate::extensions::ext_debug_marker::DebugMarkerMarkerInfoEXT,
    ) -> () {
        let function = self
            .ext_debug_marker
            .as_ref()
            .expect("`ext_debug_marker` not loaded")
            .cmd_debug_marker_insert_ext;
        let _val = function(command_buffer, marker_info);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectTagInfoEXT.html) · Structure"]
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
impl DebugMarkerObjectTagInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DebugMarkerObjectTagInfoEXTBuilder<'a> {
        DebugMarkerObjectTagInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugMarkerObjectTagInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugMarkerObjectTagInfoEXT")
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
impl Default for DebugMarkerObjectTagInfoEXT {
    fn default() -> DebugMarkerObjectTagInfoEXT {
        DebugMarkerObjectTagInfoEXT {
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
#[derive(Copy, Clone)]
#[doc = "Builder of [`DebugMarkerObjectTagInfoEXT`](struct.DebugMarkerObjectTagInfoEXT.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn object_type(
        mut self,
        object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    ) -> Self {
        self.0.object_type = object_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn object(mut self, object: u64) -> Self {
        self.0.object = object;
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
    pub fn tag(mut self, tag: &'a [std::ffi::c_void]) -> Self {
        self.0.tag_size = tag.len() as _;
        self.0.p_tag = tag.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DebugMarkerObjectTagInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    pub object: u64,
    pub p_object_name: *const std::os::raw::c_char,
}
impl DebugMarkerObjectNameInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DebugMarkerObjectNameInfoEXTBuilder<'a> {
        DebugMarkerObjectNameInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugMarkerObjectNameInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugMarkerObjectNameInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("object_type", &self.object_type)
            .field("object", &self.object)
            .field("p_object_name", &self.p_object_name)
            .finish()
    }
}
impl Default for DebugMarkerObjectNameInfoEXT {
    fn default() -> DebugMarkerObjectNameInfoEXT {
        DebugMarkerObjectNameInfoEXT {
            s_type: crate::vk1_0::StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
            p_next: std::ptr::null(),
            object_type: Default::default(),
            object: Default::default(),
            p_object_name: std::ptr::null(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DebugMarkerObjectNameInfoEXT`](struct.DebugMarkerObjectNameInfoEXT.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn object_type(
        mut self,
        object_type: crate::extensions::ext_debug_report::DebugReportObjectTypeEXT,
    ) -> Self {
        self.0.object_type = object_type;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn object(mut self, object: u64) -> Self {
        self.0.object = object;
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
    pub unsafe fn discard(self) -> DebugMarkerObjectNameInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_marker_name: *const std::os::raw::c_char,
    pub color: [f32; 4],
}
impl DebugMarkerMarkerInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> DebugMarkerMarkerInfoEXTBuilder<'a> {
        DebugMarkerMarkerInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DebugMarkerMarkerInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DebugMarkerMarkerInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_marker_name", &self.p_marker_name)
            .field("color", &self.color)
            .finish()
    }
}
impl Default for DebugMarkerMarkerInfoEXT {
    fn default() -> DebugMarkerMarkerInfoEXT {
        DebugMarkerMarkerInfoEXT {
            s_type: crate::vk1_0::StructureType::DEBUG_MARKER_MARKER_INFO_EXT,
            p_next: std::ptr::null(),
            p_marker_name: std::ptr::null(),
            color: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`DebugMarkerMarkerInfoEXT`](struct.DebugMarkerMarkerInfoEXT.html)"]
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
    #[allow(unused_mut)]
    #[inline]
    pub fn marker_name(mut self, marker_name: &'a std::ffi::CStr) -> Self {
        self.0.p_marker_name = marker_name.as_ptr();
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
    pub unsafe fn discard(self) -> DebugMarkerMarkerInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DebugMarkerMarkerInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
