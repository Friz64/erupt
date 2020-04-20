# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_AMD_shader_info.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const AMD_SHADER_INFO_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_AMD_shader_info");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetShaderInfoAMD.html) · Device Command"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    pipeline: crate::vk1_0::Pipeline,
    shader_stage: crate::vk1_0::ShaderStageFlagBits,
    info_type: crate::extensions::amd_shader_info::ShaderInfoTypeAMD,
    p_info_size: *mut usize,
    p_info: *mut std::ffi::c_void,
) -> crate::vk1_0::Result;
#[doc = "Provides Device Commands for [`AmdShaderInfoDeviceLoaderExt`](trait.AmdShaderInfoDeviceLoaderExt.html)"]
pub struct AmdShaderInfoDeviceCommands {
    pub get_shader_info_amd: PFN_vkGetShaderInfoAMD,
}
impl AmdShaderInfoDeviceCommands {
    #[inline]
    pub fn load(loader: &crate::DeviceLoader) -> Option<AmdShaderInfoDeviceCommands> {
        unsafe {
            Some(AmdShaderInfoDeviceCommands {
                get_shader_info_amd: std::mem::transmute(loader.symbol("vkGetShaderInfoAMD")?),
            })
        }
    }
}
#[doc = "Provides high level command wrappers for [`AmdShaderInfoDeviceCommands`](struct.AmdShaderInfoDeviceCommands.html)"]
pub trait AmdShaderInfoDeviceLoaderExt {
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetShaderInfoAMD.html) · Device Command"]
    unsafe fn get_shader_info_amd(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        shader_stage: crate::vk1_0::ShaderStageFlagBits,
        info_type: crate::extensions::amd_shader_info::ShaderInfoTypeAMD,
        info_size: *mut usize,
        info: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()>;
}
impl AmdShaderInfoDeviceLoaderExt for crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetShaderInfoAMD.html) · Device Command"]
    unsafe fn get_shader_info_amd(
        &self,
        pipeline: crate::vk1_0::Pipeline,
        shader_stage: crate::vk1_0::ShaderStageFlagBits,
        info_type: crate::extensions::amd_shader_info::ShaderInfoTypeAMD,
        info_size: *mut usize,
        info: *mut std::ffi::c_void,
    ) -> crate::utils::VulkanResult<()> {
        let function = self
            .amd_shader_info
            .as_ref()
            .expect("`amd_shader_info` not loaded")
            .get_shader_info_amd;
        let _val = function(
            self.handle,
            pipeline,
            shader_stage,
            info_type,
            info_size,
            info,
        );
        crate::utils::VulkanResult::new(_val, ())
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderInfoTypeAMD.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ShaderInfoTypeAMD(pub i32);
#[doc = "[Part of `extensions::amd_shader_info`](../../extensions/amd_shader_info/index.html)"]
impl ShaderInfoTypeAMD {
    pub const STATISTICS_AMD: Self = Self(0);
    pub const BINARY_AMD: Self = Self(1);
    pub const DISASSEMBLY_AMD: Self = Self(2);
}
impl std::fmt::Debug for ShaderInfoTypeAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::STATISTICS_AMD => "STATISTICS_AMD",
            &Self::BINARY_AMD => "BINARY_AMD",
            &Self::DISASSEMBLY_AMD => "DISASSEMBLY_AMD",
            _ => "(unknown)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderResourceUsageAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderResourceUsageAMD {
    pub num_used_vgprs: u32,
    pub num_used_sgprs: u32,
    pub lds_size_per_local_work_group: u32,
    pub lds_usage_size_in_bytes: usize,
    pub scratch_mem_usage_in_bytes: usize,
}
impl ShaderResourceUsageAMD {
    #[inline]
    pub fn builder<'a>(self) -> ShaderResourceUsageAMDBuilder<'a> {
        ShaderResourceUsageAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ShaderResourceUsageAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ShaderResourceUsageAMD")
            .field("num_used_vgprs", &self.num_used_vgprs)
            .field("num_used_sgprs", &self.num_used_sgprs)
            .field(
                "lds_size_per_local_work_group",
                &self.lds_size_per_local_work_group,
            )
            .field("lds_usage_size_in_bytes", &self.lds_usage_size_in_bytes)
            .field(
                "scratch_mem_usage_in_bytes",
                &self.scratch_mem_usage_in_bytes,
            )
            .finish()
    }
}
impl Default for ShaderResourceUsageAMD {
    fn default() -> ShaderResourceUsageAMD {
        ShaderResourceUsageAMD {
            num_used_vgprs: Default::default(),
            num_used_sgprs: Default::default(),
            lds_size_per_local_work_group: Default::default(),
            lds_usage_size_in_bytes: Default::default(),
            scratch_mem_usage_in_bytes: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderResourceUsageAMD.html) · Builder of [`ShaderResourceUsageAMD`](struct.ShaderResourceUsageAMD.html)"]
#[repr(transparent)]
pub struct ShaderResourceUsageAMDBuilder<'a>(
    ShaderResourceUsageAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ShaderResourceUsageAMDBuilder<'a> {
    #[inline]
    pub fn new() -> ShaderResourceUsageAMDBuilder<'a> {
        ShaderResourceUsageAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn num_used_vgprs(mut self, num_used_vgprs: u32) -> Self {
        self.0.num_used_vgprs = num_used_vgprs;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn num_used_sgprs(mut self, num_used_sgprs: u32) -> Self {
        self.0.num_used_sgprs = num_used_sgprs;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn lds_size_per_local_work_group(mut self, lds_size_per_local_work_group: u32) -> Self {
        self.0.lds_size_per_local_work_group = lds_size_per_local_work_group;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn lds_usage_size_in_bytes(mut self, lds_usage_size_in_bytes: usize) -> Self {
        self.0.lds_usage_size_in_bytes = lds_usage_size_in_bytes;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn scratch_mem_usage_in_bytes(mut self, scratch_mem_usage_in_bytes: usize) -> Self {
        self.0.scratch_mem_usage_in_bytes = scratch_mem_usage_in_bytes;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ShaderResourceUsageAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for ShaderResourceUsageAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ShaderResourceUsageAMDBuilder<'a> {
    type Target = ShaderResourceUsageAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ShaderResourceUsageAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStatisticsInfoAMD.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderStatisticsInfoAMD {
    pub shader_stage_mask: crate::vk1_0::ShaderStageFlags,
    pub resource_usage: crate::extensions::amd_shader_info::ShaderResourceUsageAMD,
    pub num_physical_vgprs: u32,
    pub num_physical_sgprs: u32,
    pub num_available_vgprs: u32,
    pub num_available_sgprs: u32,
    pub compute_work_group_size: [u32; 3],
}
impl ShaderStatisticsInfoAMD {
    #[inline]
    pub fn builder<'a>(self) -> ShaderStatisticsInfoAMDBuilder<'a> {
        ShaderStatisticsInfoAMDBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ShaderStatisticsInfoAMD {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ShaderStatisticsInfoAMD")
            .field("shader_stage_mask", &self.shader_stage_mask)
            .field("resource_usage", &self.resource_usage)
            .field("num_physical_vgprs", &self.num_physical_vgprs)
            .field("num_physical_sgprs", &self.num_physical_sgprs)
            .field("num_available_vgprs", &self.num_available_vgprs)
            .field("num_available_sgprs", &self.num_available_sgprs)
            .field("compute_work_group_size", &self.compute_work_group_size)
            .finish()
    }
}
impl Default for ShaderStatisticsInfoAMD {
    fn default() -> ShaderStatisticsInfoAMD {
        ShaderStatisticsInfoAMD {
            shader_stage_mask: Default::default(),
            resource_usage: Default::default(),
            num_physical_vgprs: Default::default(),
            num_physical_sgprs: Default::default(),
            num_available_vgprs: Default::default(),
            num_available_sgprs: Default::default(),
            compute_work_group_size: Default::default(),
        }
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStatisticsInfoAMD.html) · Builder of [`ShaderStatisticsInfoAMD`](struct.ShaderStatisticsInfoAMD.html)"]
#[repr(transparent)]
pub struct ShaderStatisticsInfoAMDBuilder<'a>(
    ShaderStatisticsInfoAMD,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ShaderStatisticsInfoAMDBuilder<'a> {
    #[inline]
    pub fn new() -> ShaderStatisticsInfoAMDBuilder<'a> {
        ShaderStatisticsInfoAMDBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn shader_stage_mask(mut self, shader_stage_mask: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.shader_stage_mask = shader_stage_mask;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn resource_usage(
        mut self,
        resource_usage: crate::extensions::amd_shader_info::ShaderResourceUsageAMD,
    ) -> Self {
        self.0.resource_usage = resource_usage;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn num_physical_vgprs(mut self, num_physical_vgprs: u32) -> Self {
        self.0.num_physical_vgprs = num_physical_vgprs;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn num_physical_sgprs(mut self, num_physical_sgprs: u32) -> Self {
        self.0.num_physical_sgprs = num_physical_sgprs;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn num_available_vgprs(mut self, num_available_vgprs: u32) -> Self {
        self.0.num_available_vgprs = num_available_vgprs;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn num_available_sgprs(mut self, num_available_sgprs: u32) -> Self {
        self.0.num_available_sgprs = num_available_sgprs;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn compute_work_group_size(mut self, compute_work_group_size: [u32; 3]) -> Self {
        self.0.compute_work_group_size = compute_work_group_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ShaderStatisticsInfoAMD {
        self.0
    }
}
impl<'a> std::fmt::Debug for ShaderStatisticsInfoAMDBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ShaderStatisticsInfoAMDBuilder<'a> {
    type Target = ShaderStatisticsInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ShaderStatisticsInfoAMDBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
