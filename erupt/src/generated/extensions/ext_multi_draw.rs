#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_MULTI_DRAW_SPEC_VERSION")]
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_MULTI_DRAW_EXTENSION_NAME")]
pub const EXT_MULTI_DRAW_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_multi_draw");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_MULTI_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawMultiEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_MULTI_INDEXED_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawMultiIndexedEXT");
#[doc = "Provided by [`crate::extensions::ext_multi_draw`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMultiEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, draw_count: u32, p_vertex_info: *const crate::extensions::ext_multi_draw::MultiDrawInfoEXT, instance_count: u32, first_instance: u32, stride: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMultiIndexedEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, draw_count: u32, p_index_info: *const crate::extensions::ext_multi_draw::MultiDrawIndexedInfoEXT, instance_count: u32, first_instance: u32, stride: u32, p_vertex_offset: *const i32) -> ();
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceMultiDrawFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFromConst<'a, PhysicalDeviceMultiDrawFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultiDrawInfoEXT.html) · Structure"]
#[doc(alias = "VkMultiDrawInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultiDrawInfoEXT {
    pub first_vertex: u32,
    pub vertex_count: u32,
}
impl Default for MultiDrawInfoEXT {
    fn default() -> Self {
        Self { first_vertex: Default::default(), vertex_count: Default::default() }
    }
}
impl std::fmt::Debug for MultiDrawInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MultiDrawInfoEXT").field("first_vertex", &self.first_vertex).field("vertex_count", &self.vertex_count).finish()
    }
}
impl MultiDrawInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> MultiDrawInfoEXTBuilder<'a> {
        MultiDrawInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultiDrawInfoEXT.html) · Builder of [`MultiDrawInfoEXT`]"]
#[repr(transparent)]
pub struct MultiDrawInfoEXTBuilder<'a>(MultiDrawInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> MultiDrawInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MultiDrawInfoEXTBuilder<'a> {
        MultiDrawInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn first_vertex(mut self, first_vertex: u32) -> Self {
        self.0.first_vertex = first_vertex as _;
        self
    }
    #[inline]
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.0.vertex_count = vertex_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MultiDrawInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for MultiDrawInfoEXTBuilder<'a> {
    fn default() -> MultiDrawInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MultiDrawInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MultiDrawInfoEXTBuilder<'a> {
    type Target = MultiDrawInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MultiDrawInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultiDrawIndexedInfoEXT.html) · Structure"]
#[doc(alias = "VkMultiDrawIndexedInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultiDrawIndexedInfoEXT {
    pub first_index: u32,
    pub index_count: u32,
    pub vertex_offset: i32,
}
impl Default for MultiDrawIndexedInfoEXT {
    fn default() -> Self {
        Self { first_index: Default::default(), index_count: Default::default(), vertex_offset: Default::default() }
    }
}
impl std::fmt::Debug for MultiDrawIndexedInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MultiDrawIndexedInfoEXT").field("first_index", &self.first_index).field("index_count", &self.index_count).field("vertex_offset", &self.vertex_offset).finish()
    }
}
impl MultiDrawIndexedInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> MultiDrawIndexedInfoEXTBuilder<'a> {
        MultiDrawIndexedInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultiDrawIndexedInfoEXT.html) · Builder of [`MultiDrawIndexedInfoEXT`]"]
#[repr(transparent)]
pub struct MultiDrawIndexedInfoEXTBuilder<'a>(MultiDrawIndexedInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> MultiDrawIndexedInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MultiDrawIndexedInfoEXTBuilder<'a> {
        MultiDrawIndexedInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn first_index(mut self, first_index: u32) -> Self {
        self.0.first_index = first_index as _;
        self
    }
    #[inline]
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.0.index_count = index_count as _;
        self
    }
    #[inline]
    pub fn vertex_offset(mut self, vertex_offset: i32) -> Self {
        self.0.vertex_offset = vertex_offset as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MultiDrawIndexedInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for MultiDrawIndexedInfoEXTBuilder<'a> {
    fn default() -> MultiDrawIndexedInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MultiDrawIndexedInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MultiDrawIndexedInfoEXTBuilder<'a> {
    type Target = MultiDrawIndexedInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MultiDrawIndexedInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMultiDrawPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_multi_draw_count: u32,
}
impl Default for PhysicalDeviceMultiDrawPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT, p_next: std::ptr::null_mut(), max_multi_draw_count: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiDrawPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMultiDrawPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_multi_draw_count", &self.max_multi_draw_count).finish()
    }
}
impl PhysicalDeviceMultiDrawPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
        PhysicalDeviceMultiDrawPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiDrawPropertiesEXT.html) · Builder of [`PhysicalDeviceMultiDrawPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a>(PhysicalDeviceMultiDrawPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
        PhysicalDeviceMultiDrawPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_multi_draw_count(mut self, max_multi_draw_count: u32) -> Self {
        self.0.max_multi_draw_count = max_multi_draw_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMultiDrawPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceMultiDrawPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMultiDrawPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceMultiDrawFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceMultiDrawFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceMultiDrawPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDeviceMultiDrawPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMultiDrawFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMultiDrawFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub multi_draw: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceMultiDrawFeaturesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT, p_next: std::ptr::null_mut(), multi_draw: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceMultiDrawFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMultiDrawFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("multi_draw", &(self.multi_draw != 0)).finish()
    }
}
impl PhysicalDeviceMultiDrawFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
        PhysicalDeviceMultiDrawFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMultiDrawFeaturesEXT.html) · Builder of [`PhysicalDeviceMultiDrawFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a>(PhysicalDeviceMultiDrawFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
        PhysicalDeviceMultiDrawFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn multi_draw(mut self, multi_draw: bool) -> Self {
        self.0.multi_draw = multi_draw as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMultiDrawFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceMultiDrawFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMultiDrawFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_multi_draw`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMultiEXT.html) · Function"]
    #[doc(alias = "vkCmdDrawMultiEXT")]
    pub unsafe fn cmd_draw_multi_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, vertex_info: &[crate::extensions::ext_multi_draw::MultiDrawInfoEXTBuilder], instance_count: u32, first_instance: u32, stride: u32) -> () {
        let _function = self.cmd_draw_multi_ext.expect(crate::NOT_LOADED_MESSAGE);
        let draw_count = vertex_info.len();
        let _return = _function(command_buffer as _, draw_count as _, vertex_info.as_ptr() as _, instance_count as _, first_instance as _, stride as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawMultiIndexedEXT.html) · Function"]
    #[doc(alias = "vkCmdDrawMultiIndexedEXT")]
    pub unsafe fn cmd_draw_multi_indexed_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, index_info: &[crate::extensions::ext_multi_draw::MultiDrawIndexedInfoEXTBuilder], instance_count: u32, first_instance: u32, stride: u32, vertex_offset: Option<&i32>) -> () {
        let _function = self.cmd_draw_multi_indexed_ext.expect(crate::NOT_LOADED_MESSAGE);
        let draw_count = index_info.len();
        let _return = _function(
            command_buffer as _,
            draw_count as _,
            index_info.as_ptr() as _,
            instance_count as _,
            first_instance as _,
            stride as _,
            match vertex_offset {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
}
