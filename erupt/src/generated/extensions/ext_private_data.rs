# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_private_data.html)\n\n## Extends\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PRIVATE_DATA_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_private_data");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePrivateDataSlotEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePrivateDataSlotEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_create_info: *const crate::extensions::ext_private_data::PrivateDataSlotCreateInfoEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_private_data_slot: *mut crate::extensions::ext_private_data::PrivateDataSlotEXT,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPrivateDataSlotEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPrivateDataSlotEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetPrivateDataEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetPrivateDataEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    object_type: crate::vk1_0::ObjectType,
    object_handle: u64,
    private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
    data: u64,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPrivateDataEXT.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPrivateDataEXT = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    object_type: crate::vk1_0::ObjectType,
    object_handle: u64,
    private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
    p_data: *mut u64,
) -> std::ffi::c_void;
#[doc = "Provides Device Commands for [`ExtPrivateDataDeviceLoaderExt`](trait.ExtPrivateDataDeviceLoaderExt.html)"]
pub struct ExtPrivateDataDeviceCommands {
    pub create_private_data_slot_ext: Option<PFN_vkCreatePrivateDataSlotEXT>,
    pub destroy_private_data_slot_ext: Option<PFN_vkDestroyPrivateDataSlotEXT>,
    pub set_private_data_ext: Option<PFN_vkSetPrivateDataEXT>,
    pub get_private_data_ext: Option<PFN_vkGetPrivateDataEXT>,
}
impl ExtPrivateDataDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<ExtPrivateDataDeviceCommands> {
        unsafe {
            let mut success = false;
            let table = ExtPrivateDataDeviceCommands {
                create_private_data_slot_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkCreatePrivateDataSlotEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                destroy_private_data_slot_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkDestroyPrivateDataSlotEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                set_private_data_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkSetPrivateDataEXT");
                    success |= symbol.is_some();
                    symbol
                }),
                get_private_data_ext: std::mem::transmute({
                    let symbol = loader.symbol("vkGetPrivateDataEXT");
                    success |= symbol.is_some();
                    symbol
                }),
            };
            if success {
                Some(table)
            } else {
                None
            }
        }
    }
}
#[inline]
fn device_commands(loader: &crate::DeviceLoader) -> &ExtPrivateDataDeviceCommands {
    loader
        .ext_private_data
        .as_ref()
        .expect("`ext_private_data` not loaded")
}
#[doc = "Provides high level command wrappers for [`ExtPrivateDataDeviceCommands`](struct.ExtPrivateDataDeviceCommands.html)"]
pub trait ExtPrivateDataDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePrivateDataSlotEXT.html) · Device Command"]
    unsafe fn create_private_data_slot_ext(
        &self,
        create_info: &crate::extensions::ext_private_data::PrivateDataSlotCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        private_data_slot: Option<crate::extensions::ext_private_data::PrivateDataSlotEXT>,
    ) -> crate::utils::VulkanResult<crate::extensions::ext_private_data::PrivateDataSlotEXT>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPrivateDataSlotEXT.html) · Device Command"]
    unsafe fn destroy_private_data_slot_ext(
        &self,
        private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetPrivateDataEXT.html) · Device Command"]
    unsafe fn set_private_data_ext(
        &self,
        object_type: crate::vk1_0::ObjectType,
        object_handle: u64,
        private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
        data: u64,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPrivateDataEXT.html) · Device Command"]
    unsafe fn get_private_data_ext(
        &self,
        object_type: crate::vk1_0::ObjectType,
        object_handle: u64,
        private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
        data: Option<u64>,
    ) -> u64;
}
impl ExtPrivateDataDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePrivateDataSlotEXT.html) · Device Command"]
    unsafe fn create_private_data_slot_ext(
        &self,
        create_info: &crate::extensions::ext_private_data::PrivateDataSlotCreateInfoEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        private_data_slot: Option<crate::extensions::ext_private_data::PrivateDataSlotEXT>,
    ) -> crate::utils::VulkanResult<crate::extensions::ext_private_data::PrivateDataSlotEXT> {
        let function = device_commands(self)
            .create_private_data_slot_ext
            .as_ref()
            .expect("`create_private_data_slot_ext` not available");
        let mut private_data_slot = private_data_slot.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            create_info,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut private_data_slot,
        );
        crate::utils::VulkanResult::new(_val, private_data_slot)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPrivateDataSlotEXT.html) · Device Command"]
    unsafe fn destroy_private_data_slot_ext(
        &self,
        private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = device_commands(self)
            .destroy_private_data_slot_ext
            .as_ref()
            .expect("`destroy_private_data_slot_ext` not available");
        let _val = function(
            self.handle,
            private_data_slot,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetPrivateDataEXT.html) · Device Command"]
    unsafe fn set_private_data_ext(
        &self,
        object_type: crate::vk1_0::ObjectType,
        object_handle: u64,
        private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
        data: u64,
    ) -> crate::utils::VulkanResult<()> {
        let function = device_commands(self)
            .set_private_data_ext
            .as_ref()
            .expect("`set_private_data_ext` not available");
        let _val = function(
            self.handle,
            object_type,
            object_handle,
            private_data_slot,
            data,
        );
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPrivateDataEXT.html) · Device Command"]
    unsafe fn get_private_data_ext(
        &self,
        object_type: crate::vk1_0::ObjectType,
        object_handle: u64,
        private_data_slot: crate::extensions::ext_private_data::PrivateDataSlotEXT,
        data: Option<u64>,
    ) -> u64 {
        let function = device_commands(self)
            .get_private_data_ext
            .as_ref()
            .expect("`get_private_data_ext` not available");
        let mut data = data.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            object_type,
            object_handle,
            private_data_slot,
            &mut data,
        );
        data
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PrivateDataSlotCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_private_data::PrivateDataSlotCreateFlagsEXT,
}
impl PrivateDataSlotCreateInfoEXT {
    #[inline]
    pub fn builder<'a>(self) -> PrivateDataSlotCreateInfoEXTBuilder<'a> {
        PrivateDataSlotCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PrivateDataSlotCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PrivateDataSlotCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .finish()
    }
}
impl Default for PrivateDataSlotCreateInfoEXT {
    fn default() -> PrivateDataSlotCreateInfoEXT {
        PrivateDataSlotCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::PRIVATE_DATA_SLOT_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateInfoEXT.html) · Builder of [`PrivateDataSlotCreateInfoEXT`](struct.PrivateDataSlotCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PrivateDataSlotCreateInfoEXTBuilder<'a>(
    PrivateDataSlotCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PrivateDataSlotCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PrivateDataSlotCreateInfoEXTBuilder<'a> {
        PrivateDataSlotCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags: crate::extensions::ext_private_data::PrivateDataSlotCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PrivateDataSlotCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PrivateDataSlotCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PrivateDataSlotCreateInfoEXTBuilder<'a> {
    type Target = PrivateDataSlotCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PrivateDataSlotCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PrivateDataSlotCreateFlagsEXT`](struct.PrivateDataSlotCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PrivateDataSlotCreateFlagBitsEXT(pub u32);
impl PrivateDataSlotCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PrivateDataSlotCreateFlagsEXT {
        PrivateDataSlotCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
#[doc = "[Part of `extensions::ext_private_data`](../../extensions/ext_private_data/index.html)"]
impl PrivateDataSlotCreateFlagBitsEXT {}
impl std::fmt::Debug for PrivateDataSlotCreateFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "(unknown)",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotCreateFlagsEXT.html) · Flags of [`PrivateDataSlotCreateFlagBitsEXT`](struct.PrivateDataSlotCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PrivateDataSlotCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
crate :: non_dispatchable_handle ! ( PrivateDataSlotEXT , PRIVATE_DATA_SLOT_EXT , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrivateDataSlotEXT.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePrivateDataFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePrivateDataFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub private_data: crate::vk1_0::Bool32,
}
impl PhysicalDevicePrivateDataFeaturesEXT {
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
    pub fn builder<'a>(self) -> PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
        PhysicalDevicePrivateDataFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePrivateDataFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePrivateDataFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("private_data", &(self.private_data != 0))
            .finish()
    }
}
impl Default for PhysicalDevicePrivateDataFeaturesEXT {
    fn default() -> PhysicalDevicePrivateDataFeaturesEXT {
        PhysicalDevicePrivateDataFeaturesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            private_data: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDevicePrivateDataFeaturesEXT>
    for crate::vk1_1::PhysicalDeviceFeatures2
{
}
impl crate::ExtendableBy<PhysicalDevicePrivateDataFeaturesEXT> for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePrivateDataFeaturesEXT.html) · Builder of [`PhysicalDevicePrivateDataFeaturesEXT`](struct.PhysicalDevicePrivateDataFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDevicePrivateDataFeaturesEXTBuilder<'a>(
    PhysicalDevicePrivateDataFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
        PhysicalDevicePrivateDataFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn private_data(mut self, private_data: bool) -> Self {
        self.0.private_data = private_data as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePrivateDataFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    type Target = PhysicalDevicePrivateDataFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePrivateDataFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevicePrivateDataCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DevicePrivateDataCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub private_data_slot_request_count: u32,
}
impl DevicePrivateDataCreateInfoEXT {
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
    pub fn builder<'a>(self) -> DevicePrivateDataCreateInfoEXTBuilder<'a> {
        DevicePrivateDataCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DevicePrivateDataCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DevicePrivateDataCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "private_data_slot_request_count",
                &self.private_data_slot_request_count,
            )
            .finish()
    }
}
impl Default for DevicePrivateDataCreateInfoEXT {
    fn default() -> DevicePrivateDataCreateInfoEXT {
        DevicePrivateDataCreateInfoEXT {
            s_type: crate::vk1_0::StructureType::DEVICE_PRIVATE_DATA_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            private_data_slot_request_count: Default::default(),
        }
    }
}
impl crate::ExtendableBy<DevicePrivateDataCreateInfoEXT> for crate::vk1_0::DeviceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevicePrivateDataCreateInfoEXT.html) · Builder of [`DevicePrivateDataCreateInfoEXT`](struct.DevicePrivateDataCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct DevicePrivateDataCreateInfoEXTBuilder<'a>(
    DevicePrivateDataCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DevicePrivateDataCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> DevicePrivateDataCreateInfoEXTBuilder<'a> {
        DevicePrivateDataCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn private_data_slot_request_count(mut self, private_data_slot_request_count: u32) -> Self {
        self.0.private_data_slot_request_count = private_data_slot_request_count;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DevicePrivateDataCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for DevicePrivateDataCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DevicePrivateDataCreateInfoEXTBuilder<'a> {
    type Target = DevicePrivateDataCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DevicePrivateDataCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
