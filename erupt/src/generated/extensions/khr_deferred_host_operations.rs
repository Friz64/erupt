# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_KHR_deferred_host_operations.html)\n\n## Extends\n- [`ObjectType`](../../vk1_0/struct.ObjectType.html)\n- [`Result`](../../vk1_0/struct.Result.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_deferred_host_operations");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_deferred_operation: *mut crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDeferredOperationKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationResultKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkDeferredOperationJoinKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`KhrDeferredHostOperationsDeviceLoaderExt`](trait.KhrDeferredHostOperationsDeviceLoaderExt.html)"]
pub struct KhrDeferredHostOperationsDeviceCommands {
    pub create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
    pub destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
    pub get_deferred_operation_max_concurrency_khr: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
    pub get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
    pub deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
}
impl KhrDeferredHostOperationsDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<KhrDeferredHostOperationsDeviceCommands> {
        unsafe {
            Some(KhrDeferredHostOperationsDeviceCommands {
                create_deferred_operation_khr: std::mem::transmute(
                    loader.symbol("vkCreateDeferredOperationKHR")?,
                ),
                destroy_deferred_operation_khr: std::mem::transmute(
                    loader.symbol("vkDestroyDeferredOperationKHR")?,
                ),
                get_deferred_operation_max_concurrency_khr: std::mem::transmute(
                    loader.symbol("vkGetDeferredOperationMaxConcurrencyKHR")?,
                ),
                get_deferred_operation_result_khr: std::mem::transmute(
                    loader.symbol("vkGetDeferredOperationResultKHR")?,
                ),
                deferred_operation_join_khr: std::mem::transmute(
                    loader.symbol("vkDeferredOperationJoinKHR")?,
                ),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`KhrDeferredHostOperationsDeviceCommands`](struct.KhrDeferredHostOperationsDeviceCommands.html)"]
pub trait KhrDeferredHostOperationsDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html) · Device Command"]
    unsafe fn create_deferred_operation_khr(
        &self,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        deferred_operation: Option<
            crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    >;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDeferredOperationKHR.html) · Device Command"]
    unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> ();
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) · Device Command"]
    unsafe fn get_deferred_operation_max_concurrency_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> u32;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html) · Device Command"]
    unsafe fn get_deferred_operation_result_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::utils::VulkanResult<()>;
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html) · Device Command"]
    unsafe fn deferred_operation_join_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::utils::VulkanResult<()>;
}
impl KhrDeferredHostOperationsDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html) · Device Command"]
    unsafe fn create_deferred_operation_khr(
        &self,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        deferred_operation: Option<
            crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    > {
        let function = self
            .khr_deferred_host_operations
            .as_ref()
            .expect("`khr_deferred_host_operations` not loaded")
            .create_deferred_operation_khr;
        let mut deferred_operation = deferred_operation.unwrap_or_else(|| Default::default());
        let _val = function(
            self.handle,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
            &mut deferred_operation,
        );
        crate::utils::VulkanResult::new(_val, deferred_operation)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDeferredOperationKHR.html) · Device Command"]
    unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let function = self
            .khr_deferred_host_operations
            .as_ref()
            .expect("`khr_deferred_host_operations` not loaded")
            .destroy_deferred_operation_khr;
        let _val = function(
            self.handle,
            operation,
            if let Some(allocator) = allocator {
                allocator
            } else {
                std::ptr::null()
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) · Device Command"]
    unsafe fn get_deferred_operation_max_concurrency_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> u32 {
        let function = self
            .khr_deferred_host_operations
            .as_ref()
            .expect("`khr_deferred_host_operations` not loaded")
            .get_deferred_operation_max_concurrency_khr;
        let _val = function(self.handle, operation);
        _val
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html) · Device Command"]
    unsafe fn get_deferred_operation_result_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_deferred_host_operations
            .as_ref()
            .expect("`khr_deferred_host_operations` not loaded")
            .get_deferred_operation_result_khr;
        let _val = function(self.handle, operation);
        crate::utils::VulkanResult::new(_val, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html) · Device Command"]
    unsafe fn deferred_operation_join_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .khr_deferred_host_operations
            .as_ref()
            .expect("`khr_deferred_host_operations` not loaded")
            .deferred_operation_join_khr;
        let _val = function(self.handle, operation);
        crate::utils::VulkanResult::new(_val, ())
    }
}
crate :: non_dispatchable_handle ! ( DeferredOperationKHR , DEFERRED_OPERATION_KHR , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationKHR.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeferredOperationInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub operation_handle: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
}
impl DeferredOperationInfoKHR {
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
    pub fn builder<'a>(self) -> DeferredOperationInfoKHRBuilder<'a> {
        DeferredOperationInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for DeferredOperationInfoKHR {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("DeferredOperationInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("operation_handle", &self.operation_handle)
            .finish()
    }
}
impl Default for DeferredOperationInfoKHR {
    fn default() -> DeferredOperationInfoKHR {
        DeferredOperationInfoKHR {
            s_type: crate::vk1_0::StructureType::DEFERRED_OPERATION_INFO_KHR,
            p_next: std::ptr::null(),
            operation_handle: Default::default(),
        }
    }
}
impl crate::ExtendableBy<DeferredOperationInfoKHR>
    for crate::extensions::khr_ray_tracing::RayTracingPipelineCreateInfoKHR
{
}
impl crate::ExtendableBy<DeferredOperationInfoKHR>
    for crate::extensions::khr_ray_tracing::AccelerationStructureBuildGeometryInfoKHR
{
}
impl crate::ExtendableBy<DeferredOperationInfoKHR>
    for crate::extensions::khr_ray_tracing::CopyAccelerationStructureInfoKHR
{
}
impl crate::ExtendableBy<DeferredOperationInfoKHR>
    for crate::extensions::khr_ray_tracing::CopyMemoryToAccelerationStructureInfoKHR
{
}
impl crate::ExtendableBy<DeferredOperationInfoKHR>
    for crate::extensions::khr_ray_tracing::CopyAccelerationStructureToMemoryInfoKHR
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationInfoKHR.html) · Builder of [`DeferredOperationInfoKHR`](struct.DeferredOperationInfoKHR.html)"]
#[repr(transparent)]
pub struct DeferredOperationInfoKHRBuilder<'a>(
    DeferredOperationInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> DeferredOperationInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> DeferredOperationInfoKHRBuilder<'a> {
        DeferredOperationInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn operation_handle(
        mut self,
        operation_handle: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> Self {
        self.0.operation_handle = operation_handle;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> DeferredOperationInfoKHR {
        self.0
    }
}
impl<'a> std::fmt::Debug for DeferredOperationInfoKHRBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for DeferredOperationInfoKHRBuilder<'a> {
    type Target = DeferredOperationInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeferredOperationInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
