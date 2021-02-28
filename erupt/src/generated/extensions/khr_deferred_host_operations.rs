#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION")]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME")]
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_deferred_host_operations");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DEFERRED_OPERATION_KHR: *const std::os::raw::c_char = crate::cstr!("vkCreateDeferredOperationKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DEFERRED_OPERATION_KHR: *const std::os::raw::c_char = crate::cstr!("vkDestroyDeferredOperationKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeferredOperationMaxConcurrencyKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEFERRED_OPERATION_RESULT_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDeferredOperationResultKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DEFERRED_OPERATION_JOIN_KHR: *const std::os::raw::c_char = crate::cstr!("vkDeferredOperationJoinKHR");
crate::non_dispatchable_handle!(
    DeferredOperationKHR,
    DEFERRED_OPERATION_KHR,
    "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeferredOperationKHR.html) · Non-dispatchable Handle",
    "VkDeferredOperationKHR"
);
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
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR = unsafe extern "system" fn(device: crate::vk1_0::Device, operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR) -> u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationResultKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDeferredOperationJoinKHR =
    unsafe extern "system" fn(device: crate::vk1_0::Device, operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR) -> crate::vk1_0::Result;
#[doc = "Provided by [`crate::extensions::khr_deferred_host_operations`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDeferredOperationKHR.html) · Function"]
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    pub unsafe fn create_deferred_operation_khr(
        &self,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
        deferred_operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR> {
        let _function = self.create_deferred_operation_khr.expect("`create_deferred_operation_khr` is not loaded");
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
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: Option<crate::extensions::khr_deferred_host_operations::DeferredOperationKHR>,
        allocator: Option<&crate::vk1_0::AllocationCallbacks>,
    ) -> () {
        let _function = self.destroy_deferred_operation_khr.expect("`destroy_deferred_operation_khr` is not loaded");
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
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    pub unsafe fn get_deferred_operation_max_concurrency_khr(&self, operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR) -> u32 {
        let _function = self.get_deferred_operation_max_concurrency_khr.expect("`get_deferred_operation_max_concurrency_khr` is not loaded");
        let _return = _function(self.handle, operation as _);
        _return
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeferredOperationResultKHR.html) · Function"]
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    pub unsafe fn get_deferred_operation_result_khr(&self, operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.get_deferred_operation_result_khr.expect("`get_deferred_operation_result_khr` is not loaded");
        let _return = _function(self.handle, operation as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeferredOperationJoinKHR.html) · Function"]
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    pub unsafe fn deferred_operation_join_khr(&self, operation: crate::extensions::khr_deferred_host_operations::DeferredOperationKHR) -> crate::utils::VulkanResult<()> {
        let _function = self.deferred_operation_join_khr.expect("`deferred_operation_join_khr` is not loaded");
        let _return = _function(self.handle, operation as _);
        crate::utils::VulkanResult::new(_return, ())
    }
}
