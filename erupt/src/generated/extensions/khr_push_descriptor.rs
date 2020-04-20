# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_push_descriptor.html)\n\n## Extends\n- [`DescriptorSetLayoutCreateFlagBits`](../../vk1_0/struct.DescriptorSetLayoutCreateFlagBits.html)\n- [`DescriptorUpdateTemplateType`](../../vk1_1/struct.DescriptorUpdateTemplateType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_push_descriptor");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    layout: crate::vk1_0::PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const crate::vk1_0::WriteDescriptorSet,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
    layout: crate::vk1_0::PipelineLayout,
    set: u32,
    p_data: *const std::ffi::c_void,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`KhrPushDescriptorDeviceLoaderExt`](trait.KhrPushDescriptorDeviceLoaderExt.html)"]
pub struct KhrPushDescriptorDeviceCommands {
    pub cmd_push_descriptor_set_khr: PFN_vkCmdPushDescriptorSetKHR,
    pub cmd_push_descriptor_set_with_template_khr: PFN_vkCmdPushDescriptorSetWithTemplateKHR,
}
impl KhrPushDescriptorDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrPushDescriptorDeviceCommands> {
        unsafe {
            Some(KhrPushDescriptorDeviceCommands {
                cmd_push_descriptor_set_khr: std::mem::transmute(
                    loader.symbol("vkCmdPushDescriptorSetKHR")?,
                ),
                cmd_push_descriptor_set_with_template_khr: std::mem::transmute(
                    loader.symbol("vkCmdPushDescriptorSetWithTemplateKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrPushDescriptorDeviceCommands`](struct.KhrPushDescriptorDeviceCommands.html)"]
pub trait KhrPushDescriptorDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetKHR.html) · Device Command"]
    unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
        layout: crate::vk1_0::PipelineLayout,
        set: u32,
        descriptor_writes: &[crate::vk1_0::WriteDescriptorSetBuilder],
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Device Command"]
    unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        layout: crate::vk1_0::PipelineLayout,
        set: u32,
        data: *const std::ffi::c_void,
    ) -> ();
}
impl KhrPushDescriptorDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetKHR.html) · Device Command"]
    unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
        layout: crate::vk1_0::PipelineLayout,
        set: u32,
        descriptor_writes: &[crate::vk1_0::WriteDescriptorSetBuilder],
    ) -> () {
        let function = self
            .khr_push_descriptor
            .as_ref()
            .expect("`khr_push_descriptor` not loaded")
            .cmd_push_descriptor_set_khr;
        let descriptor_write_count = descriptor_writes.len() as _;
        let _val = function(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_write_count,
            descriptor_writes.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Device Command"]
    unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate,
        layout: crate::vk1_0::PipelineLayout,
        set: u32,
        data: *const std::ffi::c_void,
    ) -> () {
        let function = self
            .khr_push_descriptor
            .as_ref()
            .expect("`khr_push_descriptor` not loaded")
            .cmd_push_descriptor_set_with_template_khr;
        let _val = function(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            data,
        );
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_push_descriptors: u32,
}
impl PhysicalDevicePushDescriptorPropertiesKHR {
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
    pub fn builder<'a>(self) -> PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
        PhysicalDevicePushDescriptorPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePushDescriptorPropertiesKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePushDescriptorPropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("max_push_descriptors", &self.max_push_descriptors)
            .finish()
    }
}
impl Default for PhysicalDevicePushDescriptorPropertiesKHR {
    fn default() -> PhysicalDevicePushDescriptorPropertiesKHR {
        PhysicalDevicePushDescriptorPropertiesKHR {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            max_push_descriptors: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDevicePushDescriptorPropertiesKHR>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) · Builder of [`PhysicalDevicePushDescriptorPropertiesKHR`](struct.PhysicalDevicePushDescriptorPropertiesKHR.html)"]
#[repr(transparent)]
pub struct PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a>(
    PhysicalDevicePushDescriptorPropertiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
        PhysicalDevicePushDescriptorPropertiesKHRBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_push_descriptors(mut self, max_push_descriptors: u32) -> Self {
        self.0.max_push_descriptors = max_push_descriptors;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePushDescriptorPropertiesKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    type Target = PhysicalDevicePushDescriptorPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
