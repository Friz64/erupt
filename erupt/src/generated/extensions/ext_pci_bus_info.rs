# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_pci_bus_info.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_PCI_BUS_INFO_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_pci_bus_info");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html) · Structure"]
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
    pub fn builder<'a>(self) -> PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
        PhysicalDevicePCIBusInfoPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDevicePCIBusInfoPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDevicePCIBusInfoPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("pci_domain", &self.pci_domain)
            .field("pci_bus", &self.pci_bus)
            .field("pci_device", &self.pci_device)
            .field("pci_function", &self.pci_function)
            .finish()
    }
}
impl Default for PhysicalDevicePCIBusInfoPropertiesEXT {
    fn default() -> PhysicalDevicePCIBusInfoPropertiesEXT {
        PhysicalDevicePCIBusInfoPropertiesEXT {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            pci_domain: Default::default(),
            pci_bus: Default::default(),
            pci_device: Default::default(),
            pci_function: Default::default(),
        }
    }
}
impl crate::ExtendableBy<PhysicalDevicePCIBusInfoPropertiesEXT>
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html) · Builder of [`PhysicalDevicePCIBusInfoPropertiesEXT`](struct.PhysicalDevicePCIBusInfoPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a>(
    PhysicalDevicePCIBusInfoPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
        PhysicalDevicePCIBusInfoPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pci_domain(mut self, pci_domain: u32) -> Self {
        self.0.pci_domain = pci_domain;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pci_bus(mut self, pci_bus: u32) -> Self {
        self.0.pci_bus = pci_bus;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pci_device(mut self, pci_device: u32) -> Self {
        self.0.pci_device = pci_device;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn pci_function(mut self, pci_function: u32) -> Self {
        self.0.pci_function = pci_function;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDevicePCIBusInfoPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePCIBusInfoPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
