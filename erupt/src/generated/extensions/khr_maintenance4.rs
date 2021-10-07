#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE_4_SPEC_VERSION")]
pub const KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE_4_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_4_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_maintenance4");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceBufferMemoryRequirementsKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceImageMemoryRequirementsKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceImageSparseMemoryRequirementsKHR");
#[doc = "Provided by [`crate::extensions::khr_maintenance4`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self = Self(1000413001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413002);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self(1000413003);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_info: *const crate::extensions::khr_maintenance4::DeviceBufferMemoryRequirementsKHR, p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_info: *const crate::extensions::khr_maintenance4::DeviceImageMemoryRequirementsKHR, p_memory_requirements: *mut crate::vk1_1::MemoryRequirements2) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, p_info: *const crate::extensions::khr_maintenance4::DeviceImageMemoryRequirementsKHR, p_sparse_memory_requirement_count: *mut u32, p_sparse_memory_requirements: *mut crate::vk1_1::SparseImageMemoryRequirements2) -> ();
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMaintenance4FeaturesKHR> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMaintenance4FeaturesKHRBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMaintenance4FeaturesKHR> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMaintenance4FeaturesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMaintenance4PropertiesKHR> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceMaintenance4PropertiesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceBufferMemoryRequirementsKHR.html) · Structure"]
#[doc(alias = "VkDeviceBufferMemoryRequirementsKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceBufferMemoryRequirementsKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_create_info: *const crate::vk1_0::BufferCreateInfo,
}
impl DeviceBufferMemoryRequirementsKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR;
}
impl Default for DeviceBufferMemoryRequirementsKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_create_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for DeviceBufferMemoryRequirementsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceBufferMemoryRequirementsKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_create_info", &self.p_create_info).finish()
    }
}
impl DeviceBufferMemoryRequirementsKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceBufferMemoryRequirementsKHRBuilder<'a> {
        DeviceBufferMemoryRequirementsKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceBufferMemoryRequirementsKHR.html) · Builder of [`DeviceBufferMemoryRequirementsKHR`]"]
#[repr(transparent)]
pub struct DeviceBufferMemoryRequirementsKHRBuilder<'a>(DeviceBufferMemoryRequirementsKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceBufferMemoryRequirementsKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceBufferMemoryRequirementsKHRBuilder<'a> {
        DeviceBufferMemoryRequirementsKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn create_info(mut self, create_info: &'a crate::vk1_0::BufferCreateInfo) -> Self {
        self.0.p_create_info = create_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceBufferMemoryRequirementsKHR {
        self.0
    }
}
impl<'a> std::default::Default for DeviceBufferMemoryRequirementsKHRBuilder<'a> {
    fn default() -> DeviceBufferMemoryRequirementsKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceBufferMemoryRequirementsKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceBufferMemoryRequirementsKHRBuilder<'a> {
    type Target = DeviceBufferMemoryRequirementsKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceBufferMemoryRequirementsKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceImageMemoryRequirementsKHR.html) · Structure"]
#[doc(alias = "VkDeviceImageMemoryRequirementsKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceImageMemoryRequirementsKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_create_info: *const crate::vk1_0::ImageCreateInfo,
    pub plane_aspect: crate::vk1_0::ImageAspectFlagBits,
}
impl DeviceImageMemoryRequirementsKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR;
}
impl Default for DeviceImageMemoryRequirementsKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_create_info: std::ptr::null(), plane_aspect: Default::default() }
    }
}
impl std::fmt::Debug for DeviceImageMemoryRequirementsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceImageMemoryRequirementsKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_create_info", &self.p_create_info).field("plane_aspect", &self.plane_aspect).finish()
    }
}
impl DeviceImageMemoryRequirementsKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceImageMemoryRequirementsKHRBuilder<'a> {
        DeviceImageMemoryRequirementsKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceImageMemoryRequirementsKHR.html) · Builder of [`DeviceImageMemoryRequirementsKHR`]"]
#[repr(transparent)]
pub struct DeviceImageMemoryRequirementsKHRBuilder<'a>(DeviceImageMemoryRequirementsKHR, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceImageMemoryRequirementsKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceImageMemoryRequirementsKHRBuilder<'a> {
        DeviceImageMemoryRequirementsKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn create_info(mut self, create_info: &'a crate::vk1_0::ImageCreateInfo) -> Self {
        self.0.p_create_info = create_info as _;
        self
    }
    #[inline]
    pub fn plane_aspect(mut self, plane_aspect: crate::vk1_0::ImageAspectFlagBits) -> Self {
        self.0.plane_aspect = plane_aspect as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceImageMemoryRequirementsKHR {
        self.0
    }
}
impl<'a> std::default::Default for DeviceImageMemoryRequirementsKHRBuilder<'a> {
    fn default() -> DeviceImageMemoryRequirementsKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceImageMemoryRequirementsKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceImageMemoryRequirementsKHRBuilder<'a> {
    type Target = DeviceImageMemoryRequirementsKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceImageMemoryRequirementsKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance4FeaturesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMaintenance4FeaturesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4FeaturesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub maintenance4: crate::vk1_0::Bool32,
}
impl PhysicalDeviceMaintenance4FeaturesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR;
}
impl Default for PhysicalDeviceMaintenance4FeaturesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), maintenance4: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceMaintenance4FeaturesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMaintenance4FeaturesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("maintenance4", &(self.maintenance4 != 0)).finish()
    }
}
impl PhysicalDeviceMaintenance4FeaturesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
        PhysicalDeviceMaintenance4FeaturesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance4FeaturesKHR.html) · Builder of [`PhysicalDeviceMaintenance4FeaturesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a>(PhysicalDeviceMaintenance4FeaturesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
        PhysicalDeviceMaintenance4FeaturesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn maintenance4(mut self, maintenance4: bool) -> Self {
        self.0.maintenance4 = maintenance4 as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMaintenance4FeaturesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
    type Target = PhysicalDeviceMaintenance4FeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMaintenance4FeaturesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance4PropertiesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMaintenance4PropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4PropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_buffer_size: crate::vk1_0::DeviceSize,
}
impl PhysicalDeviceMaintenance4PropertiesKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR;
}
impl Default for PhysicalDeviceMaintenance4PropertiesKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), max_buffer_size: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceMaintenance4PropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMaintenance4PropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_buffer_size", &self.max_buffer_size).finish()
    }
}
impl PhysicalDeviceMaintenance4PropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
        PhysicalDeviceMaintenance4PropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance4PropertiesKHR.html) · Builder of [`PhysicalDeviceMaintenance4PropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a>(PhysicalDeviceMaintenance4PropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
        PhysicalDeviceMaintenance4PropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_buffer_size(mut self, max_buffer_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.max_buffer_size = max_buffer_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMaintenance4PropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
    type Target = PhysicalDeviceMaintenance4PropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMaintenance4PropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_maintenance4`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html) · Function"]
    #[doc(alias = "vkGetDeviceBufferMemoryRequirementsKHR")]
    pub unsafe fn get_device_buffer_memory_requirements_khr(&self, info: &crate::extensions::khr_maintenance4::DeviceBufferMemoryRequirementsKHR, memory_requirements: Option<crate::vk1_1::MemoryRequirements2>) -> crate::vk1_1::MemoryRequirements2 {
        let _function = self.get_device_buffer_memory_requirements_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut memory_requirements = match memory_requirements {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html) · Function"]
    #[doc(alias = "vkGetDeviceImageMemoryRequirementsKHR")]
    pub unsafe fn get_device_image_memory_requirements_khr(&self, info: &crate::extensions::khr_maintenance4::DeviceImageMemoryRequirementsKHR, memory_requirements: Option<crate::vk1_1::MemoryRequirements2>) -> crate::vk1_1::MemoryRequirements2 {
        let _function = self.get_device_image_memory_requirements_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut memory_requirements = match memory_requirements {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, info as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html) · Function"]
    #[doc(alias = "vkGetDeviceImageSparseMemoryRequirementsKHR")]
    pub unsafe fn get_device_image_sparse_memory_requirements_khr(&self, info: &crate::extensions::khr_maintenance4::DeviceImageMemoryRequirementsKHR, sparse_memory_requirement_count: Option<u32>) -> crate::SmallVec<crate::vk1_1::SparseImageMemoryRequirements2> {
        let _function = self.get_device_image_sparse_memory_requirements_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, info as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut sparse_memory_requirements = crate::SmallVec::from_elem(Default::default(), sparse_memory_requirement_count as _);
        let _return = _function(self.handle, info as _, &mut sparse_memory_requirement_count, sparse_memory_requirements.as_mut_ptr());
        sparse_memory_requirements
    }
}
