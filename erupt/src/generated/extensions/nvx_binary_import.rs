#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NVX_BINARY_IMPORT_SPEC_VERSION")]
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NVX_BINARY_IMPORT_EXTENSION_NAME")]
pub const NVX_BINARY_IMPORT_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NVX_binary_import");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_CU_MODULE_NVX: *const std::os::raw::c_char = crate::cstr!("vkCreateCuModuleNVX");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_CU_FUNCTION_NVX: *const std::os::raw::c_char = crate::cstr!("vkCreateCuFunctionNVX");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_CU_MODULE_NVX: *const std::os::raw::c_char = crate::cstr!("vkDestroyCuModuleNVX");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_CU_FUNCTION_NVX: *const std::os::raw::c_char = crate::cstr!("vkDestroyCuFunctionNVX");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_CU_LAUNCH_KERNEL_NVX: *const std::os::raw::c_char = crate::cstr!("vkCmdCuLaunchKernelNVX");
crate::non_dispatchable_handle!(CuModuleNVX, CU_MODULE_NVX, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuModuleNVX.html) · Non-dispatchable Handle", "VkCuModuleNVX");
crate::non_dispatchable_handle!(CuFunctionNVX, CU_FUNCTION_NVX, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuFunctionNVX.html) · Non-dispatchable Handle", "VkCuFunctionNVX");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCuModuleNVX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::nvx_binary_import::CuModuleCreateInfoNVX, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_module: *mut crate::extensions::nvx_binary_import::CuModuleNVX) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCuFunctionNVX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::extensions::nvx_binary_import::CuFunctionCreateInfoNVX, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_function: *mut crate::extensions::nvx_binary_import::CuFunctionNVX) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCuModuleNVX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(device: crate::vk1_0::Device, module: crate::extensions::nvx_binary_import::CuModuleNVX, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCuFunctionNVX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(device: crate::vk1_0::Device, function: crate::extensions::nvx_binary_import::CuFunctionNVX, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCuLaunchKernelNVX.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_launch_info: *const crate::extensions::nvx_binary_import::CuLaunchInfoNVX) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuModuleCreateInfoNVX.html) · Structure"]
#[doc(alias = "VkCuModuleCreateInfoNVX")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CuModuleCreateInfoNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub data_size: usize,
    pub p_data: *const std::ffi::c_void,
}
impl Default for CuModuleCreateInfoNVX {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::CU_MODULE_CREATE_INFO_NVX, p_next: std::ptr::null(), data_size: Default::default(), p_data: std::ptr::null() }
    }
}
impl std::fmt::Debug for CuModuleCreateInfoNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CuModuleCreateInfoNVX").field("s_type", &self.s_type).field("p_next", &self.p_next).field("data_size", &self.data_size).field("p_data", &self.p_data).finish()
    }
}
impl CuModuleCreateInfoNVX {
    #[inline]
    pub fn into_builder<'a>(self) -> CuModuleCreateInfoNVXBuilder<'a> {
        CuModuleCreateInfoNVXBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuModuleCreateInfoNVX.html) · Builder of [`CuModuleCreateInfoNVX`]"]
#[repr(transparent)]
pub struct CuModuleCreateInfoNVXBuilder<'a>(CuModuleCreateInfoNVX, std::marker::PhantomData<&'a ()>);
impl<'a> CuModuleCreateInfoNVXBuilder<'a> {
    #[inline]
    pub fn new() -> CuModuleCreateInfoNVXBuilder<'a> {
        CuModuleCreateInfoNVXBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn data_size(mut self, data_size: usize) -> Self {
        self.0.data_size = data_size as _;
        self
    }
    #[inline]
    pub fn data(mut self, data: *const std::ffi::c_void) -> Self {
        self.0.p_data = data;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CuModuleCreateInfoNVX {
        self.0
    }
}
impl<'a> std::default::Default for CuModuleCreateInfoNVXBuilder<'a> {
    fn default() -> CuModuleCreateInfoNVXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CuModuleCreateInfoNVXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CuModuleCreateInfoNVXBuilder<'a> {
    type Target = CuModuleCreateInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CuModuleCreateInfoNVXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuFunctionCreateInfoNVX.html) · Structure"]
#[doc(alias = "VkCuFunctionCreateInfoNVX")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CuFunctionCreateInfoNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub module: crate::extensions::nvx_binary_import::CuModuleNVX,
    pub p_name: *const std::os::raw::c_char,
}
impl Default for CuFunctionCreateInfoNVX {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::CU_FUNCTION_CREATE_INFO_NVX, p_next: std::ptr::null(), module: Default::default(), p_name: std::ptr::null() }
    }
}
impl std::fmt::Debug for CuFunctionCreateInfoNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CuFunctionCreateInfoNVX").field("s_type", &self.s_type).field("p_next", &self.p_next).field("module", &self.module).field("p_name", &self.p_name).finish()
    }
}
impl CuFunctionCreateInfoNVX {
    #[inline]
    pub fn into_builder<'a>(self) -> CuFunctionCreateInfoNVXBuilder<'a> {
        CuFunctionCreateInfoNVXBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuFunctionCreateInfoNVX.html) · Builder of [`CuFunctionCreateInfoNVX`]"]
#[repr(transparent)]
pub struct CuFunctionCreateInfoNVXBuilder<'a>(CuFunctionCreateInfoNVX, std::marker::PhantomData<&'a ()>);
impl<'a> CuFunctionCreateInfoNVXBuilder<'a> {
    #[inline]
    pub fn new() -> CuFunctionCreateInfoNVXBuilder<'a> {
        CuFunctionCreateInfoNVXBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn module(mut self, module: crate::extensions::nvx_binary_import::CuModuleNVX) -> Self {
        self.0.module = module as _;
        self
    }
    #[inline]
    pub fn name(mut self, name: &'a std::ffi::CStr) -> Self {
        self.0.p_name = name.as_ptr();
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CuFunctionCreateInfoNVX {
        self.0
    }
}
impl<'a> std::default::Default for CuFunctionCreateInfoNVXBuilder<'a> {
    fn default() -> CuFunctionCreateInfoNVXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CuFunctionCreateInfoNVXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CuFunctionCreateInfoNVXBuilder<'a> {
    type Target = CuFunctionCreateInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CuFunctionCreateInfoNVXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuLaunchInfoNVX.html) · Structure"]
#[doc(alias = "VkCuLaunchInfoNVX")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CuLaunchInfoNVX {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub function: crate::extensions::nvx_binary_import::CuFunctionNVX,
    pub grid_dim_x: u32,
    pub grid_dim_y: u32,
    pub grid_dim_z: u32,
    pub block_dim_x: u32,
    pub block_dim_y: u32,
    pub block_dim_z: u32,
    pub shared_mem_bytes: u32,
    pub param_count: usize,
    pub p_params: *const *const std::ffi::c_void,
    pub extra_count: usize,
    pub p_extras: *const *const std::ffi::c_void,
}
impl Default for CuLaunchInfoNVX {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::CU_LAUNCH_INFO_NVX, p_next: std::ptr::null(), function: Default::default(), grid_dim_x: Default::default(), grid_dim_y: Default::default(), grid_dim_z: Default::default(), block_dim_x: Default::default(), block_dim_y: Default::default(), block_dim_z: Default::default(), shared_mem_bytes: Default::default(), param_count: Default::default(), p_params: std::ptr::null(), extra_count: Default::default(), p_extras: std::ptr::null() }
    }
}
impl std::fmt::Debug for CuLaunchInfoNVX {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CuLaunchInfoNVX").field("s_type", &self.s_type).field("p_next", &self.p_next).field("function", &self.function).field("grid_dim_x", &self.grid_dim_x).field("grid_dim_y", &self.grid_dim_y).field("grid_dim_z", &self.grid_dim_z).field("block_dim_x", &self.block_dim_x).field("block_dim_y", &self.block_dim_y).field("block_dim_z", &self.block_dim_z).field("shared_mem_bytes", &self.shared_mem_bytes).field("param_count", &self.param_count).field("p_params", &self.p_params).field("extra_count", &self.extra_count).field("p_extras", &self.p_extras).finish()
    }
}
impl CuLaunchInfoNVX {
    #[inline]
    pub fn into_builder<'a>(self) -> CuLaunchInfoNVXBuilder<'a> {
        CuLaunchInfoNVXBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCuLaunchInfoNVX.html) · Builder of [`CuLaunchInfoNVX`]"]
#[repr(transparent)]
pub struct CuLaunchInfoNVXBuilder<'a>(CuLaunchInfoNVX, std::marker::PhantomData<&'a ()>);
impl<'a> CuLaunchInfoNVXBuilder<'a> {
    #[inline]
    pub fn new() -> CuLaunchInfoNVXBuilder<'a> {
        CuLaunchInfoNVXBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn function(mut self, function: crate::extensions::nvx_binary_import::CuFunctionNVX) -> Self {
        self.0.function = function as _;
        self
    }
    #[inline]
    pub fn grid_dim_x(mut self, grid_dim_x: u32) -> Self {
        self.0.grid_dim_x = grid_dim_x as _;
        self
    }
    #[inline]
    pub fn grid_dim_y(mut self, grid_dim_y: u32) -> Self {
        self.0.grid_dim_y = grid_dim_y as _;
        self
    }
    #[inline]
    pub fn grid_dim_z(mut self, grid_dim_z: u32) -> Self {
        self.0.grid_dim_z = grid_dim_z as _;
        self
    }
    #[inline]
    pub fn block_dim_x(mut self, block_dim_x: u32) -> Self {
        self.0.block_dim_x = block_dim_x as _;
        self
    }
    #[inline]
    pub fn block_dim_y(mut self, block_dim_y: u32) -> Self {
        self.0.block_dim_y = block_dim_y as _;
        self
    }
    #[inline]
    pub fn block_dim_z(mut self, block_dim_z: u32) -> Self {
        self.0.block_dim_z = block_dim_z as _;
        self
    }
    #[inline]
    pub fn shared_mem_bytes(mut self, shared_mem_bytes: u32) -> Self {
        self.0.shared_mem_bytes = shared_mem_bytes as _;
        self
    }
    #[inline]
    pub fn param_count(mut self, param_count: usize) -> Self {
        self.0.param_count = param_count as _;
        self
    }
    #[inline]
    pub fn params(mut self, params: *const *const std::ffi::c_void) -> Self {
        self.0.p_params = params;
        self
    }
    #[inline]
    pub fn extra_count(mut self, extra_count: usize) -> Self {
        self.0.extra_count = extra_count as _;
        self
    }
    #[inline]
    pub fn extras(mut self, extras: *const *const std::ffi::c_void) -> Self {
        self.0.p_extras = extras;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CuLaunchInfoNVX {
        self.0
    }
}
impl<'a> std::default::Default for CuLaunchInfoNVXBuilder<'a> {
    fn default() -> CuLaunchInfoNVXBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CuLaunchInfoNVXBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CuLaunchInfoNVXBuilder<'a> {
    type Target = CuLaunchInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CuLaunchInfoNVXBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::nvx_binary_import`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCuModuleNVX.html) · Function"]
    #[doc(alias = "vkCreateCuModuleNVX")]
    pub unsafe fn create_cu_module_nvx(&self, create_info: &crate::extensions::nvx_binary_import::CuModuleCreateInfoNVX, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::nvx_binary_import::CuModuleNVX> {
        let _function = self.create_cu_module_nvx.expect("tried to call a function that isn't loaded");
        let mut module = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut module,
        );
        crate::utils::VulkanResult::new(_return, module)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCuFunctionNVX.html) · Function"]
    #[doc(alias = "vkCreateCuFunctionNVX")]
    pub unsafe fn create_cu_function_nvx(&self, create_info: &crate::extensions::nvx_binary_import::CuFunctionCreateInfoNVX, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::extensions::nvx_binary_import::CuFunctionNVX> {
        let _function = self.create_cu_function_nvx.expect("tried to call a function that isn't loaded");
        let mut function = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut function,
        );
        crate::utils::VulkanResult::new(_return, function)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCuModuleNVX.html) · Function"]
    #[doc(alias = "vkDestroyCuModuleNVX")]
    pub unsafe fn destroy_cu_module_nvx(&self, module: crate::extensions::nvx_binary_import::CuModuleNVX, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_cu_module_nvx.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            module as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCuFunctionNVX.html) · Function"]
    #[doc(alias = "vkDestroyCuFunctionNVX")]
    pub unsafe fn destroy_cu_function_nvx(&self, function: crate::extensions::nvx_binary_import::CuFunctionNVX, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_cu_function_nvx.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            function as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCuLaunchKernelNVX.html) · Function"]
    #[doc(alias = "vkCmdCuLaunchKernelNVX")]
    pub unsafe fn cmd_cu_launch_kernel_nvx(&self, command_buffer: crate::vk1_0::CommandBuffer, launch_info: &crate::extensions::nvx_binary_import::CuLaunchInfoNVX) -> () {
        let _function = self.cmd_cu_launch_kernel_nvx.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, launch_info as _);
        ()
    }
}
