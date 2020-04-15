# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_sample_locations.html)\n\n## Extends\n- [`DynamicState`](../../vk1_0/struct.DynamicState.html)\n- [`ImageCreateFlagBits`](../../vk1_0/struct.ImageCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_sample_locations");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetSampleLocationsEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_sample_locations_info: *const crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) · Instance Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn ( physical_device : crate :: vk1_0 :: PhysicalDevice , samples : crate :: vk1_0 :: SampleCountFlagBits , p_multisample_properties : * mut crate :: extensions :: ext_sample_locations :: MultisamplePropertiesEXT , ) -> std :: ffi :: c_void ;
#[doc = "Provides Instance Commands for [`ExtSampleLocationsInstanceLoaderExt`](trait.ExtSampleLocationsInstanceLoaderExt.html)"]
pub struct ExtSampleLocationsInstanceCommands {
    pub get_physical_device_multisample_properties_ext:
        PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
}
impl ExtSampleLocationsInstanceCommands {
    #[inline]
    pub fn load(loader: &crate::InstanceLoader) -> Option<ExtSampleLocationsInstanceCommands> {
        unsafe {
            Some(ExtSampleLocationsInstanceCommands {
                get_physical_device_multisample_properties_ext: std::mem::transmute(
                    loader.symbol("vkGetPhysicalDeviceMultisamplePropertiesEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtSampleLocationsInstanceCommands`](struct.ExtSampleLocationsInstanceCommands.html)"]
pub trait ExtSampleLocationsInstanceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) · Instance Command"]
    unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        samples: crate::vk1_0::SampleCountFlagBits,
        multisample_properties: Option<
            crate::extensions::ext_sample_locations::MultisamplePropertiesEXT,
        >,
    ) -> crate::extensions::ext_sample_locations::MultisamplePropertiesEXT;
}
impl ExtSampleLocationsInstanceLoaderExt for crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) · Instance Command"]
    unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        samples: crate::vk1_0::SampleCountFlagBits,
        multisample_properties: Option<
            crate::extensions::ext_sample_locations::MultisamplePropertiesEXT,
        >,
    ) -> crate::extensions::ext_sample_locations::MultisamplePropertiesEXT {
        let function = self
            .ext_sample_locations
            .as_ref()
            .expect("`ext_sample_locations` not loaded")
            .get_physical_device_multisample_properties_ext;
        let mut multisample_properties =
            multisample_properties.unwrap_or_else(|| Default::default());
        let _val = function(physical_device, samples, &mut multisample_properties);
        multisample_properties
    }
}
#[doc = "Provides Device Commands for [`ExtSampleLocationsDeviceLoaderExt`](trait.ExtSampleLocationsDeviceLoaderExt.html)"]
pub struct ExtSampleLocationsDeviceCommands {
    pub cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
}
impl ExtSampleLocationsDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtSampleLocationsDeviceCommands> {
        unsafe {
            Some(ExtSampleLocationsDeviceCommands {
                cmd_set_sample_locations_ext: std::mem::transmute(
                    loader.symbol("vkCmdSetSampleLocationsEXT")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`ExtSampleLocationsDeviceCommands`](struct.ExtSampleLocationsDeviceCommands.html)"]
pub trait ExtSampleLocationsDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetSampleLocationsEXT.html) · Device Command"]
    unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        sample_locations_info: &crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> ();
}
impl ExtSampleLocationsDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetSampleLocationsEXT.html) · Device Command"]
    unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        sample_locations_info: &crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> () {
        let function = self
            .ext_sample_locations
            .as_ref()
            .expect("`ext_sample_locations` not loaded")
            .cmd_set_sample_locations_ext;
        let _val = function(command_buffer, sample_locations_info);
        ()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationsInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleLocationsInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sample_locations_per_pixel: crate::vk1_0::SampleCountFlagBits,
    pub sample_location_grid_size: crate::vk1_0::Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const crate::extensions::ext_sample_locations::SampleLocationEXT,
}
impl SampleLocationsInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableBySampleLocationsInfoEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> SampleLocationsInfoEXTBuilder<'a> {
        SampleLocationsInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SampleLocationsInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SampleLocationsInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sample_locations_per_pixel",
                &self.sample_locations_per_pixel,
            )
            .field("sample_location_grid_size", &self.sample_location_grid_size)
            .field("sample_locations_count", &self.sample_locations_count)
            .field("p_sample_locations", &self.p_sample_locations)
            .finish()
    }
}
impl Default for SampleLocationsInfoEXT {
    fn default() -> SampleLocationsInfoEXT {
        SampleLocationsInfoEXT {
            s_type: crate::vk1_0::StructureType::SAMPLE_LOCATIONS_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_per_pixel: Default::default(),
            sample_location_grid_size: Default::default(),
            sample_locations_count: Default::default(),
            p_sample_locations: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`SampleLocationsInfoEXT::extend`](struct.SampleLocationsInfoEXT.html#method.extend)"]
pub trait ExtendableBySampleLocationsInfoEXT {}
impl ExtendableBySampleLocationsInfoEXT for crate::vk1_0::ImageMemoryBarrier {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SampleLocationsInfoEXT`](struct.SampleLocationsInfoEXT.html)"]
#[repr(transparent)]
pub struct SampleLocationsInfoEXTBuilder<'a>(
    SampleLocationsInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SampleLocationsInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SampleLocationsInfoEXTBuilder<'a> {
        SampleLocationsInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_locations_per_pixel(
        mut self,
        sample_locations_per_pixel: crate::vk1_0::SampleCountFlagBits,
    ) -> Self {
        self.0.sample_locations_per_pixel = sample_locations_per_pixel;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_location_grid_size(
        mut self,
        sample_location_grid_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.sample_location_grid_size = sample_location_grid_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_locations(
        mut self,
        sample_locations: &'a [crate::extensions::ext_sample_locations::SampleLocationEXTBuilder],
    ) -> Self {
        self.0.sample_locations_count = sample_locations.len() as _;
        self.0.p_sample_locations = sample_locations.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SampleLocationsInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SampleLocationsInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SampleLocationsInfoEXTBuilder<'a> {
    type Target = SampleLocationsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SampleLocationsInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}
impl SampleLocationEXT {
    #[inline]
    pub fn builder<'a>(self) -> SampleLocationEXTBuilder<'a> {
        SampleLocationEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SampleLocationEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SampleLocationEXT")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl Default for SampleLocationEXT {
    fn default() -> SampleLocationEXT {
        SampleLocationEXT {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SampleLocationEXT`](struct.SampleLocationEXT.html)"]
#[repr(transparent)]
pub struct SampleLocationEXTBuilder<'a>(SampleLocationEXT, std::marker::PhantomData<&'a ()>);
impl<'a> SampleLocationEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SampleLocationEXTBuilder<'a> {
        SampleLocationEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn x(mut self, x: f32) -> Self {
        self.0.x = x;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn y(mut self, y: f32) -> Self {
        self.0.y = y;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SampleLocationEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SampleLocationEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SampleLocationEXTBuilder<'a> {
    type Target = SampleLocationEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SampleLocationEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultisamplePropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultisamplePropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_sample_location_grid_size: crate::vk1_0::Extent2D,
}
impl MultisamplePropertiesEXT {
    #[inline]
    pub fn builder<'a>(self) -> MultisamplePropertiesEXTBuilder<'a> {
        MultisamplePropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for MultisamplePropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("MultisamplePropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_sample_location_grid_size",
                &self.max_sample_location_grid_size,
            )
            .finish()
    }
}
impl Default for MultisamplePropertiesEXT {
    fn default() -> MultisamplePropertiesEXT {
        MultisamplePropertiesEXT {
            s_type: crate::vk1_0::StructureType::MULTISAMPLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_sample_location_grid_size: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`MultisamplePropertiesEXT`](struct.MultisamplePropertiesEXT.html)"]
#[repr(transparent)]
pub struct MultisamplePropertiesEXTBuilder<'a>(
    MultisamplePropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MultisamplePropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MultisamplePropertiesEXTBuilder<'a> {
        MultisamplePropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_sample_location_grid_size(
        mut self,
        max_sample_location_grid_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.max_sample_location_grid_size = max_sample_location_grid_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> MultisamplePropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for MultisamplePropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for MultisamplePropertiesEXTBuilder<'a> {
    type Target = MultisamplePropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MultisamplePropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleLocationsEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: u32,
    pub sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
}
impl AttachmentSampleLocationsEXT {
    #[inline]
    pub fn builder<'a>(self) -> AttachmentSampleLocationsEXTBuilder<'a> {
        AttachmentSampleLocationsEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for AttachmentSampleLocationsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("AttachmentSampleLocationsEXT")
            .field("attachment_index", &self.attachment_index)
            .field("sample_locations_info", &self.sample_locations_info)
            .finish()
    }
}
impl Default for AttachmentSampleLocationsEXT {
    fn default() -> AttachmentSampleLocationsEXT {
        AttachmentSampleLocationsEXT {
            attachment_index: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`AttachmentSampleLocationsEXT`](struct.AttachmentSampleLocationsEXT.html)"]
#[repr(transparent)]
pub struct AttachmentSampleLocationsEXTBuilder<'a>(
    AttachmentSampleLocationsEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AttachmentSampleLocationsEXTBuilder<'a> {
    #[inline]
    pub fn new() -> AttachmentSampleLocationsEXTBuilder<'a> {
        AttachmentSampleLocationsEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn attachment_index(mut self, attachment_index: u32) -> Self {
        self.0.attachment_index = attachment_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> Self {
        self.0.sample_locations_info = sample_locations_info;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> AttachmentSampleLocationsEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for AttachmentSampleLocationsEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for AttachmentSampleLocationsEXTBuilder<'a> {
    type Target = AttachmentSampleLocationsEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentSampleLocationsEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassSampleLocationsEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: u32,
    pub sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
}
impl SubpassSampleLocationsEXT {
    #[inline]
    pub fn builder<'a>(self) -> SubpassSampleLocationsEXTBuilder<'a> {
        SubpassSampleLocationsEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for SubpassSampleLocationsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("SubpassSampleLocationsEXT")
            .field("subpass_index", &self.subpass_index)
            .field("sample_locations_info", &self.sample_locations_info)
            .finish()
    }
}
impl Default for SubpassSampleLocationsEXT {
    fn default() -> SubpassSampleLocationsEXT {
        SubpassSampleLocationsEXT {
            subpass_index: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`SubpassSampleLocationsEXT`](struct.SubpassSampleLocationsEXT.html)"]
#[repr(transparent)]
pub struct SubpassSampleLocationsEXTBuilder<'a>(
    SubpassSampleLocationsEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SubpassSampleLocationsEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassSampleLocationsEXTBuilder<'a> {
        SubpassSampleLocationsEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn subpass_index(mut self, subpass_index: u32) -> Self {
        self.0.subpass_index = subpass_index;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> Self {
        self.0.sample_locations_info = sample_locations_info;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> SubpassSampleLocationsEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for SubpassSampleLocationsEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for SubpassSampleLocationsEXTBuilder<'a> {
    type Target = SubpassSampleLocationsEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassSampleLocationsEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations:
        *const crate::extensions::ext_sample_locations::AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations:
        *const crate::extensions::ext_sample_locations::SubpassSampleLocationsEXT,
}
impl RenderPassSampleLocationsBeginInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByRenderPassSampleLocationsBeginInfoEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
        RenderPassSampleLocationsBeginInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RenderPassSampleLocationsBeginInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderPassSampleLocationsBeginInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "attachment_initial_sample_locations_count",
                &self.attachment_initial_sample_locations_count,
            )
            .field(
                "p_attachment_initial_sample_locations",
                &self.p_attachment_initial_sample_locations,
            )
            .field(
                "post_subpass_sample_locations_count",
                &self.post_subpass_sample_locations_count,
            )
            .field(
                "p_post_subpass_sample_locations",
                &self.p_post_subpass_sample_locations,
            )
            .finish()
    }
}
impl Default for RenderPassSampleLocationsBeginInfoEXT {
    fn default() -> RenderPassSampleLocationsBeginInfoEXT {
        RenderPassSampleLocationsBeginInfoEXT {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
            p_next: std::ptr::null(),
            attachment_initial_sample_locations_count: Default::default(),
            p_attachment_initial_sample_locations: std::ptr::null(),
            post_subpass_sample_locations_count: Default::default(),
            p_post_subpass_sample_locations: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`RenderPassSampleLocationsBeginInfoEXT::extend`](struct.RenderPassSampleLocationsBeginInfoEXT.html#method.extend)"]
pub trait ExtendableByRenderPassSampleLocationsBeginInfoEXT {}
impl ExtendableByRenderPassSampleLocationsBeginInfoEXT for crate::vk1_0::RenderPassBeginInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RenderPassSampleLocationsBeginInfoEXT`](struct.RenderPassSampleLocationsBeginInfoEXT.html)"]
#[repr(transparent)]
pub struct RenderPassSampleLocationsBeginInfoEXTBuilder<'a>(
    RenderPassSampleLocationsBeginInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
        RenderPassSampleLocationsBeginInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn attachment_initial_sample_locations(
        mut self,
        attachment_initial_sample_locations : &'a [ crate :: extensions :: ext_sample_locations :: AttachmentSampleLocationsEXTBuilder ],
    ) -> Self {
        self.0.attachment_initial_sample_locations_count =
            attachment_initial_sample_locations.len() as _;
        self.0.p_attachment_initial_sample_locations =
            attachment_initial_sample_locations.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn post_subpass_sample_locations(
        mut self,
        post_subpass_sample_locations : &'a [ crate :: extensions :: ext_sample_locations :: SubpassSampleLocationsEXTBuilder ],
    ) -> Self {
        self.0.post_subpass_sample_locations_count = post_subpass_sample_locations.len() as _;
        self.0.p_post_subpass_sample_locations = post_subpass_sample_locations.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RenderPassSampleLocationsBeginInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    type Target = RenderPassSampleLocationsBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sample_locations_enable: crate::vk1_0::Bool32,
    pub sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
}
impl PipelineSampleLocationsStateCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineSampleLocationsStateCreateInfoEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
        PipelineSampleLocationsStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineSampleLocationsStateCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineSampleLocationsStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sample_locations_enable",
                &(self.sample_locations_enable != 0),
            )
            .field("sample_locations_info", &self.sample_locations_info)
            .finish()
    }
}
impl Default for PipelineSampleLocationsStateCreateInfoEXT {
    fn default() -> PipelineSampleLocationsStateCreateInfoEXT {
        PipelineSampleLocationsStateCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_enable: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
#[doc = "Used by [`PipelineSampleLocationsStateCreateInfoEXT::extend`](struct.PipelineSampleLocationsStateCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByPipelineSampleLocationsStateCreateInfoEXT {}
impl ExtendableByPipelineSampleLocationsStateCreateInfoEXT
    for crate::vk1_0::PipelineMultisampleStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineSampleLocationsStateCreateInfoEXT`](struct.PipelineSampleLocationsStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineSampleLocationsStateCreateInfoEXTBuilder<'a>(
    PipelineSampleLocationsStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
        PipelineSampleLocationsStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_locations_enable(mut self, sample_locations_enable: bool) -> Self {
        self.0.sample_locations_enable = sample_locations_enable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> Self {
        self.0.sample_locations_info = sample_locations_info;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineSampleLocationsStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineSampleLocationsStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub sample_location_sample_counts: crate::vk1_0::SampleCountFlags,
    pub max_sample_location_grid_size: crate::vk1_0::Extent2D,
    pub sample_location_coordinate_range: [f32; 2],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: crate::vk1_0::Bool32,
}
impl PhysicalDeviceSampleLocationsPropertiesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceSampleLocationsPropertiesEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
        PhysicalDeviceSampleLocationsPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceSampleLocationsPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sample_location_sample_counts",
                &self.sample_location_sample_counts,
            )
            .field(
                "max_sample_location_grid_size",
                &self.max_sample_location_grid_size,
            )
            .field(
                "sample_location_coordinate_range",
                &self.sample_location_coordinate_range,
            )
            .field(
                "sample_location_sub_pixel_bits",
                &self.sample_location_sub_pixel_bits,
            )
            .field(
                "variable_sample_locations",
                &(self.variable_sample_locations != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn default() -> PhysicalDeviceSampleLocationsPropertiesEXT {
        PhysicalDeviceSampleLocationsPropertiesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            sample_location_sample_counts: Default::default(),
            max_sample_location_grid_size: Default::default(),
            sample_location_coordinate_range: Default::default(),
            sample_location_sub_pixel_bits: Default::default(),
            variable_sample_locations: Default::default(),
        }
    }
}
#[doc = "Used by [`PhysicalDeviceSampleLocationsPropertiesEXT::extend`](struct.PhysicalDeviceSampleLocationsPropertiesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceSampleLocationsPropertiesEXT {}
impl ExtendableByPhysicalDeviceSampleLocationsPropertiesEXT
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceSampleLocationsPropertiesEXT`](struct.PhysicalDeviceSampleLocationsPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a>(
    PhysicalDeviceSampleLocationsPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
        PhysicalDeviceSampleLocationsPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_location_sample_counts(
        mut self,
        sample_location_sample_counts: crate::vk1_0::SampleCountFlags,
    ) -> Self {
        self.0.sample_location_sample_counts = sample_location_sample_counts;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_sample_location_grid_size(
        mut self,
        max_sample_location_grid_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.max_sample_location_grid_size = max_sample_location_grid_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_location_coordinate_range(
        mut self,
        sample_location_coordinate_range: [f32; 2],
    ) -> Self {
        self.0.sample_location_coordinate_range = sample_location_coordinate_range;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn sample_location_sub_pixel_bits(mut self, sample_location_sub_pixel_bits: u32) -> Self {
        self.0.sample_location_sub_pixel_bits = sample_location_sub_pixel_bits;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn variable_sample_locations(mut self, variable_sample_locations: bool) -> Self {
        self.0.variable_sample_locations = variable_sample_locations as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceSampleLocationsPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceSampleLocationsPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
