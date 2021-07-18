#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PCI_BUS_INFO_SPEC_VERSION")]
pub const EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_PCI_BUS_INFO_EXTENSION_NAME")]
pub const EXT_PCI_BUS_INFO_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_pci_bus_info");
#[doc = "Provided by [`crate::extensions::ext_pci_bus_info`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePCIBusInfoPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePCIBusInfoPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePCIBusInfoPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub pci_domain: u32,
    pub pci_bus: u32,
    pub pci_device: u32,
    pub pci_function: u32,
}
impl PhysicalDevicePCIBusInfoPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT;
}
impl Default for PhysicalDevicePCIBusInfoPropertiesEXT {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT, p_next: std::ptr::null_mut(), pci_domain: Default::default(), pci_bus: Default::default(), pci_device: Default::default(), pci_function: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePCIBusInfoPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePCIBusInfoPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("pci_domain", &self.pci_domain).field("pci_bus", &self.pci_bus).field("pci_device", &self.pci_device).field("pci_function", &self.pci_function).finish()
    }
}
impl PhysicalDevicePCIBusInfoPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
        PhysicalDevicePCIBusInfoPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html) · Builder of [`PhysicalDevicePCIBusInfoPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a>(PhysicalDevicePCIBusInfoPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
        PhysicalDevicePCIBusInfoPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn pci_domain(mut self, pci_domain: u32) -> Self {
        self.0.pci_domain = pci_domain as _;
        self
    }
    #[inline]
    pub fn pci_bus(mut self, pci_bus: u32) -> Self {
        self.0.pci_bus = pci_bus as _;
        self
    }
    #[inline]
    pub fn pci_device(mut self, pci_device: u32) -> Self {
        self.0.pci_device = pci_device as _;
        self
    }
    #[inline]
    pub fn pci_function(mut self, pci_function: u32) -> Self {
        self.0.pci_function = pci_function as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePCIBusInfoPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
    type Target = PhysicalDevicePCIBusInfoPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
