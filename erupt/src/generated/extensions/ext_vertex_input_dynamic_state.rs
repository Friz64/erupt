#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_vertex_input_dynamic_state");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_VERTEX_INPUT_EXT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetVertexInputEXT");
#[doc = "Provided by [`crate::extensions::ext_vertex_input_dynamic_state`]"]
impl crate::vk1_0::DynamicState {
    pub const VERTEX_INPUT_EXT: Self = Self(1000352000);
}
#[doc = "Provided by [`crate::extensions::ext_vertex_input_dynamic_state`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000352000);
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetVertexInputEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, vertex_binding_description_count: u32, p_vertex_binding_descriptions: *const crate::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXT, vertex_attribute_description_count: u32, p_vertex_attribute_descriptions: *const crate::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXT) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceVertexInputDynamicStateFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceVertexInputDynamicStateFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub vertex_input_dynamic_state: crate::vk1_0::Bool32,
}
impl PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT;
}
impl Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), vertex_input_dynamic_state: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceVertexInputDynamicStateFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("vertex_input_dynamic_state", &(self.vertex_input_dynamic_state != 0)).finish()
    }
}
impl PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
        PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT.html) · Builder of [`PhysicalDeviceVertexInputDynamicStateFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a>(PhysicalDeviceVertexInputDynamicStateFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
        PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn vertex_input_dynamic_state(mut self, vertex_input_dynamic_state: bool) -> Self {
        self.0.vertex_input_dynamic_state = vertex_input_dynamic_state as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceVertexInputDynamicStateFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceVertexInputDynamicStateFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceVertexInputDynamicStateFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDescription2EXT.html) · Structure"]
#[doc(alias = "VkVertexInputBindingDescription2EXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexInputBindingDescription2EXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub binding: u32,
    pub stride: u32,
    pub input_rate: crate::vk1_0::VertexInputRate,
    pub divisor: u32,
}
impl VertexInputBindingDescription2EXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT;
}
impl Default for VertexInputBindingDescription2EXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), binding: Default::default(), stride: Default::default(), input_rate: Default::default(), divisor: Default::default() }
    }
}
impl std::fmt::Debug for VertexInputBindingDescription2EXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VertexInputBindingDescription2EXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("binding", &self.binding).field("stride", &self.stride).field("input_rate", &self.input_rate).field("divisor", &self.divisor).finish()
    }
}
impl VertexInputBindingDescription2EXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VertexInputBindingDescription2EXTBuilder<'a> {
        VertexInputBindingDescription2EXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDescription2EXT.html) · Builder of [`VertexInputBindingDescription2EXT`]"]
#[repr(transparent)]
pub struct VertexInputBindingDescription2EXTBuilder<'a>(VertexInputBindingDescription2EXT, std::marker::PhantomData<&'a ()>);
impl<'a> VertexInputBindingDescription2EXTBuilder<'a> {
    #[inline]
    pub fn new() -> VertexInputBindingDescription2EXTBuilder<'a> {
        VertexInputBindingDescription2EXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn binding(mut self, binding: u32) -> Self {
        self.0.binding = binding as _;
        self
    }
    #[inline]
    pub fn stride(mut self, stride: u32) -> Self {
        self.0.stride = stride as _;
        self
    }
    #[inline]
    pub fn input_rate(mut self, input_rate: crate::vk1_0::VertexInputRate) -> Self {
        self.0.input_rate = input_rate as _;
        self
    }
    #[inline]
    pub fn divisor(mut self, divisor: u32) -> Self {
        self.0.divisor = divisor as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VertexInputBindingDescription2EXT {
        self.0
    }
}
impl<'a> std::default::Default for VertexInputBindingDescription2EXTBuilder<'a> {
    fn default() -> VertexInputBindingDescription2EXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VertexInputBindingDescription2EXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VertexInputBindingDescription2EXTBuilder<'a> {
    type Target = VertexInputBindingDescription2EXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VertexInputBindingDescription2EXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputAttributeDescription2EXT.html) · Structure"]
#[doc(alias = "VkVertexInputAttributeDescription2EXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexInputAttributeDescription2EXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub location: u32,
    pub binding: u32,
    pub format: crate::vk1_0::Format,
    pub offset: u32,
}
impl VertexInputAttributeDescription2EXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT;
}
impl Default for VertexInputAttributeDescription2EXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), location: Default::default(), binding: Default::default(), format: Default::default(), offset: Default::default() }
    }
}
impl std::fmt::Debug for VertexInputAttributeDescription2EXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VertexInputAttributeDescription2EXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("location", &self.location).field("binding", &self.binding).field("format", &self.format).field("offset", &self.offset).finish()
    }
}
impl VertexInputAttributeDescription2EXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VertexInputAttributeDescription2EXTBuilder<'a> {
        VertexInputAttributeDescription2EXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputAttributeDescription2EXT.html) · Builder of [`VertexInputAttributeDescription2EXT`]"]
#[repr(transparent)]
pub struct VertexInputAttributeDescription2EXTBuilder<'a>(VertexInputAttributeDescription2EXT, std::marker::PhantomData<&'a ()>);
impl<'a> VertexInputAttributeDescription2EXTBuilder<'a> {
    #[inline]
    pub fn new() -> VertexInputAttributeDescription2EXTBuilder<'a> {
        VertexInputAttributeDescription2EXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn location(mut self, location: u32) -> Self {
        self.0.location = location as _;
        self
    }
    #[inline]
    pub fn binding(mut self, binding: u32) -> Self {
        self.0.binding = binding as _;
        self
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VertexInputAttributeDescription2EXT {
        self.0
    }
}
impl<'a> std::default::Default for VertexInputAttributeDescription2EXTBuilder<'a> {
    fn default() -> VertexInputAttributeDescription2EXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VertexInputAttributeDescription2EXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VertexInputAttributeDescription2EXTBuilder<'a> {
    type Target = VertexInputAttributeDescription2EXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VertexInputAttributeDescription2EXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::ext_vertex_input_dynamic_state`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetVertexInputEXT.html) · Function"]
    #[doc(alias = "vkCmdSetVertexInputEXT")]
    pub unsafe fn cmd_set_vertex_input_ext(&self, command_buffer: crate::vk1_0::CommandBuffer, vertex_binding_descriptions: &[crate::extensions::ext_vertex_input_dynamic_state::VertexInputBindingDescription2EXTBuilder], vertex_attribute_descriptions: &[crate::extensions::ext_vertex_input_dynamic_state::VertexInputAttributeDescription2EXTBuilder]) -> () {
        let _function = self.cmd_set_vertex_input_ext.expect(crate::NOT_LOADED_MESSAGE);
        let vertex_binding_description_count = vertex_binding_descriptions.len();
        let vertex_attribute_description_count = vertex_attribute_descriptions.len();
        let _return = _function(command_buffer as _, vertex_binding_description_count as _, vertex_binding_descriptions.as_ptr() as _, vertex_attribute_description_count as _, vertex_attribute_descriptions.as_ptr() as _);
        ()
    }
}
