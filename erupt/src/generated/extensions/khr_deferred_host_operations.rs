#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_deferred_host_operations");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DEFERRED_OPERATION_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCreateDeferredOperationKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DEFERRED_OPERATION_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkDestroyDeferredOperationKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetDeferredOperationMaxConcurrencyKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEFERRED_OPERATION_RESULT_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetDeferredOperationResultKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DEFERRED_OPERATION_JOIN_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkDeferredOperationJoinKHR");
crate :: non_dispatchable_handle ! ( DeferredOperationKHR , DEFERRED_OPERATION_KHR , doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationKHR.html) · Non-dispatchable Handle" ) ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
    p_deferred_operation: *mut crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDeferredOperationKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    p_allocator: *const crate::vk1_0::AllocationCallbacks,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationResultKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDeferredOperationJoinKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationInfoKHR.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeferredOperationInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub operation_handle: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
}
impl Default for DeferredOperationInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::DEFERRED_OPERATION_INFO_KHR,
            p_next: std::ptr::null(),
            operation_handle: Default::default(),
        }
    }
}
impl std::fmt::Debug for DeferredOperationInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeferredOperationInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("operation_handle", &self.operation_handle)
            .finish()
    }
}
impl DeferredOperationInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> DeferredOperationInfoKHRBuilder<'a> {
        DeferredOperationInfoKHRBuilder(self, std::marker::PhantomData)
    }
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
    #[inline]
    pub fn operation_handle(
        mut self,
        operation_handle: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> Self {
        self.0.operation_handle = operation_handle as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeferredOperationInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for DeferredOperationInfoKHRBuilder<'a> {
    fn default() -> DeferredOperationInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeferredOperationInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Provided by [`extensions::khr_deferred_host_operations`](extensions/khr_deferred_host_operations/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html) · Function"]
    pub unsafe fn create_deferred_operation_khr(
        &self,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        deferred_operation: Option<
            crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
        >,
    ) -> crate::utils::VulkanResult<
        crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    > {
        let _function = self
            .create_deferred_operation_khr
            .expect("`create_deferred_operation_khr` is not loaded");
        let mut deferred_operation = match deferred_operation {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            self.handle,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut deferred_operation,
        );
        crate::utils::VulkanResult::new(_return, deferred_operation)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDeferredOperationKHR.html) · Function"]
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let _function = self
            .destroy_deferred_operation_khr
            .expect("`destroy_deferred_operation_khr` is not loaded");
        let _return = _function(
            self.handle,
            match operation {
                Some(v) => v,
                None => Default::default(),
            },
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) · Function"]
    pub unsafe fn get_deferred_operation_max_concurrency_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> u32 {
        let _function = self
            .get_deferred_operation_max_concurrency_khr
            .expect("`get_deferred_operation_max_concurrency_khr` is not loaded");
        let _return = _function(self.handle, operation as _);
        _return
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html) · Function"]
    pub unsafe fn get_deferred_operation_result_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .get_deferred_operation_result_khr
            .expect("`get_deferred_operation_result_khr` is not loaded");
        let _return = _function(self.handle, operation as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html) · Function"]
    pub unsafe fn deferred_operation_join_khr(
        &self,
        operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .deferred_operation_join_khr
            .expect("`deferred_operation_join_khr` is not loaded");
        let _return = _function(self.handle, operation as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
