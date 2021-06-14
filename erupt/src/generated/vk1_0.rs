#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_PHYSICAL_DEVICE_NAME_SIZE")]
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_UUID_SIZE")]
pub const UUID_SIZE: u32 = 16;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_EXTENSION_NAME_SIZE")]
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_DESCRIPTION_SIZE")]
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_MEMORY_TYPES")]
pub const MAX_MEMORY_TYPES: u32 = 32;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_MAX_MEMORY_HEAPS")]
pub const MAX_MEMORY_HEAPS: u32 = 16;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_LOD_CLAMP_NONE")]
pub const LOD_CLAMP_NONE: f32 = 1000.0;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_REMAINING_MIP_LEVELS")]
pub const REMAINING_MIP_LEVELS: u32 = 4294967295;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_REMAINING_ARRAY_LAYERS")]
pub const REMAINING_ARRAY_LAYERS: u32 = 4294967295;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_WHOLE_SIZE")]
pub const WHOLE_SIZE: u64 = 18446744073709551615;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_ATTACHMENT_UNUSED")]
pub const ATTACHMENT_UNUSED: u32 = 4294967295;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_TRUE")]
pub const TRUE: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_FALSE")]
pub const FALSE: u32 = 0;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QUEUE_FAMILY_IGNORED")]
pub const QUEUE_FAMILY_IGNORED: u32 = 4294967295;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_SUBPASS_EXTERNAL")]
pub const SUBPASS_EXTERNAL: u32 = 4294967295;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_INSTANCE: *const std::os::raw::c_char = crate::cstr!("vkCreateInstance");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_INSTANCE: *const std::os::raw::c_char = crate::cstr!("vkDestroyInstance");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_PHYSICAL_DEVICES: *const std::os::raw::c_char = crate::cstr!("vkEnumeratePhysicalDevices");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_PROC_ADDR: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceProcAddr");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_INSTANCE_PROC_ADDR: *const std::os::raw::c_char = crate::cstr!("vkGetInstanceProcAddr");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceQueueFamilyProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceMemoryProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_FEATURES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceFeatures");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceFormatProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceImageFormatProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DEVICE: *const std::os::raw::c_char = crate::cstr!("vkCreateDevice");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DEVICE: *const std::os::raw::c_char = crate::cstr!("vkDestroyDevice");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_INSTANCE_LAYER_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkEnumerateInstanceLayerProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_INSTANCE_EXTENSION_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkEnumerateInstanceExtensionProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_DEVICE_LAYER_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkEnumerateDeviceLayerProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ENUMERATE_DEVICE_EXTENSION_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkEnumerateDeviceExtensionProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_QUEUE: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceQueue");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_SUBMIT: *const std::os::raw::c_char = crate::cstr!("vkQueueSubmit");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_WAIT_IDLE: *const std::os::raw::c_char = crate::cstr!("vkQueueWaitIdle");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DEVICE_WAIT_IDLE: *const std::os::raw::c_char = crate::cstr!("vkDeviceWaitIdle");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ALLOCATE_MEMORY: *const std::os::raw::c_char = crate::cstr!("vkAllocateMemory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_FREE_MEMORY: *const std::os::raw::c_char = crate::cstr!("vkFreeMemory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_MAP_MEMORY: *const std::os::raw::c_char = crate::cstr!("vkMapMemory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_UNMAP_MEMORY: *const std::os::raw::c_char = crate::cstr!("vkUnmapMemory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_FLUSH_MAPPED_MEMORY_RANGES: *const std::os::raw::c_char = crate::cstr!("vkFlushMappedMemoryRanges");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_INVALIDATE_MAPPED_MEMORY_RANGES: *const std::os::raw::c_char = crate::cstr!("vkInvalidateMappedMemoryRanges");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DEVICE_MEMORY_COMMITMENT: *const std::os::raw::c_char = crate::cstr!("vkGetDeviceMemoryCommitment");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_BUFFER_MEMORY_REQUIREMENTS: *const std::os::raw::c_char = crate::cstr!("vkGetBufferMemoryRequirements");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_BUFFER_MEMORY: *const std::os::raw::c_char = crate::cstr!("vkBindBufferMemory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_MEMORY_REQUIREMENTS: *const std::os::raw::c_char = crate::cstr!("vkGetImageMemoryRequirements");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BIND_IMAGE_MEMORY: *const std::os::raw::c_char = crate::cstr!("vkBindImageMemory");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS: *const std::os::raw::c_char = crate::cstr!("vkGetImageSparseMemoryRequirements");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES: *const std::os::raw::c_char = crate::cstr!("vkGetPhysicalDeviceSparseImageFormatProperties");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_QUEUE_BIND_SPARSE: *const std::os::raw::c_char = crate::cstr!("vkQueueBindSparse");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_FENCE: *const std::os::raw::c_char = crate::cstr!("vkCreateFence");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_FENCE: *const std::os::raw::c_char = crate::cstr!("vkDestroyFence");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RESET_FENCES: *const std::os::raw::c_char = crate::cstr!("vkResetFences");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_FENCE_STATUS: *const std::os::raw::c_char = crate::cstr!("vkGetFenceStatus");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_WAIT_FOR_FENCES: *const std::os::raw::c_char = crate::cstr!("vkWaitForFences");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_SEMAPHORE: *const std::os::raw::c_char = crate::cstr!("vkCreateSemaphore");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_SEMAPHORE: *const std::os::raw::c_char = crate::cstr!("vkDestroySemaphore");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_EVENT: *const std::os::raw::c_char = crate::cstr!("vkCreateEvent");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_EVENT: *const std::os::raw::c_char = crate::cstr!("vkDestroyEvent");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_EVENT_STATUS: *const std::os::raw::c_char = crate::cstr!("vkGetEventStatus");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_SET_EVENT: *const std::os::raw::c_char = crate::cstr!("vkSetEvent");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RESET_EVENT: *const std::os::raw::c_char = crate::cstr!("vkResetEvent");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_QUERY_POOL: *const std::os::raw::c_char = crate::cstr!("vkCreateQueryPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_QUERY_POOL: *const std::os::raw::c_char = crate::cstr!("vkDestroyQueryPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_QUERY_POOL_RESULTS: *const std::os::raw::c_char = crate::cstr!("vkGetQueryPoolResults");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkCreateBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkDestroyBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_BUFFER_VIEW: *const std::os::raw::c_char = crate::cstr!("vkCreateBufferView");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_BUFFER_VIEW: *const std::os::raw::c_char = crate::cstr!("vkDestroyBufferView");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkCreateImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkDestroyImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_IMAGE_SUBRESOURCE_LAYOUT: *const std::os::raw::c_char = crate::cstr!("vkGetImageSubresourceLayout");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_IMAGE_VIEW: *const std::os::raw::c_char = crate::cstr!("vkCreateImageView");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_IMAGE_VIEW: *const std::os::raw::c_char = crate::cstr!("vkDestroyImageView");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_SHADER_MODULE: *const std::os::raw::c_char = crate::cstr!("vkCreateShaderModule");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_SHADER_MODULE: *const std::os::raw::c_char = crate::cstr!("vkDestroyShaderModule");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_PIPELINE_CACHE: *const std::os::raw::c_char = crate::cstr!("vkCreatePipelineCache");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_PIPELINE_CACHE: *const std::os::raw::c_char = crate::cstr!("vkDestroyPipelineCache");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PIPELINE_CACHE_DATA: *const std::os::raw::c_char = crate::cstr!("vkGetPipelineCacheData");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_MERGE_PIPELINE_CACHES: *const std::os::raw::c_char = crate::cstr!("vkMergePipelineCaches");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_GRAPHICS_PIPELINES: *const std::os::raw::c_char = crate::cstr!("vkCreateGraphicsPipelines");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_COMPUTE_PIPELINES: *const std::os::raw::c_char = crate::cstr!("vkCreateComputePipelines");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_PIPELINE: *const std::os::raw::c_char = crate::cstr!("vkDestroyPipeline");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_PIPELINE_LAYOUT: *const std::os::raw::c_char = crate::cstr!("vkCreatePipelineLayout");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_PIPELINE_LAYOUT: *const std::os::raw::c_char = crate::cstr!("vkDestroyPipelineLayout");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_SAMPLER: *const std::os::raw::c_char = crate::cstr!("vkCreateSampler");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_SAMPLER: *const std::os::raw::c_char = crate::cstr!("vkDestroySampler");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DESCRIPTOR_SET_LAYOUT: *const std::os::raw::c_char = crate::cstr!("vkCreateDescriptorSetLayout");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DESCRIPTOR_SET_LAYOUT: *const std::os::raw::c_char = crate::cstr!("vkDestroyDescriptorSetLayout");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_DESCRIPTOR_POOL: *const std::os::raw::c_char = crate::cstr!("vkCreateDescriptorPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_DESCRIPTOR_POOL: *const std::os::raw::c_char = crate::cstr!("vkDestroyDescriptorPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RESET_DESCRIPTOR_POOL: *const std::os::raw::c_char = crate::cstr!("vkResetDescriptorPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ALLOCATE_DESCRIPTOR_SETS: *const std::os::raw::c_char = crate::cstr!("vkAllocateDescriptorSets");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_FREE_DESCRIPTOR_SETS: *const std::os::raw::c_char = crate::cstr!("vkFreeDescriptorSets");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_UPDATE_DESCRIPTOR_SETS: *const std::os::raw::c_char = crate::cstr!("vkUpdateDescriptorSets");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_FRAMEBUFFER: *const std::os::raw::c_char = crate::cstr!("vkCreateFramebuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_FRAMEBUFFER: *const std::os::raw::c_char = crate::cstr!("vkDestroyFramebuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_RENDER_PASS: *const std::os::raw::c_char = crate::cstr!("vkCreateRenderPass");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_RENDER_PASS: *const std::os::raw::c_char = crate::cstr!("vkDestroyRenderPass");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_RENDER_AREA_GRANULARITY: *const std::os::raw::c_char = crate::cstr!("vkGetRenderAreaGranularity");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CREATE_COMMAND_POOL: *const std::os::raw::c_char = crate::cstr!("vkCreateCommandPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_DESTROY_COMMAND_POOL: *const std::os::raw::c_char = crate::cstr!("vkDestroyCommandPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RESET_COMMAND_POOL: *const std::os::raw::c_char = crate::cstr!("vkResetCommandPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ALLOCATE_COMMAND_BUFFERS: *const std::os::raw::c_char = crate::cstr!("vkAllocateCommandBuffers");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_FREE_COMMAND_BUFFERS: *const std::os::raw::c_char = crate::cstr!("vkFreeCommandBuffers");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_BEGIN_COMMAND_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkBeginCommandBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_END_COMMAND_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkEndCommandBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_RESET_COMMAND_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkResetCommandBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_PIPELINE: *const std::os::raw::c_char = crate::cstr!("vkCmdBindPipeline");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_VIEWPORT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetViewport");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_SCISSOR: *const std::os::raw::c_char = crate::cstr!("vkCmdSetScissor");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_LINE_WIDTH: *const std::os::raw::c_char = crate::cstr!("vkCmdSetLineWidth");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEPTH_BIAS: *const std::os::raw::c_char = crate::cstr!("vkCmdSetDepthBias");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_BLEND_CONSTANTS: *const std::os::raw::c_char = crate::cstr!("vkCmdSetBlendConstants");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEPTH_BOUNDS: *const std::os::raw::c_char = crate::cstr!("vkCmdSetDepthBounds");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_STENCIL_COMPARE_MASK: *const std::os::raw::c_char = crate::cstr!("vkCmdSetStencilCompareMask");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_STENCIL_WRITE_MASK: *const std::os::raw::c_char = crate::cstr!("vkCmdSetStencilWriteMask");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_STENCIL_REFERENCE: *const std::os::raw::c_char = crate::cstr!("vkCmdSetStencilReference");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_DESCRIPTOR_SETS: *const std::os::raw::c_char = crate::cstr!("vkCmdBindDescriptorSets");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_INDEX_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkCmdBindIndexBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_VERTEX_BUFFERS: *const std::os::raw::c_char = crate::cstr!("vkCmdBindVertexBuffers");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW: *const std::os::raw::c_char = crate::cstr!("vkCmdDraw");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_INDEXED: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawIndexed");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_INDIRECT: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawIndirect");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DRAW_INDEXED_INDIRECT: *const std::os::raw::c_char = crate::cstr!("vkCmdDrawIndexedIndirect");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DISPATCH: *const std::os::raw::c_char = crate::cstr!("vkCmdDispatch");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DISPATCH_INDIRECT: *const std::os::raw::c_char = crate::cstr!("vkCmdDispatchIndirect");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BLIT_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkCmdBlitImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_BUFFER_TO_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyBufferToImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_IMAGE_TO_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyImageToBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_UPDATE_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkCmdUpdateBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_FILL_BUFFER: *const std::os::raw::c_char = crate::cstr!("vkCmdFillBuffer");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_CLEAR_COLOR_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkCmdClearColorImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_CLEAR_DEPTH_STENCIL_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkCmdClearDepthStencilImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_CLEAR_ATTACHMENTS: *const std::os::raw::c_char = crate::cstr!("vkCmdClearAttachments");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_RESOLVE_IMAGE: *const std::os::raw::c_char = crate::cstr!("vkCmdResolveImage");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_EVENT: *const std::os::raw::c_char = crate::cstr!("vkCmdSetEvent");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_RESET_EVENT: *const std::os::raw::c_char = crate::cstr!("vkCmdResetEvent");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WAIT_EVENTS: *const std::os::raw::c_char = crate::cstr!("vkCmdWaitEvents");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_PIPELINE_BARRIER: *const std::os::raw::c_char = crate::cstr!("vkCmdPipelineBarrier");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_QUERY: *const std::os::raw::c_char = crate::cstr!("vkCmdBeginQuery");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_QUERY: *const std::os::raw::c_char = crate::cstr!("vkCmdEndQuery");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_RESET_QUERY_POOL: *const std::os::raw::c_char = crate::cstr!("vkCmdResetQueryPool");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_WRITE_TIMESTAMP: *const std::os::raw::c_char = crate::cstr!("vkCmdWriteTimestamp");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_QUERY_POOL_RESULTS: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyQueryPoolResults");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_PUSH_CONSTANTS: *const std::os::raw::c_char = crate::cstr!("vkCmdPushConstants");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BEGIN_RENDER_PASS: *const std::os::raw::c_char = crate::cstr!("vkCmdBeginRenderPass");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_NEXT_SUBPASS: *const std::os::raw::c_char = crate::cstr!("vkCmdNextSubpass");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_END_RENDER_PASS: *const std::os::raw::c_char = crate::cstr!("vkCmdEndRenderPass");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_EXECUTE_COMMANDS: *const std::os::raw::c_char = crate::cstr!("vkCmdExecuteCommands");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleMask.html) · Basetype"]
#[doc(alias = "VkSampleMask")]
pub type SampleMask = u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBool32.html) · Basetype"]
#[doc(alias = "VkBool32")]
pub type Bool32 = u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFlags.html) · Basetype"]
#[doc(alias = "VkFlags")]
pub type Flags = u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceSize.html) · Basetype"]
#[doc(alias = "VkDeviceSize")]
pub type DeviceSize = u64;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceAddress.html) · Basetype"]
#[doc(alias = "VkDeviceAddress")]
pub type DeviceAddress = u64;
crate::dispatchable_handle!(Instance, INSTANCE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstance.html) · Dispatchable Handle", "VkInstance");
crate::dispatchable_handle!(PhysicalDevice, PHYSICAL_DEVICE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevice.html) · Dispatchable Handle", "VkPhysicalDevice");
crate::dispatchable_handle!(Device, DEVICE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDevice.html) · Dispatchable Handle", "VkDevice");
crate::dispatchable_handle!(Queue, QUEUE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueue.html) · Dispatchable Handle", "VkQueue");
crate::dispatchable_handle!(CommandBuffer, COMMAND_BUFFER, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBuffer.html) · Dispatchable Handle", "VkCommandBuffer");
crate::non_dispatchable_handle!(DeviceMemory, DEVICE_MEMORY, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceMemory.html) · Non-dispatchable Handle", "VkDeviceMemory");
crate::non_dispatchable_handle!(CommandPool, COMMAND_POOL, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPool.html) · Non-dispatchable Handle", "VkCommandPool");
crate::non_dispatchable_handle!(Buffer, BUFFER, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBuffer.html) · Non-dispatchable Handle", "VkBuffer");
crate::non_dispatchable_handle!(BufferView, BUFFER_VIEW, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferView.html) · Non-dispatchable Handle", "VkBufferView");
crate::non_dispatchable_handle!(Image, IMAGE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImage.html) · Non-dispatchable Handle", "VkImage");
crate::non_dispatchable_handle!(ImageView, IMAGE_VIEW, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageView.html) · Non-dispatchable Handle", "VkImageView");
crate::non_dispatchable_handle!(ShaderModule, SHADER_MODULE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModule.html) · Non-dispatchable Handle", "VkShaderModule");
crate::non_dispatchable_handle!(Pipeline, PIPELINE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipeline.html) · Non-dispatchable Handle", "VkPipeline");
crate::non_dispatchable_handle!(PipelineLayout, PIPELINE_LAYOUT, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayout.html) · Non-dispatchable Handle", "VkPipelineLayout");
crate::non_dispatchable_handle!(Sampler, SAMPLER, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampler.html) · Non-dispatchable Handle", "VkSampler");
crate::non_dispatchable_handle!(DescriptorSet, DESCRIPTOR_SET, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSet.html) · Non-dispatchable Handle", "VkDescriptorSet");
crate::non_dispatchable_handle!(DescriptorSetLayout, DESCRIPTOR_SET_LAYOUT, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayout.html) · Non-dispatchable Handle", "VkDescriptorSetLayout");
crate::non_dispatchable_handle!(DescriptorPool, DESCRIPTOR_POOL, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPool.html) · Non-dispatchable Handle", "VkDescriptorPool");
crate::non_dispatchable_handle!(Fence, FENCE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFence.html) · Non-dispatchable Handle", "VkFence");
crate::non_dispatchable_handle!(Semaphore, SEMAPHORE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphore.html) · Non-dispatchable Handle", "VkSemaphore");
crate::non_dispatchable_handle!(Event, EVENT, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEvent.html) · Non-dispatchable Handle", "VkEvent");
crate::non_dispatchable_handle!(QueryPool, QUERY_POOL, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPool.html) · Non-dispatchable Handle", "VkQueryPool");
crate::non_dispatchable_handle!(Framebuffer, FRAMEBUFFER, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebuffer.html) · Non-dispatchable Handle", "VkFramebuffer");
crate::non_dispatchable_handle!(RenderPass, RENDER_PASS, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPass.html) · Non-dispatchable Handle", "VkRenderPass");
crate::non_dispatchable_handle!(PipelineCache, PIPELINE_CACHE, "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCache.html) · Non-dispatchable Handle", "VkPipelineCache");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateFlags.html) · Bitmask of [`QueryPoolCreateFlagBits`]"] # [doc (alias = "VkQueryPoolCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct QueryPoolCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`QueryPoolCreateFlags`]"]
#[doc(alias = "VkQueryPoolCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueryPoolCreateFlagBits(pub u32);
impl QueryPoolCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> QueryPoolCreateFlags {
        QueryPoolCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for QueryPoolCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayoutCreateFlags.html) · Bitmask of [`PipelineLayoutCreateFlagBits`]"] # [doc (alias = "VkPipelineLayoutCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineLayoutCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineLayoutCreateFlags`]"]
#[doc(alias = "VkPipelineLayoutCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineLayoutCreateFlagBits(pub u32);
impl PipelineLayoutCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineLayoutCreateFlags {
        PipelineLayoutCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineLayoutCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDepthStencilStateCreateFlags.html) · Bitmask of [`PipelineDepthStencilStateCreateFlagBits`]"] # [doc (alias = "VkPipelineDepthStencilStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineDepthStencilStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineDepthStencilStateCreateFlags`]"]
#[doc(alias = "VkPipelineDepthStencilStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineDepthStencilStateCreateFlagBits(pub u32);
impl PipelineDepthStencilStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineDepthStencilStateCreateFlags {
        PipelineDepthStencilStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineDepthStencilStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateFlags.html) · Bitmask of [`PipelineDynamicStateCreateFlagBits`]"] # [doc (alias = "VkPipelineDynamicStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineDynamicStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineDynamicStateCreateFlags`]"]
#[doc(alias = "VkPipelineDynamicStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineDynamicStateCreateFlagBits(pub u32);
impl PipelineDynamicStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineDynamicStateCreateFlags {
        PipelineDynamicStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineDynamicStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendStateCreateFlags.html) · Bitmask of [`PipelineColorBlendStateCreateFlagBits`]"] # [doc (alias = "VkPipelineColorBlendStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineColorBlendStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineColorBlendStateCreateFlags`]"]
#[doc(alias = "VkPipelineColorBlendStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineColorBlendStateCreateFlagBits(pub u32);
impl PipelineColorBlendStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineColorBlendStateCreateFlags {
        PipelineColorBlendStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineColorBlendStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineMultisampleStateCreateFlags.html) · Bitmask of [`PipelineMultisampleStateCreateFlagBits`]"] # [doc (alias = "VkPipelineMultisampleStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineMultisampleStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineMultisampleStateCreateFlags`]"]
#[doc(alias = "VkPipelineMultisampleStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineMultisampleStateCreateFlagBits(pub u32);
impl PipelineMultisampleStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineMultisampleStateCreateFlags {
        PipelineMultisampleStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineMultisampleStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateCreateFlags.html) · Bitmask of [`PipelineRasterizationStateCreateFlagBits`]"] # [doc (alias = "VkPipelineRasterizationStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineRasterizationStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineRasterizationStateCreateFlags`]"]
#[doc(alias = "VkPipelineRasterizationStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineRasterizationStateCreateFlagBits(pub u32);
impl PipelineRasterizationStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineRasterizationStateCreateFlags {
        PipelineRasterizationStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineRasterizationStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportStateCreateFlags.html) · Bitmask of [`PipelineViewportStateCreateFlagBits`]"] # [doc (alias = "VkPipelineViewportStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineViewportStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineViewportStateCreateFlags`]"]
#[doc(alias = "VkPipelineViewportStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineViewportStateCreateFlagBits(pub u32);
impl PipelineViewportStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineViewportStateCreateFlags {
        PipelineViewportStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineViewportStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationStateCreateFlags.html) · Bitmask of [`PipelineTessellationStateCreateFlagBits`]"] # [doc (alias = "VkPipelineTessellationStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineTessellationStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineTessellationStateCreateFlags`]"]
#[doc(alias = "VkPipelineTessellationStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineTessellationStateCreateFlagBits(pub u32);
impl PipelineTessellationStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineTessellationStateCreateFlags {
        PipelineTessellationStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineTessellationStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInputAssemblyStateCreateFlags.html) · Bitmask of [`PipelineInputAssemblyStateCreateFlagBits`]"] # [doc (alias = "VkPipelineInputAssemblyStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineInputAssemblyStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineInputAssemblyStateCreateFlags`]"]
#[doc(alias = "VkPipelineInputAssemblyStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineInputAssemblyStateCreateFlagBits(pub u32);
impl PipelineInputAssemblyStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineInputAssemblyStateCreateFlags {
        PipelineInputAssemblyStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineInputAssemblyStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputStateCreateFlags.html) · Bitmask of [`PipelineVertexInputStateCreateFlagBits`]"] # [doc (alias = "VkPipelineVertexInputStateCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineVertexInputStateCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`PipelineVertexInputStateCreateFlags`]"]
#[doc(alias = "VkPipelineVertexInputStateCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineVertexInputStateCreateFlagBits(pub u32);
impl PipelineVertexInputStateCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineVertexInputStateCreateFlags {
        PipelineVertexInputStateCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineVertexInputStateCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferViewCreateFlags.html) · Bitmask of [`BufferViewCreateFlagBits`]"] # [doc (alias = "VkBufferViewCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct BufferViewCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`BufferViewCreateFlags`]"]
#[doc(alias = "VkBufferViewCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BufferViewCreateFlagBits(pub u32);
impl BufferViewCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> BufferViewCreateFlags {
        BufferViewCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for BufferViewCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstanceCreateFlags.html) · Bitmask of [`InstanceCreateFlagBits`]"] # [doc (alias = "VkInstanceCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct InstanceCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`InstanceCreateFlags`]"]
#[doc(alias = "VkInstanceCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct InstanceCreateFlagBits(pub u32);
impl InstanceCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> InstanceCreateFlags {
        InstanceCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for InstanceCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceCreateFlags.html) · Bitmask of [`DeviceCreateFlagBits`]"] # [doc (alias = "VkDeviceCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct DeviceCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DeviceCreateFlags`]"]
#[doc(alias = "VkDeviceCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DeviceCreateFlagBits(pub u32);
impl DeviceCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DeviceCreateFlags {
        DeviceCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DeviceCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreCreateFlags.html) · Bitmask of [`SemaphoreCreateFlagBits`]"] # [doc (alias = "VkSemaphoreCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct SemaphoreCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`SemaphoreCreateFlags`]"]
#[doc(alias = "VkSemaphoreCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SemaphoreCreateFlagBits(pub u32);
impl SemaphoreCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SemaphoreCreateFlags {
        SemaphoreCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SemaphoreCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryMapFlags.html) · Bitmask of [`MemoryMapFlagBits`]"] # [doc (alias = "VkMemoryMapFlags")] # [derive (Default)] # [repr (transparent)] pub struct MemoryMapFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`MemoryMapFlags`]"]
#[doc(alias = "VkMemoryMapFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MemoryMapFlagBits(pub u32);
impl MemoryMapFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MemoryMapFlags {
        MemoryMapFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MemoryMapFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolResetFlags.html) · Bitmask of [`DescriptorPoolResetFlagBits`]"] # [doc (alias = "VkDescriptorPoolResetFlags")] # [derive (Default)] # [repr (transparent)] pub struct DescriptorPoolResetFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`DescriptorPoolResetFlags`]"]
#[doc(alias = "VkDescriptorPoolResetFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DescriptorPoolResetFlagBits(pub u32);
impl DescriptorPoolResetFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DescriptorPoolResetFlags {
        DescriptorPoolResetFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DescriptorPoolResetFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentLoadOp.html) · Enum"]
#[doc(alias = "VkAttachmentLoadOp")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AttachmentLoadOp(pub i32);
impl std::fmt::Debug for AttachmentLoadOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::LOAD => "LOAD",
            &Self::CLEAR => "CLEAR",
            &Self::DONT_CARE => "DONT_CARE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::AttachmentLoadOp {
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentStoreOp.html) · Enum"]
#[doc(alias = "VkAttachmentStoreOp")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AttachmentStoreOp(pub i32);
impl std::fmt::Debug for AttachmentStoreOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::STORE => "STORE",
            &Self::DONT_CARE => "DONT_CARE",
            &Self::NONE_QCOM => "NONE_QCOM",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::AttachmentStoreOp {
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendFactor.html) · Enum"]
#[doc(alias = "VkBlendFactor")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BlendFactor(pub i32);
impl std::fmt::Debug for BlendFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ZERO => "ZERO",
            &Self::ONE => "ONE",
            &Self::SRC_COLOR => "SRC_COLOR",
            &Self::ONE_MINUS_SRC_COLOR => "ONE_MINUS_SRC_COLOR",
            &Self::DST_COLOR => "DST_COLOR",
            &Self::ONE_MINUS_DST_COLOR => "ONE_MINUS_DST_COLOR",
            &Self::SRC_ALPHA => "SRC_ALPHA",
            &Self::ONE_MINUS_SRC_ALPHA => "ONE_MINUS_SRC_ALPHA",
            &Self::DST_ALPHA => "DST_ALPHA",
            &Self::ONE_MINUS_DST_ALPHA => "ONE_MINUS_DST_ALPHA",
            &Self::CONSTANT_COLOR => "CONSTANT_COLOR",
            &Self::ONE_MINUS_CONSTANT_COLOR => "ONE_MINUS_CONSTANT_COLOR",
            &Self::CONSTANT_ALPHA => "CONSTANT_ALPHA",
            &Self::ONE_MINUS_CONSTANT_ALPHA => "ONE_MINUS_CONSTANT_ALPHA",
            &Self::SRC_ALPHA_SATURATE => "SRC_ALPHA_SATURATE",
            &Self::SRC1_COLOR => "SRC1_COLOR",
            &Self::ONE_MINUS_SRC1_COLOR => "ONE_MINUS_SRC1_COLOR",
            &Self::SRC1_ALPHA => "SRC1_ALPHA",
            &Self::ONE_MINUS_SRC1_ALPHA => "ONE_MINUS_SRC1_ALPHA",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::BlendFactor {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlendOp.html) · Enum"]
#[doc(alias = "VkBlendOp")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BlendOp(pub i32);
impl std::fmt::Debug for BlendOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ADD => "ADD",
            &Self::SUBTRACT => "SUBTRACT",
            &Self::REVERSE_SUBTRACT => "REVERSE_SUBTRACT",
            &Self::MIN => "MIN",
            &Self::MAX => "MAX",
            &Self::ZERO_EXT => "ZERO_EXT",
            &Self::SRC_EXT => "SRC_EXT",
            &Self::DST_EXT => "DST_EXT",
            &Self::SRC_OVER_EXT => "SRC_OVER_EXT",
            &Self::DST_OVER_EXT => "DST_OVER_EXT",
            &Self::SRC_IN_EXT => "SRC_IN_EXT",
            &Self::DST_IN_EXT => "DST_IN_EXT",
            &Self::SRC_OUT_EXT => "SRC_OUT_EXT",
            &Self::DST_OUT_EXT => "DST_OUT_EXT",
            &Self::SRC_ATOP_EXT => "SRC_ATOP_EXT",
            &Self::DST_ATOP_EXT => "DST_ATOP_EXT",
            &Self::XOR_EXT => "XOR_EXT",
            &Self::MULTIPLY_EXT => "MULTIPLY_EXT",
            &Self::SCREEN_EXT => "SCREEN_EXT",
            &Self::OVERLAY_EXT => "OVERLAY_EXT",
            &Self::DARKEN_EXT => "DARKEN_EXT",
            &Self::LIGHTEN_EXT => "LIGHTEN_EXT",
            &Self::COLORDODGE_EXT => "COLORDODGE_EXT",
            &Self::COLORBURN_EXT => "COLORBURN_EXT",
            &Self::HARDLIGHT_EXT => "HARDLIGHT_EXT",
            &Self::SOFTLIGHT_EXT => "SOFTLIGHT_EXT",
            &Self::DIFFERENCE_EXT => "DIFFERENCE_EXT",
            &Self::EXCLUSION_EXT => "EXCLUSION_EXT",
            &Self::INVERT_EXT => "INVERT_EXT",
            &Self::INVERT_RGB_EXT => "INVERT_RGB_EXT",
            &Self::LINEARDODGE_EXT => "LINEARDODGE_EXT",
            &Self::LINEARBURN_EXT => "LINEARBURN_EXT",
            &Self::VIVIDLIGHT_EXT => "VIVIDLIGHT_EXT",
            &Self::LINEARLIGHT_EXT => "LINEARLIGHT_EXT",
            &Self::PINLIGHT_EXT => "PINLIGHT_EXT",
            &Self::HARDMIX_EXT => "HARDMIX_EXT",
            &Self::HSL_HUE_EXT => "HSL_HUE_EXT",
            &Self::HSL_SATURATION_EXT => "HSL_SATURATION_EXT",
            &Self::HSL_COLOR_EXT => "HSL_COLOR_EXT",
            &Self::HSL_LUMINOSITY_EXT => "HSL_LUMINOSITY_EXT",
            &Self::PLUS_EXT => "PLUS_EXT",
            &Self::PLUS_CLAMPED_EXT => "PLUS_CLAMPED_EXT",
            &Self::PLUS_CLAMPED_ALPHA_EXT => "PLUS_CLAMPED_ALPHA_EXT",
            &Self::PLUS_DARKER_EXT => "PLUS_DARKER_EXT",
            &Self::MINUS_EXT => "MINUS_EXT",
            &Self::MINUS_CLAMPED_EXT => "MINUS_CLAMPED_EXT",
            &Self::CONTRAST_EXT => "CONTRAST_EXT",
            &Self::INVERT_OVG_EXT => "INVERT_OVG_EXT",
            &Self::RED_EXT => "RED_EXT",
            &Self::GREEN_EXT => "GREEN_EXT",
            &Self::BLUE_EXT => "BLUE_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::BlendOp {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBorderColor.html) · Enum"]
#[doc(alias = "VkBorderColor")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BorderColor(pub i32);
impl std::fmt::Debug for BorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FLOAT_TRANSPARENT_BLACK => "FLOAT_TRANSPARENT_BLACK",
            &Self::INT_TRANSPARENT_BLACK => "INT_TRANSPARENT_BLACK",
            &Self::FLOAT_OPAQUE_BLACK => "FLOAT_OPAQUE_BLACK",
            &Self::INT_OPAQUE_BLACK => "INT_OPAQUE_BLACK",
            &Self::FLOAT_OPAQUE_WHITE => "FLOAT_OPAQUE_WHITE",
            &Self::INT_OPAQUE_WHITE => "INT_OPAQUE_WHITE",
            &Self::FLOAT_CUSTOM_EXT => "FLOAT_CUSTOM_EXT",
            &Self::INT_CUSTOM_EXT => "INT_CUSTOM_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateFlags.html) · Bitmask of [`FramebufferCreateFlagBits`]"] # [doc (alias = "VkFramebufferCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct FramebufferCreateFlags : u32 { const IMAGELESS = FramebufferCreateFlagBits :: IMAGELESS . 0 ; const IMAGELESS_KHR = FramebufferCreateFlagBits :: IMAGELESS_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateFlagBits.html) · Bits enum of [`FramebufferCreateFlags`]"]
#[doc(alias = "VkFramebufferCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FramebufferCreateFlagBits(pub u32);
impl FramebufferCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> FramebufferCreateFlags {
        FramebufferCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for FramebufferCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::IMAGELESS => "IMAGELESS",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateFlags.html) · Bitmask of [`RenderPassCreateFlagBits`]"] # [doc (alias = "VkRenderPassCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct RenderPassCreateFlags : u32 { const TRANSFORM_QCOM = RenderPassCreateFlagBits :: TRANSFORM_QCOM . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateFlagBits.html) · Bits enum of [`RenderPassCreateFlags`]"]
#[doc(alias = "VkRenderPassCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct RenderPassCreateFlagBits(pub u32);
impl RenderPassCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> RenderPassCreateFlags {
        RenderPassCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for RenderPassCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TRANSFORM_QCOM => "TRANSFORM_QCOM",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateFlags.html) · Bitmask of [`SamplerCreateFlagBits`]"] # [doc (alias = "VkSamplerCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct SamplerCreateFlags : u32 { const SUBSAMPLED_EXT = SamplerCreateFlagBits :: SUBSAMPLED_EXT . 0 ; const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT = SamplerCreateFlagBits :: SUBSAMPLED_COARSE_RECONSTRUCTION_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateFlagBits.html) · Bits enum of [`SamplerCreateFlags`]"]
#[doc(alias = "VkSamplerCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SamplerCreateFlagBits(pub u32);
impl SamplerCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SamplerCreateFlags {
        SamplerCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SamplerCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SUBSAMPLED_EXT => "SUBSAMPLED_EXT",
            &Self::SUBSAMPLED_COARSE_RECONSTRUCTION_EXT => "SUBSAMPLED_COARSE_RECONSTRUCTION_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheHeaderVersion.html) · Enum"]
#[doc(alias = "VkPipelineCacheHeaderVersion")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCacheHeaderVersion(pub i32);
impl std::fmt::Debug for PipelineCacheHeaderVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ONE => "ONE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::PipelineCacheHeaderVersion {
    pub const ONE: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateFlags.html) · Bitmask of [`PipelineCacheCreateFlagBits`]"] # [doc (alias = "VkPipelineCacheCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineCacheCreateFlags : u32 { const EXTERNALLY_SYNCHRONIZED_EXT = PipelineCacheCreateFlagBits :: EXTERNALLY_SYNCHRONIZED_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateFlagBits.html) · Bits enum of [`PipelineCacheCreateFlags`]"]
#[doc(alias = "VkPipelineCacheCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCacheCreateFlagBits(pub u32);
impl PipelineCacheCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCacheCreateFlags {
        PipelineCacheCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCacheCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::EXTERNALLY_SYNCHRONIZED_EXT => "EXTERNALLY_SYNCHRONIZED_EXT",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateFlags.html) · Bitmask of [`PipelineShaderStageCreateFlagBits`]"] # [doc (alias = "VkPipelineShaderStageCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineShaderStageCreateFlags : u32 { const ALLOW_VARYING_SUBGROUP_SIZE_EXT = PipelineShaderStageCreateFlagBits :: ALLOW_VARYING_SUBGROUP_SIZE_EXT . 0 ; const REQUIRE_FULL_SUBGROUPS_EXT = PipelineShaderStageCreateFlagBits :: REQUIRE_FULL_SUBGROUPS_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateFlagBits.html) · Bits enum of [`PipelineShaderStageCreateFlags`]"]
#[doc(alias = "VkPipelineShaderStageCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineShaderStageCreateFlagBits(pub u32);
impl PipelineShaderStageCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineShaderStageCreateFlags {
        PipelineShaderStageCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineShaderStageCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ALLOW_VARYING_SUBGROUP_SIZE_EXT => "ALLOW_VARYING_SUBGROUP_SIZE_EXT",
            &Self::REQUIRE_FULL_SUBGROUPS_EXT => "REQUIRE_FULL_SUBGROUPS_EXT",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateFlags.html) · Bitmask of [`DescriptorSetLayoutCreateFlagBits`]"] # [doc (alias = "VkDescriptorSetLayoutCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct DescriptorSetLayoutCreateFlags : u32 { const UPDATE_AFTER_BIND_POOL = DescriptorSetLayoutCreateFlagBits :: UPDATE_AFTER_BIND_POOL . 0 ; const PUSH_DESCRIPTOR_KHR = DescriptorSetLayoutCreateFlagBits :: PUSH_DESCRIPTOR_KHR . 0 ; const HOST_ONLY_POOL_VALVE = DescriptorSetLayoutCreateFlagBits :: HOST_ONLY_POOL_VALVE . 0 ; const UPDATE_AFTER_BIND_POOL_EXT = DescriptorSetLayoutCreateFlagBits :: UPDATE_AFTER_BIND_POOL_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateFlagBits.html) · Bits enum of [`DescriptorSetLayoutCreateFlags`]"]
#[doc(alias = "VkDescriptorSetLayoutCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DescriptorSetLayoutCreateFlagBits(pub u32);
impl DescriptorSetLayoutCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DescriptorSetLayoutCreateFlags {
        DescriptorSetLayoutCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DescriptorSetLayoutCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UPDATE_AFTER_BIND_POOL => "UPDATE_AFTER_BIND_POOL",
            &Self::PUSH_DESCRIPTOR_KHR => "PUSH_DESCRIPTOR_KHR",
            &Self::HOST_ONLY_POOL_VALVE => "HOST_ONLY_POOL_VALVE",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateFlags.html) · Bitmask of [`DeviceQueueCreateFlagBits`]"] # [doc (alias = "VkDeviceQueueCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct DeviceQueueCreateFlags : u32 { const PROTECTED = DeviceQueueCreateFlagBits :: PROTECTED . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateFlagBits.html) · Bits enum of [`DeviceQueueCreateFlags`]"]
#[doc(alias = "VkDeviceQueueCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DeviceQueueCreateFlagBits(pub u32);
impl DeviceQueueCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DeviceQueueCreateFlags {
        DeviceQueueCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DeviceQueueCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PROTECTED => "PROTECTED",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateFlags.html) · Bitmask of [`BufferCreateFlagBits`]"] # [doc (alias = "VkBufferCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct BufferCreateFlags : u32 { const SPARSE_BINDING = BufferCreateFlagBits :: SPARSE_BINDING . 0 ; const SPARSE_RESIDENCY = BufferCreateFlagBits :: SPARSE_RESIDENCY . 0 ; const SPARSE_ALIASED = BufferCreateFlagBits :: SPARSE_ALIASED . 0 ; const PROTECTED = BufferCreateFlagBits :: PROTECTED . 0 ; const DEVICE_ADDRESS_CAPTURE_REPLAY = BufferCreateFlagBits :: DEVICE_ADDRESS_CAPTURE_REPLAY . 0 ; const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT = BufferCreateFlagBits :: DEVICE_ADDRESS_CAPTURE_REPLAY_EXT . 0 ; const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = BufferCreateFlagBits :: DEVICE_ADDRESS_CAPTURE_REPLAY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateFlagBits.html) · Bits enum of [`BufferCreateFlags`]"]
#[doc(alias = "VkBufferCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BufferCreateFlagBits(pub u32);
impl BufferCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> BufferCreateFlags {
        BufferCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for BufferCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SPARSE_BINDING => "SPARSE_BINDING",
            &Self::SPARSE_RESIDENCY => "SPARSE_RESIDENCY",
            &Self::SPARSE_ALIASED => "SPARSE_ALIASED",
            &Self::PROTECTED => "PROTECTED",
            &Self::DEVICE_ADDRESS_CAPTURE_REPLAY => "DEVICE_ADDRESS_CAPTURE_REPLAY",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::BufferCreateFlagBits {
    pub const SPARSE_BINDING: Self = Self(1);
    pub const SPARSE_RESIDENCY: Self = Self(2);
    pub const SPARSE_ALIASED: Self = Self(4);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferUsageFlags.html) · Bitmask of [`BufferUsageFlagBits`]"] # [doc (alias = "VkBufferUsageFlags")] # [derive (Default)] # [repr (transparent)] pub struct BufferUsageFlags : u32 { const TRANSFER_SRC = BufferUsageFlagBits :: TRANSFER_SRC . 0 ; const TRANSFER_DST = BufferUsageFlagBits :: TRANSFER_DST . 0 ; const UNIFORM_TEXEL_BUFFER = BufferUsageFlagBits :: UNIFORM_TEXEL_BUFFER . 0 ; const STORAGE_TEXEL_BUFFER = BufferUsageFlagBits :: STORAGE_TEXEL_BUFFER . 0 ; const UNIFORM_BUFFER = BufferUsageFlagBits :: UNIFORM_BUFFER . 0 ; const STORAGE_BUFFER = BufferUsageFlagBits :: STORAGE_BUFFER . 0 ; const INDEX_BUFFER = BufferUsageFlagBits :: INDEX_BUFFER . 0 ; const VERTEX_BUFFER = BufferUsageFlagBits :: VERTEX_BUFFER . 0 ; const INDIRECT_BUFFER = BufferUsageFlagBits :: INDIRECT_BUFFER . 0 ; const SHADER_DEVICE_ADDRESS = BufferUsageFlagBits :: SHADER_DEVICE_ADDRESS . 0 ; const VIDEO_DECODE_SRC_KHR = BufferUsageFlagBits :: VIDEO_DECODE_SRC_KHR . 0 ; const VIDEO_DECODE_DST_KHR = BufferUsageFlagBits :: VIDEO_DECODE_DST_KHR . 0 ; const TRANSFORM_FEEDBACK_BUFFER_EXT = BufferUsageFlagBits :: TRANSFORM_FEEDBACK_BUFFER_EXT . 0 ; const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT = BufferUsageFlagBits :: TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT . 0 ; const CONDITIONAL_RENDERING_EXT = BufferUsageFlagBits :: CONDITIONAL_RENDERING_EXT . 0 ; const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = BufferUsageFlagBits :: ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR . 0 ; const ACCELERATION_STRUCTURE_STORAGE_KHR = BufferUsageFlagBits :: ACCELERATION_STRUCTURE_STORAGE_KHR . 0 ; const SHADER_BINDING_TABLE_KHR = BufferUsageFlagBits :: SHADER_BINDING_TABLE_KHR . 0 ; const VIDEO_ENCODE_DST_KHR = BufferUsageFlagBits :: VIDEO_ENCODE_DST_KHR . 0 ; const VIDEO_ENCODE_SRC_KHR = BufferUsageFlagBits :: VIDEO_ENCODE_SRC_KHR . 0 ; const RAY_TRACING_NV = BufferUsageFlagBits :: RAY_TRACING_NV . 0 ; const SHADER_DEVICE_ADDRESS_EXT = BufferUsageFlagBits :: SHADER_DEVICE_ADDRESS_EXT . 0 ; const SHADER_DEVICE_ADDRESS_KHR = BufferUsageFlagBits :: SHADER_DEVICE_ADDRESS_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferUsageFlagBits.html) · Bits enum of [`BufferUsageFlags`]"]
#[doc(alias = "VkBufferUsageFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct BufferUsageFlagBits(pub u32);
impl BufferUsageFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> BufferUsageFlags {
        BufferUsageFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for BufferUsageFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TRANSFER_SRC => "TRANSFER_SRC",
            &Self::TRANSFER_DST => "TRANSFER_DST",
            &Self::UNIFORM_TEXEL_BUFFER => "UNIFORM_TEXEL_BUFFER",
            &Self::STORAGE_TEXEL_BUFFER => "STORAGE_TEXEL_BUFFER",
            &Self::UNIFORM_BUFFER => "UNIFORM_BUFFER",
            &Self::STORAGE_BUFFER => "STORAGE_BUFFER",
            &Self::INDEX_BUFFER => "INDEX_BUFFER",
            &Self::VERTEX_BUFFER => "VERTEX_BUFFER",
            &Self::INDIRECT_BUFFER => "INDIRECT_BUFFER",
            &Self::SHADER_DEVICE_ADDRESS => "SHADER_DEVICE_ADDRESS",
            &Self::VIDEO_DECODE_SRC_KHR => "VIDEO_DECODE_SRC_KHR",
            &Self::VIDEO_DECODE_DST_KHR => "VIDEO_DECODE_DST_KHR",
            &Self::TRANSFORM_FEEDBACK_BUFFER_EXT => "TRANSFORM_FEEDBACK_BUFFER_EXT",
            &Self::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT => "TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT",
            &Self::CONDITIONAL_RENDERING_EXT => "CONDITIONAL_RENDERING_EXT",
            &Self::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR => "ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR",
            &Self::ACCELERATION_STRUCTURE_STORAGE_KHR => "ACCELERATION_STRUCTURE_STORAGE_KHR",
            &Self::SHADER_BINDING_TABLE_KHR => "SHADER_BINDING_TABLE_KHR",
            &Self::VIDEO_ENCODE_DST_KHR => "VIDEO_ENCODE_DST_KHR",
            &Self::VIDEO_ENCODE_SRC_KHR => "VIDEO_ENCODE_SRC_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::BufferUsageFlagBits {
    pub const TRANSFER_SRC: Self = Self(1);
    pub const TRANSFER_DST: Self = Self(2);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(8);
    pub const UNIFORM_BUFFER: Self = Self(16);
    pub const STORAGE_BUFFER: Self = Self(32);
    pub const INDEX_BUFFER: Self = Self(64);
    pub const VERTEX_BUFFER: Self = Self(128);
    pub const INDIRECT_BUFFER: Self = Self(256);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorComponentFlags.html) · Bitmask of [`ColorComponentFlagBits`]"] # [doc (alias = "VkColorComponentFlags")] # [derive (Default)] # [repr (transparent)] pub struct ColorComponentFlags : u32 { const R = ColorComponentFlagBits :: R . 0 ; const G = ColorComponentFlagBits :: G . 0 ; const B = ColorComponentFlagBits :: B . 0 ; const A = ColorComponentFlagBits :: A . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkColorComponentFlagBits.html) · Bits enum of [`ColorComponentFlags`]"]
#[doc(alias = "VkColorComponentFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ColorComponentFlagBits(pub u32);
impl ColorComponentFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ColorComponentFlags {
        ColorComponentFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ColorComponentFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::R => "R",
            &Self::G => "G",
            &Self::B => "B",
            &Self::A => "A",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ColorComponentFlagBits {
    pub const R: Self = Self(1);
    pub const G: Self = Self(2);
    pub const B: Self = Self(4);
    pub const A: Self = Self(8);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentSwizzle.html) · Enum"]
#[doc(alias = "VkComponentSwizzle")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ComponentSwizzle(pub i32);
impl std::fmt::Debug for ComponentSwizzle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::IDENTITY => "IDENTITY",
            &Self::ZERO => "ZERO",
            &Self::ONE => "ONE",
            &Self::R => "R",
            &Self::G => "G",
            &Self::B => "B",
            &Self::A => "A",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ComponentSwizzle {
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateFlags.html) · Bitmask of [`CommandPoolCreateFlagBits`]"] # [doc (alias = "VkCommandPoolCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct CommandPoolCreateFlags : u32 { const TRANSIENT = CommandPoolCreateFlagBits :: TRANSIENT . 0 ; const RESET_COMMAND_BUFFER = CommandPoolCreateFlagBits :: RESET_COMMAND_BUFFER . 0 ; const PROTECTED = CommandPoolCreateFlagBits :: PROTECTED . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateFlagBits.html) · Bits enum of [`CommandPoolCreateFlags`]"]
#[doc(alias = "VkCommandPoolCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CommandPoolCreateFlagBits(pub u32);
impl CommandPoolCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CommandPoolCreateFlags {
        CommandPoolCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CommandPoolCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TRANSIENT => "TRANSIENT",
            &Self::RESET_COMMAND_BUFFER => "RESET_COMMAND_BUFFER",
            &Self::PROTECTED => "PROTECTED",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::CommandPoolCreateFlagBits {
    pub const TRANSIENT: Self = Self(1);
    pub const RESET_COMMAND_BUFFER: Self = Self(2);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolResetFlags.html) · Bitmask of [`CommandPoolResetFlagBits`]"] # [doc (alias = "VkCommandPoolResetFlags")] # [derive (Default)] # [repr (transparent)] pub struct CommandPoolResetFlags : u32 { const RELEASE_RESOURCES = CommandPoolResetFlagBits :: RELEASE_RESOURCES . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolResetFlagBits.html) · Bits enum of [`CommandPoolResetFlags`]"]
#[doc(alias = "VkCommandPoolResetFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CommandPoolResetFlagBits(pub u32);
impl CommandPoolResetFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CommandPoolResetFlags {
        CommandPoolResetFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CommandPoolResetFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::RELEASE_RESOURCES => "RELEASE_RESOURCES",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::CommandPoolResetFlagBits {
    pub const RELEASE_RESOURCES: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferResetFlags.html) · Bitmask of [`CommandBufferResetFlagBits`]"] # [doc (alias = "VkCommandBufferResetFlags")] # [derive (Default)] # [repr (transparent)] pub struct CommandBufferResetFlags : u32 { const RELEASE_RESOURCES = CommandBufferResetFlagBits :: RELEASE_RESOURCES . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferResetFlagBits.html) · Bits enum of [`CommandBufferResetFlags`]"]
#[doc(alias = "VkCommandBufferResetFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CommandBufferResetFlagBits(pub u32);
impl CommandBufferResetFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CommandBufferResetFlags {
        CommandBufferResetFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CommandBufferResetFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::RELEASE_RESOURCES => "RELEASE_RESOURCES",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::CommandBufferResetFlagBits {
    pub const RELEASE_RESOURCES: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferLevel.html) · Enum"]
#[doc(alias = "VkCommandBufferLevel")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CommandBufferLevel(pub i32);
impl std::fmt::Debug for CommandBufferLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PRIMARY => "PRIMARY",
            &Self::SECONDARY => "SECONDARY",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::CommandBufferLevel {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferUsageFlags.html) · Bitmask of [`CommandBufferUsageFlagBits`]"] # [doc (alias = "VkCommandBufferUsageFlags")] # [derive (Default)] # [repr (transparent)] pub struct CommandBufferUsageFlags : u32 { const ONE_TIME_SUBMIT = CommandBufferUsageFlagBits :: ONE_TIME_SUBMIT . 0 ; const RENDER_PASS_CONTINUE = CommandBufferUsageFlagBits :: RENDER_PASS_CONTINUE . 0 ; const SIMULTANEOUS_USE = CommandBufferUsageFlagBits :: SIMULTANEOUS_USE . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferUsageFlagBits.html) · Bits enum of [`CommandBufferUsageFlags`]"]
#[doc(alias = "VkCommandBufferUsageFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CommandBufferUsageFlagBits(pub u32);
impl CommandBufferUsageFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CommandBufferUsageFlags {
        CommandBufferUsageFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CommandBufferUsageFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::ONE_TIME_SUBMIT => "ONE_TIME_SUBMIT",
            &Self::RENDER_PASS_CONTINUE => "RENDER_PASS_CONTINUE",
            &Self::SIMULTANEOUS_USE => "SIMULTANEOUS_USE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::CommandBufferUsageFlagBits {
    pub const ONE_TIME_SUBMIT: Self = Self(1);
    pub const RENDER_PASS_CONTINUE: Self = Self(2);
    pub const SIMULTANEOUS_USE: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCompareOp.html) · Enum"]
#[doc(alias = "VkCompareOp")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CompareOp(pub i32);
impl std::fmt::Debug for CompareOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NEVER => "NEVER",
            &Self::LESS => "LESS",
            &Self::EQUAL => "EQUAL",
            &Self::LESS_OR_EQUAL => "LESS_OR_EQUAL",
            &Self::GREATER => "GREATER",
            &Self::NOT_EQUAL => "NOT_EQUAL",
            &Self::GREATER_OR_EQUAL => "GREATER_OR_EQUAL",
            &Self::ALWAYS => "ALWAYS",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::CompareOp {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCullModeFlags.html) · Bitmask of [`CullModeFlagBits`]"] # [doc (alias = "VkCullModeFlags")] # [derive (Default)] # [repr (transparent)] pub struct CullModeFlags : u32 { const NONE = CullModeFlagBits :: NONE . 0 ; const FRONT = CullModeFlagBits :: FRONT . 0 ; const BACK = CullModeFlagBits :: BACK . 0 ; const FRONT_AND_BACK = CullModeFlagBits :: FRONT_AND_BACK . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCullModeFlagBits.html) · Bits enum of [`CullModeFlags`]"]
#[doc(alias = "VkCullModeFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CullModeFlagBits(pub u32);
impl CullModeFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> CullModeFlags {
        CullModeFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for CullModeFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NONE => "NONE",
            &Self::FRONT => "FRONT",
            &Self::BACK => "BACK",
            &Self::FRONT_AND_BACK => "FRONT_AND_BACK",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::CullModeFlagBits {
    pub const NONE: Self = Self(0);
    pub const FRONT: Self = Self(1);
    pub const BACK: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorType.html) · Enum"]
#[doc(alias = "VkDescriptorType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DescriptorType(pub i32);
impl std::fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SAMPLER => "SAMPLER",
            &Self::COMBINED_IMAGE_SAMPLER => "COMBINED_IMAGE_SAMPLER",
            &Self::SAMPLED_IMAGE => "SAMPLED_IMAGE",
            &Self::STORAGE_IMAGE => "STORAGE_IMAGE",
            &Self::UNIFORM_TEXEL_BUFFER => "UNIFORM_TEXEL_BUFFER",
            &Self::STORAGE_TEXEL_BUFFER => "STORAGE_TEXEL_BUFFER",
            &Self::UNIFORM_BUFFER => "UNIFORM_BUFFER",
            &Self::STORAGE_BUFFER => "STORAGE_BUFFER",
            &Self::UNIFORM_BUFFER_DYNAMIC => "UNIFORM_BUFFER_DYNAMIC",
            &Self::STORAGE_BUFFER_DYNAMIC => "STORAGE_BUFFER_DYNAMIC",
            &Self::INPUT_ATTACHMENT => "INPUT_ATTACHMENT",
            &Self::INLINE_UNIFORM_BLOCK_EXT => "INLINE_UNIFORM_BLOCK_EXT",
            &Self::ACCELERATION_STRUCTURE_KHR => "ACCELERATION_STRUCTURE_KHR",
            &Self::ACCELERATION_STRUCTURE_NV => "ACCELERATION_STRUCTURE_NV",
            &Self::MUTABLE_VALVE => "MUTABLE_VALVE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::DescriptorType {
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDynamicState.html) · Enum"]
#[doc(alias = "VkDynamicState")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DynamicState(pub i32);
impl std::fmt::Debug for DynamicState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VIEWPORT => "VIEWPORT",
            &Self::SCISSOR => "SCISSOR",
            &Self::LINE_WIDTH => "LINE_WIDTH",
            &Self::DEPTH_BIAS => "DEPTH_BIAS",
            &Self::BLEND_CONSTANTS => "BLEND_CONSTANTS",
            &Self::DEPTH_BOUNDS => "DEPTH_BOUNDS",
            &Self::STENCIL_COMPARE_MASK => "STENCIL_COMPARE_MASK",
            &Self::STENCIL_WRITE_MASK => "STENCIL_WRITE_MASK",
            &Self::STENCIL_REFERENCE => "STENCIL_REFERENCE",
            &Self::VIEWPORT_W_SCALING_NV => "VIEWPORT_W_SCALING_NV",
            &Self::DISCARD_RECTANGLE_EXT => "DISCARD_RECTANGLE_EXT",
            &Self::SAMPLE_LOCATIONS_EXT => "SAMPLE_LOCATIONS_EXT",
            &Self::RAY_TRACING_PIPELINE_STACK_SIZE_KHR => "RAY_TRACING_PIPELINE_STACK_SIZE_KHR",
            &Self::VIEWPORT_SHADING_RATE_PALETTE_NV => "VIEWPORT_SHADING_RATE_PALETTE_NV",
            &Self::VIEWPORT_COARSE_SAMPLE_ORDER_NV => "VIEWPORT_COARSE_SAMPLE_ORDER_NV",
            &Self::EXCLUSIVE_SCISSOR_NV => "EXCLUSIVE_SCISSOR_NV",
            &Self::FRAGMENT_SHADING_RATE_KHR => "FRAGMENT_SHADING_RATE_KHR",
            &Self::LINE_STIPPLE_EXT => "LINE_STIPPLE_EXT",
            &Self::CULL_MODE_EXT => "CULL_MODE_EXT",
            &Self::FRONT_FACE_EXT => "FRONT_FACE_EXT",
            &Self::PRIMITIVE_TOPOLOGY_EXT => "PRIMITIVE_TOPOLOGY_EXT",
            &Self::VIEWPORT_WITH_COUNT_EXT => "VIEWPORT_WITH_COUNT_EXT",
            &Self::SCISSOR_WITH_COUNT_EXT => "SCISSOR_WITH_COUNT_EXT",
            &Self::VERTEX_INPUT_BINDING_STRIDE_EXT => "VERTEX_INPUT_BINDING_STRIDE_EXT",
            &Self::DEPTH_TEST_ENABLE_EXT => "DEPTH_TEST_ENABLE_EXT",
            &Self::DEPTH_WRITE_ENABLE_EXT => "DEPTH_WRITE_ENABLE_EXT",
            &Self::DEPTH_COMPARE_OP_EXT => "DEPTH_COMPARE_OP_EXT",
            &Self::DEPTH_BOUNDS_TEST_ENABLE_EXT => "DEPTH_BOUNDS_TEST_ENABLE_EXT",
            &Self::STENCIL_TEST_ENABLE_EXT => "STENCIL_TEST_ENABLE_EXT",
            &Self::STENCIL_OP_EXT => "STENCIL_OP_EXT",
            &Self::VERTEX_INPUT_EXT => "VERTEX_INPUT_EXT",
            &Self::PATCH_CONTROL_POINTS_EXT => "PATCH_CONTROL_POINTS_EXT",
            &Self::RASTERIZER_DISCARD_ENABLE_EXT => "RASTERIZER_DISCARD_ENABLE_EXT",
            &Self::DEPTH_BIAS_ENABLE_EXT => "DEPTH_BIAS_ENABLE_EXT",
            &Self::LOGIC_OP_EXT => "LOGIC_OP_EXT",
            &Self::PRIMITIVE_RESTART_ENABLE_EXT => "PRIMITIVE_RESTART_ENABLE_EXT",
            &Self::COLOR_WRITE_ENABLE_EXT => "COLOR_WRITE_ENABLE_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::DynamicState {
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceCreateFlags.html) · Bitmask of [`FenceCreateFlagBits`]"] # [doc (alias = "VkFenceCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct FenceCreateFlags : u32 { const SIGNALED = FenceCreateFlagBits :: SIGNALED . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceCreateFlagBits.html) · Bits enum of [`FenceCreateFlags`]"]
#[doc(alias = "VkFenceCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FenceCreateFlagBits(pub u32);
impl FenceCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> FenceCreateFlags {
        FenceCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for FenceCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SIGNALED => "SIGNALED",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::FenceCreateFlagBits {
    pub const SIGNALED: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPolygonMode.html) · Enum"]
#[doc(alias = "VkPolygonMode")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PolygonMode(pub i32);
impl std::fmt::Debug for PolygonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FILL => "FILL",
            &Self::LINE => "LINE",
            &Self::POINT => "POINT",
            &Self::FILL_RECTANGLE_NV => "FILL_RECTANGLE_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::PolygonMode {
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormat.html) · Enum"]
#[doc(alias = "VkFormat")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Format(pub i32);
impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNDEFINED => "UNDEFINED",
            &Self::R4G4_UNORM_PACK8 => "R4G4_UNORM_PACK8",
            &Self::R4G4B4A4_UNORM_PACK16 => "R4G4B4A4_UNORM_PACK16",
            &Self::B4G4R4A4_UNORM_PACK16 => "B4G4R4A4_UNORM_PACK16",
            &Self::R5G6B5_UNORM_PACK16 => "R5G6B5_UNORM_PACK16",
            &Self::B5G6R5_UNORM_PACK16 => "B5G6R5_UNORM_PACK16",
            &Self::R5G5B5A1_UNORM_PACK16 => "R5G5B5A1_UNORM_PACK16",
            &Self::B5G5R5A1_UNORM_PACK16 => "B5G5R5A1_UNORM_PACK16",
            &Self::A1R5G5B5_UNORM_PACK16 => "A1R5G5B5_UNORM_PACK16",
            &Self::R8_UNORM => "R8_UNORM",
            &Self::R8_SNORM => "R8_SNORM",
            &Self::R8_USCALED => "R8_USCALED",
            &Self::R8_SSCALED => "R8_SSCALED",
            &Self::R8_UINT => "R8_UINT",
            &Self::R8_SINT => "R8_SINT",
            &Self::R8_SRGB => "R8_SRGB",
            &Self::R8G8_UNORM => "R8G8_UNORM",
            &Self::R8G8_SNORM => "R8G8_SNORM",
            &Self::R8G8_USCALED => "R8G8_USCALED",
            &Self::R8G8_SSCALED => "R8G8_SSCALED",
            &Self::R8G8_UINT => "R8G8_UINT",
            &Self::R8G8_SINT => "R8G8_SINT",
            &Self::R8G8_SRGB => "R8G8_SRGB",
            &Self::R8G8B8_UNORM => "R8G8B8_UNORM",
            &Self::R8G8B8_SNORM => "R8G8B8_SNORM",
            &Self::R8G8B8_USCALED => "R8G8B8_USCALED",
            &Self::R8G8B8_SSCALED => "R8G8B8_SSCALED",
            &Self::R8G8B8_UINT => "R8G8B8_UINT",
            &Self::R8G8B8_SINT => "R8G8B8_SINT",
            &Self::R8G8B8_SRGB => "R8G8B8_SRGB",
            &Self::B8G8R8_UNORM => "B8G8R8_UNORM",
            &Self::B8G8R8_SNORM => "B8G8R8_SNORM",
            &Self::B8G8R8_USCALED => "B8G8R8_USCALED",
            &Self::B8G8R8_SSCALED => "B8G8R8_SSCALED",
            &Self::B8G8R8_UINT => "B8G8R8_UINT",
            &Self::B8G8R8_SINT => "B8G8R8_SINT",
            &Self::B8G8R8_SRGB => "B8G8R8_SRGB",
            &Self::R8G8B8A8_UNORM => "R8G8B8A8_UNORM",
            &Self::R8G8B8A8_SNORM => "R8G8B8A8_SNORM",
            &Self::R8G8B8A8_USCALED => "R8G8B8A8_USCALED",
            &Self::R8G8B8A8_SSCALED => "R8G8B8A8_SSCALED",
            &Self::R8G8B8A8_UINT => "R8G8B8A8_UINT",
            &Self::R8G8B8A8_SINT => "R8G8B8A8_SINT",
            &Self::R8G8B8A8_SRGB => "R8G8B8A8_SRGB",
            &Self::B8G8R8A8_UNORM => "B8G8R8A8_UNORM",
            &Self::B8G8R8A8_SNORM => "B8G8R8A8_SNORM",
            &Self::B8G8R8A8_USCALED => "B8G8R8A8_USCALED",
            &Self::B8G8R8A8_SSCALED => "B8G8R8A8_SSCALED",
            &Self::B8G8R8A8_UINT => "B8G8R8A8_UINT",
            &Self::B8G8R8A8_SINT => "B8G8R8A8_SINT",
            &Self::B8G8R8A8_SRGB => "B8G8R8A8_SRGB",
            &Self::A8B8G8R8_UNORM_PACK32 => "A8B8G8R8_UNORM_PACK32",
            &Self::A8B8G8R8_SNORM_PACK32 => "A8B8G8R8_SNORM_PACK32",
            &Self::A8B8G8R8_USCALED_PACK32 => "A8B8G8R8_USCALED_PACK32",
            &Self::A8B8G8R8_SSCALED_PACK32 => "A8B8G8R8_SSCALED_PACK32",
            &Self::A8B8G8R8_UINT_PACK32 => "A8B8G8R8_UINT_PACK32",
            &Self::A8B8G8R8_SINT_PACK32 => "A8B8G8R8_SINT_PACK32",
            &Self::A8B8G8R8_SRGB_PACK32 => "A8B8G8R8_SRGB_PACK32",
            &Self::A2R10G10B10_UNORM_PACK32 => "A2R10G10B10_UNORM_PACK32",
            &Self::A2R10G10B10_SNORM_PACK32 => "A2R10G10B10_SNORM_PACK32",
            &Self::A2R10G10B10_USCALED_PACK32 => "A2R10G10B10_USCALED_PACK32",
            &Self::A2R10G10B10_SSCALED_PACK32 => "A2R10G10B10_SSCALED_PACK32",
            &Self::A2R10G10B10_UINT_PACK32 => "A2R10G10B10_UINT_PACK32",
            &Self::A2R10G10B10_SINT_PACK32 => "A2R10G10B10_SINT_PACK32",
            &Self::A2B10G10R10_UNORM_PACK32 => "A2B10G10R10_UNORM_PACK32",
            &Self::A2B10G10R10_SNORM_PACK32 => "A2B10G10R10_SNORM_PACK32",
            &Self::A2B10G10R10_USCALED_PACK32 => "A2B10G10R10_USCALED_PACK32",
            &Self::A2B10G10R10_SSCALED_PACK32 => "A2B10G10R10_SSCALED_PACK32",
            &Self::A2B10G10R10_UINT_PACK32 => "A2B10G10R10_UINT_PACK32",
            &Self::A2B10G10R10_SINT_PACK32 => "A2B10G10R10_SINT_PACK32",
            &Self::R16_UNORM => "R16_UNORM",
            &Self::R16_SNORM => "R16_SNORM",
            &Self::R16_USCALED => "R16_USCALED",
            &Self::R16_SSCALED => "R16_SSCALED",
            &Self::R16_UINT => "R16_UINT",
            &Self::R16_SINT => "R16_SINT",
            &Self::R16_SFLOAT => "R16_SFLOAT",
            &Self::R16G16_UNORM => "R16G16_UNORM",
            &Self::R16G16_SNORM => "R16G16_SNORM",
            &Self::R16G16_USCALED => "R16G16_USCALED",
            &Self::R16G16_SSCALED => "R16G16_SSCALED",
            &Self::R16G16_UINT => "R16G16_UINT",
            &Self::R16G16_SINT => "R16G16_SINT",
            &Self::R16G16_SFLOAT => "R16G16_SFLOAT",
            &Self::R16G16B16_UNORM => "R16G16B16_UNORM",
            &Self::R16G16B16_SNORM => "R16G16B16_SNORM",
            &Self::R16G16B16_USCALED => "R16G16B16_USCALED",
            &Self::R16G16B16_SSCALED => "R16G16B16_SSCALED",
            &Self::R16G16B16_UINT => "R16G16B16_UINT",
            &Self::R16G16B16_SINT => "R16G16B16_SINT",
            &Self::R16G16B16_SFLOAT => "R16G16B16_SFLOAT",
            &Self::R16G16B16A16_UNORM => "R16G16B16A16_UNORM",
            &Self::R16G16B16A16_SNORM => "R16G16B16A16_SNORM",
            &Self::R16G16B16A16_USCALED => "R16G16B16A16_USCALED",
            &Self::R16G16B16A16_SSCALED => "R16G16B16A16_SSCALED",
            &Self::R16G16B16A16_UINT => "R16G16B16A16_UINT",
            &Self::R16G16B16A16_SINT => "R16G16B16A16_SINT",
            &Self::R16G16B16A16_SFLOAT => "R16G16B16A16_SFLOAT",
            &Self::R32_UINT => "R32_UINT",
            &Self::R32_SINT => "R32_SINT",
            &Self::R32_SFLOAT => "R32_SFLOAT",
            &Self::R32G32_UINT => "R32G32_UINT",
            &Self::R32G32_SINT => "R32G32_SINT",
            &Self::R32G32_SFLOAT => "R32G32_SFLOAT",
            &Self::R32G32B32_UINT => "R32G32B32_UINT",
            &Self::R32G32B32_SINT => "R32G32B32_SINT",
            &Self::R32G32B32_SFLOAT => "R32G32B32_SFLOAT",
            &Self::R32G32B32A32_UINT => "R32G32B32A32_UINT",
            &Self::R32G32B32A32_SINT => "R32G32B32A32_SINT",
            &Self::R32G32B32A32_SFLOAT => "R32G32B32A32_SFLOAT",
            &Self::R64_UINT => "R64_UINT",
            &Self::R64_SINT => "R64_SINT",
            &Self::R64_SFLOAT => "R64_SFLOAT",
            &Self::R64G64_UINT => "R64G64_UINT",
            &Self::R64G64_SINT => "R64G64_SINT",
            &Self::R64G64_SFLOAT => "R64G64_SFLOAT",
            &Self::R64G64B64_UINT => "R64G64B64_UINT",
            &Self::R64G64B64_SINT => "R64G64B64_SINT",
            &Self::R64G64B64_SFLOAT => "R64G64B64_SFLOAT",
            &Self::R64G64B64A64_UINT => "R64G64B64A64_UINT",
            &Self::R64G64B64A64_SINT => "R64G64B64A64_SINT",
            &Self::R64G64B64A64_SFLOAT => "R64G64B64A64_SFLOAT",
            &Self::B10G11R11_UFLOAT_PACK32 => "B10G11R11_UFLOAT_PACK32",
            &Self::E5B9G9R9_UFLOAT_PACK32 => "E5B9G9R9_UFLOAT_PACK32",
            &Self::D16_UNORM => "D16_UNORM",
            &Self::X8_D24_UNORM_PACK32 => "X8_D24_UNORM_PACK32",
            &Self::D32_SFLOAT => "D32_SFLOAT",
            &Self::S8_UINT => "S8_UINT",
            &Self::D16_UNORM_S8_UINT => "D16_UNORM_S8_UINT",
            &Self::D24_UNORM_S8_UINT => "D24_UNORM_S8_UINT",
            &Self::D32_SFLOAT_S8_UINT => "D32_SFLOAT_S8_UINT",
            &Self::BC1_RGB_UNORM_BLOCK => "BC1_RGB_UNORM_BLOCK",
            &Self::BC1_RGB_SRGB_BLOCK => "BC1_RGB_SRGB_BLOCK",
            &Self::BC1_RGBA_UNORM_BLOCK => "BC1_RGBA_UNORM_BLOCK",
            &Self::BC1_RGBA_SRGB_BLOCK => "BC1_RGBA_SRGB_BLOCK",
            &Self::BC2_UNORM_BLOCK => "BC2_UNORM_BLOCK",
            &Self::BC2_SRGB_BLOCK => "BC2_SRGB_BLOCK",
            &Self::BC3_UNORM_BLOCK => "BC3_UNORM_BLOCK",
            &Self::BC3_SRGB_BLOCK => "BC3_SRGB_BLOCK",
            &Self::BC4_UNORM_BLOCK => "BC4_UNORM_BLOCK",
            &Self::BC4_SNORM_BLOCK => "BC4_SNORM_BLOCK",
            &Self::BC5_UNORM_BLOCK => "BC5_UNORM_BLOCK",
            &Self::BC5_SNORM_BLOCK => "BC5_SNORM_BLOCK",
            &Self::BC6H_UFLOAT_BLOCK => "BC6H_UFLOAT_BLOCK",
            &Self::BC6H_SFLOAT_BLOCK => "BC6H_SFLOAT_BLOCK",
            &Self::BC7_UNORM_BLOCK => "BC7_UNORM_BLOCK",
            &Self::BC7_SRGB_BLOCK => "BC7_SRGB_BLOCK",
            &Self::ETC2_R8G8B8_UNORM_BLOCK => "ETC2_R8G8B8_UNORM_BLOCK",
            &Self::ETC2_R8G8B8_SRGB_BLOCK => "ETC2_R8G8B8_SRGB_BLOCK",
            &Self::ETC2_R8G8B8A1_UNORM_BLOCK => "ETC2_R8G8B8A1_UNORM_BLOCK",
            &Self::ETC2_R8G8B8A1_SRGB_BLOCK => "ETC2_R8G8B8A1_SRGB_BLOCK",
            &Self::ETC2_R8G8B8A8_UNORM_BLOCK => "ETC2_R8G8B8A8_UNORM_BLOCK",
            &Self::ETC2_R8G8B8A8_SRGB_BLOCK => "ETC2_R8G8B8A8_SRGB_BLOCK",
            &Self::EAC_R11_UNORM_BLOCK => "EAC_R11_UNORM_BLOCK",
            &Self::EAC_R11_SNORM_BLOCK => "EAC_R11_SNORM_BLOCK",
            &Self::EAC_R11G11_UNORM_BLOCK => "EAC_R11G11_UNORM_BLOCK",
            &Self::EAC_R11G11_SNORM_BLOCK => "EAC_R11G11_SNORM_BLOCK",
            &Self::ASTC_4X4_UNORM_BLOCK => "ASTC_4X4_UNORM_BLOCK",
            &Self::ASTC_4X4_SRGB_BLOCK => "ASTC_4X4_SRGB_BLOCK",
            &Self::ASTC_5X4_UNORM_BLOCK => "ASTC_5X4_UNORM_BLOCK",
            &Self::ASTC_5X4_SRGB_BLOCK => "ASTC_5X4_SRGB_BLOCK",
            &Self::ASTC_5X5_UNORM_BLOCK => "ASTC_5X5_UNORM_BLOCK",
            &Self::ASTC_5X5_SRGB_BLOCK => "ASTC_5X5_SRGB_BLOCK",
            &Self::ASTC_6X5_UNORM_BLOCK => "ASTC_6X5_UNORM_BLOCK",
            &Self::ASTC_6X5_SRGB_BLOCK => "ASTC_6X5_SRGB_BLOCK",
            &Self::ASTC_6X6_UNORM_BLOCK => "ASTC_6X6_UNORM_BLOCK",
            &Self::ASTC_6X6_SRGB_BLOCK => "ASTC_6X6_SRGB_BLOCK",
            &Self::ASTC_8X5_UNORM_BLOCK => "ASTC_8X5_UNORM_BLOCK",
            &Self::ASTC_8X5_SRGB_BLOCK => "ASTC_8X5_SRGB_BLOCK",
            &Self::ASTC_8X6_UNORM_BLOCK => "ASTC_8X6_UNORM_BLOCK",
            &Self::ASTC_8X6_SRGB_BLOCK => "ASTC_8X6_SRGB_BLOCK",
            &Self::ASTC_8X8_UNORM_BLOCK => "ASTC_8X8_UNORM_BLOCK",
            &Self::ASTC_8X8_SRGB_BLOCK => "ASTC_8X8_SRGB_BLOCK",
            &Self::ASTC_10X5_UNORM_BLOCK => "ASTC_10X5_UNORM_BLOCK",
            &Self::ASTC_10X5_SRGB_BLOCK => "ASTC_10X5_SRGB_BLOCK",
            &Self::ASTC_10X6_UNORM_BLOCK => "ASTC_10X6_UNORM_BLOCK",
            &Self::ASTC_10X6_SRGB_BLOCK => "ASTC_10X6_SRGB_BLOCK",
            &Self::ASTC_10X8_UNORM_BLOCK => "ASTC_10X8_UNORM_BLOCK",
            &Self::ASTC_10X8_SRGB_BLOCK => "ASTC_10X8_SRGB_BLOCK",
            &Self::ASTC_10X10_UNORM_BLOCK => "ASTC_10X10_UNORM_BLOCK",
            &Self::ASTC_10X10_SRGB_BLOCK => "ASTC_10X10_SRGB_BLOCK",
            &Self::ASTC_12X10_UNORM_BLOCK => "ASTC_12X10_UNORM_BLOCK",
            &Self::ASTC_12X10_SRGB_BLOCK => "ASTC_12X10_SRGB_BLOCK",
            &Self::ASTC_12X12_UNORM_BLOCK => "ASTC_12X12_UNORM_BLOCK",
            &Self::ASTC_12X12_SRGB_BLOCK => "ASTC_12X12_SRGB_BLOCK",
            &Self::G8B8G8R8_422_UNORM => "G8B8G8R8_422_UNORM",
            &Self::B8G8R8G8_422_UNORM => "B8G8R8G8_422_UNORM",
            &Self::G8_B8_R8_3PLANE_420_UNORM => "G8_B8_R8_3PLANE_420_UNORM",
            &Self::G8_B8R8_2PLANE_420_UNORM => "G8_B8R8_2PLANE_420_UNORM",
            &Self::G8_B8_R8_3PLANE_422_UNORM => "G8_B8_R8_3PLANE_422_UNORM",
            &Self::G8_B8R8_2PLANE_422_UNORM => "G8_B8R8_2PLANE_422_UNORM",
            &Self::G8_B8_R8_3PLANE_444_UNORM => "G8_B8_R8_3PLANE_444_UNORM",
            &Self::R10X6_UNORM_PACK16 => "R10X6_UNORM_PACK16",
            &Self::R10X6G10X6_UNORM_2PACK16 => "R10X6G10X6_UNORM_2PACK16",
            &Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => "R10X6G10X6B10X6A10X6_UNORM_4PACK16",
            &Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => "G10X6B10X6G10X6R10X6_422_UNORM_4PACK16",
            &Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => "B10X6G10X6R10X6G10X6_422_UNORM_4PACK16",
            &Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16",
            &Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => "G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16",
            &Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16",
            &Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => "G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16",
            &Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => "G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16",
            &Self::R12X4_UNORM_PACK16 => "R12X4_UNORM_PACK16",
            &Self::R12X4G12X4_UNORM_2PACK16 => "R12X4G12X4_UNORM_2PACK16",
            &Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => "R12X4G12X4B12X4A12X4_UNORM_4PACK16",
            &Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => "G12X4B12X4G12X4R12X4_422_UNORM_4PACK16",
            &Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => "B12X4G12X4R12X4G12X4_422_UNORM_4PACK16",
            &Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16",
            &Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => "G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16",
            &Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16",
            &Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => "G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16",
            &Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => "G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16",
            &Self::G16B16G16R16_422_UNORM => "G16B16G16R16_422_UNORM",
            &Self::B16G16R16G16_422_UNORM => "B16G16R16G16_422_UNORM",
            &Self::G16_B16_R16_3PLANE_420_UNORM => "G16_B16_R16_3PLANE_420_UNORM",
            &Self::G16_B16R16_2PLANE_420_UNORM => "G16_B16R16_2PLANE_420_UNORM",
            &Self::G16_B16_R16_3PLANE_422_UNORM => "G16_B16_R16_3PLANE_422_UNORM",
            &Self::G16_B16R16_2PLANE_422_UNORM => "G16_B16R16_2PLANE_422_UNORM",
            &Self::G16_B16_R16_3PLANE_444_UNORM => "G16_B16_R16_3PLANE_444_UNORM",
            &Self::PVRTC1_2BPP_UNORM_BLOCK_IMG => "PVRTC1_2BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC1_4BPP_UNORM_BLOCK_IMG => "PVRTC1_4BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC2_2BPP_UNORM_BLOCK_IMG => "PVRTC2_2BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC2_4BPP_UNORM_BLOCK_IMG => "PVRTC2_4BPP_UNORM_BLOCK_IMG",
            &Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => "PVRTC1_2BPP_SRGB_BLOCK_IMG",
            &Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => "PVRTC1_4BPP_SRGB_BLOCK_IMG",
            &Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => "PVRTC2_2BPP_SRGB_BLOCK_IMG",
            &Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => "PVRTC2_4BPP_SRGB_BLOCK_IMG",
            &Self::ASTC_4X4_SFLOAT_BLOCK_EXT => "ASTC_4X4_SFLOAT_BLOCK_EXT",
            &Self::ASTC_5X4_SFLOAT_BLOCK_EXT => "ASTC_5X4_SFLOAT_BLOCK_EXT",
            &Self::ASTC_5X5_SFLOAT_BLOCK_EXT => "ASTC_5X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_6X5_SFLOAT_BLOCK_EXT => "ASTC_6X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_6X6_SFLOAT_BLOCK_EXT => "ASTC_6X6_SFLOAT_BLOCK_EXT",
            &Self::ASTC_8X5_SFLOAT_BLOCK_EXT => "ASTC_8X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_8X6_SFLOAT_BLOCK_EXT => "ASTC_8X6_SFLOAT_BLOCK_EXT",
            &Self::ASTC_8X8_SFLOAT_BLOCK_EXT => "ASTC_8X8_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X5_SFLOAT_BLOCK_EXT => "ASTC_10X5_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X6_SFLOAT_BLOCK_EXT => "ASTC_10X6_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X8_SFLOAT_BLOCK_EXT => "ASTC_10X8_SFLOAT_BLOCK_EXT",
            &Self::ASTC_10X10_SFLOAT_BLOCK_EXT => "ASTC_10X10_SFLOAT_BLOCK_EXT",
            &Self::ASTC_12X10_SFLOAT_BLOCK_EXT => "ASTC_12X10_SFLOAT_BLOCK_EXT",
            &Self::ASTC_12X12_SFLOAT_BLOCK_EXT => "ASTC_12X12_SFLOAT_BLOCK_EXT",
            &Self::G8_B8R8_2PLANE_444_UNORM_EXT => "G8_B8R8_2PLANE_444_UNORM_EXT",
            &Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT => "G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT",
            &Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT => "G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT",
            &Self::G16_B16R16_2PLANE_444_UNORM_EXT => "G16_B16R16_2PLANE_444_UNORM_EXT",
            &Self::A4R4G4B4_UNORM_PACK16_EXT => "A4R4G4B4_UNORM_PACK16_EXT",
            &Self::A4B4G4R4_UNORM_PACK16_EXT => "A4B4G4R4_UNORM_PACK16_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::Format {
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4X4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4X4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5X4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5X4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5X5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5X5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6X5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6X5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6X6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6X6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8X5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8X5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8X6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8X6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8X8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8X8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10X5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10X5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10X6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10X6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10X8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10X8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10X10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10X10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12X10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12X10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12X12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12X12_SRGB_BLOCK: Self = Self(184);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatFeatureFlags.html) · Bitmask of [`FormatFeatureFlagBits`]"] # [doc (alias = "VkFormatFeatureFlags")] # [derive (Default)] # [repr (transparent)] pub struct FormatFeatureFlags : u32 { const SAMPLED_IMAGE = FormatFeatureFlagBits :: SAMPLED_IMAGE . 0 ; const STORAGE_IMAGE = FormatFeatureFlagBits :: STORAGE_IMAGE . 0 ; const STORAGE_IMAGE_ATOMIC = FormatFeatureFlagBits :: STORAGE_IMAGE_ATOMIC . 0 ; const UNIFORM_TEXEL_BUFFER = FormatFeatureFlagBits :: UNIFORM_TEXEL_BUFFER . 0 ; const STORAGE_TEXEL_BUFFER = FormatFeatureFlagBits :: STORAGE_TEXEL_BUFFER . 0 ; const STORAGE_TEXEL_BUFFER_ATOMIC = FormatFeatureFlagBits :: STORAGE_TEXEL_BUFFER_ATOMIC . 0 ; const VERTEX_BUFFER = FormatFeatureFlagBits :: VERTEX_BUFFER . 0 ; const COLOR_ATTACHMENT = FormatFeatureFlagBits :: COLOR_ATTACHMENT . 0 ; const COLOR_ATTACHMENT_BLEND = FormatFeatureFlagBits :: COLOR_ATTACHMENT_BLEND . 0 ; const DEPTH_STENCIL_ATTACHMENT = FormatFeatureFlagBits :: DEPTH_STENCIL_ATTACHMENT . 0 ; const BLIT_SRC = FormatFeatureFlagBits :: BLIT_SRC . 0 ; const BLIT_DST = FormatFeatureFlagBits :: BLIT_DST . 0 ; const SAMPLED_IMAGE_FILTER_LINEAR = FormatFeatureFlagBits :: SAMPLED_IMAGE_FILTER_LINEAR . 0 ; const TRANSFER_SRC = FormatFeatureFlagBits :: TRANSFER_SRC . 0 ; const TRANSFER_DST = FormatFeatureFlagBits :: TRANSFER_DST . 0 ; const MIDPOINT_CHROMA_SAMPLES = FormatFeatureFlagBits :: MIDPOINT_CHROMA_SAMPLES . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE . 0 ; const DISJOINT = FormatFeatureFlagBits :: DISJOINT . 0 ; const COSITED_CHROMA_SAMPLES = FormatFeatureFlagBits :: COSITED_CHROMA_SAMPLES . 0 ; const SAMPLED_IMAGE_FILTER_MINMAX = FormatFeatureFlagBits :: SAMPLED_IMAGE_FILTER_MINMAX . 0 ; const SAMPLED_IMAGE_FILTER_CUBIC_IMG = FormatFeatureFlagBits :: SAMPLED_IMAGE_FILTER_CUBIC_IMG . 0 ; const VIDEO_DECODE_OUTPUT_KHR = FormatFeatureFlagBits :: VIDEO_DECODE_OUTPUT_KHR . 0 ; const VIDEO_DECODE_DPB_KHR = FormatFeatureFlagBits :: VIDEO_DECODE_DPB_KHR . 0 ; const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = FormatFeatureFlagBits :: ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR . 0 ; const FRAGMENT_DENSITY_MAP_EXT = FormatFeatureFlagBits :: FRAGMENT_DENSITY_MAP_EXT . 0 ; const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = FormatFeatureFlagBits :: FRAGMENT_SHADING_RATE_ATTACHMENT_KHR . 0 ; const VIDEO_ENCODE_INPUT_KHR = FormatFeatureFlagBits :: VIDEO_ENCODE_INPUT_KHR . 0 ; const VIDEO_ENCODE_DPB_KHR = FormatFeatureFlagBits :: VIDEO_ENCODE_DPB_KHR . 0 ; const TRANSFER_SRC_KHR = FormatFeatureFlagBits :: TRANSFER_SRC_KHR . 0 ; const TRANSFER_DST_KHR = FormatFeatureFlagBits :: TRANSFER_DST_KHR . 0 ; const SAMPLED_IMAGE_FILTER_MINMAX_EXT = FormatFeatureFlagBits :: SAMPLED_IMAGE_FILTER_MINMAX_EXT . 0 ; const MIDPOINT_CHROMA_SAMPLES_KHR = FormatFeatureFlagBits :: MIDPOINT_CHROMA_SAMPLES_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR = FormatFeatureFlagBits :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR . 0 ; const DISJOINT_KHR = FormatFeatureFlagBits :: DISJOINT_KHR . 0 ; const COSITED_CHROMA_SAMPLES_KHR = FormatFeatureFlagBits :: COSITED_CHROMA_SAMPLES_KHR . 0 ; const SAMPLED_IMAGE_FILTER_CUBIC_EXT = FormatFeatureFlagBits :: SAMPLED_IMAGE_FILTER_CUBIC_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatFeatureFlagBits.html) · Bits enum of [`FormatFeatureFlags`]"]
#[doc(alias = "VkFormatFeatureFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FormatFeatureFlagBits(pub u32);
impl FormatFeatureFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> FormatFeatureFlags {
        FormatFeatureFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for FormatFeatureFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SAMPLED_IMAGE => "SAMPLED_IMAGE",
            &Self::STORAGE_IMAGE => "STORAGE_IMAGE",
            &Self::STORAGE_IMAGE_ATOMIC => "STORAGE_IMAGE_ATOMIC",
            &Self::UNIFORM_TEXEL_BUFFER => "UNIFORM_TEXEL_BUFFER",
            &Self::STORAGE_TEXEL_BUFFER => "STORAGE_TEXEL_BUFFER",
            &Self::STORAGE_TEXEL_BUFFER_ATOMIC => "STORAGE_TEXEL_BUFFER_ATOMIC",
            &Self::VERTEX_BUFFER => "VERTEX_BUFFER",
            &Self::COLOR_ATTACHMENT => "COLOR_ATTACHMENT",
            &Self::COLOR_ATTACHMENT_BLEND => "COLOR_ATTACHMENT_BLEND",
            &Self::DEPTH_STENCIL_ATTACHMENT => "DEPTH_STENCIL_ATTACHMENT",
            &Self::BLIT_SRC => "BLIT_SRC",
            &Self::BLIT_DST => "BLIT_DST",
            &Self::SAMPLED_IMAGE_FILTER_LINEAR => "SAMPLED_IMAGE_FILTER_LINEAR",
            &Self::TRANSFER_SRC => "TRANSFER_SRC",
            &Self::TRANSFER_DST => "TRANSFER_DST",
            &Self::MIDPOINT_CHROMA_SAMPLES => "MIDPOINT_CHROMA_SAMPLES",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER => "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER => "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT => "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE => "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE",
            &Self::DISJOINT => "DISJOINT",
            &Self::COSITED_CHROMA_SAMPLES => "COSITED_CHROMA_SAMPLES",
            &Self::SAMPLED_IMAGE_FILTER_MINMAX => "SAMPLED_IMAGE_FILTER_MINMAX",
            &Self::SAMPLED_IMAGE_FILTER_CUBIC_IMG => "SAMPLED_IMAGE_FILTER_CUBIC_IMG",
            &Self::VIDEO_DECODE_OUTPUT_KHR => "VIDEO_DECODE_OUTPUT_KHR",
            &Self::VIDEO_DECODE_DPB_KHR => "VIDEO_DECODE_DPB_KHR",
            &Self::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR => "ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR",
            &Self::FRAGMENT_DENSITY_MAP_EXT => "FRAGMENT_DENSITY_MAP_EXT",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            &Self::VIDEO_ENCODE_INPUT_KHR => "VIDEO_ENCODE_INPUT_KHR",
            &Self::VIDEO_ENCODE_DPB_KHR => "VIDEO_ENCODE_DPB_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const SAMPLED_IMAGE: Self = Self(1);
    pub const STORAGE_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32);
    pub const VERTEX_BUFFER: Self = Self(64);
    pub const COLOR_ATTACHMENT: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(512);
    pub const BLIT_SRC: Self = Self(1024);
    pub const BLIT_DST: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFrontFace.html) · Enum"]
#[doc(alias = "VkFrontFace")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FrontFace(pub i32);
impl std::fmt::Debug for FrontFace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COUNTER_CLOCKWISE => "COUNTER_CLOCKWISE",
            &Self::CLOCKWISE => "CLOCKWISE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::FrontFace {
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageAspectFlags.html) · Bitmask of [`ImageAspectFlagBits`]"] # [doc (alias = "VkImageAspectFlags")] # [derive (Default)] # [repr (transparent)] pub struct ImageAspectFlags : u32 { const COLOR = ImageAspectFlagBits :: COLOR . 0 ; const DEPTH = ImageAspectFlagBits :: DEPTH . 0 ; const STENCIL = ImageAspectFlagBits :: STENCIL . 0 ; const METADATA = ImageAspectFlagBits :: METADATA . 0 ; const PLANE_0 = ImageAspectFlagBits :: PLANE_0 . 0 ; const PLANE_1 = ImageAspectFlagBits :: PLANE_1 . 0 ; const PLANE_2 = ImageAspectFlagBits :: PLANE_2 . 0 ; const MEMORY_PLANE_0_EXT = ImageAspectFlagBits :: MEMORY_PLANE_0_EXT . 0 ; const MEMORY_PLANE_1_EXT = ImageAspectFlagBits :: MEMORY_PLANE_1_EXT . 0 ; const MEMORY_PLANE_2_EXT = ImageAspectFlagBits :: MEMORY_PLANE_2_EXT . 0 ; const MEMORY_PLANE_3_EXT = ImageAspectFlagBits :: MEMORY_PLANE_3_EXT . 0 ; const PLANE_0_KHR = ImageAspectFlagBits :: PLANE_0_KHR . 0 ; const PLANE_1_KHR = ImageAspectFlagBits :: PLANE_1_KHR . 0 ; const PLANE_2_KHR = ImageAspectFlagBits :: PLANE_2_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageAspectFlagBits.html) · Bits enum of [`ImageAspectFlags`]"]
#[doc(alias = "VkImageAspectFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageAspectFlagBits(pub u32);
impl ImageAspectFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ImageAspectFlags {
        ImageAspectFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ImageAspectFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COLOR => "COLOR",
            &Self::DEPTH => "DEPTH",
            &Self::STENCIL => "STENCIL",
            &Self::METADATA => "METADATA",
            &Self::PLANE_0 => "PLANE_0",
            &Self::PLANE_1 => "PLANE_1",
            &Self::PLANE_2 => "PLANE_2",
            &Self::MEMORY_PLANE_0_EXT => "MEMORY_PLANE_0_EXT",
            &Self::MEMORY_PLANE_1_EXT => "MEMORY_PLANE_1_EXT",
            &Self::MEMORY_PLANE_2_EXT => "MEMORY_PLANE_2_EXT",
            &Self::MEMORY_PLANE_3_EXT => "MEMORY_PLANE_3_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ImageAspectFlagBits {
    pub const COLOR: Self = Self(1);
    pub const DEPTH: Self = Self(2);
    pub const STENCIL: Self = Self(4);
    pub const METADATA: Self = Self(8);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateFlags.html) · Bitmask of [`ImageCreateFlagBits`]"] # [doc (alias = "VkImageCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct ImageCreateFlags : u32 { const SPARSE_BINDING = ImageCreateFlagBits :: SPARSE_BINDING . 0 ; const SPARSE_RESIDENCY = ImageCreateFlagBits :: SPARSE_RESIDENCY . 0 ; const SPARSE_ALIASED = ImageCreateFlagBits :: SPARSE_ALIASED . 0 ; const MUTABLE_FORMAT = ImageCreateFlagBits :: MUTABLE_FORMAT . 0 ; const CUBE_COMPATIBLE = ImageCreateFlagBits :: CUBE_COMPATIBLE . 0 ; const ALIAS = ImageCreateFlagBits :: ALIAS . 0 ; const SPLIT_INSTANCE_BIND_REGIONS = ImageCreateFlagBits :: SPLIT_INSTANCE_BIND_REGIONS . 0 ; const _2D_ARRAY_COMPATIBLE = ImageCreateFlagBits :: _2D_ARRAY_COMPATIBLE . 0 ; const BLOCK_TEXEL_VIEW_COMPATIBLE = ImageCreateFlagBits :: BLOCK_TEXEL_VIEW_COMPATIBLE . 0 ; const EXTENDED_USAGE = ImageCreateFlagBits :: EXTENDED_USAGE . 0 ; const PROTECTED = ImageCreateFlagBits :: PROTECTED . 0 ; const DISJOINT = ImageCreateFlagBits :: DISJOINT . 0 ; const CORNER_SAMPLED_NV = ImageCreateFlagBits :: CORNER_SAMPLED_NV . 0 ; const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT = ImageCreateFlagBits :: SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT . 0 ; const SUBSAMPLED_EXT = ImageCreateFlagBits :: SUBSAMPLED_EXT . 0 ; const SPLIT_INSTANCE_BIND_REGIONS_KHR = ImageCreateFlagBits :: SPLIT_INSTANCE_BIND_REGIONS_KHR . 0 ; const _2D_ARRAY_COMPATIBLE_KHR = ImageCreateFlagBits :: _2D_ARRAY_COMPATIBLE_KHR . 0 ; const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR = ImageCreateFlagBits :: BLOCK_TEXEL_VIEW_COMPATIBLE_KHR . 0 ; const EXTENDED_USAGE_KHR = ImageCreateFlagBits :: EXTENDED_USAGE_KHR . 0 ; const DISJOINT_KHR = ImageCreateFlagBits :: DISJOINT_KHR . 0 ; const ALIAS_KHR = ImageCreateFlagBits :: ALIAS_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateFlagBits.html) · Bits enum of [`ImageCreateFlags`]"]
#[doc(alias = "VkImageCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageCreateFlagBits(pub u32);
impl ImageCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ImageCreateFlags {
        ImageCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ImageCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SPARSE_BINDING => "SPARSE_BINDING",
            &Self::SPARSE_RESIDENCY => "SPARSE_RESIDENCY",
            &Self::SPARSE_ALIASED => "SPARSE_ALIASED",
            &Self::MUTABLE_FORMAT => "MUTABLE_FORMAT",
            &Self::CUBE_COMPATIBLE => "CUBE_COMPATIBLE",
            &Self::ALIAS => "ALIAS",
            &Self::SPLIT_INSTANCE_BIND_REGIONS => "SPLIT_INSTANCE_BIND_REGIONS",
            &Self::_2D_ARRAY_COMPATIBLE => "_2D_ARRAY_COMPATIBLE",
            &Self::BLOCK_TEXEL_VIEW_COMPATIBLE => "BLOCK_TEXEL_VIEW_COMPATIBLE",
            &Self::EXTENDED_USAGE => "EXTENDED_USAGE",
            &Self::PROTECTED => "PROTECTED",
            &Self::DISJOINT => "DISJOINT",
            &Self::CORNER_SAMPLED_NV => "CORNER_SAMPLED_NV",
            &Self::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT => "SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT",
            &Self::SUBSAMPLED_EXT => "SUBSAMPLED_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ImageCreateFlagBits {
    pub const SPARSE_BINDING: Self = Self(1);
    pub const SPARSE_RESIDENCY: Self = Self(2);
    pub const SPARSE_ALIASED: Self = Self(4);
    pub const MUTABLE_FORMAT: Self = Self(8);
    pub const CUBE_COMPATIBLE: Self = Self(16);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageLayout.html) · Enum"]
#[doc(alias = "VkImageLayout")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageLayout(pub i32);
impl std::fmt::Debug for ImageLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNDEFINED => "UNDEFINED",
            &Self::GENERAL => "GENERAL",
            &Self::COLOR_ATTACHMENT_OPTIMAL => "COLOR_ATTACHMENT_OPTIMAL",
            &Self::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => "DEPTH_STENCIL_ATTACHMENT_OPTIMAL",
            &Self::DEPTH_STENCIL_READ_ONLY_OPTIMAL => "DEPTH_STENCIL_READ_ONLY_OPTIMAL",
            &Self::SHADER_READ_ONLY_OPTIMAL => "SHADER_READ_ONLY_OPTIMAL",
            &Self::TRANSFER_SRC_OPTIMAL => "TRANSFER_SRC_OPTIMAL",
            &Self::TRANSFER_DST_OPTIMAL => "TRANSFER_DST_OPTIMAL",
            &Self::PREINITIALIZED => "PREINITIALIZED",
            &Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL => "DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL",
            &Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL => "DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL",
            &Self::DEPTH_ATTACHMENT_OPTIMAL => "DEPTH_ATTACHMENT_OPTIMAL",
            &Self::DEPTH_READ_ONLY_OPTIMAL => "DEPTH_READ_ONLY_OPTIMAL",
            &Self::STENCIL_ATTACHMENT_OPTIMAL => "STENCIL_ATTACHMENT_OPTIMAL",
            &Self::STENCIL_READ_ONLY_OPTIMAL => "STENCIL_READ_ONLY_OPTIMAL",
            &Self::PRESENT_SRC_KHR => "PRESENT_SRC_KHR",
            &Self::VIDEO_DECODE_DST_KHR => "VIDEO_DECODE_DST_KHR",
            &Self::VIDEO_DECODE_SRC_KHR => "VIDEO_DECODE_SRC_KHR",
            &Self::VIDEO_DECODE_DPB_KHR => "VIDEO_DECODE_DPB_KHR",
            &Self::SHARED_PRESENT_KHR => "SHARED_PRESENT_KHR",
            &Self::FRAGMENT_DENSITY_MAP_OPTIMAL_EXT => "FRAGMENT_DENSITY_MAP_OPTIMAL_EXT",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR",
            &Self::VIDEO_ENCODE_DST_KHR => "VIDEO_ENCODE_DST_KHR",
            &Self::VIDEO_ENCODE_SRC_KHR => "VIDEO_ENCODE_SRC_KHR",
            &Self::VIDEO_ENCODE_DPB_KHR => "VIDEO_ENCODE_DPB_KHR",
            &Self::READ_ONLY_OPTIMAL_KHR => "READ_ONLY_OPTIMAL_KHR",
            &Self::ATTACHMENT_OPTIMAL_KHR => "ATTACHMENT_OPTIMAL_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ImageLayout {
    pub const UNDEFINED: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const PREINITIALIZED: Self = Self(8);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageTiling.html) · Enum"]
#[doc(alias = "VkImageTiling")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageTiling(pub i32);
impl std::fmt::Debug for ImageTiling {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OPTIMAL => "OPTIMAL",
            &Self::LINEAR => "LINEAR",
            &Self::DRM_FORMAT_MODIFIER_EXT => "DRM_FORMAT_MODIFIER_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ImageTiling {
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageType.html) · Enum"]
#[doc(alias = "VkImageType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageType(pub i32);
impl std::fmt::Debug for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_1D => "_1D",
            &Self::_2D => "_2D",
            &Self::_3D => "_3D",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ImageType {
    pub const _1D: Self = Self(0);
    pub const _2D: Self = Self(1);
    pub const _3D: Self = Self(2);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageUsageFlags.html) · Bitmask of [`ImageUsageFlagBits`]"] # [doc (alias = "VkImageUsageFlags")] # [derive (Default)] # [repr (transparent)] pub struct ImageUsageFlags : u32 { const TRANSFER_SRC = ImageUsageFlagBits :: TRANSFER_SRC . 0 ; const TRANSFER_DST = ImageUsageFlagBits :: TRANSFER_DST . 0 ; const SAMPLED = ImageUsageFlagBits :: SAMPLED . 0 ; const STORAGE = ImageUsageFlagBits :: STORAGE . 0 ; const COLOR_ATTACHMENT = ImageUsageFlagBits :: COLOR_ATTACHMENT . 0 ; const DEPTH_STENCIL_ATTACHMENT = ImageUsageFlagBits :: DEPTH_STENCIL_ATTACHMENT . 0 ; const TRANSIENT_ATTACHMENT = ImageUsageFlagBits :: TRANSIENT_ATTACHMENT . 0 ; const INPUT_ATTACHMENT = ImageUsageFlagBits :: INPUT_ATTACHMENT . 0 ; const VIDEO_DECODE_DST_KHR = ImageUsageFlagBits :: VIDEO_DECODE_DST_KHR . 0 ; const VIDEO_DECODE_SRC_KHR = ImageUsageFlagBits :: VIDEO_DECODE_SRC_KHR . 0 ; const VIDEO_DECODE_DPB_KHR = ImageUsageFlagBits :: VIDEO_DECODE_DPB_KHR . 0 ; const FRAGMENT_DENSITY_MAP_EXT = ImageUsageFlagBits :: FRAGMENT_DENSITY_MAP_EXT . 0 ; const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = ImageUsageFlagBits :: FRAGMENT_SHADING_RATE_ATTACHMENT_KHR . 0 ; const VIDEO_ENCODE_DST_KHR = ImageUsageFlagBits :: VIDEO_ENCODE_DST_KHR . 0 ; const VIDEO_ENCODE_SRC_KHR = ImageUsageFlagBits :: VIDEO_ENCODE_SRC_KHR . 0 ; const VIDEO_ENCODE_DPB_KHR = ImageUsageFlagBits :: VIDEO_ENCODE_DPB_KHR . 0 ; const SHADING_RATE_IMAGE_NV = ImageUsageFlagBits :: SHADING_RATE_IMAGE_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageUsageFlagBits.html) · Bits enum of [`ImageUsageFlags`]"]
#[doc(alias = "VkImageUsageFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageUsageFlagBits(pub u32);
impl ImageUsageFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ImageUsageFlags {
        ImageUsageFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ImageUsageFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TRANSFER_SRC => "TRANSFER_SRC",
            &Self::TRANSFER_DST => "TRANSFER_DST",
            &Self::SAMPLED => "SAMPLED",
            &Self::STORAGE => "STORAGE",
            &Self::COLOR_ATTACHMENT => "COLOR_ATTACHMENT",
            &Self::DEPTH_STENCIL_ATTACHMENT => "DEPTH_STENCIL_ATTACHMENT",
            &Self::TRANSIENT_ATTACHMENT => "TRANSIENT_ATTACHMENT",
            &Self::INPUT_ATTACHMENT => "INPUT_ATTACHMENT",
            &Self::VIDEO_DECODE_DST_KHR => "VIDEO_DECODE_DST_KHR",
            &Self::VIDEO_DECODE_SRC_KHR => "VIDEO_DECODE_SRC_KHR",
            &Self::VIDEO_DECODE_DPB_KHR => "VIDEO_DECODE_DPB_KHR",
            &Self::FRAGMENT_DENSITY_MAP_EXT => "FRAGMENT_DENSITY_MAP_EXT",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            &Self::VIDEO_ENCODE_DST_KHR => "VIDEO_ENCODE_DST_KHR",
            &Self::VIDEO_ENCODE_SRC_KHR => "VIDEO_ENCODE_SRC_KHR",
            &Self::VIDEO_ENCODE_DPB_KHR => "VIDEO_ENCODE_DPB_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ImageUsageFlagBits {
    pub const TRANSFER_SRC: Self = Self(1);
    pub const TRANSFER_DST: Self = Self(2);
    pub const SAMPLED: Self = Self(4);
    pub const STORAGE: Self = Self(8);
    pub const COLOR_ATTACHMENT: Self = Self(16);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(32);
    pub const TRANSIENT_ATTACHMENT: Self = Self(64);
    pub const INPUT_ATTACHMENT: Self = Self(128);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateFlags.html) · Bitmask of [`ImageViewCreateFlagBits`]"] # [doc (alias = "VkImageViewCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct ImageViewCreateFlags : u32 { const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT = ImageViewCreateFlagBits :: FRAGMENT_DENSITY_MAP_DYNAMIC_EXT . 0 ; const FRAGMENT_DENSITY_MAP_DEFERRED_EXT = ImageViewCreateFlagBits :: FRAGMENT_DENSITY_MAP_DEFERRED_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateFlagBits.html) · Bits enum of [`ImageViewCreateFlags`]"]
#[doc(alias = "VkImageViewCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageViewCreateFlagBits(pub u32);
impl ImageViewCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ImageViewCreateFlags {
        ImageViewCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ImageViewCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FRAGMENT_DENSITY_MAP_DYNAMIC_EXT => "FRAGMENT_DENSITY_MAP_DYNAMIC_EXT",
            &Self::FRAGMENT_DENSITY_MAP_DEFERRED_EXT => "FRAGMENT_DENSITY_MAP_DEFERRED_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewType.html) · Enum"]
#[doc(alias = "VkImageViewType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ImageViewType(pub i32);
impl std::fmt::Debug for ImageViewType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_1D => "_1D",
            &Self::_2D => "_2D",
            &Self::_3D => "_3D",
            &Self::CUBE => "CUBE",
            &Self::_1D_ARRAY => "_1D_ARRAY",
            &Self::_2D_ARRAY => "_2D_ARRAY",
            &Self::CUBE_ARRAY => "CUBE_ARRAY",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ImageViewType {
    pub const _1D: Self = Self(0);
    pub const _2D: Self = Self(1);
    pub const _3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const _1D_ARRAY: Self = Self(4);
    pub const _2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSharingMode.html) · Enum"]
#[doc(alias = "VkSharingMode")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SharingMode(pub i32);
impl std::fmt::Debug for SharingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::EXCLUSIVE => "EXCLUSIVE",
            &Self::CONCURRENT => "CONCURRENT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SharingMode {
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkIndexType.html) · Enum"]
#[doc(alias = "VkIndexType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct IndexType(pub i32);
impl std::fmt::Debug for IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UINT16 => "UINT16",
            &Self::UINT32 => "UINT32",
            &Self::NONE_KHR => "NONE_KHR",
            &Self::UINT8_EXT => "UINT8_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::IndexType {
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLogicOp.html) · Enum"]
#[doc(alias = "VkLogicOp")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct LogicOp(pub i32);
impl std::fmt::Debug for LogicOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::CLEAR => "CLEAR",
            &Self::AND => "AND",
            &Self::AND_REVERSE => "AND_REVERSE",
            &Self::COPY => "COPY",
            &Self::AND_INVERTED => "AND_INVERTED",
            &Self::NO_OP => "NO_OP",
            &Self::XOR => "XOR",
            &Self::OR => "OR",
            &Self::NOR => "NOR",
            &Self::EQUIVALENT => "EQUIVALENT",
            &Self::INVERT => "INVERT",
            &Self::OR_REVERSE => "OR_REVERSE",
            &Self::COPY_INVERTED => "COPY_INVERTED",
            &Self::OR_INVERTED => "OR_INVERTED",
            &Self::NAND => "NAND",
            &Self::SET => "SET",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::LogicOp {
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHeapFlags.html) · Bitmask of [`MemoryHeapFlagBits`]"] # [doc (alias = "VkMemoryHeapFlags")] # [derive (Default)] # [repr (transparent)] pub struct MemoryHeapFlags : u32 { const DEVICE_LOCAL = MemoryHeapFlagBits :: DEVICE_LOCAL . 0 ; const MULTI_INSTANCE = MemoryHeapFlagBits :: MULTI_INSTANCE . 0 ; const MULTI_INSTANCE_KHR = MemoryHeapFlagBits :: MULTI_INSTANCE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHeapFlagBits.html) · Bits enum of [`MemoryHeapFlags`]"]
#[doc(alias = "VkMemoryHeapFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MemoryHeapFlagBits(pub u32);
impl MemoryHeapFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MemoryHeapFlags {
        MemoryHeapFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MemoryHeapFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEVICE_LOCAL => "DEVICE_LOCAL",
            &Self::MULTI_INSTANCE => "MULTI_INSTANCE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::MemoryHeapFlagBits {
    pub const DEVICE_LOCAL: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccessFlags.html) · Bitmask of [`AccessFlagBits`]"] # [doc (alias = "VkAccessFlags")] # [derive (Default)] # [repr (transparent)] pub struct AccessFlags : u32 { const INDIRECT_COMMAND_READ = AccessFlagBits :: INDIRECT_COMMAND_READ . 0 ; const INDEX_READ = AccessFlagBits :: INDEX_READ . 0 ; const VERTEX_ATTRIBUTE_READ = AccessFlagBits :: VERTEX_ATTRIBUTE_READ . 0 ; const UNIFORM_READ = AccessFlagBits :: UNIFORM_READ . 0 ; const INPUT_ATTACHMENT_READ = AccessFlagBits :: INPUT_ATTACHMENT_READ . 0 ; const SHADER_READ = AccessFlagBits :: SHADER_READ . 0 ; const SHADER_WRITE = AccessFlagBits :: SHADER_WRITE . 0 ; const COLOR_ATTACHMENT_READ = AccessFlagBits :: COLOR_ATTACHMENT_READ . 0 ; const COLOR_ATTACHMENT_WRITE = AccessFlagBits :: COLOR_ATTACHMENT_WRITE . 0 ; const DEPTH_STENCIL_ATTACHMENT_READ = AccessFlagBits :: DEPTH_STENCIL_ATTACHMENT_READ . 0 ; const DEPTH_STENCIL_ATTACHMENT_WRITE = AccessFlagBits :: DEPTH_STENCIL_ATTACHMENT_WRITE . 0 ; const TRANSFER_READ = AccessFlagBits :: TRANSFER_READ . 0 ; const TRANSFER_WRITE = AccessFlagBits :: TRANSFER_WRITE . 0 ; const HOST_READ = AccessFlagBits :: HOST_READ . 0 ; const HOST_WRITE = AccessFlagBits :: HOST_WRITE . 0 ; const MEMORY_READ = AccessFlagBits :: MEMORY_READ . 0 ; const MEMORY_WRITE = AccessFlagBits :: MEMORY_WRITE . 0 ; const TRANSFORM_FEEDBACK_WRITE_EXT = AccessFlagBits :: TRANSFORM_FEEDBACK_WRITE_EXT . 0 ; const TRANSFORM_FEEDBACK_COUNTER_READ_EXT = AccessFlagBits :: TRANSFORM_FEEDBACK_COUNTER_READ_EXT . 0 ; const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = AccessFlagBits :: TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT . 0 ; const CONDITIONAL_RENDERING_READ_EXT = AccessFlagBits :: CONDITIONAL_RENDERING_READ_EXT . 0 ; const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = AccessFlagBits :: COLOR_ATTACHMENT_READ_NONCOHERENT_EXT . 0 ; const ACCELERATION_STRUCTURE_READ_KHR = AccessFlagBits :: ACCELERATION_STRUCTURE_READ_KHR . 0 ; const ACCELERATION_STRUCTURE_WRITE_KHR = AccessFlagBits :: ACCELERATION_STRUCTURE_WRITE_KHR . 0 ; const FRAGMENT_DENSITY_MAP_READ_EXT = AccessFlagBits :: FRAGMENT_DENSITY_MAP_READ_EXT . 0 ; const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = AccessFlagBits :: FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR . 0 ; const COMMAND_PREPROCESS_READ_NV = AccessFlagBits :: COMMAND_PREPROCESS_READ_NV . 0 ; const COMMAND_PREPROCESS_WRITE_NV = AccessFlagBits :: COMMAND_PREPROCESS_WRITE_NV . 0 ; const NONE_KHR = AccessFlagBits :: NONE_KHR . 0 ; const SHADING_RATE_IMAGE_READ_NV = AccessFlagBits :: SHADING_RATE_IMAGE_READ_NV . 0 ; const ACCELERATION_STRUCTURE_READ_NV = AccessFlagBits :: ACCELERATION_STRUCTURE_READ_NV . 0 ; const ACCELERATION_STRUCTURE_WRITE_NV = AccessFlagBits :: ACCELERATION_STRUCTURE_WRITE_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAccessFlagBits.html) · Bits enum of [`AccessFlags`]"]
#[doc(alias = "VkAccessFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AccessFlagBits(pub u32);
impl AccessFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AccessFlags {
        AccessFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AccessFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INDIRECT_COMMAND_READ => "INDIRECT_COMMAND_READ",
            &Self::INDEX_READ => "INDEX_READ",
            &Self::VERTEX_ATTRIBUTE_READ => "VERTEX_ATTRIBUTE_READ",
            &Self::UNIFORM_READ => "UNIFORM_READ",
            &Self::INPUT_ATTACHMENT_READ => "INPUT_ATTACHMENT_READ",
            &Self::SHADER_READ => "SHADER_READ",
            &Self::SHADER_WRITE => "SHADER_WRITE",
            &Self::COLOR_ATTACHMENT_READ => "COLOR_ATTACHMENT_READ",
            &Self::COLOR_ATTACHMENT_WRITE => "COLOR_ATTACHMENT_WRITE",
            &Self::DEPTH_STENCIL_ATTACHMENT_READ => "DEPTH_STENCIL_ATTACHMENT_READ",
            &Self::DEPTH_STENCIL_ATTACHMENT_WRITE => "DEPTH_STENCIL_ATTACHMENT_WRITE",
            &Self::TRANSFER_READ => "TRANSFER_READ",
            &Self::TRANSFER_WRITE => "TRANSFER_WRITE",
            &Self::HOST_READ => "HOST_READ",
            &Self::HOST_WRITE => "HOST_WRITE",
            &Self::MEMORY_READ => "MEMORY_READ",
            &Self::MEMORY_WRITE => "MEMORY_WRITE",
            &Self::TRANSFORM_FEEDBACK_WRITE_EXT => "TRANSFORM_FEEDBACK_WRITE_EXT",
            &Self::TRANSFORM_FEEDBACK_COUNTER_READ_EXT => "TRANSFORM_FEEDBACK_COUNTER_READ_EXT",
            &Self::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT => "TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT",
            &Self::CONDITIONAL_RENDERING_READ_EXT => "CONDITIONAL_RENDERING_READ_EXT",
            &Self::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT => "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT",
            &Self::ACCELERATION_STRUCTURE_READ_KHR => "ACCELERATION_STRUCTURE_READ_KHR",
            &Self::ACCELERATION_STRUCTURE_WRITE_KHR => "ACCELERATION_STRUCTURE_WRITE_KHR",
            &Self::FRAGMENT_DENSITY_MAP_READ_EXT => "FRAGMENT_DENSITY_MAP_READ_EXT",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR",
            &Self::COMMAND_PREPROCESS_READ_NV => "COMMAND_PREPROCESS_READ_NV",
            &Self::COMMAND_PREPROCESS_WRITE_NV => "COMMAND_PREPROCESS_WRITE_NV",
            &Self::NONE_KHR => "NONE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::AccessFlagBits {
    pub const INDIRECT_COMMAND_READ: Self = Self(1);
    pub const INDEX_READ: Self = Self(2);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(4);
    pub const UNIFORM_READ: Self = Self(8);
    pub const INPUT_ATTACHMENT_READ: Self = Self(16);
    pub const SHADER_READ: Self = Self(32);
    pub const SHADER_WRITE: Self = Self(64);
    pub const COLOR_ATTACHMENT_READ: Self = Self(128);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024);
    pub const TRANSFER_READ: Self = Self(2048);
    pub const TRANSFER_WRITE: Self = Self(4096);
    pub const HOST_READ: Self = Self(8192);
    pub const HOST_WRITE: Self = Self(16384);
    pub const MEMORY_READ: Self = Self(32768);
    pub const MEMORY_WRITE: Self = Self(65536);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryPropertyFlags.html) · Bitmask of [`MemoryPropertyFlagBits`]"] # [doc (alias = "VkMemoryPropertyFlags")] # [derive (Default)] # [repr (transparent)] pub struct MemoryPropertyFlags : u32 { const DEVICE_LOCAL = MemoryPropertyFlagBits :: DEVICE_LOCAL . 0 ; const HOST_VISIBLE = MemoryPropertyFlagBits :: HOST_VISIBLE . 0 ; const HOST_COHERENT = MemoryPropertyFlagBits :: HOST_COHERENT . 0 ; const HOST_CACHED = MemoryPropertyFlagBits :: HOST_CACHED . 0 ; const LAZILY_ALLOCATED = MemoryPropertyFlagBits :: LAZILY_ALLOCATED . 0 ; const PROTECTED = MemoryPropertyFlagBits :: PROTECTED . 0 ; const DEVICE_COHERENT_AMD = MemoryPropertyFlagBits :: DEVICE_COHERENT_AMD . 0 ; const DEVICE_UNCACHED_AMD = MemoryPropertyFlagBits :: DEVICE_UNCACHED_AMD . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryPropertyFlagBits.html) · Bits enum of [`MemoryPropertyFlags`]"]
#[doc(alias = "VkMemoryPropertyFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct MemoryPropertyFlagBits(pub u32);
impl MemoryPropertyFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> MemoryPropertyFlags {
        MemoryPropertyFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for MemoryPropertyFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEVICE_LOCAL => "DEVICE_LOCAL",
            &Self::HOST_VISIBLE => "HOST_VISIBLE",
            &Self::HOST_COHERENT => "HOST_COHERENT",
            &Self::HOST_CACHED => "HOST_CACHED",
            &Self::LAZILY_ALLOCATED => "LAZILY_ALLOCATED",
            &Self::PROTECTED => "PROTECTED",
            &Self::DEVICE_COHERENT_AMD => "DEVICE_COHERENT_AMD",
            &Self::DEVICE_UNCACHED_AMD => "DEVICE_UNCACHED_AMD",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::MemoryPropertyFlagBits {
    pub const DEVICE_LOCAL: Self = Self(1);
    pub const HOST_VISIBLE: Self = Self(2);
    pub const HOST_COHERENT: Self = Self(4);
    pub const HOST_CACHED: Self = Self(8);
    pub const LAZILY_ALLOCATED: Self = Self(16);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceType.html) · Enum"]
#[doc(alias = "VkPhysicalDeviceType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PhysicalDeviceType(pub i32);
impl std::fmt::Debug for PhysicalDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OTHER => "OTHER",
            &Self::INTEGRATED_GPU => "INTEGRATED_GPU",
            &Self::DISCRETE_GPU => "DISCRETE_GPU",
            &Self::VIRTUAL_GPU => "VIRTUAL_GPU",
            &Self::CPU => "CPU",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::PhysicalDeviceType {
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineBindPoint.html) · Enum"]
#[doc(alias = "VkPipelineBindPoint")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineBindPoint(pub i32);
impl std::fmt::Debug for PipelineBindPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::GRAPHICS => "GRAPHICS",
            &Self::COMPUTE => "COMPUTE",
            &Self::RAY_TRACING_KHR => "RAY_TRACING_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::PipelineBindPoint {
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreateFlags.html) · Bitmask of [`PipelineCreateFlagBits`]"] # [doc (alias = "VkPipelineCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineCreateFlags : u32 { const DISABLE_OPTIMIZATION = PipelineCreateFlagBits :: DISABLE_OPTIMIZATION . 0 ; const ALLOW_DERIVATIVES = PipelineCreateFlagBits :: ALLOW_DERIVATIVES . 0 ; const DERIVATIVE = PipelineCreateFlagBits :: DERIVATIVE . 0 ; const VIEW_INDEX_FROM_DEVICE_INDEX = PipelineCreateFlagBits :: VIEW_INDEX_FROM_DEVICE_INDEX . 0 ; const DISPATCH_BASE = PipelineCreateFlagBits :: DISPATCH_BASE . 0 ; const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR = PipelineCreateFlagBits :: RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR . 0 ; const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR = PipelineCreateFlagBits :: RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR . 0 ; const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR = PipelineCreateFlagBits :: RAY_TRACING_NO_NULL_MISS_SHADERS_KHR . 0 ; const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR = PipelineCreateFlagBits :: RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR . 0 ; const RAY_TRACING_SKIP_TRIANGLES_KHR = PipelineCreateFlagBits :: RAY_TRACING_SKIP_TRIANGLES_KHR . 0 ; const RAY_TRACING_SKIP_AABBS_KHR = PipelineCreateFlagBits :: RAY_TRACING_SKIP_AABBS_KHR . 0 ; const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR = PipelineCreateFlagBits :: RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR . 0 ; const DEFER_COMPILE_NV = PipelineCreateFlagBits :: DEFER_COMPILE_NV . 0 ; const CAPTURE_STATISTICS_KHR = PipelineCreateFlagBits :: CAPTURE_STATISTICS_KHR . 0 ; const CAPTURE_INTERNAL_REPRESENTATIONS_KHR = PipelineCreateFlagBits :: CAPTURE_INTERNAL_REPRESENTATIONS_KHR . 0 ; const INDIRECT_BINDABLE_NV = PipelineCreateFlagBits :: INDIRECT_BINDABLE_NV . 0 ; const LIBRARY_KHR = PipelineCreateFlagBits :: LIBRARY_KHR . 0 ; const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT = PipelineCreateFlagBits :: FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT . 0 ; const EARLY_RETURN_ON_FAILURE_EXT = PipelineCreateFlagBits :: EARLY_RETURN_ON_FAILURE_EXT . 0 ; const VIEW_INDEX_FROM_DEVICE_INDEX_KHR = PipelineCreateFlagBits :: VIEW_INDEX_FROM_DEVICE_INDEX_KHR . 0 ; const DISPATCH_BASE_KHR = PipelineCreateFlagBits :: DISPATCH_BASE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCreateFlagBits.html) · Bits enum of [`PipelineCreateFlags`]"]
#[doc(alias = "VkPipelineCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineCreateFlagBits(pub u32);
impl PipelineCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineCreateFlags {
        PipelineCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DISABLE_OPTIMIZATION => "DISABLE_OPTIMIZATION",
            &Self::ALLOW_DERIVATIVES => "ALLOW_DERIVATIVES",
            &Self::DERIVATIVE => "DERIVATIVE",
            &Self::VIEW_INDEX_FROM_DEVICE_INDEX => "VIEW_INDEX_FROM_DEVICE_INDEX",
            &Self::DISPATCH_BASE => "DISPATCH_BASE",
            &Self::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR => "RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR",
            &Self::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR => "RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR",
            &Self::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR => "RAY_TRACING_NO_NULL_MISS_SHADERS_KHR",
            &Self::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR => "RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR",
            &Self::RAY_TRACING_SKIP_TRIANGLES_KHR => "RAY_TRACING_SKIP_TRIANGLES_KHR",
            &Self::RAY_TRACING_SKIP_AABBS_KHR => "RAY_TRACING_SKIP_AABBS_KHR",
            &Self::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR => "RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR",
            &Self::DEFER_COMPILE_NV => "DEFER_COMPILE_NV",
            &Self::CAPTURE_STATISTICS_KHR => "CAPTURE_STATISTICS_KHR",
            &Self::CAPTURE_INTERNAL_REPRESENTATIONS_KHR => "CAPTURE_INTERNAL_REPRESENTATIONS_KHR",
            &Self::INDIRECT_BINDABLE_NV => "INDIRECT_BINDABLE_NV",
            &Self::LIBRARY_KHR => "LIBRARY_KHR",
            &Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT => "FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT",
            &Self::EARLY_RETURN_ON_FAILURE_EXT => "EARLY_RETURN_ON_FAILURE_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::PipelineCreateFlagBits {
    pub const DISABLE_OPTIMIZATION: Self = Self(1);
    pub const ALLOW_DERIVATIVES: Self = Self(2);
    pub const DERIVATIVE: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPrimitiveTopology.html) · Enum"]
#[doc(alias = "VkPrimitiveTopology")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PrimitiveTopology(pub i32);
impl std::fmt::Debug for PrimitiveTopology {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::POINT_LIST => "POINT_LIST",
            &Self::LINE_LIST => "LINE_LIST",
            &Self::LINE_STRIP => "LINE_STRIP",
            &Self::TRIANGLE_LIST => "TRIANGLE_LIST",
            &Self::TRIANGLE_STRIP => "TRIANGLE_STRIP",
            &Self::TRIANGLE_FAN => "TRIANGLE_FAN",
            &Self::LINE_LIST_WITH_ADJACENCY => "LINE_LIST_WITH_ADJACENCY",
            &Self::LINE_STRIP_WITH_ADJACENCY => "LINE_STRIP_WITH_ADJACENCY",
            &Self::TRIANGLE_LIST_WITH_ADJACENCY => "TRIANGLE_LIST_WITH_ADJACENCY",
            &Self::TRIANGLE_STRIP_WITH_ADJACENCY => "TRIANGLE_STRIP_WITH_ADJACENCY",
            &Self::PATCH_LIST => "PATCH_LIST",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::PrimitiveTopology {
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryControlFlags.html) · Bitmask of [`QueryControlFlagBits`]"] # [doc (alias = "VkQueryControlFlags")] # [derive (Default)] # [repr (transparent)] pub struct QueryControlFlags : u32 { const PRECISE = QueryControlFlagBits :: PRECISE . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryControlFlagBits.html) · Bits enum of [`QueryControlFlags`]"]
#[doc(alias = "VkQueryControlFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueryControlFlagBits(pub u32);
impl QueryControlFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> QueryControlFlags {
        QueryControlFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for QueryControlFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PRECISE => "PRECISE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::QueryControlFlagBits {
    pub const PRECISE: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPipelineStatisticFlags.html) · Bitmask of [`QueryPipelineStatisticFlagBits`]"] # [doc (alias = "VkQueryPipelineStatisticFlags")] # [derive (Default)] # [repr (transparent)] pub struct QueryPipelineStatisticFlags : u32 { const INPUT_ASSEMBLY_VERTICES = QueryPipelineStatisticFlagBits :: INPUT_ASSEMBLY_VERTICES . 0 ; const INPUT_ASSEMBLY_PRIMITIVES = QueryPipelineStatisticFlagBits :: INPUT_ASSEMBLY_PRIMITIVES . 0 ; const VERTEX_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits :: VERTEX_SHADER_INVOCATIONS . 0 ; const GEOMETRY_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits :: GEOMETRY_SHADER_INVOCATIONS . 0 ; const GEOMETRY_SHADER_PRIMITIVES = QueryPipelineStatisticFlagBits :: GEOMETRY_SHADER_PRIMITIVES . 0 ; const CLIPPING_INVOCATIONS = QueryPipelineStatisticFlagBits :: CLIPPING_INVOCATIONS . 0 ; const CLIPPING_PRIMITIVES = QueryPipelineStatisticFlagBits :: CLIPPING_PRIMITIVES . 0 ; const FRAGMENT_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits :: FRAGMENT_SHADER_INVOCATIONS . 0 ; const TESSELLATION_CONTROL_SHADER_PATCHES = QueryPipelineStatisticFlagBits :: TESSELLATION_CONTROL_SHADER_PATCHES . 0 ; const TESSELLATION_EVALUATION_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits :: TESSELLATION_EVALUATION_SHADER_INVOCATIONS . 0 ; const COMPUTE_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits :: COMPUTE_SHADER_INVOCATIONS . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPipelineStatisticFlagBits.html) · Bits enum of [`QueryPipelineStatisticFlags`]"]
#[doc(alias = "VkQueryPipelineStatisticFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueryPipelineStatisticFlagBits(pub u32);
impl QueryPipelineStatisticFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> QueryPipelineStatisticFlags {
        QueryPipelineStatisticFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for QueryPipelineStatisticFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INPUT_ASSEMBLY_VERTICES => "INPUT_ASSEMBLY_VERTICES",
            &Self::INPUT_ASSEMBLY_PRIMITIVES => "INPUT_ASSEMBLY_PRIMITIVES",
            &Self::VERTEX_SHADER_INVOCATIONS => "VERTEX_SHADER_INVOCATIONS",
            &Self::GEOMETRY_SHADER_INVOCATIONS => "GEOMETRY_SHADER_INVOCATIONS",
            &Self::GEOMETRY_SHADER_PRIMITIVES => "GEOMETRY_SHADER_PRIMITIVES",
            &Self::CLIPPING_INVOCATIONS => "CLIPPING_INVOCATIONS",
            &Self::CLIPPING_PRIMITIVES => "CLIPPING_PRIMITIVES",
            &Self::FRAGMENT_SHADER_INVOCATIONS => "FRAGMENT_SHADER_INVOCATIONS",
            &Self::TESSELLATION_CONTROL_SHADER_PATCHES => "TESSELLATION_CONTROL_SHADER_PATCHES",
            &Self::TESSELLATION_EVALUATION_SHADER_INVOCATIONS => "TESSELLATION_EVALUATION_SHADER_INVOCATIONS",
            &Self::COMPUTE_SHADER_INVOCATIONS => "COMPUTE_SHADER_INVOCATIONS",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::QueryPipelineStatisticFlagBits {
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(1);
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(2);
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(4);
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(8);
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(16);
    pub const CLIPPING_INVOCATIONS: Self = Self(32);
    pub const CLIPPING_PRIMITIVES: Self = Self(64);
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(128);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(256);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(512);
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(1024);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryResultFlags.html) · Bitmask of [`QueryResultFlagBits`]"] # [doc (alias = "VkQueryResultFlags")] # [derive (Default)] # [repr (transparent)] pub struct QueryResultFlags : u32 { const _64 = QueryResultFlagBits :: _64 . 0 ; const WAIT = QueryResultFlagBits :: WAIT . 0 ; const WITH_AVAILABILITY = QueryResultFlagBits :: WITH_AVAILABILITY . 0 ; const PARTIAL = QueryResultFlagBits :: PARTIAL . 0 ; const WITH_STATUS_KHR = QueryResultFlagBits :: WITH_STATUS_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryResultFlagBits.html) · Bits enum of [`QueryResultFlags`]"]
#[doc(alias = "VkQueryResultFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueryResultFlagBits(pub u32);
impl QueryResultFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> QueryResultFlags {
        QueryResultFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for QueryResultFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_64 => "_64",
            &Self::WAIT => "WAIT",
            &Self::WITH_AVAILABILITY => "WITH_AVAILABILITY",
            &Self::PARTIAL => "PARTIAL",
            &Self::WITH_STATUS_KHR => "WITH_STATUS_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::QueryResultFlagBits {
    pub const _64: Self = Self(1);
    pub const WAIT: Self = Self(2);
    pub const WITH_AVAILABILITY: Self = Self(4);
    pub const PARTIAL: Self = Self(8);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryType.html) · Enum"]
#[doc(alias = "VkQueryType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueryType(pub i32);
impl std::fmt::Debug for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::OCCLUSION => "OCCLUSION",
            &Self::PIPELINE_STATISTICS => "PIPELINE_STATISTICS",
            &Self::TIMESTAMP => "TIMESTAMP",
            &Self::RESULT_STATUS_ONLY_KHR => "RESULT_STATUS_ONLY_KHR",
            &Self::TRANSFORM_FEEDBACK_STREAM_EXT => "TRANSFORM_FEEDBACK_STREAM_EXT",
            &Self::PERFORMANCE_QUERY_KHR => "PERFORMANCE_QUERY_KHR",
            &Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR => "ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR",
            &Self::ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR => "ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR",
            &Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV => "ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV",
            &Self::PERFORMANCE_QUERY_INTEL => "PERFORMANCE_QUERY_INTEL",
            &Self::VIDEO_ENCODESTREAM_BUFFER_RANGE_KHR => "VIDEO_ENCODESTREAM_BUFFER_RANGE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::QueryType {
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFlags.html) · Bitmask of [`QueueFlagBits`]"] # [doc (alias = "VkQueueFlags")] # [derive (Default)] # [repr (transparent)] pub struct QueueFlags : u32 { const GRAPHICS = QueueFlagBits :: GRAPHICS . 0 ; const COMPUTE = QueueFlagBits :: COMPUTE . 0 ; const TRANSFER = QueueFlagBits :: TRANSFER . 0 ; const SPARSE_BINDING = QueueFlagBits :: SPARSE_BINDING . 0 ; const PROTECTED = QueueFlagBits :: PROTECTED . 0 ; const VIDEO_DECODE_KHR = QueueFlagBits :: VIDEO_DECODE_KHR . 0 ; const VIDEO_ENCODE_KHR = QueueFlagBits :: VIDEO_ENCODE_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFlagBits.html) · Bits enum of [`QueueFlags`]"]
#[doc(alias = "VkQueueFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct QueueFlagBits(pub u32);
impl QueueFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> QueueFlags {
        QueueFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for QueueFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::GRAPHICS => "GRAPHICS",
            &Self::COMPUTE => "COMPUTE",
            &Self::TRANSFER => "TRANSFER",
            &Self::SPARSE_BINDING => "SPARSE_BINDING",
            &Self::PROTECTED => "PROTECTED",
            &Self::VIDEO_DECODE_KHR => "VIDEO_DECODE_KHR",
            &Self::VIDEO_ENCODE_KHR => "VIDEO_ENCODE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::QueueFlagBits {
    pub const GRAPHICS: Self = Self(1);
    pub const COMPUTE: Self = Self(2);
    pub const TRANSFER: Self = Self(4);
    pub const SPARSE_BINDING: Self = Self(8);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassContents.html) · Enum"]
#[doc(alias = "VkSubpassContents")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SubpassContents(pub i32);
impl std::fmt::Debug for SubpassContents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::INLINE => "INLINE",
            &Self::SECONDARY_COMMAND_BUFFERS => "SECONDARY_COMMAND_BUFFERS",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SubpassContents {
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResult.html) · Enum"]
#[doc(alias = "VkResult")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Result(pub i32);
impl std::fmt::Debug for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SUCCESS => "SUCCESS",
            &Self::NOT_READY => "NOT_READY",
            &Self::TIMEOUT => "TIMEOUT",
            &Self::EVENT_SET => "EVENT_SET",
            &Self::EVENT_RESET => "EVENT_RESET",
            &Self::INCOMPLETE => "INCOMPLETE",
            &Self::ERROR_OUT_OF_HOST_MEMORY => "ERROR_OUT_OF_HOST_MEMORY",
            &Self::ERROR_OUT_OF_DEVICE_MEMORY => "ERROR_OUT_OF_DEVICE_MEMORY",
            &Self::ERROR_INITIALIZATION_FAILED => "ERROR_INITIALIZATION_FAILED",
            &Self::ERROR_DEVICE_LOST => "ERROR_DEVICE_LOST",
            &Self::ERROR_MEMORY_MAP_FAILED => "ERROR_MEMORY_MAP_FAILED",
            &Self::ERROR_LAYER_NOT_PRESENT => "ERROR_LAYER_NOT_PRESENT",
            &Self::ERROR_EXTENSION_NOT_PRESENT => "ERROR_EXTENSION_NOT_PRESENT",
            &Self::ERROR_FEATURE_NOT_PRESENT => "ERROR_FEATURE_NOT_PRESENT",
            &Self::ERROR_INCOMPATIBLE_DRIVER => "ERROR_INCOMPATIBLE_DRIVER",
            &Self::ERROR_TOO_MANY_OBJECTS => "ERROR_TOO_MANY_OBJECTS",
            &Self::ERROR_FORMAT_NOT_SUPPORTED => "ERROR_FORMAT_NOT_SUPPORTED",
            &Self::ERROR_FRAGMENTED_POOL => "ERROR_FRAGMENTED_POOL",
            &Self::ERROR_UNKNOWN => "ERROR_UNKNOWN",
            &Self::ERROR_OUT_OF_POOL_MEMORY => "ERROR_OUT_OF_POOL_MEMORY",
            &Self::ERROR_INVALID_EXTERNAL_HANDLE => "ERROR_INVALID_EXTERNAL_HANDLE",
            &Self::ERROR_FRAGMENTATION => "ERROR_FRAGMENTATION",
            &Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => "ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS",
            &Self::ERROR_SURFACE_LOST_KHR => "ERROR_SURFACE_LOST_KHR",
            &Self::ERROR_NATIVE_WINDOW_IN_USE_KHR => "ERROR_NATIVE_WINDOW_IN_USE_KHR",
            &Self::SUBOPTIMAL_KHR => "SUBOPTIMAL_KHR",
            &Self::ERROR_OUT_OF_DATE_KHR => "ERROR_OUT_OF_DATE_KHR",
            &Self::ERROR_INCOMPATIBLE_DISPLAY_KHR => "ERROR_INCOMPATIBLE_DISPLAY_KHR",
            &Self::ERROR_VALIDATION_FAILED_EXT => "ERROR_VALIDATION_FAILED_EXT",
            &Self::ERROR_INVALID_SHADER_NV => "ERROR_INVALID_SHADER_NV",
            &Self::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => "ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT",
            &Self::ERROR_NOT_PERMITTED_EXT => "ERROR_NOT_PERMITTED_EXT",
            &Self::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => "ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT",
            &Self::THREAD_IDLE_KHR => "THREAD_IDLE_KHR",
            &Self::THREAD_DONE_KHR => "THREAD_DONE_KHR",
            &Self::OPERATION_DEFERRED_KHR => "OPERATION_DEFERRED_KHR",
            &Self::OPERATION_NOT_DEFERRED_KHR => "OPERATION_NOT_DEFERRED_KHR",
            &Self::PIPELINE_COMPILE_REQUIRED_EXT => "PIPELINE_COMPILE_REQUIRED_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::Result {
    pub const SUCCESS: Self = Self(0);
    pub const NOT_READY: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const EVENT_SET: Self = Self(3);
    pub const EVENT_RESET: Self = Self(4);
    pub const INCOMPLETE: Self = Self(5);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const ERROR_DEVICE_LOST: Self = Self(-4);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const ERROR_UNKNOWN: Self = Self(-13);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStageFlags.html) · Bitmask of [`ShaderStageFlagBits`]"] # [doc (alias = "VkShaderStageFlags")] # [derive (Default)] # [repr (transparent)] pub struct ShaderStageFlags : u32 { const VERTEX = ShaderStageFlagBits :: VERTEX . 0 ; const TESSELLATION_CONTROL = ShaderStageFlagBits :: TESSELLATION_CONTROL . 0 ; const TESSELLATION_EVALUATION = ShaderStageFlagBits :: TESSELLATION_EVALUATION . 0 ; const GEOMETRY = ShaderStageFlagBits :: GEOMETRY . 0 ; const FRAGMENT = ShaderStageFlagBits :: FRAGMENT . 0 ; const COMPUTE = ShaderStageFlagBits :: COMPUTE . 0 ; const ALL_GRAPHICS = ShaderStageFlagBits :: ALL_GRAPHICS . 0 ; const ALL = ShaderStageFlagBits :: ALL . 0 ; const RAYGEN_KHR = ShaderStageFlagBits :: RAYGEN_KHR . 0 ; const ANY_HIT_KHR = ShaderStageFlagBits :: ANY_HIT_KHR . 0 ; const CLOSEST_HIT_KHR = ShaderStageFlagBits :: CLOSEST_HIT_KHR . 0 ; const MISS_KHR = ShaderStageFlagBits :: MISS_KHR . 0 ; const INTERSECTION_KHR = ShaderStageFlagBits :: INTERSECTION_KHR . 0 ; const CALLABLE_KHR = ShaderStageFlagBits :: CALLABLE_KHR . 0 ; const TASK_NV = ShaderStageFlagBits :: TASK_NV . 0 ; const MESH_NV = ShaderStageFlagBits :: MESH_NV . 0 ; const RAYGEN_NV = ShaderStageFlagBits :: RAYGEN_NV . 0 ; const ANY_HIT_NV = ShaderStageFlagBits :: ANY_HIT_NV . 0 ; const CLOSEST_HIT_NV = ShaderStageFlagBits :: CLOSEST_HIT_NV . 0 ; const MISS_NV = ShaderStageFlagBits :: MISS_NV . 0 ; const INTERSECTION_NV = ShaderStageFlagBits :: INTERSECTION_NV . 0 ; const CALLABLE_NV = ShaderStageFlagBits :: CALLABLE_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderStageFlagBits.html) · Bits enum of [`ShaderStageFlags`]"]
#[doc(alias = "VkShaderStageFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ShaderStageFlagBits(pub u32);
impl ShaderStageFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ShaderStageFlags {
        ShaderStageFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ShaderStageFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VERTEX => "VERTEX",
            &Self::TESSELLATION_CONTROL => "TESSELLATION_CONTROL",
            &Self::TESSELLATION_EVALUATION => "TESSELLATION_EVALUATION",
            &Self::GEOMETRY => "GEOMETRY",
            &Self::FRAGMENT => "FRAGMENT",
            &Self::COMPUTE => "COMPUTE",
            &Self::ALL_GRAPHICS => "ALL_GRAPHICS",
            &Self::ALL => "ALL",
            &Self::RAYGEN_KHR => "RAYGEN_KHR",
            &Self::ANY_HIT_KHR => "ANY_HIT_KHR",
            &Self::CLOSEST_HIT_KHR => "CLOSEST_HIT_KHR",
            &Self::MISS_KHR => "MISS_KHR",
            &Self::INTERSECTION_KHR => "INTERSECTION_KHR",
            &Self::CALLABLE_KHR => "CALLABLE_KHR",
            &Self::TASK_NV => "TASK_NV",
            &Self::MESH_NV => "MESH_NV",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ShaderStageFlagBits {
    pub const VERTEX: Self = Self(1);
    pub const TESSELLATION_CONTROL: Self = Self(2);
    pub const TESSELLATION_EVALUATION: Self = Self(4);
    pub const GEOMETRY: Self = Self(8);
    pub const FRAGMENT: Self = Self(16);
    pub const COMPUTE: Self = Self(32);
    pub const ALL_GRAPHICS: Self = Self(31);
    pub const ALL: Self = Self(2147483647);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseMemoryBindFlags.html) · Bitmask of [`SparseMemoryBindFlagBits`]"] # [doc (alias = "VkSparseMemoryBindFlags")] # [derive (Default)] # [repr (transparent)] pub struct SparseMemoryBindFlags : u32 { const METADATA = SparseMemoryBindFlagBits :: METADATA . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseMemoryBindFlagBits.html) · Bits enum of [`SparseMemoryBindFlags`]"]
#[doc(alias = "VkSparseMemoryBindFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SparseMemoryBindFlagBits(pub u32);
impl SparseMemoryBindFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SparseMemoryBindFlags {
        SparseMemoryBindFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SparseMemoryBindFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::METADATA => "METADATA",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SparseMemoryBindFlagBits {
    pub const METADATA: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilFaceFlags.html) · Bitmask of [`StencilFaceFlagBits`]"] # [doc (alias = "VkStencilFaceFlags")] # [derive (Default)] # [repr (transparent)] pub struct StencilFaceFlags : u32 { const FRONT = StencilFaceFlagBits :: FRONT . 0 ; const BACK = StencilFaceFlagBits :: BACK . 0 ; const FRONT_AND_BACK = StencilFaceFlagBits :: FRONT_AND_BACK . 0 ; const STENCIL_FRONT_AND_BACK = StencilFaceFlagBits :: STENCIL_FRONT_AND_BACK . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilFaceFlagBits.html) · Bits enum of [`StencilFaceFlags`]"]
#[doc(alias = "VkStencilFaceFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StencilFaceFlagBits(pub u32);
impl StencilFaceFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> StencilFaceFlags {
        StencilFaceFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for StencilFaceFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FRONT => "FRONT",
            &Self::BACK => "BACK",
            &Self::FRONT_AND_BACK => "FRONT_AND_BACK",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::StencilFaceFlagBits {
    pub const FRONT: Self = Self(1);
    pub const BACK: Self = Self(2);
    pub const FRONT_AND_BACK: Self = Self(3);
    pub const STENCIL_FRONT_AND_BACK: Self = Self::FRONT_AND_BACK;
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOp.html) · Enum"]
#[doc(alias = "VkStencilOp")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StencilOp(pub i32);
impl std::fmt::Debug for StencilOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::KEEP => "KEEP",
            &Self::ZERO => "ZERO",
            &Self::REPLACE => "REPLACE",
            &Self::INCREMENT_AND_CLAMP => "INCREMENT_AND_CLAMP",
            &Self::DECREMENT_AND_CLAMP => "DECREMENT_AND_CLAMP",
            &Self::INVERT => "INVERT",
            &Self::INCREMENT_AND_WRAP => "INCREMENT_AND_WRAP",
            &Self::DECREMENT_AND_WRAP => "DECREMENT_AND_WRAP",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::StencilOp {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStructureType.html) · Enum"]
#[doc(alias = "VkStructureType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct StructureType(pub i32);
impl std::fmt::Debug for StructureType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::APPLICATION_INFO => "APPLICATION_INFO",
            &Self::INSTANCE_CREATE_INFO => "INSTANCE_CREATE_INFO",
            &Self::DEVICE_QUEUE_CREATE_INFO => "DEVICE_QUEUE_CREATE_INFO",
            &Self::DEVICE_CREATE_INFO => "DEVICE_CREATE_INFO",
            &Self::SUBMIT_INFO => "SUBMIT_INFO",
            &Self::MEMORY_ALLOCATE_INFO => "MEMORY_ALLOCATE_INFO",
            &Self::MAPPED_MEMORY_RANGE => "MAPPED_MEMORY_RANGE",
            &Self::BIND_SPARSE_INFO => "BIND_SPARSE_INFO",
            &Self::FENCE_CREATE_INFO => "FENCE_CREATE_INFO",
            &Self::SEMAPHORE_CREATE_INFO => "SEMAPHORE_CREATE_INFO",
            &Self::EVENT_CREATE_INFO => "EVENT_CREATE_INFO",
            &Self::QUERY_POOL_CREATE_INFO => "QUERY_POOL_CREATE_INFO",
            &Self::BUFFER_CREATE_INFO => "BUFFER_CREATE_INFO",
            &Self::BUFFER_VIEW_CREATE_INFO => "BUFFER_VIEW_CREATE_INFO",
            &Self::IMAGE_CREATE_INFO => "IMAGE_CREATE_INFO",
            &Self::IMAGE_VIEW_CREATE_INFO => "IMAGE_VIEW_CREATE_INFO",
            &Self::SHADER_MODULE_CREATE_INFO => "SHADER_MODULE_CREATE_INFO",
            &Self::PIPELINE_CACHE_CREATE_INFO => "PIPELINE_CACHE_CREATE_INFO",
            &Self::PIPELINE_SHADER_STAGE_CREATE_INFO => "PIPELINE_SHADER_STAGE_CREATE_INFO",
            &Self::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO => "PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO",
            &Self::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => "PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO",
            &Self::PIPELINE_TESSELLATION_STATE_CREATE_INFO => "PIPELINE_TESSELLATION_STATE_CREATE_INFO",
            &Self::PIPELINE_VIEWPORT_STATE_CREATE_INFO => "PIPELINE_VIEWPORT_STATE_CREATE_INFO",
            &Self::PIPELINE_RASTERIZATION_STATE_CREATE_INFO => "PIPELINE_RASTERIZATION_STATE_CREATE_INFO",
            &Self::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => "PIPELINE_MULTISAMPLE_STATE_CREATE_INFO",
            &Self::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => "PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO",
            &Self::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => "PIPELINE_COLOR_BLEND_STATE_CREATE_INFO",
            &Self::PIPELINE_DYNAMIC_STATE_CREATE_INFO => "PIPELINE_DYNAMIC_STATE_CREATE_INFO",
            &Self::GRAPHICS_PIPELINE_CREATE_INFO => "GRAPHICS_PIPELINE_CREATE_INFO",
            &Self::COMPUTE_PIPELINE_CREATE_INFO => "COMPUTE_PIPELINE_CREATE_INFO",
            &Self::PIPELINE_LAYOUT_CREATE_INFO => "PIPELINE_LAYOUT_CREATE_INFO",
            &Self::SAMPLER_CREATE_INFO => "SAMPLER_CREATE_INFO",
            &Self::DESCRIPTOR_SET_LAYOUT_CREATE_INFO => "DESCRIPTOR_SET_LAYOUT_CREATE_INFO",
            &Self::DESCRIPTOR_POOL_CREATE_INFO => "DESCRIPTOR_POOL_CREATE_INFO",
            &Self::DESCRIPTOR_SET_ALLOCATE_INFO => "DESCRIPTOR_SET_ALLOCATE_INFO",
            &Self::WRITE_DESCRIPTOR_SET => "WRITE_DESCRIPTOR_SET",
            &Self::COPY_DESCRIPTOR_SET => "COPY_DESCRIPTOR_SET",
            &Self::FRAMEBUFFER_CREATE_INFO => "FRAMEBUFFER_CREATE_INFO",
            &Self::RENDER_PASS_CREATE_INFO => "RENDER_PASS_CREATE_INFO",
            &Self::COMMAND_POOL_CREATE_INFO => "COMMAND_POOL_CREATE_INFO",
            &Self::COMMAND_BUFFER_ALLOCATE_INFO => "COMMAND_BUFFER_ALLOCATE_INFO",
            &Self::COMMAND_BUFFER_INHERITANCE_INFO => "COMMAND_BUFFER_INHERITANCE_INFO",
            &Self::COMMAND_BUFFER_BEGIN_INFO => "COMMAND_BUFFER_BEGIN_INFO",
            &Self::RENDER_PASS_BEGIN_INFO => "RENDER_PASS_BEGIN_INFO",
            &Self::BUFFER_MEMORY_BARRIER => "BUFFER_MEMORY_BARRIER",
            &Self::IMAGE_MEMORY_BARRIER => "IMAGE_MEMORY_BARRIER",
            &Self::MEMORY_BARRIER => "MEMORY_BARRIER",
            &Self::LOADER_INSTANCE_CREATE_INFO => "LOADER_INSTANCE_CREATE_INFO",
            &Self::LOADER_DEVICE_CREATE_INFO => "LOADER_DEVICE_CREATE_INFO",
            &Self::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => "PHYSICAL_DEVICE_SUBGROUP_PROPERTIES",
            &Self::BIND_BUFFER_MEMORY_INFO => "BIND_BUFFER_MEMORY_INFO",
            &Self::BIND_IMAGE_MEMORY_INFO => "BIND_IMAGE_MEMORY_INFO",
            &Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES => "PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES",
            &Self::MEMORY_DEDICATED_REQUIREMENTS => "MEMORY_DEDICATED_REQUIREMENTS",
            &Self::MEMORY_DEDICATED_ALLOCATE_INFO => "MEMORY_DEDICATED_ALLOCATE_INFO",
            &Self::MEMORY_ALLOCATE_FLAGS_INFO => "MEMORY_ALLOCATE_FLAGS_INFO",
            &Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => "DEVICE_GROUP_RENDER_PASS_BEGIN_INFO",
            &Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => "DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO",
            &Self::DEVICE_GROUP_SUBMIT_INFO => "DEVICE_GROUP_SUBMIT_INFO",
            &Self::DEVICE_GROUP_BIND_SPARSE_INFO => "DEVICE_GROUP_BIND_SPARSE_INFO",
            &Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => "BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO",
            &Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => "BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO",
            &Self::PHYSICAL_DEVICE_GROUP_PROPERTIES => "PHYSICAL_DEVICE_GROUP_PROPERTIES",
            &Self::DEVICE_GROUP_DEVICE_CREATE_INFO => "DEVICE_GROUP_DEVICE_CREATE_INFO",
            &Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2 => "BUFFER_MEMORY_REQUIREMENTS_INFO_2",
            &Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2 => "IMAGE_MEMORY_REQUIREMENTS_INFO_2",
            &Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => "IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2",
            &Self::MEMORY_REQUIREMENTS_2 => "MEMORY_REQUIREMENTS_2",
            &Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => "SPARSE_IMAGE_MEMORY_REQUIREMENTS_2",
            &Self::PHYSICAL_DEVICE_FEATURES_2 => "PHYSICAL_DEVICE_FEATURES_2",
            &Self::PHYSICAL_DEVICE_PROPERTIES_2 => "PHYSICAL_DEVICE_PROPERTIES_2",
            &Self::FORMAT_PROPERTIES_2 => "FORMAT_PROPERTIES_2",
            &Self::IMAGE_FORMAT_PROPERTIES_2 => "IMAGE_FORMAT_PROPERTIES_2",
            &Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => "PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2",
            &Self::QUEUE_FAMILY_PROPERTIES_2 => "QUEUE_FAMILY_PROPERTIES_2",
            &Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => "PHYSICAL_DEVICE_MEMORY_PROPERTIES_2",
            &Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2 => "SPARSE_IMAGE_FORMAT_PROPERTIES_2",
            &Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => "PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2",
            &Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => "PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES",
            &Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => "RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO",
            &Self::IMAGE_VIEW_USAGE_CREATE_INFO => "IMAGE_VIEW_USAGE_CREATE_INFO",
            &Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => "PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO",
            &Self::RENDER_PASS_MULTIVIEW_CREATE_INFO => "RENDER_PASS_MULTIVIEW_CREATE_INFO",
            &Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES => "PHYSICAL_DEVICE_MULTIVIEW_FEATURES",
            &Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => "PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES",
            &Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES => "PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES",
            &Self::PROTECTED_SUBMIT_INFO => "PROTECTED_SUBMIT_INFO",
            &Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => "PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES",
            &Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => "PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES",
            &Self::DEVICE_QUEUE_INFO_2 => "DEVICE_QUEUE_INFO_2",
            &Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO => "SAMPLER_YCBCR_CONVERSION_CREATE_INFO",
            &Self::SAMPLER_YCBCR_CONVERSION_INFO => "SAMPLER_YCBCR_CONVERSION_INFO",
            &Self::BIND_IMAGE_PLANE_MEMORY_INFO => "BIND_IMAGE_PLANE_MEMORY_INFO",
            &Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO => "IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO",
            &Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => "PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES",
            &Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => "SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES",
            &Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => "DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO",
            &Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => "PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO",
            &Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES => "EXTERNAL_IMAGE_FORMAT_PROPERTIES",
            &Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => "PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO",
            &Self::EXTERNAL_BUFFER_PROPERTIES => "EXTERNAL_BUFFER_PROPERTIES",
            &Self::PHYSICAL_DEVICE_ID_PROPERTIES => "PHYSICAL_DEVICE_ID_PROPERTIES",
            &Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => "EXTERNAL_MEMORY_BUFFER_CREATE_INFO",
            &Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => "EXTERNAL_MEMORY_IMAGE_CREATE_INFO",
            &Self::EXPORT_MEMORY_ALLOCATE_INFO => "EXPORT_MEMORY_ALLOCATE_INFO",
            &Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => "PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO",
            &Self::EXTERNAL_FENCE_PROPERTIES => "EXTERNAL_FENCE_PROPERTIES",
            &Self::EXPORT_FENCE_CREATE_INFO => "EXPORT_FENCE_CREATE_INFO",
            &Self::EXPORT_SEMAPHORE_CREATE_INFO => "EXPORT_SEMAPHORE_CREATE_INFO",
            &Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => "PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO",
            &Self::EXTERNAL_SEMAPHORE_PROPERTIES => "EXTERNAL_SEMAPHORE_PROPERTIES",
            &Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => "PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES",
            &Self::DESCRIPTOR_SET_LAYOUT_SUPPORT => "DESCRIPTOR_SET_LAYOUT_SUPPORT",
            &Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => "PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES",
            &Self::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => "PHYSICAL_DEVICE_VULKAN_1_1_FEATURES",
            &Self::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES => "PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES",
            &Self::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => "PHYSICAL_DEVICE_VULKAN_1_2_FEATURES",
            &Self::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES => "PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES",
            &Self::IMAGE_FORMAT_LIST_CREATE_INFO => "IMAGE_FORMAT_LIST_CREATE_INFO",
            &Self::ATTACHMENT_DESCRIPTION_2 => "ATTACHMENT_DESCRIPTION_2",
            &Self::ATTACHMENT_REFERENCE_2 => "ATTACHMENT_REFERENCE_2",
            &Self::SUBPASS_DESCRIPTION_2 => "SUBPASS_DESCRIPTION_2",
            &Self::SUBPASS_DEPENDENCY_2 => "SUBPASS_DEPENDENCY_2",
            &Self::RENDER_PASS_CREATE_INFO_2 => "RENDER_PASS_CREATE_INFO_2",
            &Self::SUBPASS_BEGIN_INFO => "SUBPASS_BEGIN_INFO",
            &Self::SUBPASS_END_INFO => "SUBPASS_END_INFO",
            &Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES => "PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES",
            &Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES => "PHYSICAL_DEVICE_DRIVER_PROPERTIES",
            &Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES => "PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES",
            &Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES => "PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES",
            &Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES => "PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES",
            &Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO => "DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO",
            &Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES",
            &Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES => "PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES",
            &Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO => "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO",
            &Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT => "DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT",
            &Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES => "PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES",
            &Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE => "SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE",
            &Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES => "PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES",
            &Self::IMAGE_STENCIL_USAGE_CREATE_INFO => "IMAGE_STENCIL_USAGE_CREATE_INFO",
            &Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES => "PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES",
            &Self::SAMPLER_REDUCTION_MODE_CREATE_INFO => "SAMPLER_REDUCTION_MODE_CREATE_INFO",
            &Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES => "PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES",
            &Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES => "PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES",
            &Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO => "FRAMEBUFFER_ATTACHMENTS_CREATE_INFO",
            &Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO => "FRAMEBUFFER_ATTACHMENT_IMAGE_INFO",
            &Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO => "RENDER_PASS_ATTACHMENT_BEGIN_INFO",
            &Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES => "PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES",
            &Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES => "PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES",
            &Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES => "PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES",
            &Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT => "ATTACHMENT_REFERENCE_STENCIL_LAYOUT",
            &Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT => "ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT",
            &Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES => "PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES",
            &Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES => "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES",
            &Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES => "PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES",
            &Self::SEMAPHORE_TYPE_CREATE_INFO => "SEMAPHORE_TYPE_CREATE_INFO",
            &Self::TIMELINE_SEMAPHORE_SUBMIT_INFO => "TIMELINE_SEMAPHORE_SUBMIT_INFO",
            &Self::SEMAPHORE_WAIT_INFO => "SEMAPHORE_WAIT_INFO",
            &Self::SEMAPHORE_SIGNAL_INFO => "SEMAPHORE_SIGNAL_INFO",
            &Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES",
            &Self::BUFFER_DEVICE_ADDRESS_INFO => "BUFFER_DEVICE_ADDRESS_INFO",
            &Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO => "BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO",
            &Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO => "MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO",
            &Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO => "DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO",
            &Self::SWAPCHAIN_CREATE_INFO_KHR => "SWAPCHAIN_CREATE_INFO_KHR",
            &Self::PRESENT_INFO_KHR => "PRESENT_INFO_KHR",
            &Self::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR => "DEVICE_GROUP_PRESENT_CAPABILITIES_KHR",
            &Self::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => "IMAGE_SWAPCHAIN_CREATE_INFO_KHR",
            &Self::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => "BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR",
            &Self::ACQUIRE_NEXT_IMAGE_INFO_KHR => "ACQUIRE_NEXT_IMAGE_INFO_KHR",
            &Self::DEVICE_GROUP_PRESENT_INFO_KHR => "DEVICE_GROUP_PRESENT_INFO_KHR",
            &Self::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => "DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR",
            &Self::DISPLAY_MODE_CREATE_INFO_KHR => "DISPLAY_MODE_CREATE_INFO_KHR",
            &Self::DISPLAY_SURFACE_CREATE_INFO_KHR => "DISPLAY_SURFACE_CREATE_INFO_KHR",
            &Self::DISPLAY_PRESENT_INFO_KHR => "DISPLAY_PRESENT_INFO_KHR",
            &Self::XLIB_SURFACE_CREATE_INFO_KHR => "XLIB_SURFACE_CREATE_INFO_KHR",
            &Self::XCB_SURFACE_CREATE_INFO_KHR => "XCB_SURFACE_CREATE_INFO_KHR",
            &Self::WAYLAND_SURFACE_CREATE_INFO_KHR => "WAYLAND_SURFACE_CREATE_INFO_KHR",
            &Self::ANDROID_SURFACE_CREATE_INFO_KHR => "ANDROID_SURFACE_CREATE_INFO_KHR",
            &Self::WIN32_SURFACE_CREATE_INFO_KHR => "WIN32_SURFACE_CREATE_INFO_KHR",
            &Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => "DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT",
            &Self::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD => "PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD",
            &Self::DEBUG_MARKER_OBJECT_NAME_INFO_EXT => "DEBUG_MARKER_OBJECT_NAME_INFO_EXT",
            &Self::DEBUG_MARKER_OBJECT_TAG_INFO_EXT => "DEBUG_MARKER_OBJECT_TAG_INFO_EXT",
            &Self::DEBUG_MARKER_MARKER_INFO_EXT => "DEBUG_MARKER_MARKER_INFO_EXT",
            &Self::VIDEO_PROFILE_KHR => "VIDEO_PROFILE_KHR",
            &Self::VIDEO_CAPABILITIES_KHR => "VIDEO_CAPABILITIES_KHR",
            &Self::VIDEO_PICTURE_RESOURCE_KHR => "VIDEO_PICTURE_RESOURCE_KHR",
            &Self::VIDEO_GET_MEMORY_PROPERTIES_KHR => "VIDEO_GET_MEMORY_PROPERTIES_KHR",
            &Self::VIDEO_BIND_MEMORY_KHR => "VIDEO_BIND_MEMORY_KHR",
            &Self::VIDEO_SESSION_CREATE_INFO_KHR => "VIDEO_SESSION_CREATE_INFO_KHR",
            &Self::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR => "VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR",
            &Self::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR => "VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR",
            &Self::VIDEO_BEGIN_CODING_INFO_KHR => "VIDEO_BEGIN_CODING_INFO_KHR",
            &Self::VIDEO_END_CODING_INFO_KHR => "VIDEO_END_CODING_INFO_KHR",
            &Self::VIDEO_CODING_CONTROL_INFO_KHR => "VIDEO_CODING_CONTROL_INFO_KHR",
            &Self::VIDEO_REFERENCE_SLOT_KHR => "VIDEO_REFERENCE_SLOT_KHR",
            &Self::VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR => "VIDEO_QUEUE_FAMILY_PROPERTIES_2_KHR",
            &Self::VIDEO_PROFILES_KHR => "VIDEO_PROFILES_KHR",
            &Self::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR => "PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR",
            &Self::VIDEO_FORMAT_PROPERTIES_KHR => "VIDEO_FORMAT_PROPERTIES_KHR",
            &Self::VIDEO_DECODE_INFO_KHR => "VIDEO_DECODE_INFO_KHR",
            &Self::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => "DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV",
            &Self::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => "DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV",
            &Self::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV => "DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV",
            &Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT => "PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT => "PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT",
            &Self::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT => "PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT",
            &Self::CU_MODULE_CREATE_INFO_NVX => "CU_MODULE_CREATE_INFO_NVX",
            &Self::CU_FUNCTION_CREATE_INFO_NVX => "CU_FUNCTION_CREATE_INFO_NVX",
            &Self::CU_LAUNCH_INFO_NVX => "CU_LAUNCH_INFO_NVX",
            &Self::IMAGE_VIEW_HANDLE_INFO_NVX => "IMAGE_VIEW_HANDLE_INFO_NVX",
            &Self::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX => "IMAGE_VIEW_ADDRESS_PROPERTIES_NVX",
            &Self::VIDEO_ENCODE_H264_CAPABILITIES_EXT => "VIDEO_ENCODE_H264_CAPABILITIES_EXT",
            &Self::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT => "VIDEO_ENCODE_H264_SESSION_CREATE_INFO_EXT",
            &Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT => "VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT",
            &Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT => "VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT",
            &Self::VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT => "VIDEO_ENCODE_H264_VCL_FRAME_INFO_EXT",
            &Self::VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT => "VIDEO_ENCODE_H264_DPB_SLOT_INFO_EXT",
            &Self::VIDEO_ENCODE_H264_NALU_SLICE_EXT => "VIDEO_ENCODE_H264_NALU_SLICE_EXT",
            &Self::VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT => "VIDEO_ENCODE_H264_EMIT_PICTURE_PARAMETERS_EXT",
            &Self::VIDEO_ENCODE_H264_PROFILE_EXT => "VIDEO_ENCODE_H264_PROFILE_EXT",
            &Self::VIDEO_DECODE_H264_CAPABILITIES_EXT => "VIDEO_DECODE_H264_CAPABILITIES_EXT",
            &Self::VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT => "VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT",
            &Self::VIDEO_DECODE_H264_PICTURE_INFO_EXT => "VIDEO_DECODE_H264_PICTURE_INFO_EXT",
            &Self::VIDEO_DECODE_H264_MVC_EXT => "VIDEO_DECODE_H264_MVC_EXT",
            &Self::VIDEO_DECODE_H264_PROFILE_EXT => "VIDEO_DECODE_H264_PROFILE_EXT",
            &Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT => "VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT",
            &Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT => "VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT",
            &Self::VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT => "VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT",
            &Self::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD => "TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD",
            &Self::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP => "STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP",
            &Self::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV => "PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV",
            &Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => "EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV",
            &Self::EXPORT_MEMORY_ALLOCATE_INFO_NV => "EXPORT_MEMORY_ALLOCATE_INFO_NV",
            &Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => "IMPORT_MEMORY_WIN32_HANDLE_INFO_NV",
            &Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV => "EXPORT_MEMORY_WIN32_HANDLE_INFO_NV",
            &Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV => "WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV",
            &Self::VALIDATION_FLAGS_EXT => "VALIDATION_FLAGS_EXT",
            &Self::VI_SURFACE_CREATE_INFO_NN => "VI_SURFACE_CREATE_INFO_NN",
            &Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT => "PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT",
            &Self::IMAGE_VIEW_ASTC_DECODE_MODE_EXT => "IMAGE_VIEW_ASTC_DECODE_MODE_EXT",
            &Self::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT => "PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT",
            &Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => "IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR",
            &Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => "EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR",
            &Self::MEMORY_WIN32_HANDLE_PROPERTIES_KHR => "MEMORY_WIN32_HANDLE_PROPERTIES_KHR",
            &Self::MEMORY_GET_WIN32_HANDLE_INFO_KHR => "MEMORY_GET_WIN32_HANDLE_INFO_KHR",
            &Self::IMPORT_MEMORY_FD_INFO_KHR => "IMPORT_MEMORY_FD_INFO_KHR",
            &Self::MEMORY_FD_PROPERTIES_KHR => "MEMORY_FD_PROPERTIES_KHR",
            &Self::MEMORY_GET_FD_INFO_KHR => "MEMORY_GET_FD_INFO_KHR",
            &Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR => "WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR",
            &Self::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => "IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR",
            &Self::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => "EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR",
            &Self::D3D12_FENCE_SUBMIT_INFO_KHR => "D3D12_FENCE_SUBMIT_INFO_KHR",
            &Self::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR => "SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR",
            &Self::IMPORT_SEMAPHORE_FD_INFO_KHR => "IMPORT_SEMAPHORE_FD_INFO_KHR",
            &Self::SEMAPHORE_GET_FD_INFO_KHR => "SEMAPHORE_GET_FD_INFO_KHR",
            &Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR => "PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR",
            &Self::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT => "COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT",
            &Self::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT => "PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT",
            &Self::CONDITIONAL_RENDERING_BEGIN_INFO_EXT => "CONDITIONAL_RENDERING_BEGIN_INFO_EXT",
            &Self::PRESENT_REGIONS_KHR => "PRESENT_REGIONS_KHR",
            &Self::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV => "PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV",
            &Self::SURFACE_CAPABILITIES_2_EXT => "SURFACE_CAPABILITIES_2_EXT",
            &Self::DISPLAY_POWER_INFO_EXT => "DISPLAY_POWER_INFO_EXT",
            &Self::DEVICE_EVENT_INFO_EXT => "DEVICE_EVENT_INFO_EXT",
            &Self::DISPLAY_EVENT_INFO_EXT => "DISPLAY_EVENT_INFO_EXT",
            &Self::SWAPCHAIN_COUNTER_CREATE_INFO_EXT => "SWAPCHAIN_COUNTER_CREATE_INFO_EXT",
            &Self::PRESENT_TIMES_INFO_GOOGLE => "PRESENT_TIMES_INFO_GOOGLE",
            &Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX => "PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX",
            &Self::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV => "PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV",
            &Self::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT => "PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT",
            &Self::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT => "PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT => "PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT",
            &Self::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT => "PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT => "PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT",
            &Self::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT => "PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT",
            &Self::HDR_METADATA_EXT => "HDR_METADATA_EXT",
            &Self::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR => "SHARED_PRESENT_SURFACE_CAPABILITIES_KHR",
            &Self::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR => "IMPORT_FENCE_WIN32_HANDLE_INFO_KHR",
            &Self::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR => "EXPORT_FENCE_WIN32_HANDLE_INFO_KHR",
            &Self::FENCE_GET_WIN32_HANDLE_INFO_KHR => "FENCE_GET_WIN32_HANDLE_INFO_KHR",
            &Self::IMPORT_FENCE_FD_INFO_KHR => "IMPORT_FENCE_FD_INFO_KHR",
            &Self::FENCE_GET_FD_INFO_KHR => "FENCE_GET_FD_INFO_KHR",
            &Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR => "PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR",
            &Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR => "PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR",
            &Self::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR => "QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR",
            &Self::PERFORMANCE_QUERY_SUBMIT_INFO_KHR => "PERFORMANCE_QUERY_SUBMIT_INFO_KHR",
            &Self::ACQUIRE_PROFILING_LOCK_INFO_KHR => "ACQUIRE_PROFILING_LOCK_INFO_KHR",
            &Self::PERFORMANCE_COUNTER_KHR => "PERFORMANCE_COUNTER_KHR",
            &Self::PERFORMANCE_COUNTER_DESCRIPTION_KHR => "PERFORMANCE_COUNTER_DESCRIPTION_KHR",
            &Self::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR => "PHYSICAL_DEVICE_SURFACE_INFO_2_KHR",
            &Self::SURFACE_CAPABILITIES_2_KHR => "SURFACE_CAPABILITIES_2_KHR",
            &Self::SURFACE_FORMAT_2_KHR => "SURFACE_FORMAT_2_KHR",
            &Self::DISPLAY_PROPERTIES_2_KHR => "DISPLAY_PROPERTIES_2_KHR",
            &Self::DISPLAY_PLANE_PROPERTIES_2_KHR => "DISPLAY_PLANE_PROPERTIES_2_KHR",
            &Self::DISPLAY_MODE_PROPERTIES_2_KHR => "DISPLAY_MODE_PROPERTIES_2_KHR",
            &Self::DISPLAY_PLANE_INFO_2_KHR => "DISPLAY_PLANE_INFO_2_KHR",
            &Self::DISPLAY_PLANE_CAPABILITIES_2_KHR => "DISPLAY_PLANE_CAPABILITIES_2_KHR",
            &Self::IOS_SURFACE_CREATE_INFO_MVK => "IOS_SURFACE_CREATE_INFO_MVK",
            &Self::MACOS_SURFACE_CREATE_INFO_MVK => "MACOS_SURFACE_CREATE_INFO_MVK",
            &Self::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => "DEBUG_UTILS_OBJECT_NAME_INFO_EXT",
            &Self::DEBUG_UTILS_OBJECT_TAG_INFO_EXT => "DEBUG_UTILS_OBJECT_TAG_INFO_EXT",
            &Self::DEBUG_UTILS_LABEL_EXT => "DEBUG_UTILS_LABEL_EXT",
            &Self::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => "DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT",
            &Self::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => "DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT",
            &Self::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID => "ANDROID_HARDWARE_BUFFER_USAGE_ANDROID",
            &Self::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID => "ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID",
            &Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID => "ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID",
            &Self::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => "IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID",
            &Self::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => "MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID",
            &Self::EXTERNAL_FORMAT_ANDROID => "EXTERNAL_FORMAT_ANDROID",
            &Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT => "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT => "PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT",
            &Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT => "WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT",
            &Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT => "DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT",
            &Self::SAMPLE_LOCATIONS_INFO_EXT => "SAMPLE_LOCATIONS_INFO_EXT",
            &Self::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT => "RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT",
            &Self::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT => "PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT => "PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT",
            &Self::MULTISAMPLE_PROPERTIES_EXT => "MULTISAMPLE_PROPERTIES_EXT",
            &Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT => "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT => "PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT",
            &Self::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT => "PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT",
            &Self::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV => "PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV",
            &Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR => "WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR",
            &Self::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR => "ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR",
            &Self::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR => "ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR",
            &Self::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR => "ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR",
            &Self::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR => "ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR",
            &Self::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR => "ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR",
            &Self::ACCELERATION_STRUCTURE_GEOMETRY_KHR => "ACCELERATION_STRUCTURE_GEOMETRY_KHR",
            &Self::ACCELERATION_STRUCTURE_VERSION_INFO_KHR => "ACCELERATION_STRUCTURE_VERSION_INFO_KHR",
            &Self::COPY_ACCELERATION_STRUCTURE_INFO_KHR => "COPY_ACCELERATION_STRUCTURE_INFO_KHR",
            &Self::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR => "COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR",
            &Self::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR => "COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR",
            &Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR => "PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR",
            &Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR => "PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR",
            &Self::ACCELERATION_STRUCTURE_CREATE_INFO_KHR => "ACCELERATION_STRUCTURE_CREATE_INFO_KHR",
            &Self::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR => "ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR",
            &Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR => "PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR",
            &Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR => "PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR",
            &Self::RAY_TRACING_PIPELINE_CREATE_INFO_KHR => "RAY_TRACING_PIPELINE_CREATE_INFO_KHR",
            &Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR => "RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR",
            &Self::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR => "RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR",
            &Self::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR => "PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR",
            &Self::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV => "PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV",
            &Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV => "PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV",
            &Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV => "PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV",
            &Self::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT => "DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT",
            &Self::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT => "PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT",
            &Self::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT => "IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT",
            &Self::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT => "IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT",
            &Self::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT => "IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT",
            &Self::VALIDATION_CACHE_CREATE_INFO_EXT => "VALIDATION_CACHE_CREATE_INFO_EXT",
            &Self::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT => "SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR => "PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR",
            &Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR => "PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR",
            &Self::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV => "PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV",
            &Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV => "PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV",
            &Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV => "PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV",
            &Self::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV => "PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV",
            &Self::RAY_TRACING_PIPELINE_CREATE_INFO_NV => "RAY_TRACING_PIPELINE_CREATE_INFO_NV",
            &Self::ACCELERATION_STRUCTURE_CREATE_INFO_NV => "ACCELERATION_STRUCTURE_CREATE_INFO_NV",
            &Self::GEOMETRY_NV => "GEOMETRY_NV",
            &Self::GEOMETRY_TRIANGLES_NV => "GEOMETRY_TRIANGLES_NV",
            &Self::GEOMETRY_AABB_NV => "GEOMETRY_AABB_NV",
            &Self::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV => "BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV",
            &Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV => "WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV",
            &Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV => "ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV",
            &Self::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV => "PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV",
            &Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV => "RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV",
            &Self::ACCELERATION_STRUCTURE_INFO_NV => "ACCELERATION_STRUCTURE_INFO_NV",
            &Self::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV => "PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV",
            &Self::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV => "PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV",
            &Self::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT => "PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT",
            &Self::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT => "FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT",
            &Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT => "DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT",
            &Self::IMPORT_MEMORY_HOST_POINTER_INFO_EXT => "IMPORT_MEMORY_HOST_POINTER_INFO_EXT",
            &Self::MEMORY_HOST_POINTER_PROPERTIES_EXT => "MEMORY_HOST_POINTER_PROPERTIES_EXT",
            &Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT => "PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT",
            &Self::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR => "PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR",
            &Self::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD => "PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD",
            &Self::CALIBRATED_TIMESTAMP_INFO_EXT => "CALIBRATED_TIMESTAMP_INFO_EXT",
            &Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD => "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD",
            &Self::VIDEO_DECODE_H265_CAPABILITIES_EXT => "VIDEO_DECODE_H265_CAPABILITIES_EXT",
            &Self::VIDEO_DECODE_H265_SESSION_CREATE_INFO_EXT => "VIDEO_DECODE_H265_SESSION_CREATE_INFO_EXT",
            &Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT => "VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_EXT",
            &Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT => "VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_EXT",
            &Self::VIDEO_DECODE_H265_PROFILE_EXT => "VIDEO_DECODE_H265_PROFILE_EXT",
            &Self::VIDEO_DECODE_H265_PICTURE_INFO_EXT => "VIDEO_DECODE_H265_PICTURE_INFO_EXT",
            &Self::VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT => "VIDEO_DECODE_H265_DPB_SLOT_INFO_EXT",
            &Self::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD => "DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD",
            &Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT => "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT",
            &Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT => "PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT => "PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT",
            &Self::PRESENT_FRAME_TOKEN_GGP => "PRESENT_FRAME_TOKEN_GGP",
            &Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT => "PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV => "PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV",
            &Self::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV => "PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV",
            &Self::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV => "PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV",
            &Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV => "PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV",
            &Self::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV => "PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV",
            &Self::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV => "PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV",
            &Self::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV => "PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV",
            &Self::CHECKPOINT_DATA_NV => "CHECKPOINT_DATA_NV",
            &Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV => "QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV",
            &Self::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL => "PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL",
            &Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL => "QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL",
            &Self::INITIALIZE_PERFORMANCE_API_INFO_INTEL => "INITIALIZE_PERFORMANCE_API_INFO_INTEL",
            &Self::PERFORMANCE_MARKER_INFO_INTEL => "PERFORMANCE_MARKER_INFO_INTEL",
            &Self::PERFORMANCE_STREAM_MARKER_INFO_INTEL => "PERFORMANCE_STREAM_MARKER_INFO_INTEL",
            &Self::PERFORMANCE_OVERRIDE_INFO_INTEL => "PERFORMANCE_OVERRIDE_INFO_INTEL",
            &Self::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL => "PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL",
            &Self::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT => "PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT",
            &Self::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD => "DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD",
            &Self::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD => "SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD",
            &Self::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA => "IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA",
            &Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR => "PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR",
            &Self::METAL_SURFACE_CREATE_INFO_EXT => "METAL_SURFACE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT => "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT => "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT",
            &Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT => "RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT => "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT",
            &Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT => "PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT => "PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR",
            &Self::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR => "PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR",
            &Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR => "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR",
            &Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR => "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR",
            &Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR => "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR",
            &Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD => "PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD",
            &Self::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD => "PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD",
            &Self::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT => "PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT => "PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT",
            &Self::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT => "PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT",
            &Self::MEMORY_PRIORITY_ALLOCATE_INFO_EXT => "MEMORY_PRIORITY_ALLOCATE_INFO_EXT",
            &Self::SURFACE_PROTECTED_CAPABILITIES_KHR => "SURFACE_PROTECTED_CAPABILITIES_KHR",
            &Self::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV => "PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV",
            &Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT => "PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT",
            &Self::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT => "BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT => "PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT",
            &Self::VALIDATION_FEATURES_EXT => "VALIDATION_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV => "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV",
            &Self::COOPERATIVE_MATRIX_PROPERTIES_NV => "COOPERATIVE_MATRIX_PROPERTIES_NV",
            &Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV => "PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV",
            &Self::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV => "PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV",
            &Self::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV => "PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV",
            &Self::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV => "FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV",
            &Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT => "PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT => "PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT => "PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT",
            &Self::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT => "PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT => "PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT",
            &Self::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT => "SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT",
            &Self::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT => "SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT",
            &Self::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT => "SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT",
            &Self::HEADLESS_SURFACE_CREATE_INFO_EXT => "HEADLESS_SURFACE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT => "PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT",
            &Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT => "PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT => "PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT",
            &Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT => "PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT => "PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT => "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR => "PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR",
            &Self::PIPELINE_INFO_KHR => "PIPELINE_INFO_KHR",
            &Self::PIPELINE_EXECUTABLE_PROPERTIES_KHR => "PIPELINE_EXECUTABLE_PROPERTIES_KHR",
            &Self::PIPELINE_EXECUTABLE_INFO_KHR => "PIPELINE_EXECUTABLE_INFO_KHR",
            &Self::PIPELINE_EXECUTABLE_STATISTIC_KHR => "PIPELINE_EXECUTABLE_STATISTIC_KHR",
            &Self::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR => "PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR",
            &Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT => "PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV => "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV",
            &Self::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV => "GRAPHICS_SHADER_GROUP_CREATE_INFO_NV",
            &Self::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV => "GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV",
            &Self::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV => "INDIRECT_COMMANDS_LAYOUT_TOKEN_NV",
            &Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV => "INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV",
            &Self::GENERATED_COMMANDS_INFO_NV => "GENERATED_COMMANDS_INFO_NV",
            &Self::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV => "GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV",
            &Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV => "PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV",
            &Self::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV => "PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV",
            &Self::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV => "COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV",
            &Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT => "PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT => "PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT",
            &Self::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM => "COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM",
            &Self::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM => "RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM",
            &Self::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT => "PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT",
            &Self::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT => "DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT",
            &Self::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT => "DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT",
            &Self::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT => "PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT => "PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT",
            &Self::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT => "SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT => "PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT",
            &Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT => "PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT",
            &Self::PIPELINE_LIBRARY_CREATE_INFO_KHR => "PIPELINE_LIBRARY_CREATE_INFO_KHR",
            &Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT => "PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT",
            &Self::DEVICE_PRIVATE_DATA_CREATE_INFO_EXT => "DEVICE_PRIVATE_DATA_CREATE_INFO_EXT",
            &Self::PRIVATE_DATA_SLOT_CREATE_INFO_EXT => "PRIVATE_DATA_SLOT_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT => "PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT",
            &Self::VIDEO_ENCODE_INFO_KHR => "VIDEO_ENCODE_INFO_KHR",
            &Self::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR => "VIDEO_ENCODE_RATE_CONTROL_INFO_KHR",
            &Self::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV => "PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV",
            &Self::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV => "DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV",
            &Self::MEMORY_BARRIER_2_KHR => "MEMORY_BARRIER_2_KHR",
            &Self::BUFFER_MEMORY_BARRIER_2_KHR => "BUFFER_MEMORY_BARRIER_2_KHR",
            &Self::IMAGE_MEMORY_BARRIER_2_KHR => "IMAGE_MEMORY_BARRIER_2_KHR",
            &Self::DEPENDENCY_INFO_KHR => "DEPENDENCY_INFO_KHR",
            &Self::SUBMIT_INFO_2_KHR => "SUBMIT_INFO_2_KHR",
            &Self::SEMAPHORE_SUBMIT_INFO_KHR => "SEMAPHORE_SUBMIT_INFO_KHR",
            &Self::COMMAND_BUFFER_SUBMIT_INFO_KHR => "COMMAND_BUFFER_SUBMIT_INFO_KHR",
            &Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR => "PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR",
            &Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV => "QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV",
            &Self::CHECKPOINT_DATA_2_NV => "CHECKPOINT_DATA_2_NV",
            &Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR => "PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR",
            &Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR => "PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR",
            &Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV => "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV",
            &Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV => "PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV",
            &Self::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV => "PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV",
            &Self::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT => "PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT => "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT => "PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT",
            &Self::COPY_COMMAND_TRANSFORM_INFO_QCOM => "COPY_COMMAND_TRANSFORM_INFO_QCOM",
            &Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT => "PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT",
            &Self::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR => "PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR",
            &Self::COPY_BUFFER_INFO_2_KHR => "COPY_BUFFER_INFO_2_KHR",
            &Self::COPY_IMAGE_INFO_2_KHR => "COPY_IMAGE_INFO_2_KHR",
            &Self::COPY_BUFFER_TO_IMAGE_INFO_2_KHR => "COPY_BUFFER_TO_IMAGE_INFO_2_KHR",
            &Self::COPY_IMAGE_TO_BUFFER_INFO_2_KHR => "COPY_IMAGE_TO_BUFFER_INFO_2_KHR",
            &Self::BLIT_IMAGE_INFO_2_KHR => "BLIT_IMAGE_INFO_2_KHR",
            &Self::RESOLVE_IMAGE_INFO_2_KHR => "RESOLVE_IMAGE_INFO_2_KHR",
            &Self::BUFFER_COPY_2_KHR => "BUFFER_COPY_2_KHR",
            &Self::IMAGE_COPY_2_KHR => "IMAGE_COPY_2_KHR",
            &Self::IMAGE_BLIT_2_KHR => "IMAGE_BLIT_2_KHR",
            &Self::BUFFER_IMAGE_COPY_2_KHR => "BUFFER_IMAGE_COPY_2_KHR",
            &Self::IMAGE_RESOLVE_2_KHR => "IMAGE_RESOLVE_2_KHR",
            &Self::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT => "PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT",
            &Self::DIRECTFB_SURFACE_CREATE_INFO_EXT => "DIRECTFB_SURFACE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE => "PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE",
            &Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE => "MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE",
            &Self::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT => "PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT",
            &Self::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT => "VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT",
            &Self::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT => "VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT",
            &Self::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA => "IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA",
            &Self::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA => "MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA",
            &Self::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA => "MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA",
            &Self::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA => "IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA",
            &Self::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA => "SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA",
            &Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT => "PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT",
            &Self::SCREEN_SURFACE_CREATE_INFO_QNX => "SCREEN_SURFACE_CREATE_INFO_QNX",
            &Self::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT => "PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT",
            &Self::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT => "PIPELINE_COLOR_WRITE_CREATE_INFO_EXT",
            &Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT => "PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT",
            &Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT => "QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::StructureType {
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSystemAllocationScope.html) · Enum"]
#[doc(alias = "VkSystemAllocationScope")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SystemAllocationScope(pub i32);
impl std::fmt::Debug for SystemAllocationScope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::COMMAND => "COMMAND",
            &Self::OBJECT => "OBJECT",
            &Self::CACHE => "CACHE",
            &Self::DEVICE => "DEVICE",
            &Self::INSTANCE => "INSTANCE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SystemAllocationScope {
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInternalAllocationType.html) · Enum"]
#[doc(alias = "VkInternalAllocationType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct InternalAllocationType(pub i32);
impl std::fmt::Debug for InternalAllocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::EXECUTABLE => "EXECUTABLE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::InternalAllocationType {
    pub const EXECUTABLE: Self = Self(0);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerAddressMode.html) · Enum"]
#[doc(alias = "VkSamplerAddressMode")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SamplerAddressMode(pub i32);
impl std::fmt::Debug for SamplerAddressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::REPEAT => "REPEAT",
            &Self::MIRRORED_REPEAT => "MIRRORED_REPEAT",
            &Self::CLAMP_TO_EDGE => "CLAMP_TO_EDGE",
            &Self::CLAMP_TO_BORDER => "CLAMP_TO_BORDER",
            &Self::MIRROR_CLAMP_TO_EDGE => "MIRROR_CLAMP_TO_EDGE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SamplerAddressMode {
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFilter.html) · Enum"]
#[doc(alias = "VkFilter")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Filter(pub i32);
impl std::fmt::Debug for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NEAREST => "NEAREST",
            &Self::LINEAR => "LINEAR",
            &Self::CUBIC_IMG => "CUBIC_IMG",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::Filter {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerMipmapMode.html) · Enum"]
#[doc(alias = "VkSamplerMipmapMode")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SamplerMipmapMode(pub i32);
impl std::fmt::Debug for SamplerMipmapMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::NEAREST => "NEAREST",
            &Self::LINEAR => "LINEAR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SamplerMipmapMode {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputRate.html) · Enum"]
#[doc(alias = "VkVertexInputRate")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VertexInputRate(pub i32);
impl std::fmt::Debug for VertexInputRate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VERTEX => "VERTEX",
            &Self::INSTANCE => "INSTANCE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::VertexInputRate {
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineStageFlags.html) · Bitmask of [`PipelineStageFlagBits`]"] # [doc (alias = "VkPipelineStageFlags")] # [derive (Default)] # [repr (transparent)] pub struct PipelineStageFlags : u32 { const TOP_OF_PIPE = PipelineStageFlagBits :: TOP_OF_PIPE . 0 ; const DRAW_INDIRECT = PipelineStageFlagBits :: DRAW_INDIRECT . 0 ; const VERTEX_INPUT = PipelineStageFlagBits :: VERTEX_INPUT . 0 ; const VERTEX_SHADER = PipelineStageFlagBits :: VERTEX_SHADER . 0 ; const TESSELLATION_CONTROL_SHADER = PipelineStageFlagBits :: TESSELLATION_CONTROL_SHADER . 0 ; const TESSELLATION_EVALUATION_SHADER = PipelineStageFlagBits :: TESSELLATION_EVALUATION_SHADER . 0 ; const GEOMETRY_SHADER = PipelineStageFlagBits :: GEOMETRY_SHADER . 0 ; const FRAGMENT_SHADER = PipelineStageFlagBits :: FRAGMENT_SHADER . 0 ; const EARLY_FRAGMENT_TESTS = PipelineStageFlagBits :: EARLY_FRAGMENT_TESTS . 0 ; const LATE_FRAGMENT_TESTS = PipelineStageFlagBits :: LATE_FRAGMENT_TESTS . 0 ; const COLOR_ATTACHMENT_OUTPUT = PipelineStageFlagBits :: COLOR_ATTACHMENT_OUTPUT . 0 ; const COMPUTE_SHADER = PipelineStageFlagBits :: COMPUTE_SHADER . 0 ; const TRANSFER = PipelineStageFlagBits :: TRANSFER . 0 ; const BOTTOM_OF_PIPE = PipelineStageFlagBits :: BOTTOM_OF_PIPE . 0 ; const HOST = PipelineStageFlagBits :: HOST . 0 ; const ALL_GRAPHICS = PipelineStageFlagBits :: ALL_GRAPHICS . 0 ; const ALL_COMMANDS = PipelineStageFlagBits :: ALL_COMMANDS . 0 ; const TRANSFORM_FEEDBACK_EXT = PipelineStageFlagBits :: TRANSFORM_FEEDBACK_EXT . 0 ; const CONDITIONAL_RENDERING_EXT = PipelineStageFlagBits :: CONDITIONAL_RENDERING_EXT . 0 ; const ACCELERATION_STRUCTURE_BUILD_KHR = PipelineStageFlagBits :: ACCELERATION_STRUCTURE_BUILD_KHR . 0 ; const RAY_TRACING_SHADER_KHR = PipelineStageFlagBits :: RAY_TRACING_SHADER_KHR . 0 ; const TASK_SHADER_NV = PipelineStageFlagBits :: TASK_SHADER_NV . 0 ; const MESH_SHADER_NV = PipelineStageFlagBits :: MESH_SHADER_NV . 0 ; const FRAGMENT_DENSITY_PROCESS_EXT = PipelineStageFlagBits :: FRAGMENT_DENSITY_PROCESS_EXT . 0 ; const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = PipelineStageFlagBits :: FRAGMENT_SHADING_RATE_ATTACHMENT_KHR . 0 ; const COMMAND_PREPROCESS_NV = PipelineStageFlagBits :: COMMAND_PREPROCESS_NV . 0 ; const NONE_KHR = PipelineStageFlagBits :: NONE_KHR . 0 ; const SHADING_RATE_IMAGE_NV = PipelineStageFlagBits :: SHADING_RATE_IMAGE_NV . 0 ; const RAY_TRACING_SHADER_NV = PipelineStageFlagBits :: RAY_TRACING_SHADER_NV . 0 ; const ACCELERATION_STRUCTURE_BUILD_NV = PipelineStageFlagBits :: ACCELERATION_STRUCTURE_BUILD_NV . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineStageFlagBits.html) · Bits enum of [`PipelineStageFlags`]"]
#[doc(alias = "VkPipelineStageFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct PipelineStageFlagBits(pub u32);
impl PipelineStageFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineStageFlags {
        PipelineStageFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineStageFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::TOP_OF_PIPE => "TOP_OF_PIPE",
            &Self::DRAW_INDIRECT => "DRAW_INDIRECT",
            &Self::VERTEX_INPUT => "VERTEX_INPUT",
            &Self::VERTEX_SHADER => "VERTEX_SHADER",
            &Self::TESSELLATION_CONTROL_SHADER => "TESSELLATION_CONTROL_SHADER",
            &Self::TESSELLATION_EVALUATION_SHADER => "TESSELLATION_EVALUATION_SHADER",
            &Self::GEOMETRY_SHADER => "GEOMETRY_SHADER",
            &Self::FRAGMENT_SHADER => "FRAGMENT_SHADER",
            &Self::EARLY_FRAGMENT_TESTS => "EARLY_FRAGMENT_TESTS",
            &Self::LATE_FRAGMENT_TESTS => "LATE_FRAGMENT_TESTS",
            &Self::COLOR_ATTACHMENT_OUTPUT => "COLOR_ATTACHMENT_OUTPUT",
            &Self::COMPUTE_SHADER => "COMPUTE_SHADER",
            &Self::TRANSFER => "TRANSFER",
            &Self::BOTTOM_OF_PIPE => "BOTTOM_OF_PIPE",
            &Self::HOST => "HOST",
            &Self::ALL_GRAPHICS => "ALL_GRAPHICS",
            &Self::ALL_COMMANDS => "ALL_COMMANDS",
            &Self::TRANSFORM_FEEDBACK_EXT => "TRANSFORM_FEEDBACK_EXT",
            &Self::CONDITIONAL_RENDERING_EXT => "CONDITIONAL_RENDERING_EXT",
            &Self::ACCELERATION_STRUCTURE_BUILD_KHR => "ACCELERATION_STRUCTURE_BUILD_KHR",
            &Self::RAY_TRACING_SHADER_KHR => "RAY_TRACING_SHADER_KHR",
            &Self::TASK_SHADER_NV => "TASK_SHADER_NV",
            &Self::MESH_SHADER_NV => "MESH_SHADER_NV",
            &Self::FRAGMENT_DENSITY_PROCESS_EXT => "FRAGMENT_DENSITY_PROCESS_EXT",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            &Self::COMMAND_PREPROCESS_NV => "COMMAND_PREPROCESS_NV",
            &Self::NONE_KHR => "NONE_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::PipelineStageFlagBits {
    pub const TOP_OF_PIPE: Self = Self(1);
    pub const DRAW_INDIRECT: Self = Self(2);
    pub const VERTEX_INPUT: Self = Self(4);
    pub const VERTEX_SHADER: Self = Self(8);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(16);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(32);
    pub const GEOMETRY_SHADER: Self = Self(64);
    pub const FRAGMENT_SHADER: Self = Self(128);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(256);
    pub const LATE_FRAGMENT_TESTS: Self = Self(512);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1024);
    pub const COMPUTE_SHADER: Self = Self(2048);
    pub const TRANSFER: Self = Self(4096);
    pub const BOTTOM_OF_PIPE: Self = Self(8192);
    pub const HOST: Self = Self(16384);
    pub const ALL_GRAPHICS: Self = Self(32768);
    pub const ALL_COMMANDS: Self = Self(65536);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatFlags.html) · Bitmask of [`SparseImageFormatFlagBits`]"] # [doc (alias = "VkSparseImageFormatFlags")] # [derive (Default)] # [repr (transparent)] pub struct SparseImageFormatFlags : u32 { const SINGLE_MIPTAIL = SparseImageFormatFlagBits :: SINGLE_MIPTAIL . 0 ; const ALIGNED_MIP_SIZE = SparseImageFormatFlagBits :: ALIGNED_MIP_SIZE . 0 ; const NONSTANDARD_BLOCK_SIZE = SparseImageFormatFlagBits :: NONSTANDARD_BLOCK_SIZE . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatFlagBits.html) · Bits enum of [`SparseImageFormatFlags`]"]
#[doc(alias = "VkSparseImageFormatFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SparseImageFormatFlagBits(pub u32);
impl SparseImageFormatFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SparseImageFormatFlags {
        SparseImageFormatFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SparseImageFormatFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SINGLE_MIPTAIL => "SINGLE_MIPTAIL",
            &Self::ALIGNED_MIP_SIZE => "ALIGNED_MIP_SIZE",
            &Self::NONSTANDARD_BLOCK_SIZE => "NONSTANDARD_BLOCK_SIZE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SparseImageFormatFlagBits {
    pub const SINGLE_MIPTAIL: Self = Self(1);
    pub const ALIGNED_MIP_SIZE: Self = Self(2);
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(4);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleCountFlags.html) · Bitmask of [`SampleCountFlagBits`]"] # [doc (alias = "VkSampleCountFlags")] # [derive (Default)] # [repr (transparent)] pub struct SampleCountFlags : u32 { const _1 = SampleCountFlagBits :: _1 . 0 ; const _2 = SampleCountFlagBits :: _2 . 0 ; const _4 = SampleCountFlagBits :: _4 . 0 ; const _8 = SampleCountFlagBits :: _8 . 0 ; const _16 = SampleCountFlagBits :: _16 . 0 ; const _32 = SampleCountFlagBits :: _32 . 0 ; const _64 = SampleCountFlagBits :: _64 . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleCountFlagBits.html) · Bits enum of [`SampleCountFlags`]"]
#[doc(alias = "VkSampleCountFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SampleCountFlagBits(pub u32);
impl SampleCountFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SampleCountFlags {
        SampleCountFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SampleCountFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::_1 => "_1",
            &Self::_2 => "_2",
            &Self::_4 => "_4",
            &Self::_8 => "_8",
            &Self::_16 => "_16",
            &Self::_32 => "_32",
            &Self::_64 => "_64",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::SampleCountFlagBits {
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const _4: Self = Self(4);
    pub const _8: Self = Self(8);
    pub const _16: Self = Self(16);
    pub const _32: Self = Self(32);
    pub const _64: Self = Self(64);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionFlags.html) · Bitmask of [`AttachmentDescriptionFlagBits`]"] # [doc (alias = "VkAttachmentDescriptionFlags")] # [derive (Default)] # [repr (transparent)] pub struct AttachmentDescriptionFlags : u32 { const MAY_ALIAS = AttachmentDescriptionFlagBits :: MAY_ALIAS . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionFlagBits.html) · Bits enum of [`AttachmentDescriptionFlags`]"]
#[doc(alias = "VkAttachmentDescriptionFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AttachmentDescriptionFlagBits(pub u32);
impl AttachmentDescriptionFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> AttachmentDescriptionFlags {
        AttachmentDescriptionFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for AttachmentDescriptionFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::MAY_ALIAS => "MAY_ALIAS",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::AttachmentDescriptionFlagBits {
    pub const MAY_ALIAS: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateFlags.html) · Bitmask of [`DescriptorPoolCreateFlagBits`]"] # [doc (alias = "VkDescriptorPoolCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct DescriptorPoolCreateFlags : u32 { const FREE_DESCRIPTOR_SET = DescriptorPoolCreateFlagBits :: FREE_DESCRIPTOR_SET . 0 ; const UPDATE_AFTER_BIND = DescriptorPoolCreateFlagBits :: UPDATE_AFTER_BIND . 0 ; const HOST_ONLY_VALVE = DescriptorPoolCreateFlagBits :: HOST_ONLY_VALVE . 0 ; const UPDATE_AFTER_BIND_EXT = DescriptorPoolCreateFlagBits :: UPDATE_AFTER_BIND_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateFlagBits.html) · Bits enum of [`DescriptorPoolCreateFlags`]"]
#[doc(alias = "VkDescriptorPoolCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DescriptorPoolCreateFlagBits(pub u32);
impl DescriptorPoolCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DescriptorPoolCreateFlags {
        DescriptorPoolCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DescriptorPoolCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::FREE_DESCRIPTOR_SET => "FREE_DESCRIPTOR_SET",
            &Self::UPDATE_AFTER_BIND => "UPDATE_AFTER_BIND",
            &Self::HOST_ONLY_VALVE => "HOST_ONLY_VALVE",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::DescriptorPoolCreateFlagBits {
    pub const FREE_DESCRIPTOR_SET: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDependencyFlags.html) · Bitmask of [`DependencyFlagBits`]"] # [doc (alias = "VkDependencyFlags")] # [derive (Default)] # [repr (transparent)] pub struct DependencyFlags : u32 { const BY_REGION = DependencyFlagBits :: BY_REGION . 0 ; const DEVICE_GROUP = DependencyFlagBits :: DEVICE_GROUP . 0 ; const VIEW_LOCAL = DependencyFlagBits :: VIEW_LOCAL . 0 ; const VIEW_LOCAL_KHR = DependencyFlagBits :: VIEW_LOCAL_KHR . 0 ; const DEVICE_GROUP_KHR = DependencyFlagBits :: DEVICE_GROUP_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDependencyFlagBits.html) · Bits enum of [`DependencyFlags`]"]
#[doc(alias = "VkDependencyFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DependencyFlagBits(pub u32);
impl DependencyFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> DependencyFlags {
        DependencyFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for DependencyFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::BY_REGION => "BY_REGION",
            &Self::DEVICE_GROUP => "DEVICE_GROUP",
            &Self::VIEW_LOCAL => "VIEW_LOCAL",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::DependencyFlagBits {
    pub const BY_REGION: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkObjectType.html) · Enum"]
#[doc(alias = "VkObjectType")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ObjectType(pub i32);
impl std::fmt::Debug for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::UNKNOWN => "UNKNOWN",
            &Self::INSTANCE => "INSTANCE",
            &Self::PHYSICAL_DEVICE => "PHYSICAL_DEVICE",
            &Self::DEVICE => "DEVICE",
            &Self::QUEUE => "QUEUE",
            &Self::SEMAPHORE => "SEMAPHORE",
            &Self::COMMAND_BUFFER => "COMMAND_BUFFER",
            &Self::FENCE => "FENCE",
            &Self::DEVICE_MEMORY => "DEVICE_MEMORY",
            &Self::BUFFER => "BUFFER",
            &Self::IMAGE => "IMAGE",
            &Self::EVENT => "EVENT",
            &Self::QUERY_POOL => "QUERY_POOL",
            &Self::BUFFER_VIEW => "BUFFER_VIEW",
            &Self::IMAGE_VIEW => "IMAGE_VIEW",
            &Self::SHADER_MODULE => "SHADER_MODULE",
            &Self::PIPELINE_CACHE => "PIPELINE_CACHE",
            &Self::PIPELINE_LAYOUT => "PIPELINE_LAYOUT",
            &Self::RENDER_PASS => "RENDER_PASS",
            &Self::PIPELINE => "PIPELINE",
            &Self::DESCRIPTOR_SET_LAYOUT => "DESCRIPTOR_SET_LAYOUT",
            &Self::SAMPLER => "SAMPLER",
            &Self::DESCRIPTOR_POOL => "DESCRIPTOR_POOL",
            &Self::DESCRIPTOR_SET => "DESCRIPTOR_SET",
            &Self::FRAMEBUFFER => "FRAMEBUFFER",
            &Self::COMMAND_POOL => "COMMAND_POOL",
            &Self::SAMPLER_YCBCR_CONVERSION => "SAMPLER_YCBCR_CONVERSION",
            &Self::DESCRIPTOR_UPDATE_TEMPLATE => "DESCRIPTOR_UPDATE_TEMPLATE",
            &Self::SURFACE_KHR => "SURFACE_KHR",
            &Self::SWAPCHAIN_KHR => "SWAPCHAIN_KHR",
            &Self::DISPLAY_KHR => "DISPLAY_KHR",
            &Self::DISPLAY_MODE_KHR => "DISPLAY_MODE_KHR",
            &Self::DEBUG_REPORT_CALLBACK_EXT => "DEBUG_REPORT_CALLBACK_EXT",
            &Self::VIDEO_SESSION_KHR => "VIDEO_SESSION_KHR",
            &Self::VIDEO_SESSION_PARAMETERS_KHR => "VIDEO_SESSION_PARAMETERS_KHR",
            &Self::CU_MODULE_NVX => "CU_MODULE_NVX",
            &Self::CU_FUNCTION_NVX => "CU_FUNCTION_NVX",
            &Self::DEBUG_UTILS_MESSENGER_EXT => "DEBUG_UTILS_MESSENGER_EXT",
            &Self::ACCELERATION_STRUCTURE_KHR => "ACCELERATION_STRUCTURE_KHR",
            &Self::VALIDATION_CACHE_EXT => "VALIDATION_CACHE_EXT",
            &Self::ACCELERATION_STRUCTURE_NV => "ACCELERATION_STRUCTURE_NV",
            &Self::PERFORMANCE_CONFIGURATION_INTEL => "PERFORMANCE_CONFIGURATION_INTEL",
            &Self::DEFERRED_OPERATION_KHR => "DEFERRED_OPERATION_KHR",
            &Self::INDIRECT_COMMANDS_LAYOUT_NV => "INDIRECT_COMMANDS_LAYOUT_NV",
            &Self::PRIVATE_DATA_SLOT_EXT => "PRIVATE_DATA_SLOT_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::ObjectType {
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEventCreateFlags.html) · Bitmask of [`EventCreateFlagBits`]"] # [doc (alias = "VkEventCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct EventCreateFlags : u32 { const DEVICE_ONLY_KHR = EventCreateFlagBits :: DEVICE_ONLY_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEventCreateFlagBits.html) · Bits enum of [`EventCreateFlags`]"]
#[doc(alias = "VkEventCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct EventCreateFlagBits(pub u32);
impl EventCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> EventCreateFlags {
        EventCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for EventCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEVICE_ONLY_KHR => "DEVICE_ONLY_KHR",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescriptionFlags.html) · Bitmask of [`SubpassDescriptionFlagBits`]"] # [doc (alias = "VkSubpassDescriptionFlags")] # [derive (Default)] # [repr (transparent)] pub struct SubpassDescriptionFlags : u32 { const PER_VIEW_ATTRIBUTES_NVX = SubpassDescriptionFlagBits :: PER_VIEW_ATTRIBUTES_NVX . 0 ; const PER_VIEW_POSITION_X_ONLY_NVX = SubpassDescriptionFlagBits :: PER_VIEW_POSITION_X_ONLY_NVX . 0 ; const FRAGMENT_REGION_QCOM = SubpassDescriptionFlagBits :: FRAGMENT_REGION_QCOM . 0 ; const SHADER_RESOLVE_QCOM = SubpassDescriptionFlagBits :: SHADER_RESOLVE_QCOM . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescriptionFlagBits.html) · Bits enum of [`SubpassDescriptionFlags`]"]
#[doc(alias = "VkSubpassDescriptionFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct SubpassDescriptionFlagBits(pub u32);
impl SubpassDescriptionFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> SubpassDescriptionFlags {
        SubpassDescriptionFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for SubpassDescriptionFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PER_VIEW_ATTRIBUTES_NVX => "PER_VIEW_ATTRIBUTES_NVX",
            &Self::PER_VIEW_POSITION_X_ONLY_NVX => "PER_VIEW_POSITION_X_ONLY_NVX",
            &Self::FRAGMENT_REGION_QCOM => "FRAGMENT_REGION_QCOM",
            &Self::SHADER_RESOLVE_QCOM => "SHADER_RESOLVE_QCOM",
            _ => "(unknown variant)",
        })
    }
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleCreateFlags.html) · Bitmask of [`ShaderModuleCreateFlagBits`]"] # [doc (alias = "VkShaderModuleCreateFlags")] # [derive (Default)] # [repr (transparent)] pub struct ShaderModuleCreateFlags : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`ShaderModuleCreateFlags`]"]
#[doc(alias = "VkShaderModuleCreateFlagBits")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ShaderModuleCreateFlagBits(pub u32);
impl ShaderModuleCreateFlagBits {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> ShaderModuleCreateFlags {
        ShaderModuleCreateFlags::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for ShaderModuleCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVendorId.html) · Enum"]
#[doc(alias = "VkVendorId")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VendorId(pub i32);
impl std::fmt::Debug for VendorId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::VIV => "VIV",
            &Self::VSI => "VSI",
            &Self::KAZAN => "KAZAN",
            &Self::CODEPLAY => "CODEPLAY",
            &Self::MESA => "MESA",
            &Self::POCL => "POCL",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::vk1_0::VendorId {
    pub const VIV: Self = Self(65537);
    pub const VSI: Self = Self(65538);
    pub const KAZAN: Self = Self(65539);
    pub const CODEPLAY: Self = Self(65540);
    pub const MESA: Self = Self(65541);
    pub const POCL: Self = Self(65542);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateInstance = unsafe extern "system" fn(p_create_info: *const crate::vk1_0::InstanceCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_instance: *mut crate::vk1_0::Instance) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyInstance.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyInstance = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDevices.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_physical_device_count: *mut u32, p_physical_devices: *mut crate::vk1_0::PhysicalDevice) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceProcAddr.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(device: crate::vk1_0::Device, p_name: *const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(instance: crate::vk1_0::Instance, p_name: *const std::os::raw::c_char) -> Option<crate::vk1_0::PFN_vkVoidFunction>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_properties: *mut crate::vk1_0::PhysicalDeviceProperties) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_queue_family_property_count: *mut u32, p_queue_family_properties: *mut crate::vk1_0::QueueFamilyProperties) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_memory_properties: *mut crate::vk1_0::PhysicalDeviceMemoryProperties) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_features: *mut crate::vk1_0::PhysicalDeviceFeatures) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, format: crate::vk1_0::Format, p_format_properties: *mut crate::vk1_0::FormatProperties) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, format: crate::vk1_0::Format, _type: crate::vk1_0::ImageType, tiling: crate::vk1_0::ImageTiling, usage: crate::vk1_0::ImageUsageFlags, flags: crate::vk1_0::ImageCreateFlags, p_image_format_properties: *mut crate::vk1_0::ImageFormatProperties) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDevice.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDevice = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_create_info: *const crate::vk1_0::DeviceCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_device: *mut crate::vk1_0::Device) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDevice.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDevice = unsafe extern "system" fn(device: crate::vk1_0::Device, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceLayerProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(p_property_count: *mut u32, p_properties: *mut crate::vk1_0::LayerProperties) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(p_layer_name: *const std::os::raw::c_char, p_property_count: *mut u32, p_properties: *mut crate::vk1_0::ExtensionProperties) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceLayerProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_property_count: *mut u32, p_properties: *mut crate::vk1_0::LayerProperties) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceExtensionProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, p_layer_name: *const std::os::raw::c_char, p_property_count: *mut u32, p_properties: *mut crate::vk1_0::ExtensionProperties) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(device: crate::vk1_0::Device, queue_family_index: u32, queue_index: u32, p_queue: *mut crate::vk1_0::Queue) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSubmit.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(queue: crate::vk1_0::Queue, submit_count: u32, p_submits: *const crate::vk1_0::SubmitInfo, fence: crate::vk1_0::Fence) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueWaitIdle.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: crate::vk1_0::Queue) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeviceWaitIdle.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: crate::vk1_0::Device) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateMemory.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(device: crate::vk1_0::Device, p_allocate_info: *const crate::vk1_0::MemoryAllocateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_memory: *mut crate::vk1_0::DeviceMemory) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeMemory.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkFreeMemory = unsafe extern "system" fn(device: crate::vk1_0::Device, memory: crate::vk1_0::DeviceMemory, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMapMemory.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkMapMemory = unsafe extern "system" fn(device: crate::vk1_0::Device, memory: crate::vk1_0::DeviceMemory, offset: crate::vk1_0::DeviceSize, size: crate::vk1_0::DeviceSize, flags: crate::vk1_0::MemoryMapFlags, pp_data: *mut *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUnmapMemory.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: crate::vk1_0::Device, memory: crate::vk1_0::DeviceMemory) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFlushMappedMemoryRanges.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(device: crate::vk1_0::Device, memory_range_count: u32, p_memory_ranges: *const crate::vk1_0::MappedMemoryRange) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInvalidateMappedMemoryRanges.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(device: crate::vk1_0::Device, memory_range_count: u32, p_memory_ranges: *const crate::vk1_0::MappedMemoryRange) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryCommitment.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(device: crate::vk1_0::Device, memory: crate::vk1_0::DeviceMemory, p_committed_memory_in_bytes: *mut crate::vk1_0::DeviceSize) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(device: crate::vk1_0::Device, buffer: crate::vk1_0::Buffer, p_memory_requirements: *mut crate::vk1_0::MemoryRequirements) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(device: crate::vk1_0::Device, buffer: crate::vk1_0::Buffer, memory: crate::vk1_0::DeviceMemory, memory_offset: crate::vk1_0::DeviceSize) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(device: crate::vk1_0::Device, image: crate::vk1_0::Image, p_memory_requirements: *mut crate::vk1_0::MemoryRequirements) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(device: crate::vk1_0::Device, image: crate::vk1_0::Image, memory: crate::vk1_0::DeviceMemory, memory_offset: crate::vk1_0::DeviceSize) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(device: crate::vk1_0::Device, image: crate::vk1_0::Image, p_sparse_memory_requirement_count: *mut u32, p_sparse_memory_requirements: *mut crate::vk1_0::SparseImageMemoryRequirements) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(physical_device: crate::vk1_0::PhysicalDevice, format: crate::vk1_0::Format, _type: crate::vk1_0::ImageType, samples: crate::vk1_0::SampleCountFlagBits, usage: crate::vk1_0::ImageUsageFlags, tiling: crate::vk1_0::ImageTiling, p_property_count: *mut u32, p_properties: *mut crate::vk1_0::SparseImageFormatProperties) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBindSparse.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(queue: crate::vk1_0::Queue, bind_info_count: u32, p_bind_info: *const crate::vk1_0::BindSparseInfo, fence: crate::vk1_0::Fence) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFence.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFence = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::FenceCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_fence: *mut crate::vk1_0::Fence) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFence.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFence = unsafe extern "system" fn(device: crate::vk1_0::Device, fence: crate::vk1_0::Fence, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetFences.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetFences = unsafe extern "system" fn(device: crate::vk1_0::Device, fence_count: u32, p_fences: *const crate::vk1_0::Fence) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceStatus.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(device: crate::vk1_0::Device, fence: crate::vk1_0::Fence) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitForFences.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForFences = unsafe extern "system" fn(device: crate::vk1_0::Device, fence_count: u32, p_fences: *const crate::vk1_0::Fence, wait_all: crate::vk1_0::Bool32, timeout: u64) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSemaphore.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::SemaphoreCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_semaphore: *mut crate::vk1_0::Semaphore) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySemaphore.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(device: crate::vk1_0::Device, semaphore: crate::vk1_0::Semaphore, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateEvent.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateEvent = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::EventCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_event: *mut crate::vk1_0::Event) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyEvent.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(device: crate::vk1_0::Device, event: crate::vk1_0::Event, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetEventStatus.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(device: crate::vk1_0::Device, event: crate::vk1_0::Event) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetEvent.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: crate::vk1_0::Device, event: crate::vk1_0::Event) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetEvent.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: crate::vk1_0::Device, event: crate::vk1_0::Event) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateQueryPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::QueryPoolCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_query_pool: *mut crate::vk1_0::QueryPool) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyQueryPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(device: crate::vk1_0::Device, query_pool: crate::vk1_0::QueryPool, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueryPoolResults.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(device: crate::vk1_0::Device, query_pool: crate::vk1_0::QueryPool, first_query: u32, query_count: u32, data_size: usize, p_data: *mut std::ffi::c_void, stride: crate::vk1_0::DeviceSize, flags: crate::vk1_0::QueryResultFlags) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::BufferCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_buffer: *mut crate::vk1_0::Buffer) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(device: crate::vk1_0::Device, buffer: crate::vk1_0::Buffer, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBufferView.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::BufferViewCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_view: *mut crate::vk1_0::BufferView) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBufferView.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(device: crate::vk1_0::Device, buffer_view: crate::vk1_0::BufferView, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImage = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::ImageCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_image: *mut crate::vk1_0::Image) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImage = unsafe extern "system" fn(device: crate::vk1_0::Device, image: crate::vk1_0::Image, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSubresourceLayout.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(device: crate::vk1_0::Device, image: crate::vk1_0::Image, p_subresource: *const crate::vk1_0::ImageSubresource, p_layout: *mut crate::vk1_0::SubresourceLayout) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImageView.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImageView = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::ImageViewCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_view: *mut crate::vk1_0::ImageView) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImageView.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(device: crate::vk1_0::Device, image_view: crate::vk1_0::ImageView, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateShaderModule.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::ShaderModuleCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_shader_module: *mut crate::vk1_0::ShaderModule) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyShaderModule.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(device: crate::vk1_0::Device, shader_module: crate::vk1_0::ShaderModule, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineCache.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::PipelineCacheCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_pipeline_cache: *mut crate::vk1_0::PipelineCache) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineCache.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline_cache: crate::vk1_0::PipelineCache, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineCacheData.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline_cache: crate::vk1_0::PipelineCache, p_data_size: *mut usize, p_data: *mut std::ffi::c_void) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergePipelineCaches.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(device: crate::vk1_0::Device, dst_cache: crate::vk1_0::PipelineCache, src_cache_count: u32, p_src_caches: *const crate::vk1_0::PipelineCache) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateGraphicsPipelines.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline_cache: crate::vk1_0::PipelineCache, create_info_count: u32, p_create_infos: *const crate::vk1_0::GraphicsPipelineCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_pipelines: *mut crate::vk1_0::Pipeline) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateComputePipelines.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline_cache: crate::vk1_0::PipelineCache, create_info_count: u32, p_create_infos: *const crate::vk1_0::ComputePipelineCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_pipelines: *mut crate::vk1_0::Pipeline) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipeline.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline: crate::vk1_0::Pipeline, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineLayout.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::PipelineLayoutCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_pipeline_layout: *mut crate::vk1_0::PipelineLayout) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineLayout.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(device: crate::vk1_0::Device, pipeline_layout: crate::vk1_0::PipelineLayout, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSampler.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSampler = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::SamplerCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_sampler: *mut crate::vk1_0::Sampler) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySampler.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySampler = unsafe extern "system" fn(device: crate::vk1_0::Device, sampler: crate::vk1_0::Sampler, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorSetLayout.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::DescriptorSetLayoutCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_set_layout: *mut crate::vk1_0::DescriptorSetLayout) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorSetLayout.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(device: crate::vk1_0::Device, descriptor_set_layout: crate::vk1_0::DescriptorSetLayout, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::DescriptorPoolCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_descriptor_pool: *mut crate::vk1_0::DescriptorPool) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(device: crate::vk1_0::Device, descriptor_pool: crate::vk1_0::DescriptorPool, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetDescriptorPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(device: crate::vk1_0::Device, descriptor_pool: crate::vk1_0::DescriptorPool, flags: crate::vk1_0::DescriptorPoolResetFlags) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateDescriptorSets.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(device: crate::vk1_0::Device, p_allocate_info: *const crate::vk1_0::DescriptorSetAllocateInfo, p_descriptor_sets: *mut crate::vk1_0::DescriptorSet) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeDescriptorSets.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(device: crate::vk1_0::Device, descriptor_pool: crate::vk1_0::DescriptorPool, descriptor_set_count: u32, p_descriptor_sets: *const crate::vk1_0::DescriptorSet) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSets.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(device: crate::vk1_0::Device, descriptor_write_count: u32, p_descriptor_writes: *const crate::vk1_0::WriteDescriptorSet, descriptor_copy_count: u32, p_descriptor_copies: *const crate::vk1_0::CopyDescriptorSet) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFramebuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::FramebufferCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_framebuffer: *mut crate::vk1_0::Framebuffer) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFramebuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(device: crate::vk1_0::Device, framebuffer: crate::vk1_0::Framebuffer, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::RenderPassCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_render_pass: *mut crate::vk1_0::RenderPass) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyRenderPass.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(device: crate::vk1_0::Device, render_pass: crate::vk1_0::RenderPass, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRenderAreaGranularity.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(device: crate::vk1_0::Device, render_pass: crate::vk1_0::RenderPass, p_granularity: *mut crate::vk1_0::Extent2D) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCommandPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(device: crate::vk1_0::Device, p_create_info: *const crate::vk1_0::CommandPoolCreateInfo, p_allocator: *const crate::vk1_0::AllocationCallbacks, p_command_pool: *mut crate::vk1_0::CommandPool) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCommandPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(device: crate::vk1_0::Device, command_pool: crate::vk1_0::CommandPool, p_allocator: *const crate::vk1_0::AllocationCallbacks) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(device: crate::vk1_0::Device, command_pool: crate::vk1_0::CommandPool, flags: crate::vk1_0::CommandPoolResetFlags) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateCommandBuffers.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(device: crate::vk1_0::Device, p_allocate_info: *const crate::vk1_0::CommandBufferAllocateInfo, p_command_buffers: *mut crate::vk1_0::CommandBuffer) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeCommandBuffers.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(device: crate::vk1_0::Device, command_pool: crate::vk1_0::CommandPool, command_buffer_count: u32, p_command_buffers: *const crate::vk1_0::CommandBuffer) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBeginCommandBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_begin_info: *const crate::vk1_0::CommandBufferBeginInfo) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEndCommandBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkEndCommandBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, flags: crate::vk1_0::CommandBufferResetFlags) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipeline.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, pipeline_bind_point: crate::vk1_0::PipelineBindPoint, pipeline: crate::vk1_0::Pipeline) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewport.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, first_viewport: u32, viewport_count: u32, p_viewports: *const crate::vk1_0::Viewport) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissor.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, first_scissor: u32, scissor_count: u32, p_scissors: *const crate::vk1_0::Rect2D) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineWidth.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineWidth = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, line_width: std::os::raw::c_float) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBias.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, depth_bias_constant_factor: std::os::raw::c_float, depth_bias_clamp: std::os::raw::c_float, depth_bias_slope_factor: std::os::raw::c_float) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetBlendConstants.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetBlendConstants = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, blend_constants: [std::os::raw::c_float; 4]) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBounds.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, min_depth_bounds: std::os::raw::c_float, max_depth_bounds: std::os::raw::c_float) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilCompareMask.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, face_mask: crate::vk1_0::StencilFaceFlags, compare_mask: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilWriteMask.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, face_mask: crate::vk1_0::StencilFaceFlags, write_mask: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilReference.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, face_mask: crate::vk1_0::StencilFaceFlags, reference: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindDescriptorSets.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, pipeline_bind_point: crate::vk1_0::PipelineBindPoint, layout: crate::vk1_0::PipelineLayout, first_set: u32, descriptor_set_count: u32, p_descriptor_sets: *const crate::vk1_0::DescriptorSet, dynamic_offset_count: u32, p_dynamic_offsets: *const u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindIndexBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, index_type: crate::vk1_0::IndexType) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, first_binding: u32, binding_count: u32, p_buffers: *const crate::vk1_0::Buffer, p_offsets: *const crate::vk1_0::DeviceSize) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDraw.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDraw = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexed.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirect.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, draw_count: u32, stride: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirect.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, draw_count: u32, stride: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatch.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchIndirect.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchIndirect = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, src_buffer: crate::vk1_0::Buffer, dst_buffer: crate::vk1_0::Buffer, region_count: u32, p_regions: *const crate::vk1_0::BufferCopy) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, region_count: u32, p_regions: *const crate::vk1_0::ImageCopy) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, region_count: u32, p_regions: *const crate::vk1_0::ImageBlit, filter: crate::vk1_0::Filter) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, src_buffer: crate::vk1_0::Buffer, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, region_count: u32, p_regions: *const crate::vk1_0::BufferImageCopy) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_buffer: crate::vk1_0::Buffer, region_count: u32, p_regions: *const crate::vk1_0::BufferImageCopy) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdUpdateBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, data_size: crate::vk1_0::DeviceSize, p_data: *const std::ffi::c_void) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdFillBuffer.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, size: crate::vk1_0::DeviceSize, data: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearColorImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, image: crate::vk1_0::Image, image_layout: crate::vk1_0::ImageLayout, p_color: *const crate::vk1_0::ClearColorValue, range_count: u32, p_ranges: *const crate::vk1_0::ImageSubresourceRange) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearDepthStencilImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, image: crate::vk1_0::Image, image_layout: crate::vk1_0::ImageLayout, p_depth_stencil: *const crate::vk1_0::ClearDepthStencilValue, range_count: u32, p_ranges: *const crate::vk1_0::ImageSubresourceRange) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearAttachments.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, attachment_count: u32, p_attachments: *const crate::vk1_0::ClearAttachment, rect_count: u32, p_rects: *const crate::vk1_0::ClearRect) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, region_count: u32, p_regions: *const crate::vk1_0::ImageResolve) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetEvent.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, event: crate::vk1_0::Event, stage_mask: crate::vk1_0::PipelineStageFlags) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetEvent.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, event: crate::vk1_0::Event, stage_mask: crate::vk1_0::PipelineStageFlags) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWaitEvents.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, event_count: u32, p_events: *const crate::vk1_0::Event, src_stage_mask: crate::vk1_0::PipelineStageFlags, dst_stage_mask: crate::vk1_0::PipelineStageFlags, memory_barrier_count: u32, p_memory_barriers: *const crate::vk1_0::MemoryBarrier, buffer_memory_barrier_count: u32, p_buffer_memory_barriers: *const crate::vk1_0::BufferMemoryBarrier, image_memory_barrier_count: u32, p_image_memory_barriers: *const crate::vk1_0::ImageMemoryBarrier) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, src_stage_mask: crate::vk1_0::PipelineStageFlags, dst_stage_mask: crate::vk1_0::PipelineStageFlags, dependency_flags: crate::vk1_0::DependencyFlags, memory_barrier_count: u32, p_memory_barriers: *const crate::vk1_0::MemoryBarrier, buffer_memory_barrier_count: u32, p_buffer_memory_barriers: *const crate::vk1_0::BufferMemoryBarrier, image_memory_barrier_count: u32, p_image_memory_barriers: *const crate::vk1_0::ImageMemoryBarrier) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQuery.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, query: u32, flags: crate::vk1_0::QueryControlFlags) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQuery.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQuery = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, query: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetQueryPool.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, first_query: u32, query_count: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteTimestamp.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, pipeline_stage: crate::vk1_0::PipelineStageFlagBits, query_pool: crate::vk1_0::QueryPool, query: u32) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyQueryPoolResults.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, first_query: u32, query_count: u32, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, stride: crate::vk1_0::DeviceSize, flags: crate::vk1_0::QueryResultFlags) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushConstants.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, layout: crate::vk1_0::PipelineLayout, stage_flags: crate::vk1_0::ShaderStageFlags, offset: u32, size: u32, p_values: *const std::ffi::c_void) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_render_pass_begin: *const crate::vk1_0::RenderPassBeginInfo, contents: crate::vk1_0::SubpassContents) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, contents: crate::vk1_0::SubpassContents) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteCommands.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, command_buffer_count: u32, p_command_buffers: *const crate::vk1_0::CommandBuffer) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkInternalAllocationNotification.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(p_user_data: *mut std::ffi::c_void, size: usize, allocation_type: crate::vk1_0::InternalAllocationType, allocation_scope: crate::vk1_0::SystemAllocationScope) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkInternalFreeNotification.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(p_user_data: *mut std::ffi::c_void, size: usize, allocation_type: crate::vk1_0::InternalAllocationType, allocation_scope: crate::vk1_0::SystemAllocationScope) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkReallocationFunction.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkReallocationFunction = unsafe extern "system" fn(p_user_data: *mut std::ffi::c_void, p_original: *mut std::ffi::c_void, size: usize, alignment: usize, allocation_scope: crate::vk1_0::SystemAllocationScope) -> *mut std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkAllocationFunction.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAllocationFunction = unsafe extern "system" fn(p_user_data: *mut std::ffi::c_void, size: usize, alignment: usize, allocation_scope: crate::vk1_0::SystemAllocationScope) -> *mut std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkFreeFunction.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkFreeFunction = unsafe extern "system" fn(p_user_data: *mut std::ffi::c_void, p_memory: *mut std::ffi::c_void) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/PFN_vkVoidFunction.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkVoidFunction = unsafe extern "system" fn(unnamed: std::ffi::c_void) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseOutStructure.html) · Structure"]
#[doc(alias = "VkBaseOutStructure")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseOutStructure {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut crate::vk1_0::BaseOutStructure,
}
impl Default for BaseOutStructure {
    fn default() -> Self {
        Self { s_type: Default::default(), p_next: std::ptr::null_mut() }
    }
}
impl std::fmt::Debug for BaseOutStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseOutStructure").field("s_type", &self.s_type).field("p_next", &self.p_next).finish()
    }
}
impl BaseOutStructure {
    #[inline]
    pub fn into_builder<'a>(self) -> BaseOutStructureBuilder<'a> {
        BaseOutStructureBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseOutStructure.html) · Builder of [`BaseOutStructure`]"]
#[repr(transparent)]
pub struct BaseOutStructureBuilder<'a>(BaseOutStructure, std::marker::PhantomData<&'a ()>);
impl<'a> BaseOutStructureBuilder<'a> {
    #[inline]
    pub fn new() -> BaseOutStructureBuilder<'a> {
        BaseOutStructureBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn next(mut self, next: &'a mut crate::vk1_0::BaseOutStructure) -> Self {
        self.0.p_next = next as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BaseOutStructure {
        self.0
    }
}
impl<'a> std::default::Default for BaseOutStructureBuilder<'a> {
    fn default() -> BaseOutStructureBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BaseOutStructureBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BaseOutStructureBuilder<'a> {
    type Target = BaseOutStructure;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BaseOutStructureBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseInStructure.html) · Structure"]
#[doc(alias = "VkBaseInStructure")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseInStructure {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const crate::vk1_0::BaseInStructure,
}
impl Default for BaseInStructure {
    fn default() -> Self {
        Self { s_type: Default::default(), p_next: std::ptr::null() }
    }
}
impl std::fmt::Debug for BaseInStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseInStructure").field("s_type", &self.s_type).field("p_next", &self.p_next).finish()
    }
}
impl BaseInStructure {
    #[inline]
    pub fn into_builder<'a>(self) -> BaseInStructureBuilder<'a> {
        BaseInStructureBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBaseInStructure.html) · Builder of [`BaseInStructure`]"]
#[repr(transparent)]
pub struct BaseInStructureBuilder<'a>(BaseInStructure, std::marker::PhantomData<&'a ()>);
impl<'a> BaseInStructureBuilder<'a> {
    #[inline]
    pub fn new() -> BaseInStructureBuilder<'a> {
        BaseInStructureBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn next(mut self, next: &'a crate::vk1_0::BaseInStructure) -> Self {
        self.0.p_next = next as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BaseInStructure {
        self.0
    }
}
impl<'a> std::default::Default for BaseInStructureBuilder<'a> {
    fn default() -> BaseInStructureBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BaseInStructureBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BaseInStructureBuilder<'a> {
    type Target = BaseInStructure;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BaseInStructureBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset2D.html) · Structure"]
#[doc(alias = "VkOffset2D")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}
impl Default for Offset2D {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}
impl std::fmt::Debug for Offset2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Offset2D").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl Offset2D {
    #[inline]
    pub fn into_builder<'a>(self) -> Offset2DBuilder<'a> {
        Offset2DBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset2D.html) · Builder of [`Offset2D`]"]
#[repr(transparent)]
pub struct Offset2DBuilder<'a>(Offset2D, std::marker::PhantomData<&'a ()>);
impl<'a> Offset2DBuilder<'a> {
    #[inline]
    pub fn new() -> Offset2DBuilder<'a> {
        Offset2DBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn x(mut self, x: i32) -> Self {
        self.0.x = x as _;
        self
    }
    #[inline]
    pub fn y(mut self, y: i32) -> Self {
        self.0.y = y as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Offset2D {
        self.0
    }
}
impl<'a> std::default::Default for Offset2DBuilder<'a> {
    fn default() -> Offset2DBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Offset2DBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Offset2DBuilder<'a> {
    type Target = Offset2D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Offset2DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset3D.html) · Structure"]
#[doc(alias = "VkOffset3D")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Default for Offset3D {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default() }
    }
}
impl std::fmt::Debug for Offset3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Offset3D").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl Offset3D {
    #[inline]
    pub fn into_builder<'a>(self) -> Offset3DBuilder<'a> {
        Offset3DBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkOffset3D.html) · Builder of [`Offset3D`]"]
#[repr(transparent)]
pub struct Offset3DBuilder<'a>(Offset3D, std::marker::PhantomData<&'a ()>);
impl<'a> Offset3DBuilder<'a> {
    #[inline]
    pub fn new() -> Offset3DBuilder<'a> {
        Offset3DBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn x(mut self, x: i32) -> Self {
        self.0.x = x as _;
        self
    }
    #[inline]
    pub fn y(mut self, y: i32) -> Self {
        self.0.y = y as _;
        self
    }
    #[inline]
    pub fn z(mut self, z: i32) -> Self {
        self.0.z = z as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Offset3D {
        self.0
    }
}
impl<'a> std::default::Default for Offset3DBuilder<'a> {
    fn default() -> Offset3DBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Offset3DBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Offset3DBuilder<'a> {
    type Target = Offset3D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Offset3DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html) · Structure"]
#[doc(alias = "VkExtent2D")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}
impl Default for Extent2D {
    fn default() -> Self {
        Self { width: Default::default(), height: Default::default() }
    }
}
impl std::fmt::Debug for Extent2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Extent2D").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl Extent2D {
    #[inline]
    pub fn into_builder<'a>(self) -> Extent2DBuilder<'a> {
        Extent2DBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent2D.html) · Builder of [`Extent2D`]"]
#[repr(transparent)]
pub struct Extent2DBuilder<'a>(Extent2D, std::marker::PhantomData<&'a ()>);
impl<'a> Extent2DBuilder<'a> {
    #[inline]
    pub fn new() -> Extent2DBuilder<'a> {
        Extent2DBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.0.width = width as _;
        self
    }
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.0.height = height as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Extent2D {
        self.0
    }
}
impl<'a> std::default::Default for Extent2DBuilder<'a> {
    fn default() -> Extent2DBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Extent2DBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Extent2DBuilder<'a> {
    type Target = Extent2D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Extent2DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent3D.html) · Structure"]
#[doc(alias = "VkExtent3D")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
impl Default for Extent3D {
    fn default() -> Self {
        Self { width: Default::default(), height: Default::default(), depth: Default::default() }
    }
}
impl std::fmt::Debug for Extent3D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Extent3D").field("width", &self.width).field("height", &self.height).field("depth", &self.depth).finish()
    }
}
impl Extent3D {
    #[inline]
    pub fn into_builder<'a>(self) -> Extent3DBuilder<'a> {
        Extent3DBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtent3D.html) · Builder of [`Extent3D`]"]
#[repr(transparent)]
pub struct Extent3DBuilder<'a>(Extent3D, std::marker::PhantomData<&'a ()>);
impl<'a> Extent3DBuilder<'a> {
    #[inline]
    pub fn new() -> Extent3DBuilder<'a> {
        Extent3DBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.0.width = width as _;
        self
    }
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.0.height = height as _;
        self
    }
    #[inline]
    pub fn depth(mut self, depth: u32) -> Self {
        self.0.depth = depth as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Extent3D {
        self.0
    }
}
impl<'a> std::default::Default for Extent3DBuilder<'a> {
    fn default() -> Extent3DBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Extent3DBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Extent3DBuilder<'a> {
    type Target = Extent3D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Extent3DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewport.html) · Structure"]
#[doc(alias = "VkViewport")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Viewport {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
    pub width: std::os::raw::c_float,
    pub height: std::os::raw::c_float,
    pub min_depth: std::os::raw::c_float,
    pub max_depth: std::os::raw::c_float,
}
impl Default for Viewport {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), width: Default::default(), height: Default::default(), min_depth: Default::default(), max_depth: Default::default() }
    }
}
impl std::fmt::Debug for Viewport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Viewport").field("x", &self.x).field("y", &self.y).field("width", &self.width).field("height", &self.height).field("min_depth", &self.min_depth).field("max_depth", &self.max_depth).finish()
    }
}
impl Viewport {
    #[inline]
    pub fn into_builder<'a>(self) -> ViewportBuilder<'a> {
        ViewportBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkViewport.html) · Builder of [`Viewport`]"]
#[repr(transparent)]
pub struct ViewportBuilder<'a>(Viewport, std::marker::PhantomData<&'a ()>);
impl<'a> ViewportBuilder<'a> {
    #[inline]
    pub fn new() -> ViewportBuilder<'a> {
        ViewportBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn x(mut self, x: std::os::raw::c_float) -> Self {
        self.0.x = x as _;
        self
    }
    #[inline]
    pub fn y(mut self, y: std::os::raw::c_float) -> Self {
        self.0.y = y as _;
        self
    }
    #[inline]
    pub fn width(mut self, width: std::os::raw::c_float) -> Self {
        self.0.width = width as _;
        self
    }
    #[inline]
    pub fn height(mut self, height: std::os::raw::c_float) -> Self {
        self.0.height = height as _;
        self
    }
    #[inline]
    pub fn min_depth(mut self, min_depth: std::os::raw::c_float) -> Self {
        self.0.min_depth = min_depth as _;
        self
    }
    #[inline]
    pub fn max_depth(mut self, max_depth: std::os::raw::c_float) -> Self {
        self.0.max_depth = max_depth as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Viewport {
        self.0
    }
}
impl<'a> std::default::Default for ViewportBuilder<'a> {
    fn default() -> ViewportBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ViewportBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ViewportBuilder<'a> {
    type Target = Viewport;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ViewportBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRect2D.html) · Structure"]
#[doc(alias = "VkRect2D")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rect2D {
    pub offset: crate::vk1_0::Offset2D,
    pub extent: crate::vk1_0::Extent2D,
}
impl Default for Rect2D {
    fn default() -> Self {
        Self { offset: Default::default(), extent: Default::default() }
    }
}
impl std::fmt::Debug for Rect2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Rect2D").field("offset", &self.offset).field("extent", &self.extent).finish()
    }
}
impl Rect2D {
    #[inline]
    pub fn into_builder<'a>(self) -> Rect2DBuilder<'a> {
        Rect2DBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRect2D.html) · Builder of [`Rect2D`]"]
#[repr(transparent)]
pub struct Rect2DBuilder<'a>(Rect2D, std::marker::PhantomData<&'a ()>);
impl<'a> Rect2DBuilder<'a> {
    #[inline]
    pub fn new() -> Rect2DBuilder<'a> {
        Rect2DBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::Offset2D) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn extent(mut self, extent: crate::vk1_0::Extent2D) -> Self {
        self.0.extent = extent as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> Rect2D {
        self.0
    }
}
impl<'a> std::default::Default for Rect2DBuilder<'a> {
    fn default() -> Rect2DBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for Rect2DBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for Rect2DBuilder<'a> {
    type Target = Rect2D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for Rect2DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearRect.html) · Structure"]
#[doc(alias = "VkClearRect")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClearRect {
    pub rect: crate::vk1_0::Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
impl Default for ClearRect {
    fn default() -> Self {
        Self { rect: Default::default(), base_array_layer: Default::default(), layer_count: Default::default() }
    }
}
impl std::fmt::Debug for ClearRect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClearRect").field("rect", &self.rect).field("base_array_layer", &self.base_array_layer).field("layer_count", &self.layer_count).finish()
    }
}
impl ClearRect {
    #[inline]
    pub fn into_builder<'a>(self) -> ClearRectBuilder<'a> {
        ClearRectBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearRect.html) · Builder of [`ClearRect`]"]
#[repr(transparent)]
pub struct ClearRectBuilder<'a>(ClearRect, std::marker::PhantomData<&'a ()>);
impl<'a> ClearRectBuilder<'a> {
    #[inline]
    pub fn new() -> ClearRectBuilder<'a> {
        ClearRectBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn rect(mut self, rect: crate::vk1_0::Rect2D) -> Self {
        self.0.rect = rect as _;
        self
    }
    #[inline]
    pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.0.base_array_layer = base_array_layer as _;
        self
    }
    #[inline]
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.0.layer_count = layer_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ClearRect {
        self.0
    }
}
impl<'a> std::default::Default for ClearRectBuilder<'a> {
    fn default() -> ClearRectBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ClearRectBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ClearRectBuilder<'a> {
    type Target = ClearRect;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ClearRectBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentMapping.html) · Structure"]
#[doc(alias = "VkComponentMapping")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComponentMapping {
    pub r: crate::vk1_0::ComponentSwizzle,
    pub g: crate::vk1_0::ComponentSwizzle,
    pub b: crate::vk1_0::ComponentSwizzle,
    pub a: crate::vk1_0::ComponentSwizzle,
}
impl Default for ComponentMapping {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default(), a: Default::default() }
    }
}
impl std::fmt::Debug for ComponentMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ComponentMapping").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ComponentMapping {
    #[inline]
    pub fn into_builder<'a>(self) -> ComponentMappingBuilder<'a> {
        ComponentMappingBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComponentMapping.html) · Builder of [`ComponentMapping`]"]
#[repr(transparent)]
pub struct ComponentMappingBuilder<'a>(ComponentMapping, std::marker::PhantomData<&'a ()>);
impl<'a> ComponentMappingBuilder<'a> {
    #[inline]
    pub fn new() -> ComponentMappingBuilder<'a> {
        ComponentMappingBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn r(mut self, r: crate::vk1_0::ComponentSwizzle) -> Self {
        self.0.r = r as _;
        self
    }
    #[inline]
    pub fn g(mut self, g: crate::vk1_0::ComponentSwizzle) -> Self {
        self.0.g = g as _;
        self
    }
    #[inline]
    pub fn b(mut self, b: crate::vk1_0::ComponentSwizzle) -> Self {
        self.0.b = b as _;
        self
    }
    #[inline]
    pub fn a(mut self, a: crate::vk1_0::ComponentSwizzle) -> Self {
        self.0.a = a as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ComponentMapping {
        self.0
    }
}
impl<'a> std::default::Default for ComponentMappingBuilder<'a> {
    fn default() -> ComponentMappingBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ComponentMappingBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ComponentMappingBuilder<'a> {
    type Target = ComponentMapping;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ComponentMappingBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: crate::vk1_0::PhysicalDeviceType,
    pub device_name: [std::os::raw::c_char; 256],
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: crate::vk1_0::PhysicalDeviceLimits,
    pub sparse_properties: crate::vk1_0::PhysicalDeviceSparseProperties,
}
impl Default for PhysicalDeviceProperties {
    fn default() -> Self {
        Self { api_version: Default::default(), driver_version: Default::default(), vendor_id: Default::default(), device_id: Default::default(), device_type: Default::default(), device_name: unsafe { std::mem::zeroed() }, pipeline_cache_uuid: unsafe { std::mem::zeroed() }, limits: Default::default(), sparse_properties: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceProperties").field("api_version", &self.api_version).field("driver_version", &self.driver_version).field("vendor_id", &self.vendor_id).field("device_id", &self.device_id).field("device_type", &self.device_type).field("device_name", unsafe { &std::ffi::CStr::from_ptr(self.device_name.as_ptr()) }).field("pipeline_cache_uuid", &self.pipeline_cache_uuid).field("limits", &self.limits).field("sparse_properties", &self.sparse_properties).finish()
    }
}
impl PhysicalDeviceProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePropertiesBuilder<'a> {
        PhysicalDevicePropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceProperties.html) · Builder of [`PhysicalDeviceProperties`]"]
#[repr(transparent)]
pub struct PhysicalDevicePropertiesBuilder<'a>(PhysicalDeviceProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePropertiesBuilder<'a> {
        PhysicalDevicePropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn api_version(mut self, api_version: u32) -> Self {
        self.0.api_version = api_version as _;
        self
    }
    #[inline]
    pub fn driver_version(mut self, driver_version: u32) -> Self {
        self.0.driver_version = driver_version as _;
        self
    }
    #[inline]
    pub fn vendor_id(mut self, vendor_id: u32) -> Self {
        self.0.vendor_id = vendor_id as _;
        self
    }
    #[inline]
    pub fn device_id(mut self, device_id: u32) -> Self {
        self.0.device_id = device_id as _;
        self
    }
    #[inline]
    pub fn device_type(mut self, device_type: crate::vk1_0::PhysicalDeviceType) -> Self {
        self.0.device_type = device_type as _;
        self
    }
    #[inline]
    pub fn device_name(mut self, device_name: [std::os::raw::c_char; 256]) -> Self {
        self.0.device_name = device_name as _;
        self
    }
    #[inline]
    pub fn pipeline_cache_uuid(mut self, pipeline_cache_uuid: [u8; 16]) -> Self {
        self.0.pipeline_cache_uuid = pipeline_cache_uuid as _;
        self
    }
    #[inline]
    pub fn limits(mut self, limits: crate::vk1_0::PhysicalDeviceLimits) -> Self {
        self.0.limits = limits as _;
        self
    }
    #[inline]
    pub fn sparse_properties(mut self, sparse_properties: crate::vk1_0::PhysicalDeviceSparseProperties) -> Self {
        self.0.sparse_properties = sparse_properties as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePropertiesBuilder<'a> {
    fn default() -> PhysicalDevicePropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePropertiesBuilder<'a> {
    type Target = PhysicalDeviceProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtensionProperties.html) · Structure"]
#[doc(alias = "VkExtensionProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExtensionProperties {
    pub extension_name: [std::os::raw::c_char; 256],
    pub spec_version: u32,
}
impl Default for ExtensionProperties {
    fn default() -> Self {
        Self { extension_name: unsafe { std::mem::zeroed() }, spec_version: Default::default() }
    }
}
impl std::fmt::Debug for ExtensionProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ExtensionProperties").field("extension_name", unsafe { &std::ffi::CStr::from_ptr(self.extension_name.as_ptr()) }).field("spec_version", &self.spec_version).finish()
    }
}
impl ExtensionProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> ExtensionPropertiesBuilder<'a> {
        ExtensionPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExtensionProperties.html) · Builder of [`ExtensionProperties`]"]
#[repr(transparent)]
pub struct ExtensionPropertiesBuilder<'a>(ExtensionProperties, std::marker::PhantomData<&'a ()>);
impl<'a> ExtensionPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ExtensionPropertiesBuilder<'a> {
        ExtensionPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn extension_name(mut self, extension_name: [std::os::raw::c_char; 256]) -> Self {
        self.0.extension_name = extension_name as _;
        self
    }
    #[inline]
    pub fn spec_version(mut self, spec_version: u32) -> Self {
        self.0.spec_version = spec_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ExtensionProperties {
        self.0
    }
}
impl<'a> std::default::Default for ExtensionPropertiesBuilder<'a> {
    fn default() -> ExtensionPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ExtensionPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ExtensionPropertiesBuilder<'a> {
    type Target = ExtensionProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ExtensionPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLayerProperties.html) · Structure"]
#[doc(alias = "VkLayerProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayerProperties {
    pub layer_name: [std::os::raw::c_char; 256],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [std::os::raw::c_char; 256],
}
impl Default for LayerProperties {
    fn default() -> Self {
        Self { layer_name: unsafe { std::mem::zeroed() }, spec_version: Default::default(), implementation_version: Default::default(), description: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for LayerProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LayerProperties").field("layer_name", unsafe { &std::ffi::CStr::from_ptr(self.layer_name.as_ptr()) }).field("spec_version", &self.spec_version).field("implementation_version", &self.implementation_version).field("description", unsafe { &std::ffi::CStr::from_ptr(self.description.as_ptr()) }).finish()
    }
}
impl LayerProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> LayerPropertiesBuilder<'a> {
        LayerPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkLayerProperties.html) · Builder of [`LayerProperties`]"]
#[repr(transparent)]
pub struct LayerPropertiesBuilder<'a>(LayerProperties, std::marker::PhantomData<&'a ()>);
impl<'a> LayerPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> LayerPropertiesBuilder<'a> {
        LayerPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn layer_name(mut self, layer_name: [std::os::raw::c_char; 256]) -> Self {
        self.0.layer_name = layer_name as _;
        self
    }
    #[inline]
    pub fn spec_version(mut self, spec_version: u32) -> Self {
        self.0.spec_version = spec_version as _;
        self
    }
    #[inline]
    pub fn implementation_version(mut self, implementation_version: u32) -> Self {
        self.0.implementation_version = implementation_version as _;
        self
    }
    #[inline]
    pub fn description(mut self, description: [std::os::raw::c_char; 256]) -> Self {
        self.0.description = description as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> LayerProperties {
        self.0
    }
}
impl<'a> std::default::Default for LayerPropertiesBuilder<'a> {
    fn default() -> LayerPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for LayerPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for LayerPropertiesBuilder<'a> {
    type Target = LayerProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for LayerPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkApplicationInfo.html) · Structure"]
#[doc(alias = "VkApplicationInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ApplicationInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_application_name: *const std::os::raw::c_char,
    pub application_version: u32,
    pub p_engine_name: *const std::os::raw::c_char,
    pub engine_version: u32,
    pub api_version: u32,
}
impl Default for ApplicationInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::APPLICATION_INFO, p_next: std::ptr::null(), p_application_name: std::ptr::null(), application_version: Default::default(), p_engine_name: std::ptr::null(), engine_version: Default::default(), api_version: Default::default() }
    }
}
impl std::fmt::Debug for ApplicationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ApplicationInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_application_name", &self.p_application_name).field("application_version", &self.application_version).field("p_engine_name", &self.p_engine_name).field("engine_version", &self.engine_version).field("api_version", &self.api_version).finish()
    }
}
impl ApplicationInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ApplicationInfoBuilder<'a> {
        ApplicationInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkApplicationInfo.html) · Builder of [`ApplicationInfo`]"]
#[repr(transparent)]
pub struct ApplicationInfoBuilder<'a>(ApplicationInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ApplicationInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ApplicationInfoBuilder<'a> {
        ApplicationInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn application_name(mut self, application_name: &'a std::ffi::CStr) -> Self {
        self.0.p_application_name = application_name.as_ptr();
        self
    }
    #[inline]
    pub fn application_version(mut self, application_version: u32) -> Self {
        self.0.application_version = application_version as _;
        self
    }
    #[inline]
    pub fn engine_name(mut self, engine_name: &'a std::ffi::CStr) -> Self {
        self.0.p_engine_name = engine_name.as_ptr();
        self
    }
    #[inline]
    pub fn engine_version(mut self, engine_version: u32) -> Self {
        self.0.engine_version = engine_version as _;
        self
    }
    #[inline]
    pub fn api_version(mut self, api_version: u32) -> Self {
        self.0.api_version = api_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ApplicationInfo {
        self.0
    }
}
impl<'a> std::default::Default for ApplicationInfoBuilder<'a> {
    fn default() -> ApplicationInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ApplicationInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ApplicationInfoBuilder<'a> {
    type Target = ApplicationInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ApplicationInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAllocationCallbacks.html) · Structure"]
#[doc(alias = "VkAllocationCallbacks")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AllocationCallbacks {
    pub p_user_data: *mut std::ffi::c_void,
    pub pfn_allocation: Option<crate::vk1_0::PFN_vkAllocationFunction>,
    pub pfn_reallocation: Option<crate::vk1_0::PFN_vkReallocationFunction>,
    pub pfn_free: Option<crate::vk1_0::PFN_vkFreeFunction>,
    pub pfn_internal_allocation: Option<crate::vk1_0::PFN_vkInternalAllocationNotification>,
    pub pfn_internal_free: Option<crate::vk1_0::PFN_vkInternalFreeNotification>,
}
impl Default for AllocationCallbacks {
    fn default() -> Self {
        Self { p_user_data: std::ptr::null_mut(), pfn_allocation: Default::default(), pfn_reallocation: Default::default(), pfn_free: Default::default(), pfn_internal_allocation: Default::default(), pfn_internal_free: Default::default() }
    }
}
impl std::fmt::Debug for AllocationCallbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AllocationCallbacks").field("p_user_data", &self.p_user_data).field("pfn_allocation", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_allocation) }).field("pfn_reallocation", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_reallocation) }).field("pfn_free", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_free) }).field("pfn_internal_allocation", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_internal_allocation) }).field("pfn_internal_free", unsafe { &std::mem::transmute::<_, *const ()>(self.pfn_internal_free) }).finish()
    }
}
impl AllocationCallbacks {
    #[inline]
    pub fn into_builder<'a>(self) -> AllocationCallbacksBuilder<'a> {
        AllocationCallbacksBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAllocationCallbacks.html) · Builder of [`AllocationCallbacks`]"]
#[repr(transparent)]
pub struct AllocationCallbacksBuilder<'a>(AllocationCallbacks, std::marker::PhantomData<&'a ()>);
impl<'a> AllocationCallbacksBuilder<'a> {
    #[inline]
    pub fn new() -> AllocationCallbacksBuilder<'a> {
        AllocationCallbacksBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn user_data(mut self, user_data: *mut std::ffi::c_void) -> Self {
        self.0.p_user_data = user_data;
        self
    }
    #[inline]
    pub fn pfn_allocation(mut self, pfn_allocation: Option<crate::vk1_0::PFN_vkAllocationFunction>) -> Self {
        self.0.pfn_allocation = pfn_allocation as _;
        self
    }
    #[inline]
    pub fn pfn_reallocation(mut self, pfn_reallocation: Option<crate::vk1_0::PFN_vkReallocationFunction>) -> Self {
        self.0.pfn_reallocation = pfn_reallocation as _;
        self
    }
    #[inline]
    pub fn pfn_free(mut self, pfn_free: Option<crate::vk1_0::PFN_vkFreeFunction>) -> Self {
        self.0.pfn_free = pfn_free as _;
        self
    }
    #[inline]
    pub fn pfn_internal_allocation(mut self, pfn_internal_allocation: Option<crate::vk1_0::PFN_vkInternalAllocationNotification>) -> Self {
        self.0.pfn_internal_allocation = pfn_internal_allocation as _;
        self
    }
    #[inline]
    pub fn pfn_internal_free(mut self, pfn_internal_free: Option<crate::vk1_0::PFN_vkInternalFreeNotification>) -> Self {
        self.0.pfn_internal_free = pfn_internal_free as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AllocationCallbacks {
        self.0
    }
}
impl<'a> std::default::Default for AllocationCallbacksBuilder<'a> {
    fn default() -> AllocationCallbacksBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AllocationCallbacksBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AllocationCallbacksBuilder<'a> {
    type Target = AllocationCallbacks;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AllocationCallbacksBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateInfo.html) · Structure"]
#[doc(alias = "VkDeviceQueueCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const std::os::raw::c_float,
}
impl Default for DeviceQueueCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEVICE_QUEUE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), queue_family_index: Default::default(), queue_count: Default::default(), p_queue_priorities: std::ptr::null() }
    }
}
impl std::fmt::Debug for DeviceQueueCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceQueueCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("queue_family_index", &self.queue_family_index).field("queue_count", &self.queue_count).field("p_queue_priorities", &self.p_queue_priorities).finish()
    }
}
impl DeviceQueueCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceQueueCreateInfoBuilder<'a> {
        DeviceQueueCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceQueueCreateInfo.html) · Builder of [`DeviceQueueCreateInfo`]"]
#[repr(transparent)]
pub struct DeviceQueueCreateInfoBuilder<'a>(DeviceQueueCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceQueueCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceQueueCreateInfoBuilder<'a> {
        DeviceQueueCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::DeviceQueueCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.0.queue_family_index = queue_family_index as _;
        self
    }
    #[inline]
    pub fn queue_priorities(mut self, queue_priorities: &'a [std::os::raw::c_float]) -> Self {
        self.0.p_queue_priorities = queue_priorities.as_ptr() as _;
        self.0.queue_count = queue_priorities.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceQueueCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for DeviceQueueCreateInfoBuilder<'a> {
    fn default() -> DeviceQueueCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceQueueCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceQueueCreateInfoBuilder<'a> {
    type Target = DeviceQueueCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceQueueCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceCreateInfo.html) · Structure"]
#[doc(alias = "VkDeviceCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DeviceCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const crate::vk1_0::DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const std::os::raw::c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const std::os::raw::c_char,
    pub p_enabled_features: *const crate::vk1_0::PhysicalDeviceFeatures,
}
impl Default for DeviceCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DEVICE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), queue_create_info_count: Default::default(), p_queue_create_infos: std::ptr::null(), enabled_layer_count: Default::default(), pp_enabled_layer_names: std::ptr::null(), enabled_extension_count: Default::default(), pp_enabled_extension_names: std::ptr::null(), p_enabled_features: std::ptr::null() }
    }
}
impl std::fmt::Debug for DeviceCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeviceCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("queue_create_info_count", &self.queue_create_info_count).field("p_queue_create_infos", &self.p_queue_create_infos).field("enabled_layer_count", &self.enabled_layer_count).field("pp_enabled_layer_names", &self.pp_enabled_layer_names).field("enabled_extension_count", &self.enabled_extension_count).field("pp_enabled_extension_names", &self.pp_enabled_extension_names).field("p_enabled_features", &self.p_enabled_features).finish()
    }
}
impl DeviceCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DeviceCreateInfoBuilder<'a> {
        DeviceCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDeviceCreateInfo.html) · Builder of [`DeviceCreateInfo`]"]
#[repr(transparent)]
pub struct DeviceCreateInfoBuilder<'a>(DeviceCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DeviceCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DeviceCreateInfoBuilder<'a> {
        DeviceCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::DeviceCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn queue_create_infos(mut self, queue_create_infos: &'a [crate::vk1_0::DeviceQueueCreateInfoBuilder]) -> Self {
        self.0.p_queue_create_infos = queue_create_infos.as_ptr() as _;
        self.0.queue_create_info_count = queue_create_infos.len() as _;
        self
    }
    #[inline]
    pub fn enabled_layer_names(mut self, enabled_layer_names: &'a [*const std::os::raw::c_char]) -> Self {
        self.0.pp_enabled_layer_names = enabled_layer_names.as_ptr() as _;
        self.0.enabled_layer_count = enabled_layer_names.len() as _;
        self
    }
    #[inline]
    pub fn enabled_extension_names(mut self, enabled_extension_names: &'a [*const std::os::raw::c_char]) -> Self {
        self.0.pp_enabled_extension_names = enabled_extension_names.as_ptr() as _;
        self.0.enabled_extension_count = enabled_extension_names.len() as _;
        self
    }
    #[inline]
    pub fn enabled_features(mut self, enabled_features: &'a crate::vk1_0::PhysicalDeviceFeatures) -> Self {
        self.0.p_enabled_features = enabled_features as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DeviceCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for DeviceCreateInfoBuilder<'a> {
    fn default() -> DeviceCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DeviceCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DeviceCreateInfoBuilder<'a> {
    type Target = DeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DeviceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstanceCreateInfo.html) · Structure"]
#[doc(alias = "VkInstanceCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstanceCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::InstanceCreateFlags,
    pub p_application_info: *const crate::vk1_0::ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const std::os::raw::c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const std::os::raw::c_char,
}
impl Default for InstanceCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::INSTANCE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), p_application_info: std::ptr::null(), enabled_layer_count: Default::default(), pp_enabled_layer_names: std::ptr::null(), enabled_extension_count: Default::default(), pp_enabled_extension_names: std::ptr::null() }
    }
}
impl std::fmt::Debug for InstanceCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InstanceCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("p_application_info", &self.p_application_info).field("enabled_layer_count", &self.enabled_layer_count).field("pp_enabled_layer_names", &self.pp_enabled_layer_names).field("enabled_extension_count", &self.enabled_extension_count).field("pp_enabled_extension_names", &self.pp_enabled_extension_names).finish()
    }
}
impl InstanceCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> InstanceCreateInfoBuilder<'a> {
        InstanceCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkInstanceCreateInfo.html) · Builder of [`InstanceCreateInfo`]"]
#[repr(transparent)]
pub struct InstanceCreateInfoBuilder<'a>(InstanceCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> InstanceCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> InstanceCreateInfoBuilder<'a> {
        InstanceCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::InstanceCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn application_info(mut self, application_info: &'a crate::vk1_0::ApplicationInfo) -> Self {
        self.0.p_application_info = application_info as _;
        self
    }
    #[inline]
    pub fn enabled_layer_names(mut self, enabled_layer_names: &'a [*const std::os::raw::c_char]) -> Self {
        self.0.pp_enabled_layer_names = enabled_layer_names.as_ptr() as _;
        self.0.enabled_layer_count = enabled_layer_names.len() as _;
        self
    }
    #[inline]
    pub fn enabled_extension_names(mut self, enabled_extension_names: &'a [*const std::os::raw::c_char]) -> Self {
        self.0.pp_enabled_extension_names = enabled_extension_names.as_ptr() as _;
        self.0.enabled_extension_count = enabled_extension_names.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> InstanceCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for InstanceCreateInfoBuilder<'a> {
    fn default() -> InstanceCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for InstanceCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for InstanceCreateInfoBuilder<'a> {
    type Target = InstanceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for InstanceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties.html) · Structure"]
#[doc(alias = "VkQueueFamilyProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: crate::vk1_0::QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: crate::vk1_0::Extent3D,
}
impl Default for QueueFamilyProperties {
    fn default() -> Self {
        Self { queue_flags: Default::default(), queue_count: Default::default(), timestamp_valid_bits: Default::default(), min_image_transfer_granularity: Default::default() }
    }
}
impl std::fmt::Debug for QueueFamilyProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueueFamilyProperties").field("queue_flags", &self.queue_flags).field("queue_count", &self.queue_count).field("timestamp_valid_bits", &self.timestamp_valid_bits).field("min_image_transfer_granularity", &self.min_image_transfer_granularity).finish()
    }
}
impl QueueFamilyProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> QueueFamilyPropertiesBuilder<'a> {
        QueueFamilyPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueueFamilyProperties.html) · Builder of [`QueueFamilyProperties`]"]
#[repr(transparent)]
pub struct QueueFamilyPropertiesBuilder<'a>(QueueFamilyProperties, std::marker::PhantomData<&'a ()>);
impl<'a> QueueFamilyPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> QueueFamilyPropertiesBuilder<'a> {
        QueueFamilyPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn queue_flags(mut self, queue_flags: crate::vk1_0::QueueFlags) -> Self {
        self.0.queue_flags = queue_flags as _;
        self
    }
    #[inline]
    pub fn queue_count(mut self, queue_count: u32) -> Self {
        self.0.queue_count = queue_count as _;
        self
    }
    #[inline]
    pub fn timestamp_valid_bits(mut self, timestamp_valid_bits: u32) -> Self {
        self.0.timestamp_valid_bits = timestamp_valid_bits as _;
        self
    }
    #[inline]
    pub fn min_image_transfer_granularity(mut self, min_image_transfer_granularity: crate::vk1_0::Extent3D) -> Self {
        self.0.min_image_transfer_granularity = min_image_transfer_granularity as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> QueueFamilyProperties {
        self.0
    }
}
impl<'a> std::default::Default for QueueFamilyPropertiesBuilder<'a> {
    fn default() -> QueueFamilyPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueueFamilyPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for QueueFamilyPropertiesBuilder<'a> {
    type Target = QueueFamilyProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueueFamilyPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMemoryProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [crate::vk1_0::MemoryType; 32],
    pub memory_heap_count: u32,
    pub memory_heaps: [crate::vk1_0::MemoryHeap; 16],
}
impl Default for PhysicalDeviceMemoryProperties {
    fn default() -> Self {
        Self { memory_type_count: Default::default(), memory_types: unsafe { std::mem::zeroed() }, memory_heap_count: Default::default(), memory_heaps: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for PhysicalDeviceMemoryProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMemoryProperties").field("memory_type_count", &self.memory_type_count).field("memory_types", &self.memory_types).field("memory_heap_count", &self.memory_heap_count).field("memory_heaps", &self.memory_heaps).finish()
    }
}
impl PhysicalDeviceMemoryProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMemoryPropertiesBuilder<'a> {
        PhysicalDeviceMemoryPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryProperties.html) · Builder of [`PhysicalDeviceMemoryProperties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceMemoryPropertiesBuilder<'a>(PhysicalDeviceMemoryProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceMemoryPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMemoryPropertiesBuilder<'a> {
        PhysicalDeviceMemoryPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_type_count(mut self, memory_type_count: u32) -> Self {
        self.0.memory_type_count = memory_type_count as _;
        self
    }
    #[inline]
    pub fn memory_types(mut self, memory_types: [crate::vk1_0::MemoryType; 32]) -> Self {
        self.0.memory_types = memory_types as _;
        self
    }
    #[inline]
    pub fn memory_heap_count(mut self, memory_heap_count: u32) -> Self {
        self.0.memory_heap_count = memory_heap_count as _;
        self
    }
    #[inline]
    pub fn memory_heaps(mut self, memory_heaps: [crate::vk1_0::MemoryHeap; 16]) -> Self {
        self.0.memory_heaps = memory_heaps as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMemoryProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMemoryPropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceMemoryPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMemoryPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMemoryPropertiesBuilder<'a> {
    type Target = PhysicalDeviceMemoryProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMemoryPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateInfo.html) · Structure"]
#[doc(alias = "VkMemoryAllocateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub allocation_size: crate::vk1_0::DeviceSize,
    pub memory_type_index: u32,
}
impl Default for MemoryAllocateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::MEMORY_ALLOCATE_INFO, p_next: std::ptr::null(), allocation_size: Default::default(), memory_type_index: Default::default() }
    }
}
impl std::fmt::Debug for MemoryAllocateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryAllocateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("allocation_size", &self.allocation_size).field("memory_type_index", &self.memory_type_index).finish()
    }
}
impl MemoryAllocateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryAllocateInfoBuilder<'a> {
        MemoryAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryAllocateInfo.html) · Builder of [`MemoryAllocateInfo`]"]
#[repr(transparent)]
pub struct MemoryAllocateInfoBuilder<'a>(MemoryAllocateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryAllocateInfoBuilder<'a> {
        MemoryAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn allocation_size(mut self, allocation_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.allocation_size = allocation_size as _;
        self
    }
    #[inline]
    pub fn memory_type_index(mut self, memory_type_index: u32) -> Self {
        self.0.memory_type_index = memory_type_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryAllocateInfo {
        self.0
    }
}
impl<'a> std::default::Default for MemoryAllocateInfoBuilder<'a> {
    fn default() -> MemoryAllocateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryAllocateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryAllocateInfoBuilder<'a> {
    type Target = MemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements.html) · Structure"]
#[doc(alias = "VkMemoryRequirements")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryRequirements {
    pub size: crate::vk1_0::DeviceSize,
    pub alignment: crate::vk1_0::DeviceSize,
    pub memory_type_bits: u32,
}
impl Default for MemoryRequirements {
    fn default() -> Self {
        Self { size: Default::default(), alignment: Default::default(), memory_type_bits: Default::default() }
    }
}
impl std::fmt::Debug for MemoryRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryRequirements").field("size", &self.size).field("alignment", &self.alignment).field("memory_type_bits", &self.memory_type_bits).finish()
    }
}
impl MemoryRequirements {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryRequirementsBuilder<'a> {
        MemoryRequirementsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryRequirements.html) · Builder of [`MemoryRequirements`]"]
#[repr(transparent)]
pub struct MemoryRequirementsBuilder<'a>(MemoryRequirements, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryRequirementsBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryRequirementsBuilder<'a> {
        MemoryRequirementsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn alignment(mut self, alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.alignment = alignment as _;
        self
    }
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryRequirements {
        self.0
    }
}
impl<'a> std::default::Default for MemoryRequirementsBuilder<'a> {
    fn default() -> MemoryRequirementsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryRequirementsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryRequirementsBuilder<'a> {
    type Target = MemoryRequirements;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryRequirementsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties.html) · Structure"]
#[doc(alias = "VkSparseImageFormatProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
    pub image_granularity: crate::vk1_0::Extent3D,
    pub flags: crate::vk1_0::SparseImageFormatFlags,
}
impl Default for SparseImageFormatProperties {
    fn default() -> Self {
        Self { aspect_mask: Default::default(), image_granularity: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for SparseImageFormatProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseImageFormatProperties").field("aspect_mask", &self.aspect_mask).field("image_granularity", &self.image_granularity).field("flags", &self.flags).finish()
    }
}
impl SparseImageFormatProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseImageFormatPropertiesBuilder<'a> {
        SparseImageFormatPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageFormatProperties.html) · Builder of [`SparseImageFormatProperties`]"]
#[repr(transparent)]
pub struct SparseImageFormatPropertiesBuilder<'a>(SparseImageFormatProperties, std::marker::PhantomData<&'a ()>);
impl<'a> SparseImageFormatPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> SparseImageFormatPropertiesBuilder<'a> {
        SparseImageFormatPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask as _;
        self
    }
    #[inline]
    pub fn image_granularity(mut self, image_granularity: crate::vk1_0::Extent3D) -> Self {
        self.0.image_granularity = image_granularity as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::SparseImageFormatFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseImageFormatProperties {
        self.0
    }
}
impl<'a> std::default::Default for SparseImageFormatPropertiesBuilder<'a> {
    fn default() -> SparseImageFormatPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseImageFormatPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseImageFormatPropertiesBuilder<'a> {
    type Target = SparseImageFormatProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements.html) · Structure"]
#[doc(alias = "VkSparseImageMemoryRequirements")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: crate::vk1_0::SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: crate::vk1_0::DeviceSize,
    pub image_mip_tail_offset: crate::vk1_0::DeviceSize,
    pub image_mip_tail_stride: crate::vk1_0::DeviceSize,
}
impl Default for SparseImageMemoryRequirements {
    fn default() -> Self {
        Self { format_properties: Default::default(), image_mip_tail_first_lod: Default::default(), image_mip_tail_size: Default::default(), image_mip_tail_offset: Default::default(), image_mip_tail_stride: Default::default() }
    }
}
impl std::fmt::Debug for SparseImageMemoryRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseImageMemoryRequirements").field("format_properties", &self.format_properties).field("image_mip_tail_first_lod", &self.image_mip_tail_first_lod).field("image_mip_tail_size", &self.image_mip_tail_size).field("image_mip_tail_offset", &self.image_mip_tail_offset).field("image_mip_tail_stride", &self.image_mip_tail_stride).finish()
    }
}
impl SparseImageMemoryRequirements {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseImageMemoryRequirementsBuilder<'a> {
        SparseImageMemoryRequirementsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryRequirements.html) · Builder of [`SparseImageMemoryRequirements`]"]
#[repr(transparent)]
pub struct SparseImageMemoryRequirementsBuilder<'a>(SparseImageMemoryRequirements, std::marker::PhantomData<&'a ()>);
impl<'a> SparseImageMemoryRequirementsBuilder<'a> {
    #[inline]
    pub fn new() -> SparseImageMemoryRequirementsBuilder<'a> {
        SparseImageMemoryRequirementsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn format_properties(mut self, format_properties: crate::vk1_0::SparseImageFormatProperties) -> Self {
        self.0.format_properties = format_properties as _;
        self
    }
    #[inline]
    pub fn image_mip_tail_first_lod(mut self, image_mip_tail_first_lod: u32) -> Self {
        self.0.image_mip_tail_first_lod = image_mip_tail_first_lod as _;
        self
    }
    #[inline]
    pub fn image_mip_tail_size(mut self, image_mip_tail_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.image_mip_tail_size = image_mip_tail_size as _;
        self
    }
    #[inline]
    pub fn image_mip_tail_offset(mut self, image_mip_tail_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.image_mip_tail_offset = image_mip_tail_offset as _;
        self
    }
    #[inline]
    pub fn image_mip_tail_stride(mut self, image_mip_tail_stride: crate::vk1_0::DeviceSize) -> Self {
        self.0.image_mip_tail_stride = image_mip_tail_stride as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseImageMemoryRequirements {
        self.0
    }
}
impl<'a> std::default::Default for SparseImageMemoryRequirementsBuilder<'a> {
    fn default() -> SparseImageMemoryRequirementsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseImageMemoryRequirementsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseImageMemoryRequirementsBuilder<'a> {
    type Target = SparseImageMemoryRequirements;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageMemoryRequirementsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryType.html) · Structure"]
#[doc(alias = "VkMemoryType")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryType {
    pub property_flags: crate::vk1_0::MemoryPropertyFlags,
    pub heap_index: u32,
}
impl Default for MemoryType {
    fn default() -> Self {
        Self { property_flags: Default::default(), heap_index: Default::default() }
    }
}
impl std::fmt::Debug for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryType").field("property_flags", &self.property_flags).field("heap_index", &self.heap_index).finish()
    }
}
impl MemoryType {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryTypeBuilder<'a> {
        MemoryTypeBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryType.html) · Builder of [`MemoryType`]"]
#[repr(transparent)]
pub struct MemoryTypeBuilder<'a>(MemoryType, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryTypeBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryTypeBuilder<'a> {
        MemoryTypeBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn property_flags(mut self, property_flags: crate::vk1_0::MemoryPropertyFlags) -> Self {
        self.0.property_flags = property_flags as _;
        self
    }
    #[inline]
    pub fn heap_index(mut self, heap_index: u32) -> Self {
        self.0.heap_index = heap_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryType {
        self.0
    }
}
impl<'a> std::default::Default for MemoryTypeBuilder<'a> {
    fn default() -> MemoryTypeBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryTypeBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryTypeBuilder<'a> {
    type Target = MemoryType;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryTypeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHeap.html) · Structure"]
#[doc(alias = "VkMemoryHeap")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryHeap {
    pub size: crate::vk1_0::DeviceSize,
    pub flags: crate::vk1_0::MemoryHeapFlags,
}
impl Default for MemoryHeap {
    fn default() -> Self {
        Self { size: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for MemoryHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryHeap").field("size", &self.size).field("flags", &self.flags).finish()
    }
}
impl MemoryHeap {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryHeapBuilder<'a> {
        MemoryHeapBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryHeap.html) · Builder of [`MemoryHeap`]"]
#[repr(transparent)]
pub struct MemoryHeapBuilder<'a>(MemoryHeap, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryHeapBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryHeapBuilder<'a> {
        MemoryHeapBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::MemoryHeapFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryHeap {
        self.0
    }
}
impl<'a> std::default::Default for MemoryHeapBuilder<'a> {
    fn default() -> MemoryHeapBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryHeapBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryHeapBuilder<'a> {
    type Target = MemoryHeap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryHeapBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMappedMemoryRange.html) · Structure"]
#[doc(alias = "VkMappedMemoryRange")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MappedMemoryRange {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
    pub offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
}
impl Default for MappedMemoryRange {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::MAPPED_MEMORY_RANGE, p_next: std::ptr::null(), memory: Default::default(), offset: Default::default(), size: Default::default() }
    }
}
impl std::fmt::Debug for MappedMemoryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MappedMemoryRange").field("s_type", &self.s_type).field("p_next", &self.p_next).field("memory", &self.memory).field("offset", &self.offset).field("size", &self.size).finish()
    }
}
impl MappedMemoryRange {
    #[inline]
    pub fn into_builder<'a>(self) -> MappedMemoryRangeBuilder<'a> {
        MappedMemoryRangeBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMappedMemoryRange.html) · Builder of [`MappedMemoryRange`]"]
#[repr(transparent)]
pub struct MappedMemoryRangeBuilder<'a>(MappedMemoryRange, std::marker::PhantomData<&'a ()>);
impl<'a> MappedMemoryRangeBuilder<'a> {
    #[inline]
    pub fn new() -> MappedMemoryRangeBuilder<'a> {
        MappedMemoryRangeBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MappedMemoryRange {
        self.0
    }
}
impl<'a> std::default::Default for MappedMemoryRangeBuilder<'a> {
    fn default() -> MappedMemoryRangeBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MappedMemoryRangeBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MappedMemoryRangeBuilder<'a> {
    type Target = MappedMemoryRange;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MappedMemoryRangeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties.html) · Structure"]
#[doc(alias = "VkFormatProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormatProperties {
    pub linear_tiling_features: crate::vk1_0::FormatFeatureFlags,
    pub optimal_tiling_features: crate::vk1_0::FormatFeatureFlags,
    pub buffer_features: crate::vk1_0::FormatFeatureFlags,
}
impl Default for FormatProperties {
    fn default() -> Self {
        Self { linear_tiling_features: Default::default(), optimal_tiling_features: Default::default(), buffer_features: Default::default() }
    }
}
impl std::fmt::Debug for FormatProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FormatProperties").field("linear_tiling_features", &self.linear_tiling_features).field("optimal_tiling_features", &self.optimal_tiling_features).field("buffer_features", &self.buffer_features).finish()
    }
}
impl FormatProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> FormatPropertiesBuilder<'a> {
        FormatPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties.html) · Builder of [`FormatProperties`]"]
#[repr(transparent)]
pub struct FormatPropertiesBuilder<'a>(FormatProperties, std::marker::PhantomData<&'a ()>);
impl<'a> FormatPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> FormatPropertiesBuilder<'a> {
        FormatPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn linear_tiling_features(mut self, linear_tiling_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.linear_tiling_features = linear_tiling_features as _;
        self
    }
    #[inline]
    pub fn optimal_tiling_features(mut self, optimal_tiling_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.optimal_tiling_features = optimal_tiling_features as _;
        self
    }
    #[inline]
    pub fn buffer_features(mut self, buffer_features: crate::vk1_0::FormatFeatureFlags) -> Self {
        self.0.buffer_features = buffer_features as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FormatProperties {
        self.0
    }
}
impl<'a> std::default::Default for FormatPropertiesBuilder<'a> {
    fn default() -> FormatPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FormatPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FormatPropertiesBuilder<'a> {
    type Target = FormatProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties.html) · Structure"]
#[doc(alias = "VkImageFormatProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageFormatProperties {
    pub max_extent: crate::vk1_0::Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: crate::vk1_0::SampleCountFlags,
    pub max_resource_size: crate::vk1_0::DeviceSize,
}
impl Default for ImageFormatProperties {
    fn default() -> Self {
        Self { max_extent: Default::default(), max_mip_levels: Default::default(), max_array_layers: Default::default(), sample_counts: Default::default(), max_resource_size: Default::default() }
    }
}
impl std::fmt::Debug for ImageFormatProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageFormatProperties").field("max_extent", &self.max_extent).field("max_mip_levels", &self.max_mip_levels).field("max_array_layers", &self.max_array_layers).field("sample_counts", &self.sample_counts).field("max_resource_size", &self.max_resource_size).finish()
    }
}
impl ImageFormatProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageFormatPropertiesBuilder<'a> {
        ImageFormatPropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageFormatProperties.html) · Builder of [`ImageFormatProperties`]"]
#[repr(transparent)]
pub struct ImageFormatPropertiesBuilder<'a>(ImageFormatProperties, std::marker::PhantomData<&'a ()>);
impl<'a> ImageFormatPropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> ImageFormatPropertiesBuilder<'a> {
        ImageFormatPropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_extent(mut self, max_extent: crate::vk1_0::Extent3D) -> Self {
        self.0.max_extent = max_extent as _;
        self
    }
    #[inline]
    pub fn max_mip_levels(mut self, max_mip_levels: u32) -> Self {
        self.0.max_mip_levels = max_mip_levels as _;
        self
    }
    #[inline]
    pub fn max_array_layers(mut self, max_array_layers: u32) -> Self {
        self.0.max_array_layers = max_array_layers as _;
        self
    }
    #[inline]
    pub fn sample_counts(mut self, sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.sample_counts = sample_counts as _;
        self
    }
    #[inline]
    pub fn max_resource_size(mut self, max_resource_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.max_resource_size = max_resource_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageFormatProperties {
        self.0
    }
}
impl<'a> std::default::Default for ImageFormatPropertiesBuilder<'a> {
    fn default() -> ImageFormatPropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageFormatPropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageFormatPropertiesBuilder<'a> {
    type Target = ImageFormatProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBufferInfo.html) · Structure"]
#[doc(alias = "VkDescriptorBufferInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
    pub range: crate::vk1_0::DeviceSize,
}
impl Default for DescriptorBufferInfo {
    fn default() -> Self {
        Self { buffer: Default::default(), offset: Default::default(), range: Default::default() }
    }
}
impl std::fmt::Debug for DescriptorBufferInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorBufferInfo").field("buffer", &self.buffer).field("offset", &self.offset).field("range", &self.range).finish()
    }
}
impl DescriptorBufferInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorBufferInfoBuilder<'a> {
        DescriptorBufferInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorBufferInfo.html) · Builder of [`DescriptorBufferInfo`]"]
#[repr(transparent)]
pub struct DescriptorBufferInfoBuilder<'a>(DescriptorBufferInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorBufferInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorBufferInfoBuilder<'a> {
        DescriptorBufferInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn range(mut self, range: crate::vk1_0::DeviceSize) -> Self {
        self.0.range = range as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorBufferInfo {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorBufferInfoBuilder<'a> {
    fn default() -> DescriptorBufferInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorBufferInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorBufferInfoBuilder<'a> {
    type Target = DescriptorBufferInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorBufferInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorImageInfo.html) · Structure"]
#[doc(alias = "VkDescriptorImageInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: crate::vk1_0::Sampler,
    pub image_view: crate::vk1_0::ImageView,
    pub image_layout: crate::vk1_0::ImageLayout,
}
impl Default for DescriptorImageInfo {
    fn default() -> Self {
        Self { sampler: Default::default(), image_view: Default::default(), image_layout: Default::default() }
    }
}
impl std::fmt::Debug for DescriptorImageInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorImageInfo").field("sampler", &self.sampler).field("image_view", &self.image_view).field("image_layout", &self.image_layout).finish()
    }
}
impl DescriptorImageInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorImageInfoBuilder<'a> {
        DescriptorImageInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorImageInfo.html) · Builder of [`DescriptorImageInfo`]"]
#[repr(transparent)]
pub struct DescriptorImageInfoBuilder<'a>(DescriptorImageInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorImageInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorImageInfoBuilder<'a> {
        DescriptorImageInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sampler(mut self, sampler: crate::vk1_0::Sampler) -> Self {
        self.0.sampler = sampler as _;
        self
    }
    #[inline]
    pub fn image_view(mut self, image_view: crate::vk1_0::ImageView) -> Self {
        self.0.image_view = image_view as _;
        self
    }
    #[inline]
    pub fn image_layout(mut self, image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.image_layout = image_layout as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorImageInfo {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorImageInfoBuilder<'a> {
    fn default() -> DescriptorImageInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorImageInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorImageInfoBuilder<'a> {
    type Target = DescriptorImageInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorImageInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSet.html) · Structure"]
#[doc(alias = "VkWriteDescriptorSet")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WriteDescriptorSet {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub dst_set: crate::vk1_0::DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: crate::vk1_0::DescriptorType,
    pub p_image_info: *const crate::vk1_0::DescriptorImageInfo,
    pub p_buffer_info: *const crate::vk1_0::DescriptorBufferInfo,
    pub p_texel_buffer_view: *const crate::vk1_0::BufferView,
}
impl Default for WriteDescriptorSet {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::WRITE_DESCRIPTOR_SET, p_next: std::ptr::null(), dst_set: Default::default(), dst_binding: Default::default(), dst_array_element: Default::default(), descriptor_count: Default::default(), descriptor_type: Default::default(), p_image_info: std::ptr::null(), p_buffer_info: std::ptr::null(), p_texel_buffer_view: std::ptr::null() }
    }
}
impl std::fmt::Debug for WriteDescriptorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WriteDescriptorSet").field("s_type", &self.s_type).field("p_next", &self.p_next).field("dst_set", &self.dst_set).field("dst_binding", &self.dst_binding).field("dst_array_element", &self.dst_array_element).field("descriptor_count", &self.descriptor_count).field("descriptor_type", &self.descriptor_type).field("p_image_info", &self.p_image_info).field("p_buffer_info", &self.p_buffer_info).field("p_texel_buffer_view", &self.p_texel_buffer_view).finish()
    }
}
impl WriteDescriptorSet {
    #[inline]
    pub fn into_builder<'a>(self) -> WriteDescriptorSetBuilder<'a> {
        WriteDescriptorSetBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkWriteDescriptorSet.html) · Builder of [`WriteDescriptorSet`]"]
#[repr(transparent)]
pub struct WriteDescriptorSetBuilder<'a>(WriteDescriptorSet, std::marker::PhantomData<&'a ()>);
impl<'a> WriteDescriptorSetBuilder<'a> {
    #[inline]
    pub fn new() -> WriteDescriptorSetBuilder<'a> {
        WriteDescriptorSetBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn dst_set(mut self, dst_set: crate::vk1_0::DescriptorSet) -> Self {
        self.0.dst_set = dst_set as _;
        self
    }
    #[inline]
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.0.dst_binding = dst_binding as _;
        self
    }
    #[inline]
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.0.dst_array_element = dst_array_element as _;
        self
    }
    #[inline]
    pub fn descriptor_type(mut self, descriptor_type: crate::vk1_0::DescriptorType) -> Self {
        self.0.descriptor_type = descriptor_type as _;
        self
    }
    #[inline]
    pub fn image_info(mut self, image_info: &'a [crate::vk1_0::DescriptorImageInfoBuilder]) -> Self {
        self.0.p_image_info = image_info.as_ptr() as _;
        self.0.descriptor_count = image_info.len() as _;
        self
    }
    #[inline]
    pub fn buffer_info(mut self, buffer_info: &'a [crate::vk1_0::DescriptorBufferInfoBuilder]) -> Self {
        self.0.p_buffer_info = buffer_info.as_ptr() as _;
        self.0.descriptor_count = buffer_info.len() as _;
        self
    }
    #[inline]
    pub fn texel_buffer_view(mut self, texel_buffer_view: &'a [crate::vk1_0::BufferView]) -> Self {
        self.0.p_texel_buffer_view = texel_buffer_view.as_ptr() as _;
        self.0.descriptor_count = texel_buffer_view.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> WriteDescriptorSet {
        self.0
    }
}
impl<'a> std::default::Default for WriteDescriptorSetBuilder<'a> {
    fn default() -> WriteDescriptorSetBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for WriteDescriptorSetBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for WriteDescriptorSetBuilder<'a> {
    type Target = WriteDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for WriteDescriptorSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyDescriptorSet.html) · Structure"]
#[doc(alias = "VkCopyDescriptorSet")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyDescriptorSet {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_set: crate::vk1_0::DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: crate::vk1_0::DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}
impl Default for CopyDescriptorSet {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COPY_DESCRIPTOR_SET, p_next: std::ptr::null(), src_set: Default::default(), src_binding: Default::default(), src_array_element: Default::default(), dst_set: Default::default(), dst_binding: Default::default(), dst_array_element: Default::default(), descriptor_count: Default::default() }
    }
}
impl std::fmt::Debug for CopyDescriptorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyDescriptorSet").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src_set", &self.src_set).field("src_binding", &self.src_binding).field("src_array_element", &self.src_array_element).field("dst_set", &self.dst_set).field("dst_binding", &self.dst_binding).field("dst_array_element", &self.dst_array_element).field("descriptor_count", &self.descriptor_count).finish()
    }
}
impl CopyDescriptorSet {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyDescriptorSetBuilder<'a> {
        CopyDescriptorSetBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyDescriptorSet.html) · Builder of [`CopyDescriptorSet`]"]
#[repr(transparent)]
pub struct CopyDescriptorSetBuilder<'a>(CopyDescriptorSet, std::marker::PhantomData<&'a ()>);
impl<'a> CopyDescriptorSetBuilder<'a> {
    #[inline]
    pub fn new() -> CopyDescriptorSetBuilder<'a> {
        CopyDescriptorSetBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_set(mut self, src_set: crate::vk1_0::DescriptorSet) -> Self {
        self.0.src_set = src_set as _;
        self
    }
    #[inline]
    pub fn src_binding(mut self, src_binding: u32) -> Self {
        self.0.src_binding = src_binding as _;
        self
    }
    #[inline]
    pub fn src_array_element(mut self, src_array_element: u32) -> Self {
        self.0.src_array_element = src_array_element as _;
        self
    }
    #[inline]
    pub fn dst_set(mut self, dst_set: crate::vk1_0::DescriptorSet) -> Self {
        self.0.dst_set = dst_set as _;
        self
    }
    #[inline]
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.0.dst_binding = dst_binding as _;
        self
    }
    #[inline]
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.0.dst_array_element = dst_array_element as _;
        self
    }
    #[inline]
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.0.descriptor_count = descriptor_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyDescriptorSet {
        self.0
    }
}
impl<'a> std::default::Default for CopyDescriptorSetBuilder<'a> {
    fn default() -> CopyDescriptorSetBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyDescriptorSetBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyDescriptorSetBuilder<'a> {
    type Target = CopyDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyDescriptorSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateInfo.html) · Structure"]
#[doc(alias = "VkBufferCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::BufferCreateFlags,
    pub size: crate::vk1_0::DeviceSize,
    pub usage: crate::vk1_0::BufferUsageFlags,
    pub sharing_mode: crate::vk1_0::SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}
impl Default for BufferCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::BUFFER_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), size: Default::default(), usage: Default::default(), sharing_mode: Default::default(), queue_family_index_count: Default::default(), p_queue_family_indices: std::ptr::null() }
    }
}
impl std::fmt::Debug for BufferCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("size", &self.size).field("usage", &self.usage).field("sharing_mode", &self.sharing_mode).field("queue_family_index_count", &self.queue_family_index_count).field("p_queue_family_indices", &self.p_queue_family_indices).finish()
    }
}
impl BufferCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCreateInfoBuilder<'a> {
        BufferCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCreateInfo.html) · Builder of [`BufferCreateInfo`]"]
#[repr(transparent)]
pub struct BufferCreateInfoBuilder<'a>(BufferCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BufferCreateInfoBuilder<'a> {
        BufferCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::BufferCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::BufferUsageFlags) -> Self {
        self.0.usage = usage as _;
        self
    }
    #[inline]
    pub fn sharing_mode(mut self, sharing_mode: crate::vk1_0::SharingMode) -> Self {
        self.0.sharing_mode = sharing_mode as _;
        self
    }
    #[inline]
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.0.p_queue_family_indices = queue_family_indices.as_ptr() as _;
        self.0.queue_family_index_count = queue_family_indices.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for BufferCreateInfoBuilder<'a> {
    fn default() -> BufferCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCreateInfoBuilder<'a> {
    type Target = BufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferViewCreateInfo.html) · Structure"]
#[doc(alias = "VkBufferViewCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferViewCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::BufferViewCreateFlags,
    pub buffer: crate::vk1_0::Buffer,
    pub format: crate::vk1_0::Format,
    pub offset: crate::vk1_0::DeviceSize,
    pub range: crate::vk1_0::DeviceSize,
}
impl Default for BufferViewCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::BUFFER_VIEW_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), buffer: Default::default(), format: Default::default(), offset: Default::default(), range: Default::default() }
    }
}
impl std::fmt::Debug for BufferViewCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferViewCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("buffer", &self.buffer).field("format", &self.format).field("offset", &self.offset).field("range", &self.range).finish()
    }
}
impl BufferViewCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferViewCreateInfoBuilder<'a> {
        BufferViewCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferViewCreateInfo.html) · Builder of [`BufferViewCreateInfo`]"]
#[repr(transparent)]
pub struct BufferViewCreateInfoBuilder<'a>(BufferViewCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BufferViewCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BufferViewCreateInfoBuilder<'a> {
        BufferViewCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::BufferViewCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn range(mut self, range: crate::vk1_0::DeviceSize) -> Self {
        self.0.range = range as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferViewCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for BufferViewCreateInfoBuilder<'a> {
    fn default() -> BufferViewCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferViewCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferViewCreateInfoBuilder<'a> {
    type Target = BufferViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferViewCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresource.html) · Structure"]
#[doc(alias = "VkImageSubresource")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageSubresource {
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}
impl Default for ImageSubresource {
    fn default() -> Self {
        Self { aspect_mask: Default::default(), mip_level: Default::default(), array_layer: Default::default() }
    }
}
impl std::fmt::Debug for ImageSubresource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageSubresource").field("aspect_mask", &self.aspect_mask).field("mip_level", &self.mip_level).field("array_layer", &self.array_layer).finish()
    }
}
impl ImageSubresource {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageSubresourceBuilder<'a> {
        ImageSubresourceBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresource.html) · Builder of [`ImageSubresource`]"]
#[repr(transparent)]
pub struct ImageSubresourceBuilder<'a>(ImageSubresource, std::marker::PhantomData<&'a ()>);
impl<'a> ImageSubresourceBuilder<'a> {
    #[inline]
    pub fn new() -> ImageSubresourceBuilder<'a> {
        ImageSubresourceBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask as _;
        self
    }
    #[inline]
    pub fn mip_level(mut self, mip_level: u32) -> Self {
        self.0.mip_level = mip_level as _;
        self
    }
    #[inline]
    pub fn array_layer(mut self, array_layer: u32) -> Self {
        self.0.array_layer = array_layer as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageSubresource {
        self.0
    }
}
impl<'a> std::default::Default for ImageSubresourceBuilder<'a> {
    fn default() -> ImageSubresourceBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageSubresourceBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageSubresourceBuilder<'a> {
    type Target = ImageSubresource;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageSubresourceBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceLayers.html) · Structure"]
#[doc(alias = "VkImageSubresourceLayers")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
impl Default for ImageSubresourceLayers {
    fn default() -> Self {
        Self { aspect_mask: Default::default(), mip_level: Default::default(), base_array_layer: Default::default(), layer_count: Default::default() }
    }
}
impl std::fmt::Debug for ImageSubresourceLayers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageSubresourceLayers").field("aspect_mask", &self.aspect_mask).field("mip_level", &self.mip_level).field("base_array_layer", &self.base_array_layer).field("layer_count", &self.layer_count).finish()
    }
}
impl ImageSubresourceLayers {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageSubresourceLayersBuilder<'a> {
        ImageSubresourceLayersBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceLayers.html) · Builder of [`ImageSubresourceLayers`]"]
#[repr(transparent)]
pub struct ImageSubresourceLayersBuilder<'a>(ImageSubresourceLayers, std::marker::PhantomData<&'a ()>);
impl<'a> ImageSubresourceLayersBuilder<'a> {
    #[inline]
    pub fn new() -> ImageSubresourceLayersBuilder<'a> {
        ImageSubresourceLayersBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask as _;
        self
    }
    #[inline]
    pub fn mip_level(mut self, mip_level: u32) -> Self {
        self.0.mip_level = mip_level as _;
        self
    }
    #[inline]
    pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.0.base_array_layer = base_array_layer as _;
        self
    }
    #[inline]
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.0.layer_count = layer_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageSubresourceLayers {
        self.0
    }
}
impl<'a> std::default::Default for ImageSubresourceLayersBuilder<'a> {
    fn default() -> ImageSubresourceLayersBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageSubresourceLayersBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageSubresourceLayersBuilder<'a> {
    type Target = ImageSubresourceLayers;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageSubresourceLayersBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceRange.html) · Structure"]
#[doc(alias = "VkImageSubresourceRange")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageSubresourceRange {
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
impl Default for ImageSubresourceRange {
    fn default() -> Self {
        Self { aspect_mask: Default::default(), base_mip_level: Default::default(), level_count: Default::default(), base_array_layer: Default::default(), layer_count: Default::default() }
    }
}
impl std::fmt::Debug for ImageSubresourceRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageSubresourceRange").field("aspect_mask", &self.aspect_mask).field("base_mip_level", &self.base_mip_level).field("level_count", &self.level_count).field("base_array_layer", &self.base_array_layer).field("layer_count", &self.layer_count).finish()
    }
}
impl ImageSubresourceRange {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageSubresourceRangeBuilder<'a> {
        ImageSubresourceRangeBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageSubresourceRange.html) · Builder of [`ImageSubresourceRange`]"]
#[repr(transparent)]
pub struct ImageSubresourceRangeBuilder<'a>(ImageSubresourceRange, std::marker::PhantomData<&'a ()>);
impl<'a> ImageSubresourceRangeBuilder<'a> {
    #[inline]
    pub fn new() -> ImageSubresourceRangeBuilder<'a> {
        ImageSubresourceRangeBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask as _;
        self
    }
    #[inline]
    pub fn base_mip_level(mut self, base_mip_level: u32) -> Self {
        self.0.base_mip_level = base_mip_level as _;
        self
    }
    #[inline]
    pub fn level_count(mut self, level_count: u32) -> Self {
        self.0.level_count = level_count as _;
        self
    }
    #[inline]
    pub fn base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.0.base_array_layer = base_array_layer as _;
        self
    }
    #[inline]
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.0.layer_count = layer_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageSubresourceRange {
        self.0
    }
}
impl<'a> std::default::Default for ImageSubresourceRangeBuilder<'a> {
    fn default() -> ImageSubresourceRangeBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageSubresourceRangeBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageSubresourceRangeBuilder<'a> {
    type Target = ImageSubresourceRange;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageSubresourceRangeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryBarrier.html) · Structure"]
#[doc(alias = "VkMemoryBarrier")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryBarrier {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_access_mask: crate::vk1_0::AccessFlags,
    pub dst_access_mask: crate::vk1_0::AccessFlags,
}
impl Default for MemoryBarrier {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::MEMORY_BARRIER, p_next: std::ptr::null(), src_access_mask: Default::default(), dst_access_mask: Default::default() }
    }
}
impl std::fmt::Debug for MemoryBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryBarrier").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src_access_mask", &self.src_access_mask).field("dst_access_mask", &self.dst_access_mask).finish()
    }
}
impl MemoryBarrier {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryBarrierBuilder<'a> {
        MemoryBarrierBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryBarrier.html) · Builder of [`MemoryBarrier`]"]
#[repr(transparent)]
pub struct MemoryBarrierBuilder<'a>(MemoryBarrier, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryBarrierBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryBarrierBuilder<'a> {
        MemoryBarrierBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.src_access_mask = src_access_mask as _;
        self
    }
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.dst_access_mask = dst_access_mask as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryBarrier {
        self.0
    }
}
impl<'a> std::default::Default for MemoryBarrierBuilder<'a> {
    fn default() -> MemoryBarrierBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryBarrierBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryBarrierBuilder<'a> {
    type Target = MemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryBarrierBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryBarrier.html) · Structure"]
#[doc(alias = "VkBufferMemoryBarrier")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferMemoryBarrier {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_access_mask: crate::vk1_0::AccessFlags,
    pub dst_access_mask: crate::vk1_0::AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: crate::vk1_0::Buffer,
    pub offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
}
impl Default for BufferMemoryBarrier {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::BUFFER_MEMORY_BARRIER, p_next: std::ptr::null(), src_access_mask: Default::default(), dst_access_mask: Default::default(), src_queue_family_index: Default::default(), dst_queue_family_index: Default::default(), buffer: Default::default(), offset: Default::default(), size: Default::default() }
    }
}
impl std::fmt::Debug for BufferMemoryBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferMemoryBarrier").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src_access_mask", &self.src_access_mask).field("dst_access_mask", &self.dst_access_mask).field("src_queue_family_index", &self.src_queue_family_index).field("dst_queue_family_index", &self.dst_queue_family_index).field("buffer", &self.buffer).field("offset", &self.offset).field("size", &self.size).finish()
    }
}
impl BufferMemoryBarrier {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferMemoryBarrierBuilder<'a> {
        BufferMemoryBarrierBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferMemoryBarrier.html) · Builder of [`BufferMemoryBarrier`]"]
#[repr(transparent)]
pub struct BufferMemoryBarrierBuilder<'a>(BufferMemoryBarrier, std::marker::PhantomData<&'a ()>);
impl<'a> BufferMemoryBarrierBuilder<'a> {
    #[inline]
    pub fn new() -> BufferMemoryBarrierBuilder<'a> {
        BufferMemoryBarrierBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.src_access_mask = src_access_mask as _;
        self
    }
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.dst_access_mask = dst_access_mask as _;
        self
    }
    #[inline]
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.0.src_queue_family_index = src_queue_family_index as _;
        self
    }
    #[inline]
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.0.dst_queue_family_index = dst_queue_family_index as _;
        self
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferMemoryBarrier {
        self.0
    }
}
impl<'a> std::default::Default for BufferMemoryBarrierBuilder<'a> {
    fn default() -> BufferMemoryBarrierBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferMemoryBarrierBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferMemoryBarrierBuilder<'a> {
    type Target = BufferMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferMemoryBarrierBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier.html) · Structure"]
#[doc(alias = "VkImageMemoryBarrier")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageMemoryBarrier {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_access_mask: crate::vk1_0::AccessFlags,
    pub dst_access_mask: crate::vk1_0::AccessFlags,
    pub old_layout: crate::vk1_0::ImageLayout,
    pub new_layout: crate::vk1_0::ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: crate::vk1_0::Image,
    pub subresource_range: crate::vk1_0::ImageSubresourceRange,
}
impl Default for ImageMemoryBarrier {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMAGE_MEMORY_BARRIER, p_next: std::ptr::null(), src_access_mask: Default::default(), dst_access_mask: Default::default(), old_layout: Default::default(), new_layout: Default::default(), src_queue_family_index: Default::default(), dst_queue_family_index: Default::default(), image: Default::default(), subresource_range: Default::default() }
    }
}
impl std::fmt::Debug for ImageMemoryBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageMemoryBarrier").field("s_type", &self.s_type).field("p_next", &self.p_next).field("src_access_mask", &self.src_access_mask).field("dst_access_mask", &self.dst_access_mask).field("old_layout", &self.old_layout).field("new_layout", &self.new_layout).field("src_queue_family_index", &self.src_queue_family_index).field("dst_queue_family_index", &self.dst_queue_family_index).field("image", &self.image).field("subresource_range", &self.subresource_range).finish()
    }
}
impl ImageMemoryBarrier {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageMemoryBarrierBuilder<'a> {
        ImageMemoryBarrierBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageMemoryBarrier.html) · Builder of [`ImageMemoryBarrier`]"]
#[repr(transparent)]
pub struct ImageMemoryBarrierBuilder<'a>(ImageMemoryBarrier, std::marker::PhantomData<&'a ()>);
impl<'a> ImageMemoryBarrierBuilder<'a> {
    #[inline]
    pub fn new() -> ImageMemoryBarrierBuilder<'a> {
        ImageMemoryBarrierBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.src_access_mask = src_access_mask as _;
        self
    }
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.dst_access_mask = dst_access_mask as _;
        self
    }
    #[inline]
    pub fn old_layout(mut self, old_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.old_layout = old_layout as _;
        self
    }
    #[inline]
    pub fn new_layout(mut self, new_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.new_layout = new_layout as _;
        self
    }
    #[inline]
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.0.src_queue_family_index = src_queue_family_index as _;
        self
    }
    #[inline]
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.0.dst_queue_family_index = dst_queue_family_index as _;
        self
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn subresource_range(mut self, subresource_range: crate::vk1_0::ImageSubresourceRange) -> Self {
        self.0.subresource_range = subresource_range as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageMemoryBarrier {
        self.0
    }
}
impl<'a> std::default::Default for ImageMemoryBarrierBuilder<'a> {
    fn default() -> ImageMemoryBarrierBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageMemoryBarrierBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageMemoryBarrierBuilder<'a> {
    type Target = ImageMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageMemoryBarrierBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateInfo.html) · Structure"]
#[doc(alias = "VkImageCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::ImageCreateFlags,
    pub image_type: crate::vk1_0::ImageType,
    pub format: crate::vk1_0::Format,
    pub extent: crate::vk1_0::Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: crate::vk1_0::SampleCountFlagBits,
    pub tiling: crate::vk1_0::ImageTiling,
    pub usage: crate::vk1_0::ImageUsageFlags,
    pub sharing_mode: crate::vk1_0::SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: crate::vk1_0::ImageLayout,
}
impl Default for ImageCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMAGE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), image_type: Default::default(), format: Default::default(), extent: Default::default(), mip_levels: Default::default(), array_layers: Default::default(), samples: Default::default(), tiling: Default::default(), usage: Default::default(), sharing_mode: Default::default(), queue_family_index_count: Default::default(), p_queue_family_indices: std::ptr::null(), initial_layout: Default::default() }
    }
}
impl std::fmt::Debug for ImageCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("image_type", &self.image_type).field("format", &self.format).field("extent", &self.extent).field("mip_levels", &self.mip_levels).field("array_layers", &self.array_layers).field("samples", &self.samples).field("tiling", &self.tiling).field("usage", &self.usage).field("sharing_mode", &self.sharing_mode).field("queue_family_index_count", &self.queue_family_index_count).field("p_queue_family_indices", &self.p_queue_family_indices).field("initial_layout", &self.initial_layout).finish()
    }
}
impl ImageCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageCreateInfoBuilder<'a> {
        ImageCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCreateInfo.html) · Builder of [`ImageCreateInfo`]"]
#[repr(transparent)]
pub struct ImageCreateInfoBuilder<'a>(ImageCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ImageCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImageCreateInfoBuilder<'a> {
        ImageCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::ImageCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn image_type(mut self, image_type: crate::vk1_0::ImageType) -> Self {
        self.0.image_type = image_type as _;
        self
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn extent(mut self, extent: crate::vk1_0::Extent3D) -> Self {
        self.0.extent = extent as _;
        self
    }
    #[inline]
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.0.mip_levels = mip_levels as _;
        self
    }
    #[inline]
    pub fn array_layers(mut self, array_layers: u32) -> Self {
        self.0.array_layers = array_layers as _;
        self
    }
    #[inline]
    pub fn samples(mut self, samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.samples = samples as _;
        self
    }
    #[inline]
    pub fn tiling(mut self, tiling: crate::vk1_0::ImageTiling) -> Self {
        self.0.tiling = tiling as _;
        self
    }
    #[inline]
    pub fn usage(mut self, usage: crate::vk1_0::ImageUsageFlags) -> Self {
        self.0.usage = usage as _;
        self
    }
    #[inline]
    pub fn sharing_mode(mut self, sharing_mode: crate::vk1_0::SharingMode) -> Self {
        self.0.sharing_mode = sharing_mode as _;
        self
    }
    #[inline]
    pub fn queue_family_indices(mut self, queue_family_indices: &'a [u32]) -> Self {
        self.0.p_queue_family_indices = queue_family_indices.as_ptr() as _;
        self.0.queue_family_index_count = queue_family_indices.len() as _;
        self
    }
    #[inline]
    pub fn initial_layout(mut self, initial_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.initial_layout = initial_layout as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ImageCreateInfoBuilder<'a> {
    fn default() -> ImageCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageCreateInfoBuilder<'a> {
    type Target = ImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubresourceLayout.html) · Structure"]
#[doc(alias = "VkSubresourceLayout")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubresourceLayout {
    pub offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
    pub row_pitch: crate::vk1_0::DeviceSize,
    pub array_pitch: crate::vk1_0::DeviceSize,
    pub depth_pitch: crate::vk1_0::DeviceSize,
}
impl Default for SubresourceLayout {
    fn default() -> Self {
        Self { offset: Default::default(), size: Default::default(), row_pitch: Default::default(), array_pitch: Default::default(), depth_pitch: Default::default() }
    }
}
impl std::fmt::Debug for SubresourceLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubresourceLayout").field("offset", &self.offset).field("size", &self.size).field("row_pitch", &self.row_pitch).field("array_pitch", &self.array_pitch).field("depth_pitch", &self.depth_pitch).finish()
    }
}
impl SubresourceLayout {
    #[inline]
    pub fn into_builder<'a>(self) -> SubresourceLayoutBuilder<'a> {
        SubresourceLayoutBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubresourceLayout.html) · Builder of [`SubresourceLayout`]"]
#[repr(transparent)]
pub struct SubresourceLayoutBuilder<'a>(SubresourceLayout, std::marker::PhantomData<&'a ()>);
impl<'a> SubresourceLayoutBuilder<'a> {
    #[inline]
    pub fn new() -> SubresourceLayoutBuilder<'a> {
        SubresourceLayoutBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn row_pitch(mut self, row_pitch: crate::vk1_0::DeviceSize) -> Self {
        self.0.row_pitch = row_pitch as _;
        self
    }
    #[inline]
    pub fn array_pitch(mut self, array_pitch: crate::vk1_0::DeviceSize) -> Self {
        self.0.array_pitch = array_pitch as _;
        self
    }
    #[inline]
    pub fn depth_pitch(mut self, depth_pitch: crate::vk1_0::DeviceSize) -> Self {
        self.0.depth_pitch = depth_pitch as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SubresourceLayout {
        self.0
    }
}
impl<'a> std::default::Default for SubresourceLayoutBuilder<'a> {
    fn default() -> SubresourceLayoutBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubresourceLayoutBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubresourceLayoutBuilder<'a> {
    type Target = SubresourceLayout;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubresourceLayoutBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateInfo.html) · Structure"]
#[doc(alias = "VkImageViewCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageViewCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::ImageViewCreateFlags,
    pub image: crate::vk1_0::Image,
    pub view_type: crate::vk1_0::ImageViewType,
    pub format: crate::vk1_0::Format,
    pub components: crate::vk1_0::ComponentMapping,
    pub subresource_range: crate::vk1_0::ImageSubresourceRange,
}
impl Default for ImageViewCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::IMAGE_VIEW_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), image: Default::default(), view_type: Default::default(), format: Default::default(), components: Default::default(), subresource_range: Default::default() }
    }
}
impl std::fmt::Debug for ImageViewCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageViewCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("image", &self.image).field("view_type", &self.view_type).field("format", &self.format).field("components", &self.components).field("subresource_range", &self.subresource_range).finish()
    }
}
impl ImageViewCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageViewCreateInfoBuilder<'a> {
        ImageViewCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageViewCreateInfo.html) · Builder of [`ImageViewCreateInfo`]"]
#[repr(transparent)]
pub struct ImageViewCreateInfoBuilder<'a>(ImageViewCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ImageViewCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ImageViewCreateInfoBuilder<'a> {
        ImageViewCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::ImageViewCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn view_type(mut self, view_type: crate::vk1_0::ImageViewType) -> Self {
        self.0.view_type = view_type as _;
        self
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn components(mut self, components: crate::vk1_0::ComponentMapping) -> Self {
        self.0.components = components as _;
        self
    }
    #[inline]
    pub fn subresource_range(mut self, subresource_range: crate::vk1_0::ImageSubresourceRange) -> Self {
        self.0.subresource_range = subresource_range as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageViewCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ImageViewCreateInfoBuilder<'a> {
    fn default() -> ImageViewCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageViewCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageViewCreateInfoBuilder<'a> {
    type Target = ImageViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageViewCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy.html) · Structure"]
#[doc(alias = "VkBufferCopy")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCopy {
    pub src_offset: crate::vk1_0::DeviceSize,
    pub dst_offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
}
impl Default for BufferCopy {
    fn default() -> Self {
        Self { src_offset: Default::default(), dst_offset: Default::default(), size: Default::default() }
    }
}
impl std::fmt::Debug for BufferCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCopy").field("src_offset", &self.src_offset).field("dst_offset", &self.dst_offset).field("size", &self.size).finish()
    }
}
impl BufferCopy {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCopyBuilder<'a> {
        BufferCopyBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy.html) · Builder of [`BufferCopy`]"]
#[repr(transparent)]
pub struct BufferCopyBuilder<'a>(BufferCopy, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCopyBuilder<'a> {
    #[inline]
    pub fn new() -> BufferCopyBuilder<'a> {
        BufferCopyBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_offset(mut self, src_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.src_offset = src_offset as _;
        self
    }
    #[inline]
    pub fn dst_offset(mut self, dst_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.dst_offset = dst_offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCopy {
        self.0
    }
}
impl<'a> std::default::Default for BufferCopyBuilder<'a> {
    fn default() -> BufferCopyBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCopyBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCopyBuilder<'a> {
    type Target = BufferCopy;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCopyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseMemoryBind.html) · Structure"]
#[doc(alias = "VkSparseMemoryBind")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseMemoryBind {
    pub resource_offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
    pub flags: crate::vk1_0::SparseMemoryBindFlags,
}
impl Default for SparseMemoryBind {
    fn default() -> Self {
        Self { resource_offset: Default::default(), size: Default::default(), memory: Default::default(), memory_offset: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for SparseMemoryBind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseMemoryBind").field("resource_offset", &self.resource_offset).field("size", &self.size).field("memory", &self.memory).field("memory_offset", &self.memory_offset).field("flags", &self.flags).finish()
    }
}
impl SparseMemoryBind {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseMemoryBindBuilder<'a> {
        SparseMemoryBindBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseMemoryBind.html) · Builder of [`SparseMemoryBind`]"]
#[repr(transparent)]
pub struct SparseMemoryBindBuilder<'a>(SparseMemoryBind, std::marker::PhantomData<&'a ()>);
impl<'a> SparseMemoryBindBuilder<'a> {
    #[inline]
    pub fn new() -> SparseMemoryBindBuilder<'a> {
        SparseMemoryBindBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn resource_offset(mut self, resource_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.resource_offset = resource_offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: crate::vk1_0::DeviceSize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::SparseMemoryBindFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseMemoryBind {
        self.0
    }
}
impl<'a> std::default::Default for SparseMemoryBindBuilder<'a> {
    fn default() -> SparseMemoryBindBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseMemoryBindBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseMemoryBindBuilder<'a> {
    type Target = SparseMemoryBind;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseMemoryBindBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBind.html) · Structure"]
#[doc(alias = "VkSparseImageMemoryBind")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageMemoryBind {
    pub subresource: crate::vk1_0::ImageSubresource,
    pub offset: crate::vk1_0::Offset3D,
    pub extent: crate::vk1_0::Extent3D,
    pub memory: crate::vk1_0::DeviceMemory,
    pub memory_offset: crate::vk1_0::DeviceSize,
    pub flags: crate::vk1_0::SparseMemoryBindFlags,
}
impl Default for SparseImageMemoryBind {
    fn default() -> Self {
        Self { subresource: Default::default(), offset: Default::default(), extent: Default::default(), memory: Default::default(), memory_offset: Default::default(), flags: Default::default() }
    }
}
impl std::fmt::Debug for SparseImageMemoryBind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseImageMemoryBind").field("subresource", &self.subresource).field("offset", &self.offset).field("extent", &self.extent).field("memory", &self.memory).field("memory_offset", &self.memory_offset).field("flags", &self.flags).finish()
    }
}
impl SparseImageMemoryBind {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseImageMemoryBindBuilder<'a> {
        SparseImageMemoryBindBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBind.html) · Builder of [`SparseImageMemoryBind`]"]
#[repr(transparent)]
pub struct SparseImageMemoryBindBuilder<'a>(SparseImageMemoryBind, std::marker::PhantomData<&'a ()>);
impl<'a> SparseImageMemoryBindBuilder<'a> {
    #[inline]
    pub fn new() -> SparseImageMemoryBindBuilder<'a> {
        SparseImageMemoryBindBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn subresource(mut self, subresource: crate::vk1_0::ImageSubresource) -> Self {
        self.0.subresource = subresource as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: crate::vk1_0::Offset3D) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn extent(mut self, extent: crate::vk1_0::Extent3D) -> Self {
        self.0.extent = extent as _;
        self
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn memory_offset(mut self, memory_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.memory_offset = memory_offset as _;
        self
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::SparseMemoryBindFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseImageMemoryBind {
        self.0
    }
}
impl<'a> std::default::Default for SparseImageMemoryBindBuilder<'a> {
    fn default() -> SparseImageMemoryBindBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseImageMemoryBindBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseImageMemoryBindBuilder<'a> {
    type Target = SparseImageMemoryBind;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageMemoryBindBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseBufferMemoryBindInfo.html) · Structure"]
#[doc(alias = "VkSparseBufferMemoryBindInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: crate::vk1_0::Buffer,
    pub bind_count: u32,
    pub p_binds: *const crate::vk1_0::SparseMemoryBind,
}
impl Default for SparseBufferMemoryBindInfo {
    fn default() -> Self {
        Self { buffer: Default::default(), bind_count: Default::default(), p_binds: std::ptr::null() }
    }
}
impl std::fmt::Debug for SparseBufferMemoryBindInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseBufferMemoryBindInfo").field("buffer", &self.buffer).field("bind_count", &self.bind_count).field("p_binds", &self.p_binds).finish()
    }
}
impl SparseBufferMemoryBindInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseBufferMemoryBindInfoBuilder<'a> {
        SparseBufferMemoryBindInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseBufferMemoryBindInfo.html) · Builder of [`SparseBufferMemoryBindInfo`]"]
#[repr(transparent)]
pub struct SparseBufferMemoryBindInfoBuilder<'a>(SparseBufferMemoryBindInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SparseBufferMemoryBindInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SparseBufferMemoryBindInfoBuilder<'a> {
        SparseBufferMemoryBindInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer(mut self, buffer: crate::vk1_0::Buffer) -> Self {
        self.0.buffer = buffer as _;
        self
    }
    #[inline]
    pub fn binds(mut self, binds: &'a [crate::vk1_0::SparseMemoryBindBuilder]) -> Self {
        self.0.p_binds = binds.as_ptr() as _;
        self.0.bind_count = binds.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseBufferMemoryBindInfo {
        self.0
    }
}
impl<'a> std::default::Default for SparseBufferMemoryBindInfoBuilder<'a> {
    fn default() -> SparseBufferMemoryBindInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseBufferMemoryBindInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseBufferMemoryBindInfoBuilder<'a> {
    type Target = SparseBufferMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseBufferMemoryBindInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html) · Structure"]
#[doc(alias = "VkSparseImageOpaqueMemoryBindInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: crate::vk1_0::Image,
    pub bind_count: u32,
    pub p_binds: *const crate::vk1_0::SparseMemoryBind,
}
impl Default for SparseImageOpaqueMemoryBindInfo {
    fn default() -> Self {
        Self { image: Default::default(), bind_count: Default::default(), p_binds: std::ptr::null() }
    }
}
impl std::fmt::Debug for SparseImageOpaqueMemoryBindInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseImageOpaqueMemoryBindInfo").field("image", &self.image).field("bind_count", &self.bind_count).field("p_binds", &self.p_binds).finish()
    }
}
impl SparseImageOpaqueMemoryBindInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
        SparseImageOpaqueMemoryBindInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageOpaqueMemoryBindInfo.html) · Builder of [`SparseImageOpaqueMemoryBindInfo`]"]
#[repr(transparent)]
pub struct SparseImageOpaqueMemoryBindInfoBuilder<'a>(SparseImageOpaqueMemoryBindInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
        SparseImageOpaqueMemoryBindInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn binds(mut self, binds: &'a [crate::vk1_0::SparseMemoryBindBuilder]) -> Self {
        self.0.p_binds = binds.as_ptr() as _;
        self.0.bind_count = binds.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseImageOpaqueMemoryBindInfo {
        self.0
    }
}
impl<'a> std::default::Default for SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    fn default() -> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    type Target = SparseImageOpaqueMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBindInfo.html) · Structure"]
#[doc(alias = "VkSparseImageMemoryBindInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SparseImageMemoryBindInfo {
    pub image: crate::vk1_0::Image,
    pub bind_count: u32,
    pub p_binds: *const crate::vk1_0::SparseImageMemoryBind,
}
impl Default for SparseImageMemoryBindInfo {
    fn default() -> Self {
        Self { image: Default::default(), bind_count: Default::default(), p_binds: std::ptr::null() }
    }
}
impl std::fmt::Debug for SparseImageMemoryBindInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SparseImageMemoryBindInfo").field("image", &self.image).field("bind_count", &self.bind_count).field("p_binds", &self.p_binds).finish()
    }
}
impl SparseImageMemoryBindInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SparseImageMemoryBindInfoBuilder<'a> {
        SparseImageMemoryBindInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSparseImageMemoryBindInfo.html) · Builder of [`SparseImageMemoryBindInfo`]"]
#[repr(transparent)]
pub struct SparseImageMemoryBindInfoBuilder<'a>(SparseImageMemoryBindInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SparseImageMemoryBindInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SparseImageMemoryBindInfoBuilder<'a> {
        SparseImageMemoryBindInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn image(mut self, image: crate::vk1_0::Image) -> Self {
        self.0.image = image as _;
        self
    }
    #[inline]
    pub fn binds(mut self, binds: &'a [crate::vk1_0::SparseImageMemoryBindBuilder]) -> Self {
        self.0.p_binds = binds.as_ptr() as _;
        self.0.bind_count = binds.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SparseImageMemoryBindInfo {
        self.0
    }
}
impl<'a> std::default::Default for SparseImageMemoryBindInfoBuilder<'a> {
    fn default() -> SparseImageMemoryBindInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SparseImageMemoryBindInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SparseImageMemoryBindInfoBuilder<'a> {
    type Target = SparseImageMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SparseImageMemoryBindInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindSparseInfo.html) · Structure"]
#[doc(alias = "VkBindSparseInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BindSparseInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const crate::vk1_0::Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const crate::vk1_0::SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const crate::vk1_0::SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const crate::vk1_0::SparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const crate::vk1_0::Semaphore,
}
impl Default for BindSparseInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::BIND_SPARSE_INFO, p_next: std::ptr::null(), wait_semaphore_count: Default::default(), p_wait_semaphores: std::ptr::null(), buffer_bind_count: Default::default(), p_buffer_binds: std::ptr::null(), image_opaque_bind_count: Default::default(), p_image_opaque_binds: std::ptr::null(), image_bind_count: Default::default(), p_image_binds: std::ptr::null(), signal_semaphore_count: Default::default(), p_signal_semaphores: std::ptr::null() }
    }
}
impl std::fmt::Debug for BindSparseInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BindSparseInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("wait_semaphore_count", &self.wait_semaphore_count).field("p_wait_semaphores", &self.p_wait_semaphores).field("buffer_bind_count", &self.buffer_bind_count).field("p_buffer_binds", &self.p_buffer_binds).field("image_opaque_bind_count", &self.image_opaque_bind_count).field("p_image_opaque_binds", &self.p_image_opaque_binds).field("image_bind_count", &self.image_bind_count).field("p_image_binds", &self.p_image_binds).field("signal_semaphore_count", &self.signal_semaphore_count).field("p_signal_semaphores", &self.p_signal_semaphores).finish()
    }
}
impl BindSparseInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> BindSparseInfoBuilder<'a> {
        BindSparseInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBindSparseInfo.html) · Builder of [`BindSparseInfo`]"]
#[repr(transparent)]
pub struct BindSparseInfoBuilder<'a>(BindSparseInfo, std::marker::PhantomData<&'a ()>);
impl<'a> BindSparseInfoBuilder<'a> {
    #[inline]
    pub fn new() -> BindSparseInfoBuilder<'a> {
        BindSparseInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn wait_semaphores(mut self, wait_semaphores: &'a [crate::vk1_0::Semaphore]) -> Self {
        self.0.p_wait_semaphores = wait_semaphores.as_ptr() as _;
        self.0.wait_semaphore_count = wait_semaphores.len() as _;
        self
    }
    #[inline]
    pub fn buffer_binds(mut self, buffer_binds: &'a [crate::vk1_0::SparseBufferMemoryBindInfoBuilder]) -> Self {
        self.0.p_buffer_binds = buffer_binds.as_ptr() as _;
        self.0.buffer_bind_count = buffer_binds.len() as _;
        self
    }
    #[inline]
    pub fn image_opaque_binds(mut self, image_opaque_binds: &'a [crate::vk1_0::SparseImageOpaqueMemoryBindInfoBuilder]) -> Self {
        self.0.p_image_opaque_binds = image_opaque_binds.as_ptr() as _;
        self.0.image_opaque_bind_count = image_opaque_binds.len() as _;
        self
    }
    #[inline]
    pub fn image_binds(mut self, image_binds: &'a [crate::vk1_0::SparseImageMemoryBindInfoBuilder]) -> Self {
        self.0.p_image_binds = image_binds.as_ptr() as _;
        self.0.image_bind_count = image_binds.len() as _;
        self
    }
    #[inline]
    pub fn signal_semaphores(mut self, signal_semaphores: &'a [crate::vk1_0::Semaphore]) -> Self {
        self.0.p_signal_semaphores = signal_semaphores.as_ptr() as _;
        self.0.signal_semaphore_count = signal_semaphores.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BindSparseInfo {
        self.0
    }
}
impl<'a> std::default::Default for BindSparseInfoBuilder<'a> {
    fn default() -> BindSparseInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BindSparseInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BindSparseInfoBuilder<'a> {
    type Target = BindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BindSparseInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCopy.html) · Structure"]
#[doc(alias = "VkImageCopy")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageCopy {
    pub src_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub src_offset: crate::vk1_0::Offset3D,
    pub dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub dst_offset: crate::vk1_0::Offset3D,
    pub extent: crate::vk1_0::Extent3D,
}
impl Default for ImageCopy {
    fn default() -> Self {
        Self { src_subresource: Default::default(), src_offset: Default::default(), dst_subresource: Default::default(), dst_offset: Default::default(), extent: Default::default() }
    }
}
impl std::fmt::Debug for ImageCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageCopy").field("src_subresource", &self.src_subresource).field("src_offset", &self.src_offset).field("dst_subresource", &self.dst_subresource).field("dst_offset", &self.dst_offset).field("extent", &self.extent).finish()
    }
}
impl ImageCopy {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageCopyBuilder<'a> {
        ImageCopyBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCopy.html) · Builder of [`ImageCopy`]"]
#[repr(transparent)]
pub struct ImageCopyBuilder<'a>(ImageCopy, std::marker::PhantomData<&'a ()>);
impl<'a> ImageCopyBuilder<'a> {
    #[inline]
    pub fn new() -> ImageCopyBuilder<'a> {
        ImageCopyBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_subresource(mut self, src_subresource: crate::vk1_0::ImageSubresourceLayers) -> Self {
        self.0.src_subresource = src_subresource as _;
        self
    }
    #[inline]
    pub fn src_offset(mut self, src_offset: crate::vk1_0::Offset3D) -> Self {
        self.0.src_offset = src_offset as _;
        self
    }
    #[inline]
    pub fn dst_subresource(mut self, dst_subresource: crate::vk1_0::ImageSubresourceLayers) -> Self {
        self.0.dst_subresource = dst_subresource as _;
        self
    }
    #[inline]
    pub fn dst_offset(mut self, dst_offset: crate::vk1_0::Offset3D) -> Self {
        self.0.dst_offset = dst_offset as _;
        self
    }
    #[inline]
    pub fn extent(mut self, extent: crate::vk1_0::Extent3D) -> Self {
        self.0.extent = extent as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageCopy {
        self.0
    }
}
impl<'a> std::default::Default for ImageCopyBuilder<'a> {
    fn default() -> ImageCopyBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageCopyBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageCopyBuilder<'a> {
    type Target = ImageCopy;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageCopyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit.html) · Structure"]
#[doc(alias = "VkImageBlit")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageBlit {
    pub src_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub src_offsets: [crate::vk1_0::Offset3D; 2],
    pub dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub dst_offsets: [crate::vk1_0::Offset3D; 2],
}
impl Default for ImageBlit {
    fn default() -> Self {
        Self { src_subresource: Default::default(), src_offsets: unsafe { std::mem::zeroed() }, dst_subresource: Default::default(), dst_offsets: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for ImageBlit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageBlit").field("src_subresource", &self.src_subresource).field("src_offsets", &self.src_offsets).field("dst_subresource", &self.dst_subresource).field("dst_offsets", &self.dst_offsets).finish()
    }
}
impl ImageBlit {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageBlitBuilder<'a> {
        ImageBlitBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit.html) · Builder of [`ImageBlit`]"]
#[repr(transparent)]
pub struct ImageBlitBuilder<'a>(ImageBlit, std::marker::PhantomData<&'a ()>);
impl<'a> ImageBlitBuilder<'a> {
    #[inline]
    pub fn new() -> ImageBlitBuilder<'a> {
        ImageBlitBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_subresource(mut self, src_subresource: crate::vk1_0::ImageSubresourceLayers) -> Self {
        self.0.src_subresource = src_subresource as _;
        self
    }
    #[inline]
    pub fn src_offsets(mut self, src_offsets: [crate::vk1_0::Offset3D; 2]) -> Self {
        self.0.src_offsets = src_offsets as _;
        self
    }
    #[inline]
    pub fn dst_subresource(mut self, dst_subresource: crate::vk1_0::ImageSubresourceLayers) -> Self {
        self.0.dst_subresource = dst_subresource as _;
        self
    }
    #[inline]
    pub fn dst_offsets(mut self, dst_offsets: [crate::vk1_0::Offset3D; 2]) -> Self {
        self.0.dst_offsets = dst_offsets as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageBlit {
        self.0
    }
}
impl<'a> std::default::Default for ImageBlitBuilder<'a> {
    fn default() -> ImageBlitBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageBlitBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageBlitBuilder<'a> {
    type Target = ImageBlit;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageBlitBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy.html) · Structure"]
#[doc(alias = "VkBufferImageCopy")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferImageCopy {
    pub buffer_offset: crate::vk1_0::DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub image_offset: crate::vk1_0::Offset3D,
    pub image_extent: crate::vk1_0::Extent3D,
}
impl Default for BufferImageCopy {
    fn default() -> Self {
        Self { buffer_offset: Default::default(), buffer_row_length: Default::default(), buffer_image_height: Default::default(), image_subresource: Default::default(), image_offset: Default::default(), image_extent: Default::default() }
    }
}
impl std::fmt::Debug for BufferImageCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferImageCopy").field("buffer_offset", &self.buffer_offset).field("buffer_row_length", &self.buffer_row_length).field("buffer_image_height", &self.buffer_image_height).field("image_subresource", &self.image_subresource).field("image_offset", &self.image_offset).field("image_extent", &self.image_extent).finish()
    }
}
impl BufferImageCopy {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferImageCopyBuilder<'a> {
        BufferImageCopyBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy.html) · Builder of [`BufferImageCopy`]"]
#[repr(transparent)]
pub struct BufferImageCopyBuilder<'a>(BufferImageCopy, std::marker::PhantomData<&'a ()>);
impl<'a> BufferImageCopyBuilder<'a> {
    #[inline]
    pub fn new() -> BufferImageCopyBuilder<'a> {
        BufferImageCopyBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn buffer_offset(mut self, buffer_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.buffer_offset = buffer_offset as _;
        self
    }
    #[inline]
    pub fn buffer_row_length(mut self, buffer_row_length: u32) -> Self {
        self.0.buffer_row_length = buffer_row_length as _;
        self
    }
    #[inline]
    pub fn buffer_image_height(mut self, buffer_image_height: u32) -> Self {
        self.0.buffer_image_height = buffer_image_height as _;
        self
    }
    #[inline]
    pub fn image_subresource(mut self, image_subresource: crate::vk1_0::ImageSubresourceLayers) -> Self {
        self.0.image_subresource = image_subresource as _;
        self
    }
    #[inline]
    pub fn image_offset(mut self, image_offset: crate::vk1_0::Offset3D) -> Self {
        self.0.image_offset = image_offset as _;
        self
    }
    #[inline]
    pub fn image_extent(mut self, image_extent: crate::vk1_0::Extent3D) -> Self {
        self.0.image_extent = image_extent as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferImageCopy {
        self.0
    }
}
impl<'a> std::default::Default for BufferImageCopyBuilder<'a> {
    fn default() -> BufferImageCopyBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferImageCopyBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferImageCopyBuilder<'a> {
    type Target = BufferImageCopy;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferImageCopyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageResolve.html) · Structure"]
#[doc(alias = "VkImageResolve")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageResolve {
    pub src_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub src_offset: crate::vk1_0::Offset3D,
    pub dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub dst_offset: crate::vk1_0::Offset3D,
    pub extent: crate::vk1_0::Extent3D,
}
impl Default for ImageResolve {
    fn default() -> Self {
        Self { src_subresource: Default::default(), src_offset: Default::default(), dst_subresource: Default::default(), dst_offset: Default::default(), extent: Default::default() }
    }
}
impl std::fmt::Debug for ImageResolve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageResolve").field("src_subresource", &self.src_subresource).field("src_offset", &self.src_offset).field("dst_subresource", &self.dst_subresource).field("dst_offset", &self.dst_offset).field("extent", &self.extent).finish()
    }
}
impl ImageResolve {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageResolveBuilder<'a> {
        ImageResolveBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageResolve.html) · Builder of [`ImageResolve`]"]
#[repr(transparent)]
pub struct ImageResolveBuilder<'a>(ImageResolve, std::marker::PhantomData<&'a ()>);
impl<'a> ImageResolveBuilder<'a> {
    #[inline]
    pub fn new() -> ImageResolveBuilder<'a> {
        ImageResolveBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_subresource(mut self, src_subresource: crate::vk1_0::ImageSubresourceLayers) -> Self {
        self.0.src_subresource = src_subresource as _;
        self
    }
    #[inline]
    pub fn src_offset(mut self, src_offset: crate::vk1_0::Offset3D) -> Self {
        self.0.src_offset = src_offset as _;
        self
    }
    #[inline]
    pub fn dst_subresource(mut self, dst_subresource: crate::vk1_0::ImageSubresourceLayers) -> Self {
        self.0.dst_subresource = dst_subresource as _;
        self
    }
    #[inline]
    pub fn dst_offset(mut self, dst_offset: crate::vk1_0::Offset3D) -> Self {
        self.0.dst_offset = dst_offset as _;
        self
    }
    #[inline]
    pub fn extent(mut self, extent: crate::vk1_0::Extent3D) -> Self {
        self.0.extent = extent as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageResolve {
        self.0
    }
}
impl<'a> std::default::Default for ImageResolveBuilder<'a> {
    fn default() -> ImageResolveBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageResolveBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageResolveBuilder<'a> {
    type Target = ImageResolve;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageResolveBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleCreateInfo.html) · Structure"]
#[doc(alias = "VkShaderModuleCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShaderModuleCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}
impl Default for ShaderModuleCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::SHADER_MODULE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), code_size: Default::default(), p_code: std::ptr::null() }
    }
}
impl std::fmt::Debug for ShaderModuleCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ShaderModuleCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("code_size", &self.code_size).field("p_code", &self.p_code).finish()
    }
}
impl ShaderModuleCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ShaderModuleCreateInfoBuilder<'a> {
        ShaderModuleCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkShaderModuleCreateInfo.html) · Builder of [`ShaderModuleCreateInfo`]"]
#[repr(transparent)]
pub struct ShaderModuleCreateInfoBuilder<'a>(ShaderModuleCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ShaderModuleCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ShaderModuleCreateInfoBuilder<'a> {
        ShaderModuleCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::ShaderModuleCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn code(mut self, code: &'a [u32]) -> Self {
        self.0.p_code = code.as_ptr() as _;
        self.0.code_size = code.len() * 4;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ShaderModuleCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ShaderModuleCreateInfoBuilder<'a> {
    fn default() -> ShaderModuleCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ShaderModuleCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ShaderModuleCreateInfoBuilder<'a> {
    type Target = ShaderModuleCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ShaderModuleCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBinding.html) · Structure"]
#[doc(alias = "VkDescriptorSetLayoutBinding")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: crate::vk1_0::DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: crate::vk1_0::ShaderStageFlags,
    pub p_immutable_samplers: *const crate::vk1_0::Sampler,
}
impl Default for DescriptorSetLayoutBinding {
    fn default() -> Self {
        Self { binding: Default::default(), descriptor_type: Default::default(), descriptor_count: Default::default(), stage_flags: Default::default(), p_immutable_samplers: std::ptr::null() }
    }
}
impl std::fmt::Debug for DescriptorSetLayoutBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorSetLayoutBinding").field("binding", &self.binding).field("descriptor_type", &self.descriptor_type).field("descriptor_count", &self.descriptor_count).field("stage_flags", &self.stage_flags).field("p_immutable_samplers", &self.p_immutable_samplers).finish()
    }
}
impl DescriptorSetLayoutBinding {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorSetLayoutBindingBuilder<'a> {
        DescriptorSetLayoutBindingBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutBinding.html) · Builder of [`DescriptorSetLayoutBinding`]"]
#[repr(transparent)]
pub struct DescriptorSetLayoutBindingBuilder<'a>(DescriptorSetLayoutBinding, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorSetLayoutBindingBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetLayoutBindingBuilder<'a> {
        DescriptorSetLayoutBindingBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn binding(mut self, binding: u32) -> Self {
        self.0.binding = binding as _;
        self
    }
    #[inline]
    pub fn descriptor_type(mut self, descriptor_type: crate::vk1_0::DescriptorType) -> Self {
        self.0.descriptor_type = descriptor_type as _;
        self
    }
    #[inline]
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.0.descriptor_count = descriptor_count as _;
        self
    }
    #[inline]
    pub fn stage_flags(mut self, stage_flags: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.stage_flags = stage_flags as _;
        self
    }
    #[inline]
    pub fn immutable_samplers(mut self, immutable_samplers: &'a [crate::vk1_0::Sampler]) -> Self {
        self.0.p_immutable_samplers = immutable_samplers.as_ptr() as _;
        self.0.descriptor_count = immutable_samplers.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorSetLayoutBinding {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorSetLayoutBindingBuilder<'a> {
    fn default() -> DescriptorSetLayoutBindingBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorSetLayoutBindingBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorSetLayoutBindingBuilder<'a> {
    type Target = DescriptorSetLayoutBinding;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetLayoutBindingBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html) · Structure"]
#[doc(alias = "VkDescriptorSetLayoutCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const crate::vk1_0::DescriptorSetLayoutBinding,
}
impl Default for DescriptorSetLayoutCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), binding_count: Default::default(), p_bindings: std::ptr::null() }
    }
}
impl std::fmt::Debug for DescriptorSetLayoutCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorSetLayoutCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("binding_count", &self.binding_count).field("p_bindings", &self.p_bindings).finish()
    }
}
impl DescriptorSetLayoutCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorSetLayoutCreateInfoBuilder<'a> {
        DescriptorSetLayoutCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutCreateInfo.html) · Builder of [`DescriptorSetLayoutCreateInfo`]"]
#[repr(transparent)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'a>(DescriptorSetLayoutCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorSetLayoutCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetLayoutCreateInfoBuilder<'a> {
        DescriptorSetLayoutCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::DescriptorSetLayoutCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn bindings(mut self, bindings: &'a [crate::vk1_0::DescriptorSetLayoutBindingBuilder]) -> Self {
        self.0.p_bindings = bindings.as_ptr() as _;
        self.0.binding_count = bindings.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorSetLayoutCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorSetLayoutCreateInfoBuilder<'a> {
    fn default() -> DescriptorSetLayoutCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorSetLayoutCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorSetLayoutCreateInfoBuilder<'a> {
    type Target = DescriptorSetLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetLayoutCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolSize.html) · Structure"]
#[doc(alias = "VkDescriptorPoolSize")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorPoolSize {
    pub _type: crate::vk1_0::DescriptorType,
    pub descriptor_count: u32,
}
impl Default for DescriptorPoolSize {
    fn default() -> Self {
        Self { _type: Default::default(), descriptor_count: Default::default() }
    }
}
impl std::fmt::Debug for DescriptorPoolSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorPoolSize").field("_type", &self._type).field("descriptor_count", &self.descriptor_count).finish()
    }
}
impl DescriptorPoolSize {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorPoolSizeBuilder<'a> {
        DescriptorPoolSizeBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolSize.html) · Builder of [`DescriptorPoolSize`]"]
#[repr(transparent)]
pub struct DescriptorPoolSizeBuilder<'a>(DescriptorPoolSize, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorPoolSizeBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorPoolSizeBuilder<'a> {
        DescriptorPoolSizeBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn _type(mut self, _type: crate::vk1_0::DescriptorType) -> Self {
        self.0._type = _type as _;
        self
    }
    #[inline]
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.0.descriptor_count = descriptor_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorPoolSize {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorPoolSizeBuilder<'a> {
    fn default() -> DescriptorPoolSizeBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorPoolSizeBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorPoolSizeBuilder<'a> {
    type Target = DescriptorPoolSize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorPoolSizeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateInfo.html) · Structure"]
#[doc(alias = "VkDescriptorPoolCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const crate::vk1_0::DescriptorPoolSize,
}
impl Default for DescriptorPoolCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DESCRIPTOR_POOL_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), max_sets: Default::default(), pool_size_count: Default::default(), p_pool_sizes: std::ptr::null() }
    }
}
impl std::fmt::Debug for DescriptorPoolCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorPoolCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("max_sets", &self.max_sets).field("pool_size_count", &self.pool_size_count).field("p_pool_sizes", &self.p_pool_sizes).finish()
    }
}
impl DescriptorPoolCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorPoolCreateInfoBuilder<'a> {
        DescriptorPoolCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorPoolCreateInfo.html) · Builder of [`DescriptorPoolCreateInfo`]"]
#[repr(transparent)]
pub struct DescriptorPoolCreateInfoBuilder<'a>(DescriptorPoolCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorPoolCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorPoolCreateInfoBuilder<'a> {
        DescriptorPoolCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::DescriptorPoolCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn max_sets(mut self, max_sets: u32) -> Self {
        self.0.max_sets = max_sets as _;
        self
    }
    #[inline]
    pub fn pool_sizes(mut self, pool_sizes: &'a [crate::vk1_0::DescriptorPoolSizeBuilder]) -> Self {
        self.0.p_pool_sizes = pool_sizes.as_ptr() as _;
        self.0.pool_size_count = pool_sizes.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorPoolCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorPoolCreateInfoBuilder<'a> {
    fn default() -> DescriptorPoolCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorPoolCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorPoolCreateInfoBuilder<'a> {
    type Target = DescriptorPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorPoolCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetAllocateInfo.html) · Structure"]
#[doc(alias = "VkDescriptorSetAllocateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub descriptor_pool: crate::vk1_0::DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const crate::vk1_0::DescriptorSetLayout,
}
impl Default for DescriptorSetAllocateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO, p_next: std::ptr::null(), descriptor_pool: Default::default(), descriptor_set_count: Default::default(), p_set_layouts: std::ptr::null() }
    }
}
impl std::fmt::Debug for DescriptorSetAllocateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DescriptorSetAllocateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("descriptor_pool", &self.descriptor_pool).field("descriptor_set_count", &self.descriptor_set_count).field("p_set_layouts", &self.p_set_layouts).finish()
    }
}
impl DescriptorSetAllocateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> DescriptorSetAllocateInfoBuilder<'a> {
        DescriptorSetAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetAllocateInfo.html) · Builder of [`DescriptorSetAllocateInfo`]"]
#[repr(transparent)]
pub struct DescriptorSetAllocateInfoBuilder<'a>(DescriptorSetAllocateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> DescriptorSetAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> DescriptorSetAllocateInfoBuilder<'a> {
        DescriptorSetAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn descriptor_pool(mut self, descriptor_pool: crate::vk1_0::DescriptorPool) -> Self {
        self.0.descriptor_pool = descriptor_pool as _;
        self
    }
    #[inline]
    pub fn set_layouts(mut self, set_layouts: &'a [crate::vk1_0::DescriptorSetLayout]) -> Self {
        self.0.p_set_layouts = set_layouts.as_ptr() as _;
        self.0.descriptor_set_count = set_layouts.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DescriptorSetAllocateInfo {
        self.0
    }
}
impl<'a> std::default::Default for DescriptorSetAllocateInfoBuilder<'a> {
    fn default() -> DescriptorSetAllocateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DescriptorSetAllocateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DescriptorSetAllocateInfoBuilder<'a> {
    type Target = DescriptorSetAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DescriptorSetAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationMapEntry.html) · Structure"]
#[doc(alias = "VkSpecializationMapEntry")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}
impl Default for SpecializationMapEntry {
    fn default() -> Self {
        Self { constant_id: Default::default(), offset: Default::default(), size: Default::default() }
    }
}
impl std::fmt::Debug for SpecializationMapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SpecializationMapEntry").field("constant_id", &self.constant_id).field("offset", &self.offset).field("size", &self.size).finish()
    }
}
impl SpecializationMapEntry {
    #[inline]
    pub fn into_builder<'a>(self) -> SpecializationMapEntryBuilder<'a> {
        SpecializationMapEntryBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationMapEntry.html) · Builder of [`SpecializationMapEntry`]"]
#[repr(transparent)]
pub struct SpecializationMapEntryBuilder<'a>(SpecializationMapEntry, std::marker::PhantomData<&'a ()>);
impl<'a> SpecializationMapEntryBuilder<'a> {
    #[inline]
    pub fn new() -> SpecializationMapEntryBuilder<'a> {
        SpecializationMapEntryBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn constant_id(mut self, constant_id: u32) -> Self {
        self.0.constant_id = constant_id as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: usize) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SpecializationMapEntry {
        self.0
    }
}
impl<'a> std::default::Default for SpecializationMapEntryBuilder<'a> {
    fn default() -> SpecializationMapEntryBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SpecializationMapEntryBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SpecializationMapEntryBuilder<'a> {
    type Target = SpecializationMapEntry;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SpecializationMapEntryBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationInfo.html) · Structure"]
#[doc(alias = "VkSpecializationInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const crate::vk1_0::SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const std::ffi::c_void,
}
impl Default for SpecializationInfo {
    fn default() -> Self {
        Self { map_entry_count: Default::default(), p_map_entries: std::ptr::null(), data_size: Default::default(), p_data: std::ptr::null() }
    }
}
impl std::fmt::Debug for SpecializationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SpecializationInfo").field("map_entry_count", &self.map_entry_count).field("p_map_entries", &self.p_map_entries).field("data_size", &self.data_size).field("p_data", &self.p_data).finish()
    }
}
impl SpecializationInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SpecializationInfoBuilder<'a> {
        SpecializationInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSpecializationInfo.html) · Builder of [`SpecializationInfo`]"]
#[repr(transparent)]
pub struct SpecializationInfoBuilder<'a>(SpecializationInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SpecializationInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SpecializationInfoBuilder<'a> {
        SpecializationInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn map_entries(mut self, map_entries: &'a [crate::vk1_0::SpecializationMapEntryBuilder]) -> Self {
        self.0.p_map_entries = map_entries.as_ptr() as _;
        self.0.map_entry_count = map_entries.len() as _;
        self
    }
    #[inline]
    pub fn data_size(mut self, data_size: usize) -> Self {
        self.0.data_size = data_size;
        self
    }
    #[inline]
    pub fn data(mut self, data: *const std::ffi::c_void) -> Self {
        self.0.p_data = data;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SpecializationInfo {
        self.0
    }
}
impl<'a> std::default::Default for SpecializationInfoBuilder<'a> {
    fn default() -> SpecializationInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SpecializationInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SpecializationInfoBuilder<'a> {
    type Target = SpecializationInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SpecializationInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineShaderStageCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineShaderStageCreateFlags,
    pub stage: crate::vk1_0::ShaderStageFlagBits,
    pub module: crate::vk1_0::ShaderModule,
    pub p_name: *const std::os::raw::c_char,
    pub p_specialization_info: *const crate::vk1_0::SpecializationInfo,
}
impl Default for PipelineShaderStageCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), stage: Default::default(), module: Default::default(), p_name: std::ptr::null(), p_specialization_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineShaderStageCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineShaderStageCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("stage", &self.stage).field("module", &self.module).field("p_name", &self.p_name).field("p_specialization_info", &self.p_specialization_info).finish()
    }
}
impl PipelineShaderStageCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineShaderStageCreateInfoBuilder<'a> {
        PipelineShaderStageCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageCreateInfo.html) · Builder of [`PipelineShaderStageCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineShaderStageCreateInfoBuilder<'a>(PipelineShaderStageCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineShaderStageCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineShaderStageCreateInfoBuilder<'a> {
        PipelineShaderStageCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineShaderStageCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn stage(mut self, stage: crate::vk1_0::ShaderStageFlagBits) -> Self {
        self.0.stage = stage as _;
        self
    }
    #[inline]
    pub fn module(mut self, module: crate::vk1_0::ShaderModule) -> Self {
        self.0.module = module as _;
        self
    }
    #[inline]
    pub fn name(mut self, name: &'a std::ffi::CStr) -> Self {
        self.0.p_name = name.as_ptr();
        self
    }
    #[inline]
    pub fn specialization_info(mut self, specialization_info: &'a crate::vk1_0::SpecializationInfo) -> Self {
        self.0.p_specialization_info = specialization_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineShaderStageCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineShaderStageCreateInfoBuilder<'a> {
    fn default() -> PipelineShaderStageCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineShaderStageCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineShaderStageCreateInfoBuilder<'a> {
    type Target = PipelineShaderStageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineShaderStageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComputePipelineCreateInfo.html) · Structure"]
#[doc(alias = "VkComputePipelineCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ComputePipelineCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineCreateFlags,
    pub stage: crate::vk1_0::PipelineShaderStageCreateInfo,
    pub layout: crate::vk1_0::PipelineLayout,
    pub base_pipeline_handle: crate::vk1_0::Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for ComputePipelineCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COMPUTE_PIPELINE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), stage: Default::default(), layout: Default::default(), base_pipeline_handle: Default::default(), base_pipeline_index: Default::default() }
    }
}
impl std::fmt::Debug for ComputePipelineCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ComputePipelineCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("stage", &self.stage).field("layout", &self.layout).field("base_pipeline_handle", &self.base_pipeline_handle).field("base_pipeline_index", &self.base_pipeline_index).finish()
    }
}
impl ComputePipelineCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> ComputePipelineCreateInfoBuilder<'a> {
        ComputePipelineCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkComputePipelineCreateInfo.html) · Builder of [`ComputePipelineCreateInfo`]"]
#[repr(transparent)]
pub struct ComputePipelineCreateInfoBuilder<'a>(ComputePipelineCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> ComputePipelineCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> ComputePipelineCreateInfoBuilder<'a> {
        ComputePipelineCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn stage(mut self, stage: crate::vk1_0::PipelineShaderStageCreateInfo) -> Self {
        self.0.stage = stage as _;
        self
    }
    #[inline]
    pub fn layout(mut self, layout: crate::vk1_0::PipelineLayout) -> Self {
        self.0.layout = layout as _;
        self
    }
    #[inline]
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: crate::vk1_0::Pipeline) -> Self {
        self.0.base_pipeline_handle = base_pipeline_handle as _;
        self
    }
    #[inline]
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.0.base_pipeline_index = base_pipeline_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ComputePipelineCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for ComputePipelineCreateInfoBuilder<'a> {
    fn default() -> ComputePipelineCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ComputePipelineCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ComputePipelineCreateInfoBuilder<'a> {
    type Target = ComputePipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ComputePipelineCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDescription.html) · Structure"]
#[doc(alias = "VkVertexInputBindingDescription")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: crate::vk1_0::VertexInputRate,
}
impl Default for VertexInputBindingDescription {
    fn default() -> Self {
        Self { binding: Default::default(), stride: Default::default(), input_rate: Default::default() }
    }
}
impl std::fmt::Debug for VertexInputBindingDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VertexInputBindingDescription").field("binding", &self.binding).field("stride", &self.stride).field("input_rate", &self.input_rate).finish()
    }
}
impl VertexInputBindingDescription {
    #[inline]
    pub fn into_builder<'a>(self) -> VertexInputBindingDescriptionBuilder<'a> {
        VertexInputBindingDescriptionBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputBindingDescription.html) · Builder of [`VertexInputBindingDescription`]"]
#[repr(transparent)]
pub struct VertexInputBindingDescriptionBuilder<'a>(VertexInputBindingDescription, std::marker::PhantomData<&'a ()>);
impl<'a> VertexInputBindingDescriptionBuilder<'a> {
    #[inline]
    pub fn new() -> VertexInputBindingDescriptionBuilder<'a> {
        VertexInputBindingDescriptionBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn binding(mut self, binding: u32) -> Self {
        self.0.binding = binding as _;
        self
    }
    #[inline]
    pub fn stride(mut self, stride: u32) -> Self {
        self.0.stride = stride as _;
        self
    }
    #[inline]
    pub fn input_rate(mut self, input_rate: crate::vk1_0::VertexInputRate) -> Self {
        self.0.input_rate = input_rate as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VertexInputBindingDescription {
        self.0
    }
}
impl<'a> std::default::Default for VertexInputBindingDescriptionBuilder<'a> {
    fn default() -> VertexInputBindingDescriptionBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VertexInputBindingDescriptionBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VertexInputBindingDescriptionBuilder<'a> {
    type Target = VertexInputBindingDescription;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VertexInputBindingDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputAttributeDescription.html) · Structure"]
#[doc(alias = "VkVertexInputAttributeDescription")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: crate::vk1_0::Format,
    pub offset: u32,
}
impl Default for VertexInputAttributeDescription {
    fn default() -> Self {
        Self { location: Default::default(), binding: Default::default(), format: Default::default(), offset: Default::default() }
    }
}
impl std::fmt::Debug for VertexInputAttributeDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VertexInputAttributeDescription").field("location", &self.location).field("binding", &self.binding).field("format", &self.format).field("offset", &self.offset).finish()
    }
}
impl VertexInputAttributeDescription {
    #[inline]
    pub fn into_builder<'a>(self) -> VertexInputAttributeDescriptionBuilder<'a> {
        VertexInputAttributeDescriptionBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVertexInputAttributeDescription.html) · Builder of [`VertexInputAttributeDescription`]"]
#[repr(transparent)]
pub struct VertexInputAttributeDescriptionBuilder<'a>(VertexInputAttributeDescription, std::marker::PhantomData<&'a ()>);
impl<'a> VertexInputAttributeDescriptionBuilder<'a> {
    #[inline]
    pub fn new() -> VertexInputAttributeDescriptionBuilder<'a> {
        VertexInputAttributeDescriptionBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn location(mut self, location: u32) -> Self {
        self.0.location = location as _;
        self
    }
    #[inline]
    pub fn binding(mut self, binding: u32) -> Self {
        self.0.binding = binding as _;
        self
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> VertexInputAttributeDescription {
        self.0
    }
}
impl<'a> std::default::Default for VertexInputAttributeDescriptionBuilder<'a> {
    fn default() -> VertexInputAttributeDescriptionBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VertexInputAttributeDescriptionBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VertexInputAttributeDescriptionBuilder<'a> {
    type Target = VertexInputAttributeDescription;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VertexInputAttributeDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineVertexInputStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const crate::vk1_0::VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const crate::vk1_0::VertexInputAttributeDescription,
}
impl Default for PipelineVertexInputStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), vertex_binding_description_count: Default::default(), p_vertex_binding_descriptions: std::ptr::null(), vertex_attribute_description_count: Default::default(), p_vertex_attribute_descriptions: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineVertexInputStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineVertexInputStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("vertex_binding_description_count", &self.vertex_binding_description_count).field("p_vertex_binding_descriptions", &self.p_vertex_binding_descriptions).field("vertex_attribute_description_count", &self.vertex_attribute_description_count).field("p_vertex_attribute_descriptions", &self.p_vertex_attribute_descriptions).finish()
    }
}
impl PipelineVertexInputStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineVertexInputStateCreateInfoBuilder<'a> {
        PipelineVertexInputStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineVertexInputStateCreateInfo.html) · Builder of [`PipelineVertexInputStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineVertexInputStateCreateInfoBuilder<'a>(PipelineVertexInputStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineVertexInputStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineVertexInputStateCreateInfoBuilder<'a> {
        PipelineVertexInputStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineVertexInputStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn vertex_binding_descriptions(mut self, vertex_binding_descriptions: &'a [crate::vk1_0::VertexInputBindingDescriptionBuilder]) -> Self {
        self.0.p_vertex_binding_descriptions = vertex_binding_descriptions.as_ptr() as _;
        self.0.vertex_binding_description_count = vertex_binding_descriptions.len() as _;
        self
    }
    #[inline]
    pub fn vertex_attribute_descriptions(mut self, vertex_attribute_descriptions: &'a [crate::vk1_0::VertexInputAttributeDescriptionBuilder]) -> Self {
        self.0.p_vertex_attribute_descriptions = vertex_attribute_descriptions.as_ptr() as _;
        self.0.vertex_attribute_description_count = vertex_attribute_descriptions.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineVertexInputStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineVertexInputStateCreateInfoBuilder<'a> {
    fn default() -> PipelineVertexInputStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineVertexInputStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineVertexInputStateCreateInfoBuilder<'a> {
    type Target = PipelineVertexInputStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineVertexInputStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineInputAssemblyStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineInputAssemblyStateCreateFlags,
    pub topology: crate::vk1_0::PrimitiveTopology,
    pub primitive_restart_enable: crate::vk1_0::Bool32,
}
impl Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), topology: Default::default(), primitive_restart_enable: Default::default() }
    }
}
impl std::fmt::Debug for PipelineInputAssemblyStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineInputAssemblyStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("topology", &self.topology).field("primitive_restart_enable", &(self.primitive_restart_enable != 0)).finish()
    }
}
impl PipelineInputAssemblyStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineInputAssemblyStateCreateInfoBuilder<'a> {
        PipelineInputAssemblyStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineInputAssemblyStateCreateInfo.html) · Builder of [`PipelineInputAssemblyStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineInputAssemblyStateCreateInfoBuilder<'a>(PipelineInputAssemblyStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineInputAssemblyStateCreateInfoBuilder<'a> {
        PipelineInputAssemblyStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineInputAssemblyStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn topology(mut self, topology: crate::vk1_0::PrimitiveTopology) -> Self {
        self.0.topology = topology as _;
        self
    }
    #[inline]
    pub fn primitive_restart_enable(mut self, primitive_restart_enable: bool) -> Self {
        self.0.primitive_restart_enable = primitive_restart_enable as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineInputAssemblyStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    fn default() -> PipelineInputAssemblyStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    type Target = PipelineInputAssemblyStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineTessellationStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}
impl Default for PipelineTessellationStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), patch_control_points: Default::default() }
    }
}
impl std::fmt::Debug for PipelineTessellationStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineTessellationStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("patch_control_points", &self.patch_control_points).finish()
    }
}
impl PipelineTessellationStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineTessellationStateCreateInfoBuilder<'a> {
        PipelineTessellationStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineTessellationStateCreateInfo.html) · Builder of [`PipelineTessellationStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineTessellationStateCreateInfoBuilder<'a>(PipelineTessellationStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineTessellationStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineTessellationStateCreateInfoBuilder<'a> {
        PipelineTessellationStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineTessellationStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn patch_control_points(mut self, patch_control_points: u32) -> Self {
        self.0.patch_control_points = patch_control_points as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineTessellationStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineTessellationStateCreateInfoBuilder<'a> {
    fn default() -> PipelineTessellationStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineTessellationStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineTessellationStateCreateInfoBuilder<'a> {
    type Target = PipelineTessellationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineTessellationStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineViewportStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const crate::vk1_0::Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const crate::vk1_0::Rect2D,
}
impl Default for PipelineViewportStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), viewport_count: Default::default(), p_viewports: std::ptr::null(), scissor_count: Default::default(), p_scissors: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineViewportStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineViewportStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("viewport_count", &self.viewport_count).field("p_viewports", &self.p_viewports).field("scissor_count", &self.scissor_count).field("p_scissors", &self.p_scissors).finish()
    }
}
impl PipelineViewportStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineViewportStateCreateInfoBuilder<'a> {
        PipelineViewportStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineViewportStateCreateInfo.html) · Builder of [`PipelineViewportStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineViewportStateCreateInfoBuilder<'a>(PipelineViewportStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineViewportStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineViewportStateCreateInfoBuilder<'a> {
        PipelineViewportStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineViewportStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.0.viewport_count = viewport_count as _;
        self
    }
    #[inline]
    pub fn viewports(mut self, viewports: &'a [crate::vk1_0::ViewportBuilder]) -> Self {
        self.0.p_viewports = viewports.as_ptr() as _;
        self.0.viewport_count = viewports.len() as _;
        self
    }
    #[inline]
    pub fn scissor_count(mut self, scissor_count: u32) -> Self {
        self.0.scissor_count = scissor_count as _;
        self
    }
    #[inline]
    pub fn scissors(mut self, scissors: &'a [crate::vk1_0::Rect2DBuilder]) -> Self {
        self.0.p_scissors = scissors.as_ptr() as _;
        self.0.scissor_count = scissors.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineViewportStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineViewportStateCreateInfoBuilder<'a> {
    fn default() -> PipelineViewportStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineViewportStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineViewportStateCreateInfoBuilder<'a> {
    type Target = PipelineViewportStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineViewportStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineRasterizationStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: crate::vk1_0::Bool32,
    pub rasterizer_discard_enable: crate::vk1_0::Bool32,
    pub polygon_mode: crate::vk1_0::PolygonMode,
    pub cull_mode: crate::vk1_0::CullModeFlags,
    pub front_face: crate::vk1_0::FrontFace,
    pub depth_bias_enable: crate::vk1_0::Bool32,
    pub depth_bias_constant_factor: std::os::raw::c_float,
    pub depth_bias_clamp: std::os::raw::c_float,
    pub depth_bias_slope_factor: std::os::raw::c_float,
    pub line_width: std::os::raw::c_float,
}
impl Default for PipelineRasterizationStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), depth_clamp_enable: Default::default(), rasterizer_discard_enable: Default::default(), polygon_mode: Default::default(), cull_mode: Default::default(), front_face: Default::default(), depth_bias_enable: Default::default(), depth_bias_constant_factor: Default::default(), depth_bias_clamp: Default::default(), depth_bias_slope_factor: Default::default(), line_width: Default::default() }
    }
}
impl std::fmt::Debug for PipelineRasterizationStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineRasterizationStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("depth_clamp_enable", &(self.depth_clamp_enable != 0)).field("rasterizer_discard_enable", &(self.rasterizer_discard_enable != 0)).field("polygon_mode", &self.polygon_mode).field("cull_mode", &self.cull_mode).field("front_face", &self.front_face).field("depth_bias_enable", &(self.depth_bias_enable != 0)).field("depth_bias_constant_factor", &self.depth_bias_constant_factor).field("depth_bias_clamp", &self.depth_bias_clamp).field("depth_bias_slope_factor", &self.depth_bias_slope_factor).field("line_width", &self.line_width).finish()
    }
}
impl PipelineRasterizationStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineRasterizationStateCreateInfoBuilder<'a> {
        PipelineRasterizationStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationStateCreateInfo.html) · Builder of [`PipelineRasterizationStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineRasterizationStateCreateInfoBuilder<'a>(PipelineRasterizationStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineRasterizationStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationStateCreateInfoBuilder<'a> {
        PipelineRasterizationStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineRasterizationStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn depth_clamp_enable(mut self, depth_clamp_enable: bool) -> Self {
        self.0.depth_clamp_enable = depth_clamp_enable as _;
        self
    }
    #[inline]
    pub fn rasterizer_discard_enable(mut self, rasterizer_discard_enable: bool) -> Self {
        self.0.rasterizer_discard_enable = rasterizer_discard_enable as _;
        self
    }
    #[inline]
    pub fn polygon_mode(mut self, polygon_mode: crate::vk1_0::PolygonMode) -> Self {
        self.0.polygon_mode = polygon_mode as _;
        self
    }
    #[inline]
    pub fn cull_mode(mut self, cull_mode: crate::vk1_0::CullModeFlags) -> Self {
        self.0.cull_mode = cull_mode as _;
        self
    }
    #[inline]
    pub fn front_face(mut self, front_face: crate::vk1_0::FrontFace) -> Self {
        self.0.front_face = front_face as _;
        self
    }
    #[inline]
    pub fn depth_bias_enable(mut self, depth_bias_enable: bool) -> Self {
        self.0.depth_bias_enable = depth_bias_enable as _;
        self
    }
    #[inline]
    pub fn depth_bias_constant_factor(mut self, depth_bias_constant_factor: std::os::raw::c_float) -> Self {
        self.0.depth_bias_constant_factor = depth_bias_constant_factor as _;
        self
    }
    #[inline]
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: std::os::raw::c_float) -> Self {
        self.0.depth_bias_clamp = depth_bias_clamp as _;
        self
    }
    #[inline]
    pub fn depth_bias_slope_factor(mut self, depth_bias_slope_factor: std::os::raw::c_float) -> Self {
        self.0.depth_bias_slope_factor = depth_bias_slope_factor as _;
        self
    }
    #[inline]
    pub fn line_width(mut self, line_width: std::os::raw::c_float) -> Self {
        self.0.line_width = line_width as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineRasterizationStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineRasterizationStateCreateInfoBuilder<'a> {
    fn default() -> PipelineRasterizationStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationStateCreateInfoBuilder<'a> {
    type Target = PipelineRasterizationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineMultisampleStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: crate::vk1_0::SampleCountFlagBits,
    pub sample_shading_enable: crate::vk1_0::Bool32,
    pub min_sample_shading: std::os::raw::c_float,
    pub p_sample_mask: *const crate::vk1_0::SampleMask,
    pub alpha_to_coverage_enable: crate::vk1_0::Bool32,
    pub alpha_to_one_enable: crate::vk1_0::Bool32,
}
impl Default for PipelineMultisampleStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), rasterization_samples: Default::default(), sample_shading_enable: Default::default(), min_sample_shading: Default::default(), p_sample_mask: std::ptr::null(), alpha_to_coverage_enable: Default::default(), alpha_to_one_enable: Default::default() }
    }
}
impl std::fmt::Debug for PipelineMultisampleStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineMultisampleStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("rasterization_samples", &self.rasterization_samples).field("sample_shading_enable", &(self.sample_shading_enable != 0)).field("min_sample_shading", &self.min_sample_shading).field("p_sample_mask", &self.p_sample_mask).field("alpha_to_coverage_enable", &(self.alpha_to_coverage_enable != 0)).field("alpha_to_one_enable", &(self.alpha_to_one_enable != 0)).finish()
    }
}
impl PipelineMultisampleStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineMultisampleStateCreateInfoBuilder<'a> {
        PipelineMultisampleStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html) · Builder of [`PipelineMultisampleStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineMultisampleStateCreateInfoBuilder<'a>(PipelineMultisampleStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineMultisampleStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineMultisampleStateCreateInfoBuilder<'a> {
        PipelineMultisampleStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineMultisampleStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn rasterization_samples(mut self, rasterization_samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.rasterization_samples = rasterization_samples as _;
        self
    }
    #[inline]
    pub fn sample_shading_enable(mut self, sample_shading_enable: bool) -> Self {
        self.0.sample_shading_enable = sample_shading_enable as _;
        self
    }
    #[inline]
    pub fn min_sample_shading(mut self, min_sample_shading: std::os::raw::c_float) -> Self {
        self.0.min_sample_shading = min_sample_shading as _;
        self
    }
    #[inline]
    pub fn sample_mask(mut self, sample_mask: &'a [crate::vk1_0::SampleMask]) -> Self {
        self.0.p_sample_mask = if sample_mask.is_empty() { std::ptr::null() } else { sample_mask.as_ptr() as _ };
        self
    }
    #[inline]
    pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.0.alpha_to_coverage_enable = alpha_to_coverage_enable as _;
        self
    }
    #[inline]
    pub fn alpha_to_one_enable(mut self, alpha_to_one_enable: bool) -> Self {
        self.0.alpha_to_one_enable = alpha_to_one_enable as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineMultisampleStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineMultisampleStateCreateInfoBuilder<'a> {
    fn default() -> PipelineMultisampleStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineMultisampleStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineMultisampleStateCreateInfoBuilder<'a> {
    type Target = PipelineMultisampleStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineMultisampleStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAttachmentState.html) · Structure"]
#[doc(alias = "VkPipelineColorBlendAttachmentState")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: crate::vk1_0::Bool32,
    pub src_color_blend_factor: crate::vk1_0::BlendFactor,
    pub dst_color_blend_factor: crate::vk1_0::BlendFactor,
    pub color_blend_op: crate::vk1_0::BlendOp,
    pub src_alpha_blend_factor: crate::vk1_0::BlendFactor,
    pub dst_alpha_blend_factor: crate::vk1_0::BlendFactor,
    pub alpha_blend_op: crate::vk1_0::BlendOp,
    pub color_write_mask: crate::vk1_0::ColorComponentFlags,
}
impl Default for PipelineColorBlendAttachmentState {
    fn default() -> Self {
        Self { blend_enable: Default::default(), src_color_blend_factor: Default::default(), dst_color_blend_factor: Default::default(), color_blend_op: Default::default(), src_alpha_blend_factor: Default::default(), dst_alpha_blend_factor: Default::default(), alpha_blend_op: Default::default(), color_write_mask: Default::default() }
    }
}
impl std::fmt::Debug for PipelineColorBlendAttachmentState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineColorBlendAttachmentState").field("blend_enable", &(self.blend_enable != 0)).field("src_color_blend_factor", &self.src_color_blend_factor).field("dst_color_blend_factor", &self.dst_color_blend_factor).field("color_blend_op", &self.color_blend_op).field("src_alpha_blend_factor", &self.src_alpha_blend_factor).field("dst_alpha_blend_factor", &self.dst_alpha_blend_factor).field("alpha_blend_op", &self.alpha_blend_op).field("color_write_mask", &self.color_write_mask).finish()
    }
}
impl PipelineColorBlendAttachmentState {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineColorBlendAttachmentStateBuilder<'a> {
        PipelineColorBlendAttachmentStateBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendAttachmentState.html) · Builder of [`PipelineColorBlendAttachmentState`]"]
#[repr(transparent)]
pub struct PipelineColorBlendAttachmentStateBuilder<'a>(PipelineColorBlendAttachmentState, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineColorBlendAttachmentStateBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineColorBlendAttachmentStateBuilder<'a> {
        PipelineColorBlendAttachmentStateBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn blend_enable(mut self, blend_enable: bool) -> Self {
        self.0.blend_enable = blend_enable as _;
        self
    }
    #[inline]
    pub fn src_color_blend_factor(mut self, src_color_blend_factor: crate::vk1_0::BlendFactor) -> Self {
        self.0.src_color_blend_factor = src_color_blend_factor as _;
        self
    }
    #[inline]
    pub fn dst_color_blend_factor(mut self, dst_color_blend_factor: crate::vk1_0::BlendFactor) -> Self {
        self.0.dst_color_blend_factor = dst_color_blend_factor as _;
        self
    }
    #[inline]
    pub fn color_blend_op(mut self, color_blend_op: crate::vk1_0::BlendOp) -> Self {
        self.0.color_blend_op = color_blend_op as _;
        self
    }
    #[inline]
    pub fn src_alpha_blend_factor(mut self, src_alpha_blend_factor: crate::vk1_0::BlendFactor) -> Self {
        self.0.src_alpha_blend_factor = src_alpha_blend_factor as _;
        self
    }
    #[inline]
    pub fn dst_alpha_blend_factor(mut self, dst_alpha_blend_factor: crate::vk1_0::BlendFactor) -> Self {
        self.0.dst_alpha_blend_factor = dst_alpha_blend_factor as _;
        self
    }
    #[inline]
    pub fn alpha_blend_op(mut self, alpha_blend_op: crate::vk1_0::BlendOp) -> Self {
        self.0.alpha_blend_op = alpha_blend_op as _;
        self
    }
    #[inline]
    pub fn color_write_mask(mut self, color_write_mask: crate::vk1_0::ColorComponentFlags) -> Self {
        self.0.color_write_mask = color_write_mask as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineColorBlendAttachmentState {
        self.0
    }
}
impl<'a> std::default::Default for PipelineColorBlendAttachmentStateBuilder<'a> {
    fn default() -> PipelineColorBlendAttachmentStateBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineColorBlendAttachmentStateBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineColorBlendAttachmentStateBuilder<'a> {
    type Target = PipelineColorBlendAttachmentState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineColorBlendAttachmentStateBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineColorBlendStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: crate::vk1_0::Bool32,
    pub logic_op: crate::vk1_0::LogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const crate::vk1_0::PipelineColorBlendAttachmentState,
    pub blend_constants: [std::os::raw::c_float; 4],
}
impl Default for PipelineColorBlendStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), logic_op_enable: Default::default(), logic_op: Default::default(), attachment_count: Default::default(), p_attachments: std::ptr::null(), blend_constants: unsafe { std::mem::zeroed() } }
    }
}
impl std::fmt::Debug for PipelineColorBlendStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineColorBlendStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("logic_op_enable", &(self.logic_op_enable != 0)).field("logic_op", &self.logic_op).field("attachment_count", &self.attachment_count).field("p_attachments", &self.p_attachments).field("blend_constants", &self.blend_constants).finish()
    }
}
impl PipelineColorBlendStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineColorBlendStateCreateInfoBuilder<'a> {
        PipelineColorBlendStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineColorBlendStateCreateInfo.html) · Builder of [`PipelineColorBlendStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineColorBlendStateCreateInfoBuilder<'a>(PipelineColorBlendStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineColorBlendStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineColorBlendStateCreateInfoBuilder<'a> {
        PipelineColorBlendStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineColorBlendStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn logic_op_enable(mut self, logic_op_enable: bool) -> Self {
        self.0.logic_op_enable = logic_op_enable as _;
        self
    }
    #[inline]
    pub fn logic_op(mut self, logic_op: crate::vk1_0::LogicOp) -> Self {
        self.0.logic_op = logic_op as _;
        self
    }
    #[inline]
    pub fn attachments(mut self, attachments: &'a [crate::vk1_0::PipelineColorBlendAttachmentStateBuilder]) -> Self {
        self.0.p_attachments = attachments.as_ptr() as _;
        self.0.attachment_count = attachments.len() as _;
        self
    }
    #[inline]
    pub fn blend_constants(mut self, blend_constants: [std::os::raw::c_float; 4]) -> Self {
        self.0.blend_constants = blend_constants as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineColorBlendStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineColorBlendStateCreateInfoBuilder<'a> {
    fn default() -> PipelineColorBlendStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineColorBlendStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineColorBlendStateCreateInfoBuilder<'a> {
    type Target = PipelineColorBlendStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineColorBlendStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineDynamicStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const crate::vk1_0::DynamicState,
}
impl Default for PipelineDynamicStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), dynamic_state_count: Default::default(), p_dynamic_states: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineDynamicStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineDynamicStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("dynamic_state_count", &self.dynamic_state_count).field("p_dynamic_states", &self.p_dynamic_states).finish()
    }
}
impl PipelineDynamicStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineDynamicStateCreateInfoBuilder<'a> {
        PipelineDynamicStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDynamicStateCreateInfo.html) · Builder of [`PipelineDynamicStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineDynamicStateCreateInfoBuilder<'a>(PipelineDynamicStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineDynamicStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineDynamicStateCreateInfoBuilder<'a> {
        PipelineDynamicStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineDynamicStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn dynamic_states(mut self, dynamic_states: &'a [crate::vk1_0::DynamicState]) -> Self {
        self.0.p_dynamic_states = dynamic_states.as_ptr() as _;
        self.0.dynamic_state_count = dynamic_states.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineDynamicStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineDynamicStateCreateInfoBuilder<'a> {
    fn default() -> PipelineDynamicStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineDynamicStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineDynamicStateCreateInfoBuilder<'a> {
    type Target = PipelineDynamicStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineDynamicStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOpState.html) · Structure"]
#[doc(alias = "VkStencilOpState")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StencilOpState {
    pub fail_op: crate::vk1_0::StencilOp,
    pub pass_op: crate::vk1_0::StencilOp,
    pub depth_fail_op: crate::vk1_0::StencilOp,
    pub compare_op: crate::vk1_0::CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}
impl Default for StencilOpState {
    fn default() -> Self {
        Self { fail_op: Default::default(), pass_op: Default::default(), depth_fail_op: Default::default(), compare_op: Default::default(), compare_mask: Default::default(), write_mask: Default::default(), reference: Default::default() }
    }
}
impl std::fmt::Debug for StencilOpState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StencilOpState").field("fail_op", &self.fail_op).field("pass_op", &self.pass_op).field("depth_fail_op", &self.depth_fail_op).field("compare_op", &self.compare_op).field("compare_mask", &self.compare_mask).field("write_mask", &self.write_mask).field("reference", &self.reference).finish()
    }
}
impl StencilOpState {
    #[inline]
    pub fn into_builder<'a>(self) -> StencilOpStateBuilder<'a> {
        StencilOpStateBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkStencilOpState.html) · Builder of [`StencilOpState`]"]
#[repr(transparent)]
pub struct StencilOpStateBuilder<'a>(StencilOpState, std::marker::PhantomData<&'a ()>);
impl<'a> StencilOpStateBuilder<'a> {
    #[inline]
    pub fn new() -> StencilOpStateBuilder<'a> {
        StencilOpStateBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn fail_op(mut self, fail_op: crate::vk1_0::StencilOp) -> Self {
        self.0.fail_op = fail_op as _;
        self
    }
    #[inline]
    pub fn pass_op(mut self, pass_op: crate::vk1_0::StencilOp) -> Self {
        self.0.pass_op = pass_op as _;
        self
    }
    #[inline]
    pub fn depth_fail_op(mut self, depth_fail_op: crate::vk1_0::StencilOp) -> Self {
        self.0.depth_fail_op = depth_fail_op as _;
        self
    }
    #[inline]
    pub fn compare_op(mut self, compare_op: crate::vk1_0::CompareOp) -> Self {
        self.0.compare_op = compare_op as _;
        self
    }
    #[inline]
    pub fn compare_mask(mut self, compare_mask: u32) -> Self {
        self.0.compare_mask = compare_mask as _;
        self
    }
    #[inline]
    pub fn write_mask(mut self, write_mask: u32) -> Self {
        self.0.write_mask = write_mask as _;
        self
    }
    #[inline]
    pub fn reference(mut self, reference: u32) -> Self {
        self.0.reference = reference as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> StencilOpState {
        self.0
    }
}
impl<'a> std::default::Default for StencilOpStateBuilder<'a> {
    fn default() -> StencilOpStateBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for StencilOpStateBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for StencilOpStateBuilder<'a> {
    type Target = StencilOpState;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for StencilOpStateBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineDepthStencilStateCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: crate::vk1_0::Bool32,
    pub depth_write_enable: crate::vk1_0::Bool32,
    pub depth_compare_op: crate::vk1_0::CompareOp,
    pub depth_bounds_test_enable: crate::vk1_0::Bool32,
    pub stencil_test_enable: crate::vk1_0::Bool32,
    pub front: crate::vk1_0::StencilOpState,
    pub back: crate::vk1_0::StencilOpState,
    pub min_depth_bounds: std::os::raw::c_float,
    pub max_depth_bounds: std::os::raw::c_float,
}
impl Default for PipelineDepthStencilStateCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), depth_test_enable: Default::default(), depth_write_enable: Default::default(), depth_compare_op: Default::default(), depth_bounds_test_enable: Default::default(), stencil_test_enable: Default::default(), front: Default::default(), back: Default::default(), min_depth_bounds: Default::default(), max_depth_bounds: Default::default() }
    }
}
impl std::fmt::Debug for PipelineDepthStencilStateCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineDepthStencilStateCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("depth_test_enable", &(self.depth_test_enable != 0)).field("depth_write_enable", &(self.depth_write_enable != 0)).field("depth_compare_op", &self.depth_compare_op).field("depth_bounds_test_enable", &(self.depth_bounds_test_enable != 0)).field("stencil_test_enable", &(self.stencil_test_enable != 0)).field("front", &self.front).field("back", &self.back).field("min_depth_bounds", &self.min_depth_bounds).field("max_depth_bounds", &self.max_depth_bounds).finish()
    }
}
impl PipelineDepthStencilStateCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineDepthStencilStateCreateInfoBuilder<'a> {
        PipelineDepthStencilStateCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineDepthStencilStateCreateInfo.html) · Builder of [`PipelineDepthStencilStateCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineDepthStencilStateCreateInfoBuilder<'a>(PipelineDepthStencilStateCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineDepthStencilStateCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineDepthStencilStateCreateInfoBuilder<'a> {
        PipelineDepthStencilStateCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineDepthStencilStateCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn depth_test_enable(mut self, depth_test_enable: bool) -> Self {
        self.0.depth_test_enable = depth_test_enable as _;
        self
    }
    #[inline]
    pub fn depth_write_enable(mut self, depth_write_enable: bool) -> Self {
        self.0.depth_write_enable = depth_write_enable as _;
        self
    }
    #[inline]
    pub fn depth_compare_op(mut self, depth_compare_op: crate::vk1_0::CompareOp) -> Self {
        self.0.depth_compare_op = depth_compare_op as _;
        self
    }
    #[inline]
    pub fn depth_bounds_test_enable(mut self, depth_bounds_test_enable: bool) -> Self {
        self.0.depth_bounds_test_enable = depth_bounds_test_enable as _;
        self
    }
    #[inline]
    pub fn stencil_test_enable(mut self, stencil_test_enable: bool) -> Self {
        self.0.stencil_test_enable = stencil_test_enable as _;
        self
    }
    #[inline]
    pub fn front(mut self, front: crate::vk1_0::StencilOpState) -> Self {
        self.0.front = front as _;
        self
    }
    #[inline]
    pub fn back(mut self, back: crate::vk1_0::StencilOpState) -> Self {
        self.0.back = back as _;
        self
    }
    #[inline]
    pub fn min_depth_bounds(mut self, min_depth_bounds: std::os::raw::c_float) -> Self {
        self.0.min_depth_bounds = min_depth_bounds as _;
        self
    }
    #[inline]
    pub fn max_depth_bounds(mut self, max_depth_bounds: std::os::raw::c_float) -> Self {
        self.0.max_depth_bounds = max_depth_bounds as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineDepthStencilStateCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineDepthStencilStateCreateInfoBuilder<'a> {
    fn default() -> PipelineDepthStencilStateCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineDepthStencilStateCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineDepthStencilStateCreateInfoBuilder<'a> {
    type Target = PipelineDepthStencilStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineDepthStencilStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineCreateInfo.html) · Structure"]
#[doc(alias = "VkGraphicsPipelineCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const crate::vk1_0::PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const crate::vk1_0::PipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const crate::vk1_0::PipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const crate::vk1_0::PipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const crate::vk1_0::PipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const crate::vk1_0::PipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const crate::vk1_0::PipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const crate::vk1_0::PipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const crate::vk1_0::PipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const crate::vk1_0::PipelineDynamicStateCreateInfo,
    pub layout: crate::vk1_0::PipelineLayout,
    pub render_pass: crate::vk1_0::RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: crate::vk1_0::Pipeline,
    pub base_pipeline_index: i32,
}
impl Default for GraphicsPipelineCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::GRAPHICS_PIPELINE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), stage_count: Default::default(), p_stages: std::ptr::null(), p_vertex_input_state: std::ptr::null(), p_input_assembly_state: std::ptr::null(), p_tessellation_state: std::ptr::null(), p_viewport_state: std::ptr::null(), p_rasterization_state: std::ptr::null(), p_multisample_state: std::ptr::null(), p_depth_stencil_state: std::ptr::null(), p_color_blend_state: std::ptr::null(), p_dynamic_state: std::ptr::null(), layout: Default::default(), render_pass: Default::default(), subpass: Default::default(), base_pipeline_handle: Default::default(), base_pipeline_index: Default::default() }
    }
}
impl std::fmt::Debug for GraphicsPipelineCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GraphicsPipelineCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("stage_count", &self.stage_count).field("p_stages", &self.p_stages).field("p_vertex_input_state", &self.p_vertex_input_state).field("p_input_assembly_state", &self.p_input_assembly_state).field("p_tessellation_state", &self.p_tessellation_state).field("p_viewport_state", &self.p_viewport_state).field("p_rasterization_state", &self.p_rasterization_state).field("p_multisample_state", &self.p_multisample_state).field("p_depth_stencil_state", &self.p_depth_stencil_state).field("p_color_blend_state", &self.p_color_blend_state).field("p_dynamic_state", &self.p_dynamic_state).field("layout", &self.layout).field("render_pass", &self.render_pass).field("subpass", &self.subpass).field("base_pipeline_handle", &self.base_pipeline_handle).field("base_pipeline_index", &self.base_pipeline_index).finish()
    }
}
impl GraphicsPipelineCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> GraphicsPipelineCreateInfoBuilder<'a> {
        GraphicsPipelineCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkGraphicsPipelineCreateInfo.html) · Builder of [`GraphicsPipelineCreateInfo`]"]
#[repr(transparent)]
pub struct GraphicsPipelineCreateInfoBuilder<'a>(GraphicsPipelineCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> GraphicsPipelineCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> GraphicsPipelineCreateInfoBuilder<'a> {
        GraphicsPipelineCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn stages(mut self, stages: &'a [crate::vk1_0::PipelineShaderStageCreateInfoBuilder]) -> Self {
        self.0.p_stages = stages.as_ptr() as _;
        self.0.stage_count = stages.len() as _;
        self
    }
    #[inline]
    pub fn vertex_input_state(mut self, vertex_input_state: &'a crate::vk1_0::PipelineVertexInputStateCreateInfo) -> Self {
        self.0.p_vertex_input_state = vertex_input_state as _;
        self
    }
    #[inline]
    pub fn input_assembly_state(mut self, input_assembly_state: &'a crate::vk1_0::PipelineInputAssemblyStateCreateInfo) -> Self {
        self.0.p_input_assembly_state = input_assembly_state as _;
        self
    }
    #[inline]
    pub fn tessellation_state(mut self, tessellation_state: &'a crate::vk1_0::PipelineTessellationStateCreateInfo) -> Self {
        self.0.p_tessellation_state = tessellation_state as _;
        self
    }
    #[inline]
    pub fn viewport_state(mut self, viewport_state: &'a crate::vk1_0::PipelineViewportStateCreateInfo) -> Self {
        self.0.p_viewport_state = viewport_state as _;
        self
    }
    #[inline]
    pub fn rasterization_state(mut self, rasterization_state: &'a crate::vk1_0::PipelineRasterizationStateCreateInfo) -> Self {
        self.0.p_rasterization_state = rasterization_state as _;
        self
    }
    #[inline]
    pub fn multisample_state(mut self, multisample_state: &'a crate::vk1_0::PipelineMultisampleStateCreateInfo) -> Self {
        self.0.p_multisample_state = multisample_state as _;
        self
    }
    #[inline]
    pub fn depth_stencil_state(mut self, depth_stencil_state: &'a crate::vk1_0::PipelineDepthStencilStateCreateInfo) -> Self {
        self.0.p_depth_stencil_state = depth_stencil_state as _;
        self
    }
    #[inline]
    pub fn color_blend_state(mut self, color_blend_state: &'a crate::vk1_0::PipelineColorBlendStateCreateInfo) -> Self {
        self.0.p_color_blend_state = color_blend_state as _;
        self
    }
    #[inline]
    pub fn dynamic_state(mut self, dynamic_state: &'a crate::vk1_0::PipelineDynamicStateCreateInfo) -> Self {
        self.0.p_dynamic_state = dynamic_state as _;
        self
    }
    #[inline]
    pub fn layout(mut self, layout: crate::vk1_0::PipelineLayout) -> Self {
        self.0.layout = layout as _;
        self
    }
    #[inline]
    pub fn render_pass(mut self, render_pass: crate::vk1_0::RenderPass) -> Self {
        self.0.render_pass = render_pass as _;
        self
    }
    #[inline]
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.0.subpass = subpass as _;
        self
    }
    #[inline]
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: crate::vk1_0::Pipeline) -> Self {
        self.0.base_pipeline_handle = base_pipeline_handle as _;
        self
    }
    #[inline]
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.0.base_pipeline_index = base_pipeline_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> GraphicsPipelineCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for GraphicsPipelineCreateInfoBuilder<'a> {
    fn default() -> GraphicsPipelineCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for GraphicsPipelineCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for GraphicsPipelineCreateInfoBuilder<'a> {
    type Target = GraphicsPipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for GraphicsPipelineCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineCacheCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineCacheCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const std::ffi::c_void,
}
impl Default for PipelineCacheCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_CACHE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), initial_data_size: Default::default(), p_initial_data: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineCacheCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineCacheCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("initial_data_size", &self.initial_data_size).field("p_initial_data", &self.p_initial_data).finish()
    }
}
impl PipelineCacheCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineCacheCreateInfoBuilder<'a> {
        PipelineCacheCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineCacheCreateInfo.html) · Builder of [`PipelineCacheCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineCacheCreateInfoBuilder<'a>(PipelineCacheCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineCacheCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineCacheCreateInfoBuilder<'a> {
        PipelineCacheCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineCacheCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn initial_data_size(mut self, initial_data_size: usize) -> Self {
        self.0.initial_data_size = initial_data_size;
        self
    }
    #[inline]
    pub fn initial_data(mut self, initial_data: *const std::ffi::c_void) -> Self {
        self.0.p_initial_data = initial_data;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineCacheCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineCacheCreateInfoBuilder<'a> {
    fn default() -> PipelineCacheCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineCacheCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineCacheCreateInfoBuilder<'a> {
    type Target = PipelineCacheCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineCacheCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPushConstantRange.html) · Structure"]
#[doc(alias = "VkPushConstantRange")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PushConstantRange {
    pub stage_flags: crate::vk1_0::ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}
impl Default for PushConstantRange {
    fn default() -> Self {
        Self { stage_flags: Default::default(), offset: Default::default(), size: Default::default() }
    }
}
impl std::fmt::Debug for PushConstantRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PushConstantRange").field("stage_flags", &self.stage_flags).field("offset", &self.offset).field("size", &self.size).finish()
    }
}
impl PushConstantRange {
    #[inline]
    pub fn into_builder<'a>(self) -> PushConstantRangeBuilder<'a> {
        PushConstantRangeBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPushConstantRange.html) · Builder of [`PushConstantRange`]"]
#[repr(transparent)]
pub struct PushConstantRangeBuilder<'a>(PushConstantRange, std::marker::PhantomData<&'a ()>);
impl<'a> PushConstantRangeBuilder<'a> {
    #[inline]
    pub fn new() -> PushConstantRangeBuilder<'a> {
        PushConstantRangeBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn stage_flags(mut self, stage_flags: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.stage_flags = stage_flags as _;
        self
    }
    #[inline]
    pub fn offset(mut self, offset: u32) -> Self {
        self.0.offset = offset as _;
        self
    }
    #[inline]
    pub fn size(mut self, size: u32) -> Self {
        self.0.size = size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PushConstantRange {
        self.0
    }
}
impl<'a> std::default::Default for PushConstantRangeBuilder<'a> {
    fn default() -> PushConstantRangeBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PushConstantRangeBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PushConstantRangeBuilder<'a> {
    type Target = PushConstantRange;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PushConstantRangeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayoutCreateInfo.html) · Structure"]
#[doc(alias = "VkPipelineLayoutCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const crate::vk1_0::DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const crate::vk1_0::PushConstantRange,
}
impl Default for PipelineLayoutCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PIPELINE_LAYOUT_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), set_layout_count: Default::default(), p_set_layouts: std::ptr::null(), push_constant_range_count: Default::default(), p_push_constant_ranges: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineLayoutCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineLayoutCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("set_layout_count", &self.set_layout_count).field("p_set_layouts", &self.p_set_layouts).field("push_constant_range_count", &self.push_constant_range_count).field("p_push_constant_ranges", &self.p_push_constant_ranges).finish()
    }
}
impl PipelineLayoutCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineLayoutCreateInfoBuilder<'a> {
        PipelineLayoutCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLayoutCreateInfo.html) · Builder of [`PipelineLayoutCreateInfo`]"]
#[repr(transparent)]
pub struct PipelineLayoutCreateInfoBuilder<'a>(PipelineLayoutCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineLayoutCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineLayoutCreateInfoBuilder<'a> {
        PipelineLayoutCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::PipelineLayoutCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn set_layouts(mut self, set_layouts: &'a [crate::vk1_0::DescriptorSetLayout]) -> Self {
        self.0.p_set_layouts = set_layouts.as_ptr() as _;
        self.0.set_layout_count = set_layouts.len() as _;
        self
    }
    #[inline]
    pub fn push_constant_ranges(mut self, push_constant_ranges: &'a [crate::vk1_0::PushConstantRangeBuilder]) -> Self {
        self.0.p_push_constant_ranges = push_constant_ranges.as_ptr() as _;
        self.0.push_constant_range_count = push_constant_ranges.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineLayoutCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for PipelineLayoutCreateInfoBuilder<'a> {
    fn default() -> PipelineLayoutCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineLayoutCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineLayoutCreateInfoBuilder<'a> {
    type Target = PipelineLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineLayoutCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateInfo.html) · Structure"]
#[doc(alias = "VkSamplerCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SamplerCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::SamplerCreateFlags,
    pub mag_filter: crate::vk1_0::Filter,
    pub min_filter: crate::vk1_0::Filter,
    pub mipmap_mode: crate::vk1_0::SamplerMipmapMode,
    pub address_mode_u: crate::vk1_0::SamplerAddressMode,
    pub address_mode_v: crate::vk1_0::SamplerAddressMode,
    pub address_mode_w: crate::vk1_0::SamplerAddressMode,
    pub mip_lod_bias: std::os::raw::c_float,
    pub anisotropy_enable: crate::vk1_0::Bool32,
    pub max_anisotropy: std::os::raw::c_float,
    pub compare_enable: crate::vk1_0::Bool32,
    pub compare_op: crate::vk1_0::CompareOp,
    pub min_lod: std::os::raw::c_float,
    pub max_lod: std::os::raw::c_float,
    pub border_color: crate::vk1_0::BorderColor,
    pub unnormalized_coordinates: crate::vk1_0::Bool32,
}
impl Default for SamplerCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::SAMPLER_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), mag_filter: Default::default(), min_filter: Default::default(), mipmap_mode: Default::default(), address_mode_u: Default::default(), address_mode_v: Default::default(), address_mode_w: Default::default(), mip_lod_bias: Default::default(), anisotropy_enable: Default::default(), max_anisotropy: Default::default(), compare_enable: Default::default(), compare_op: Default::default(), min_lod: Default::default(), max_lod: Default::default(), border_color: Default::default(), unnormalized_coordinates: Default::default() }
    }
}
impl std::fmt::Debug for SamplerCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SamplerCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("mag_filter", &self.mag_filter).field("min_filter", &self.min_filter).field("mipmap_mode", &self.mipmap_mode).field("address_mode_u", &self.address_mode_u).field("address_mode_v", &self.address_mode_v).field("address_mode_w", &self.address_mode_w).field("mip_lod_bias", &self.mip_lod_bias).field("anisotropy_enable", &(self.anisotropy_enable != 0)).field("max_anisotropy", &self.max_anisotropy).field("compare_enable", &(self.compare_enable != 0)).field("compare_op", &self.compare_op).field("min_lod", &self.min_lod).field("max_lod", &self.max_lod).field("border_color", &self.border_color).field("unnormalized_coordinates", &(self.unnormalized_coordinates != 0)).finish()
    }
}
impl SamplerCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SamplerCreateInfoBuilder<'a> {
        SamplerCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSamplerCreateInfo.html) · Builder of [`SamplerCreateInfo`]"]
#[repr(transparent)]
pub struct SamplerCreateInfoBuilder<'a>(SamplerCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SamplerCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SamplerCreateInfoBuilder<'a> {
        SamplerCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::SamplerCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn mag_filter(mut self, mag_filter: crate::vk1_0::Filter) -> Self {
        self.0.mag_filter = mag_filter as _;
        self
    }
    #[inline]
    pub fn min_filter(mut self, min_filter: crate::vk1_0::Filter) -> Self {
        self.0.min_filter = min_filter as _;
        self
    }
    #[inline]
    pub fn mipmap_mode(mut self, mipmap_mode: crate::vk1_0::SamplerMipmapMode) -> Self {
        self.0.mipmap_mode = mipmap_mode as _;
        self
    }
    #[inline]
    pub fn address_mode_u(mut self, address_mode_u: crate::vk1_0::SamplerAddressMode) -> Self {
        self.0.address_mode_u = address_mode_u as _;
        self
    }
    #[inline]
    pub fn address_mode_v(mut self, address_mode_v: crate::vk1_0::SamplerAddressMode) -> Self {
        self.0.address_mode_v = address_mode_v as _;
        self
    }
    #[inline]
    pub fn address_mode_w(mut self, address_mode_w: crate::vk1_0::SamplerAddressMode) -> Self {
        self.0.address_mode_w = address_mode_w as _;
        self
    }
    #[inline]
    pub fn mip_lod_bias(mut self, mip_lod_bias: std::os::raw::c_float) -> Self {
        self.0.mip_lod_bias = mip_lod_bias as _;
        self
    }
    #[inline]
    pub fn anisotropy_enable(mut self, anisotropy_enable: bool) -> Self {
        self.0.anisotropy_enable = anisotropy_enable as _;
        self
    }
    #[inline]
    pub fn max_anisotropy(mut self, max_anisotropy: std::os::raw::c_float) -> Self {
        self.0.max_anisotropy = max_anisotropy as _;
        self
    }
    #[inline]
    pub fn compare_enable(mut self, compare_enable: bool) -> Self {
        self.0.compare_enable = compare_enable as _;
        self
    }
    #[inline]
    pub fn compare_op(mut self, compare_op: crate::vk1_0::CompareOp) -> Self {
        self.0.compare_op = compare_op as _;
        self
    }
    #[inline]
    pub fn min_lod(mut self, min_lod: std::os::raw::c_float) -> Self {
        self.0.min_lod = min_lod as _;
        self
    }
    #[inline]
    pub fn max_lod(mut self, max_lod: std::os::raw::c_float) -> Self {
        self.0.max_lod = max_lod as _;
        self
    }
    #[inline]
    pub fn border_color(mut self, border_color: crate::vk1_0::BorderColor) -> Self {
        self.0.border_color = border_color as _;
        self
    }
    #[inline]
    pub fn unnormalized_coordinates(mut self, unnormalized_coordinates: bool) -> Self {
        self.0.unnormalized_coordinates = unnormalized_coordinates as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SamplerCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for SamplerCreateInfoBuilder<'a> {
    fn default() -> SamplerCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SamplerCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SamplerCreateInfoBuilder<'a> {
    type Target = SamplerCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SamplerCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateInfo.html) · Structure"]
#[doc(alias = "VkCommandPoolCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandPoolCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::CommandPoolCreateFlags,
    pub queue_family_index: u32,
}
impl Default for CommandPoolCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COMMAND_POOL_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), queue_family_index: Default::default() }
    }
}
impl std::fmt::Debug for CommandPoolCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandPoolCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("queue_family_index", &self.queue_family_index).finish()
    }
}
impl CommandPoolCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandPoolCreateInfoBuilder<'a> {
        CommandPoolCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandPoolCreateInfo.html) · Builder of [`CommandPoolCreateInfo`]"]
#[repr(transparent)]
pub struct CommandPoolCreateInfoBuilder<'a>(CommandPoolCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> CommandPoolCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> CommandPoolCreateInfoBuilder<'a> {
        CommandPoolCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::CommandPoolCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.0.queue_family_index = queue_family_index as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandPoolCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for CommandPoolCreateInfoBuilder<'a> {
    fn default() -> CommandPoolCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandPoolCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandPoolCreateInfoBuilder<'a> {
    type Target = CommandPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandPoolCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferAllocateInfo.html) · Structure"]
#[doc(alias = "VkCommandBufferAllocateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub command_pool: crate::vk1_0::CommandPool,
    pub level: crate::vk1_0::CommandBufferLevel,
    pub command_buffer_count: u32,
}
impl Default for CommandBufferAllocateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COMMAND_BUFFER_ALLOCATE_INFO, p_next: std::ptr::null(), command_pool: Default::default(), level: Default::default(), command_buffer_count: Default::default() }
    }
}
impl std::fmt::Debug for CommandBufferAllocateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferAllocateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("command_pool", &self.command_pool).field("level", &self.level).field("command_buffer_count", &self.command_buffer_count).finish()
    }
}
impl CommandBufferAllocateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferAllocateInfoBuilder<'a> {
        CommandBufferAllocateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferAllocateInfo.html) · Builder of [`CommandBufferAllocateInfo`]"]
#[repr(transparent)]
pub struct CommandBufferAllocateInfoBuilder<'a>(CommandBufferAllocateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferAllocateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferAllocateInfoBuilder<'a> {
        CommandBufferAllocateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn command_pool(mut self, command_pool: crate::vk1_0::CommandPool) -> Self {
        self.0.command_pool = command_pool as _;
        self
    }
    #[inline]
    pub fn level(mut self, level: crate::vk1_0::CommandBufferLevel) -> Self {
        self.0.level = level as _;
        self
    }
    #[inline]
    pub fn command_buffer_count(mut self, command_buffer_count: u32) -> Self {
        self.0.command_buffer_count = command_buffer_count as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandBufferAllocateInfo {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferAllocateInfoBuilder<'a> {
    fn default() -> CommandBufferAllocateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferAllocateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferAllocateInfoBuilder<'a> {
    type Target = CommandBufferAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceInfo.html) · Structure"]
#[doc(alias = "VkCommandBufferInheritanceInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub render_pass: crate::vk1_0::RenderPass,
    pub subpass: u32,
    pub framebuffer: crate::vk1_0::Framebuffer,
    pub occlusion_query_enable: crate::vk1_0::Bool32,
    pub query_flags: crate::vk1_0::QueryControlFlags,
    pub pipeline_statistics: crate::vk1_0::QueryPipelineStatisticFlags,
}
impl Default for CommandBufferInheritanceInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COMMAND_BUFFER_INHERITANCE_INFO, p_next: std::ptr::null(), render_pass: Default::default(), subpass: Default::default(), framebuffer: Default::default(), occlusion_query_enable: Default::default(), query_flags: Default::default(), pipeline_statistics: Default::default() }
    }
}
impl std::fmt::Debug for CommandBufferInheritanceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferInheritanceInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("render_pass", &self.render_pass).field("subpass", &self.subpass).field("framebuffer", &self.framebuffer).field("occlusion_query_enable", &(self.occlusion_query_enable != 0)).field("query_flags", &self.query_flags).field("pipeline_statistics", &self.pipeline_statistics).finish()
    }
}
impl CommandBufferInheritanceInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferInheritanceInfoBuilder<'a> {
        CommandBufferInheritanceInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceInfo.html) · Builder of [`CommandBufferInheritanceInfo`]"]
#[repr(transparent)]
pub struct CommandBufferInheritanceInfoBuilder<'a>(CommandBufferInheritanceInfo, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferInheritanceInfoBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferInheritanceInfoBuilder<'a> {
        CommandBufferInheritanceInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn render_pass(mut self, render_pass: crate::vk1_0::RenderPass) -> Self {
        self.0.render_pass = render_pass as _;
        self
    }
    #[inline]
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.0.subpass = subpass as _;
        self
    }
    #[inline]
    pub fn framebuffer(mut self, framebuffer: crate::vk1_0::Framebuffer) -> Self {
        self.0.framebuffer = framebuffer as _;
        self
    }
    #[inline]
    pub fn occlusion_query_enable(mut self, occlusion_query_enable: bool) -> Self {
        self.0.occlusion_query_enable = occlusion_query_enable as _;
        self
    }
    #[inline]
    pub fn query_flags(mut self, query_flags: crate::vk1_0::QueryControlFlags) -> Self {
        self.0.query_flags = query_flags as _;
        self
    }
    #[inline]
    pub fn pipeline_statistics(mut self, pipeline_statistics: crate::vk1_0::QueryPipelineStatisticFlags) -> Self {
        self.0.pipeline_statistics = pipeline_statistics as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandBufferInheritanceInfo {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferInheritanceInfoBuilder<'a> {
    fn default() -> CommandBufferInheritanceInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferInheritanceInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferInheritanceInfoBuilder<'a> {
    type Target = CommandBufferInheritanceInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferInheritanceInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferBeginInfo.html) · Structure"]
#[doc(alias = "VkCommandBufferBeginInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::CommandBufferUsageFlags,
    pub p_inheritance_info: *const crate::vk1_0::CommandBufferInheritanceInfo,
}
impl Default for CommandBufferBeginInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::COMMAND_BUFFER_BEGIN_INFO, p_next: std::ptr::null(), flags: Default::default(), p_inheritance_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for CommandBufferBeginInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferBeginInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("p_inheritance_info", &self.p_inheritance_info).finish()
    }
}
impl CommandBufferBeginInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferBeginInfoBuilder<'a> {
        CommandBufferBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferBeginInfo.html) · Builder of [`CommandBufferBeginInfo`]"]
#[repr(transparent)]
pub struct CommandBufferBeginInfoBuilder<'a>(CommandBufferBeginInfo, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferBeginInfoBuilder<'a> {
        CommandBufferBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::CommandBufferUsageFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn inheritance_info(mut self, inheritance_info: &'a crate::vk1_0::CommandBufferInheritanceInfo) -> Self {
        self.0.p_inheritance_info = inheritance_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandBufferBeginInfo {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferBeginInfoBuilder<'a> {
    fn default() -> CommandBufferBeginInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferBeginInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferBeginInfoBuilder<'a> {
    type Target = CommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassBeginInfo.html) · Structure"]
#[doc(alias = "VkRenderPassBeginInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassBeginInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub render_pass: crate::vk1_0::RenderPass,
    pub framebuffer: crate::vk1_0::Framebuffer,
    pub render_area: crate::vk1_0::Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const crate::vk1_0::ClearValue,
}
impl Default for RenderPassBeginInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::RENDER_PASS_BEGIN_INFO, p_next: std::ptr::null(), render_pass: Default::default(), framebuffer: Default::default(), render_area: Default::default(), clear_value_count: Default::default(), p_clear_values: std::ptr::null() }
    }
}
impl std::fmt::Debug for RenderPassBeginInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderPassBeginInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("render_pass", &self.render_pass).field("framebuffer", &self.framebuffer).field("render_area", &self.render_area).field("clear_value_count", &self.clear_value_count).field("p_clear_values", &self.p_clear_values).finish()
    }
}
impl RenderPassBeginInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderPassBeginInfoBuilder<'a> {
        RenderPassBeginInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassBeginInfo.html) · Builder of [`RenderPassBeginInfo`]"]
#[repr(transparent)]
pub struct RenderPassBeginInfoBuilder<'a>(RenderPassBeginInfo, std::marker::PhantomData<&'a ()>);
impl<'a> RenderPassBeginInfoBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassBeginInfoBuilder<'a> {
        RenderPassBeginInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn render_pass(mut self, render_pass: crate::vk1_0::RenderPass) -> Self {
        self.0.render_pass = render_pass as _;
        self
    }
    #[inline]
    pub fn framebuffer(mut self, framebuffer: crate::vk1_0::Framebuffer) -> Self {
        self.0.framebuffer = framebuffer as _;
        self
    }
    #[inline]
    pub fn render_area(mut self, render_area: crate::vk1_0::Rect2D) -> Self {
        self.0.render_area = render_area as _;
        self
    }
    #[inline]
    pub fn clear_values(mut self, clear_values: &'a [crate::vk1_0::ClearValue]) -> Self {
        self.0.p_clear_values = clear_values.as_ptr() as _;
        self.0.clear_value_count = clear_values.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RenderPassBeginInfo {
        self.0
    }
}
impl<'a> std::default::Default for RenderPassBeginInfoBuilder<'a> {
    fn default() -> RenderPassBeginInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderPassBeginInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderPassBeginInfoBuilder<'a> {
    type Target = RenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearColorValue.html) · Structure"]
#[doc(alias = "VkClearColorValue")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClearColorValue {
    pub float32: [std::os::raw::c_float; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}
impl Default for ClearColorValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for ClearColorValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClearColorValue").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearDepthStencilValue.html) · Structure"]
#[doc(alias = "VkClearDepthStencilValue")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClearDepthStencilValue {
    pub depth: std::os::raw::c_float,
    pub stencil: u32,
}
impl Default for ClearDepthStencilValue {
    fn default() -> Self {
        Self { depth: Default::default(), stencil: Default::default() }
    }
}
impl std::fmt::Debug for ClearDepthStencilValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClearDepthStencilValue").field("depth", &self.depth).field("stencil", &self.stencil).finish()
    }
}
impl ClearDepthStencilValue {
    #[inline]
    pub fn into_builder<'a>(self) -> ClearDepthStencilValueBuilder<'a> {
        ClearDepthStencilValueBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearDepthStencilValue.html) · Builder of [`ClearDepthStencilValue`]"]
#[repr(transparent)]
pub struct ClearDepthStencilValueBuilder<'a>(ClearDepthStencilValue, std::marker::PhantomData<&'a ()>);
impl<'a> ClearDepthStencilValueBuilder<'a> {
    #[inline]
    pub fn new() -> ClearDepthStencilValueBuilder<'a> {
        ClearDepthStencilValueBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn depth(mut self, depth: std::os::raw::c_float) -> Self {
        self.0.depth = depth as _;
        self
    }
    #[inline]
    pub fn stencil(mut self, stencil: u32) -> Self {
        self.0.stencil = stencil as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ClearDepthStencilValue {
        self.0
    }
}
impl<'a> std::default::Default for ClearDepthStencilValueBuilder<'a> {
    fn default() -> ClearDepthStencilValueBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ClearDepthStencilValueBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ClearDepthStencilValueBuilder<'a> {
    type Target = ClearDepthStencilValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ClearDepthStencilValueBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearValue.html) · Structure"]
#[doc(alias = "VkClearValue")]
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClearValue {
    pub color: crate::vk1_0::ClearColorValue,
    pub depth_stencil: crate::vk1_0::ClearDepthStencilValue,
}
impl Default for ClearValue {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl std::fmt::Debug for ClearValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClearValue").finish()
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearAttachment.html) · Structure"]
#[doc(alias = "VkClearAttachment")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClearAttachment {
    pub aspect_mask: crate::vk1_0::ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: crate::vk1_0::ClearValue,
}
impl Default for ClearAttachment {
    fn default() -> Self {
        Self { aspect_mask: Default::default(), color_attachment: Default::default(), clear_value: Default::default() }
    }
}
impl std::fmt::Debug for ClearAttachment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClearAttachment").field("aspect_mask", &self.aspect_mask).field("color_attachment", &self.color_attachment).field("clear_value", &self.clear_value).finish()
    }
}
impl ClearAttachment {
    #[inline]
    pub fn into_builder<'a>(self) -> ClearAttachmentBuilder<'a> {
        ClearAttachmentBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkClearAttachment.html) · Builder of [`ClearAttachment`]"]
#[repr(transparent)]
pub struct ClearAttachmentBuilder<'a>(ClearAttachment, std::marker::PhantomData<&'a ()>);
impl<'a> ClearAttachmentBuilder<'a> {
    #[inline]
    pub fn new() -> ClearAttachmentBuilder<'a> {
        ClearAttachmentBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn aspect_mask(mut self, aspect_mask: crate::vk1_0::ImageAspectFlags) -> Self {
        self.0.aspect_mask = aspect_mask as _;
        self
    }
    #[inline]
    pub fn color_attachment(mut self, color_attachment: u32) -> Self {
        self.0.color_attachment = color_attachment as _;
        self
    }
    #[inline]
    pub fn clear_value(mut self, clear_value: crate::vk1_0::ClearValue) -> Self {
        self.0.clear_value = clear_value as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ClearAttachment {
        self.0
    }
}
impl<'a> std::default::Default for ClearAttachmentBuilder<'a> {
    fn default() -> ClearAttachmentBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ClearAttachmentBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ClearAttachmentBuilder<'a> {
    type Target = ClearAttachment;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ClearAttachmentBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription.html) · Structure"]
#[doc(alias = "VkAttachmentDescription")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentDescription {
    pub flags: crate::vk1_0::AttachmentDescriptionFlags,
    pub format: crate::vk1_0::Format,
    pub samples: crate::vk1_0::SampleCountFlagBits,
    pub load_op: crate::vk1_0::AttachmentLoadOp,
    pub store_op: crate::vk1_0::AttachmentStoreOp,
    pub stencil_load_op: crate::vk1_0::AttachmentLoadOp,
    pub stencil_store_op: crate::vk1_0::AttachmentStoreOp,
    pub initial_layout: crate::vk1_0::ImageLayout,
    pub final_layout: crate::vk1_0::ImageLayout,
}
impl Default for AttachmentDescription {
    fn default() -> Self {
        Self { flags: Default::default(), format: Default::default(), samples: Default::default(), load_op: Default::default(), store_op: Default::default(), stencil_load_op: Default::default(), stencil_store_op: Default::default(), initial_layout: Default::default(), final_layout: Default::default() }
    }
}
impl std::fmt::Debug for AttachmentDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AttachmentDescription").field("flags", &self.flags).field("format", &self.format).field("samples", &self.samples).field("load_op", &self.load_op).field("store_op", &self.store_op).field("stencil_load_op", &self.stencil_load_op).field("stencil_store_op", &self.stencil_store_op).field("initial_layout", &self.initial_layout).field("final_layout", &self.final_layout).finish()
    }
}
impl AttachmentDescription {
    #[inline]
    pub fn into_builder<'a>(self) -> AttachmentDescriptionBuilder<'a> {
        AttachmentDescriptionBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescription.html) · Builder of [`AttachmentDescription`]"]
#[repr(transparent)]
pub struct AttachmentDescriptionBuilder<'a>(AttachmentDescription, std::marker::PhantomData<&'a ()>);
impl<'a> AttachmentDescriptionBuilder<'a> {
    #[inline]
    pub fn new() -> AttachmentDescriptionBuilder<'a> {
        AttachmentDescriptionBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::AttachmentDescriptionFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn format(mut self, format: crate::vk1_0::Format) -> Self {
        self.0.format = format as _;
        self
    }
    #[inline]
    pub fn samples(mut self, samples: crate::vk1_0::SampleCountFlagBits) -> Self {
        self.0.samples = samples as _;
        self
    }
    #[inline]
    pub fn load_op(mut self, load_op: crate::vk1_0::AttachmentLoadOp) -> Self {
        self.0.load_op = load_op as _;
        self
    }
    #[inline]
    pub fn store_op(mut self, store_op: crate::vk1_0::AttachmentStoreOp) -> Self {
        self.0.store_op = store_op as _;
        self
    }
    #[inline]
    pub fn stencil_load_op(mut self, stencil_load_op: crate::vk1_0::AttachmentLoadOp) -> Self {
        self.0.stencil_load_op = stencil_load_op as _;
        self
    }
    #[inline]
    pub fn stencil_store_op(mut self, stencil_store_op: crate::vk1_0::AttachmentStoreOp) -> Self {
        self.0.stencil_store_op = stencil_store_op as _;
        self
    }
    #[inline]
    pub fn initial_layout(mut self, initial_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.initial_layout = initial_layout as _;
        self
    }
    #[inline]
    pub fn final_layout(mut self, final_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.final_layout = final_layout as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AttachmentDescription {
        self.0
    }
}
impl<'a> std::default::Default for AttachmentDescriptionBuilder<'a> {
    fn default() -> AttachmentDescriptionBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AttachmentDescriptionBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AttachmentDescriptionBuilder<'a> {
    type Target = AttachmentDescription;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference.html) · Structure"]
#[doc(alias = "VkAttachmentReference")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: crate::vk1_0::ImageLayout,
}
impl Default for AttachmentReference {
    fn default() -> Self {
        Self { attachment: Default::default(), layout: Default::default() }
    }
}
impl std::fmt::Debug for AttachmentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AttachmentReference").field("attachment", &self.attachment).field("layout", &self.layout).finish()
    }
}
impl AttachmentReference {
    #[inline]
    pub fn into_builder<'a>(self) -> AttachmentReferenceBuilder<'a> {
        AttachmentReferenceBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReference.html) · Builder of [`AttachmentReference`]"]
#[repr(transparent)]
pub struct AttachmentReferenceBuilder<'a>(AttachmentReference, std::marker::PhantomData<&'a ()>);
impl<'a> AttachmentReferenceBuilder<'a> {
    #[inline]
    pub fn new() -> AttachmentReferenceBuilder<'a> {
        AttachmentReferenceBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn attachment(mut self, attachment: u32) -> Self {
        self.0.attachment = attachment as _;
        self
    }
    #[inline]
    pub fn layout(mut self, layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.layout = layout as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AttachmentReference {
        self.0
    }
}
impl<'a> std::default::Default for AttachmentReferenceBuilder<'a> {
    fn default() -> AttachmentReferenceBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AttachmentReferenceBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AttachmentReferenceBuilder<'a> {
    type Target = AttachmentReference;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentReferenceBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription.html) · Structure"]
#[doc(alias = "VkSubpassDescription")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassDescription {
    pub flags: crate::vk1_0::SubpassDescriptionFlags,
    pub pipeline_bind_point: crate::vk1_0::PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const crate::vk1_0::AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const crate::vk1_0::AttachmentReference,
    pub p_resolve_attachments: *const crate::vk1_0::AttachmentReference,
    pub p_depth_stencil_attachment: *const crate::vk1_0::AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
impl Default for SubpassDescription {
    fn default() -> Self {
        Self { flags: Default::default(), pipeline_bind_point: Default::default(), input_attachment_count: Default::default(), p_input_attachments: std::ptr::null(), color_attachment_count: Default::default(), p_color_attachments: std::ptr::null(), p_resolve_attachments: std::ptr::null(), p_depth_stencil_attachment: std::ptr::null(), preserve_attachment_count: Default::default(), p_preserve_attachments: std::ptr::null() }
    }
}
impl std::fmt::Debug for SubpassDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubpassDescription").field("flags", &self.flags).field("pipeline_bind_point", &self.pipeline_bind_point).field("input_attachment_count", &self.input_attachment_count).field("p_input_attachments", &self.p_input_attachments).field("color_attachment_count", &self.color_attachment_count).field("p_color_attachments", &self.p_color_attachments).field("p_resolve_attachments", &self.p_resolve_attachments).field("p_depth_stencil_attachment", &self.p_depth_stencil_attachment).field("preserve_attachment_count", &self.preserve_attachment_count).field("p_preserve_attachments", &self.p_preserve_attachments).finish()
    }
}
impl SubpassDescription {
    #[inline]
    pub fn into_builder<'a>(self) -> SubpassDescriptionBuilder<'a> {
        SubpassDescriptionBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDescription.html) · Builder of [`SubpassDescription`]"]
#[repr(transparent)]
pub struct SubpassDescriptionBuilder<'a>(SubpassDescription, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassDescriptionBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassDescriptionBuilder<'a> {
        SubpassDescriptionBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::SubpassDescriptionFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: crate::vk1_0::PipelineBindPoint) -> Self {
        self.0.pipeline_bind_point = pipeline_bind_point as _;
        self
    }
    #[inline]
    pub fn input_attachments(mut self, input_attachments: &'a [crate::vk1_0::AttachmentReferenceBuilder]) -> Self {
        self.0.p_input_attachments = input_attachments.as_ptr() as _;
        self.0.input_attachment_count = input_attachments.len() as _;
        self
    }
    #[inline]
    pub fn color_attachments(mut self, color_attachments: &'a [crate::vk1_0::AttachmentReferenceBuilder]) -> Self {
        self.0.p_color_attachments = color_attachments.as_ptr() as _;
        self.0.color_attachment_count = color_attachments.len() as _;
        self
    }
    #[inline]
    pub fn resolve_attachments(mut self, resolve_attachments: &'a [crate::vk1_0::AttachmentReferenceBuilder]) -> Self {
        self.0.p_resolve_attachments = resolve_attachments.as_ptr() as _;
        self.0.color_attachment_count = resolve_attachments.len() as _;
        self
    }
    #[inline]
    pub fn depth_stencil_attachment(mut self, depth_stencil_attachment: &'a crate::vk1_0::AttachmentReference) -> Self {
        self.0.p_depth_stencil_attachment = depth_stencil_attachment as _;
        self
    }
    #[inline]
    pub fn preserve_attachments(mut self, preserve_attachments: &'a [u32]) -> Self {
        self.0.p_preserve_attachments = preserve_attachments.as_ptr() as _;
        self.0.preserve_attachment_count = preserve_attachments.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SubpassDescription {
        self.0
    }
}
impl<'a> std::default::Default for SubpassDescriptionBuilder<'a> {
    fn default() -> SubpassDescriptionBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubpassDescriptionBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubpassDescriptionBuilder<'a> {
    type Target = SubpassDescription;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency.html) · Structure"]
#[doc(alias = "VkSubpassDependency")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: crate::vk1_0::PipelineStageFlags,
    pub dst_stage_mask: crate::vk1_0::PipelineStageFlags,
    pub src_access_mask: crate::vk1_0::AccessFlags,
    pub dst_access_mask: crate::vk1_0::AccessFlags,
    pub dependency_flags: crate::vk1_0::DependencyFlags,
}
impl Default for SubpassDependency {
    fn default() -> Self {
        Self { src_subpass: Default::default(), dst_subpass: Default::default(), src_stage_mask: Default::default(), dst_stage_mask: Default::default(), src_access_mask: Default::default(), dst_access_mask: Default::default(), dependency_flags: Default::default() }
    }
}
impl std::fmt::Debug for SubpassDependency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubpassDependency").field("src_subpass", &self.src_subpass).field("dst_subpass", &self.dst_subpass).field("src_stage_mask", &self.src_stage_mask).field("dst_stage_mask", &self.dst_stage_mask).field("src_access_mask", &self.src_access_mask).field("dst_access_mask", &self.dst_access_mask).field("dependency_flags", &self.dependency_flags).finish()
    }
}
impl SubpassDependency {
    #[inline]
    pub fn into_builder<'a>(self) -> SubpassDependencyBuilder<'a> {
        SubpassDependencyBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassDependency.html) · Builder of [`SubpassDependency`]"]
#[repr(transparent)]
pub struct SubpassDependencyBuilder<'a>(SubpassDependency, std::marker::PhantomData<&'a ()>);
impl<'a> SubpassDependencyBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassDependencyBuilder<'a> {
        SubpassDependencyBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_subpass(mut self, src_subpass: u32) -> Self {
        self.0.src_subpass = src_subpass as _;
        self
    }
    #[inline]
    pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.0.dst_subpass = dst_subpass as _;
        self
    }
    #[inline]
    pub fn src_stage_mask(mut self, src_stage_mask: crate::vk1_0::PipelineStageFlags) -> Self {
        self.0.src_stage_mask = src_stage_mask as _;
        self
    }
    #[inline]
    pub fn dst_stage_mask(mut self, dst_stage_mask: crate::vk1_0::PipelineStageFlags) -> Self {
        self.0.dst_stage_mask = dst_stage_mask as _;
        self
    }
    #[inline]
    pub fn src_access_mask(mut self, src_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.src_access_mask = src_access_mask as _;
        self
    }
    #[inline]
    pub fn dst_access_mask(mut self, dst_access_mask: crate::vk1_0::AccessFlags) -> Self {
        self.0.dst_access_mask = dst_access_mask as _;
        self
    }
    #[inline]
    pub fn dependency_flags(mut self, dependency_flags: crate::vk1_0::DependencyFlags) -> Self {
        self.0.dependency_flags = dependency_flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SubpassDependency {
        self.0
    }
}
impl<'a> std::default::Default for SubpassDependencyBuilder<'a> {
    fn default() -> SubpassDependencyBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubpassDependencyBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubpassDependencyBuilder<'a> {
    type Target = SubpassDependency;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassDependencyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo.html) · Structure"]
#[doc(alias = "VkRenderPassCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const crate::vk1_0::AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const crate::vk1_0::SubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const crate::vk1_0::SubpassDependency,
}
impl Default for RenderPassCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::RENDER_PASS_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), attachment_count: Default::default(), p_attachments: std::ptr::null(), subpass_count: Default::default(), p_subpasses: std::ptr::null(), dependency_count: Default::default(), p_dependencies: std::ptr::null() }
    }
}
impl std::fmt::Debug for RenderPassCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderPassCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("attachment_count", &self.attachment_count).field("p_attachments", &self.p_attachments).field("subpass_count", &self.subpass_count).field("p_subpasses", &self.p_subpasses).field("dependency_count", &self.dependency_count).field("p_dependencies", &self.p_dependencies).finish()
    }
}
impl RenderPassCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderPassCreateInfoBuilder<'a> {
        RenderPassCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassCreateInfo.html) · Builder of [`RenderPassCreateInfo`]"]
#[repr(transparent)]
pub struct RenderPassCreateInfoBuilder<'a>(RenderPassCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> RenderPassCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassCreateInfoBuilder<'a> {
        RenderPassCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::RenderPassCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn attachments(mut self, attachments: &'a [crate::vk1_0::AttachmentDescriptionBuilder]) -> Self {
        self.0.p_attachments = attachments.as_ptr() as _;
        self.0.attachment_count = attachments.len() as _;
        self
    }
    #[inline]
    pub fn subpasses(mut self, subpasses: &'a [crate::vk1_0::SubpassDescriptionBuilder]) -> Self {
        self.0.p_subpasses = subpasses.as_ptr() as _;
        self.0.subpass_count = subpasses.len() as _;
        self
    }
    #[inline]
    pub fn dependencies(mut self, dependencies: &'a [crate::vk1_0::SubpassDependencyBuilder]) -> Self {
        self.0.p_dependencies = dependencies.as_ptr() as _;
        self.0.dependency_count = dependencies.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RenderPassCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for RenderPassCreateInfoBuilder<'a> {
    fn default() -> RenderPassCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderPassCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderPassCreateInfoBuilder<'a> {
    type Target = RenderPassCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEventCreateInfo.html) · Structure"]
#[doc(alias = "VkEventCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EventCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::EventCreateFlags,
}
impl Default for EventCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::EVENT_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default() }
    }
}
impl std::fmt::Debug for EventCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EventCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).finish()
    }
}
impl EventCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> EventCreateInfoBuilder<'a> {
        EventCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkEventCreateInfo.html) · Builder of [`EventCreateInfo`]"]
#[repr(transparent)]
pub struct EventCreateInfoBuilder<'a>(EventCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> EventCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> EventCreateInfoBuilder<'a> {
        EventCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::EventCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> EventCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for EventCreateInfoBuilder<'a> {
    fn default() -> EventCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for EventCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for EventCreateInfoBuilder<'a> {
    type Target = EventCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for EventCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceCreateInfo.html) · Structure"]
#[doc(alias = "VkFenceCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FenceCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::FenceCreateFlags,
}
impl Default for FenceCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::FENCE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default() }
    }
}
impl std::fmt::Debug for FenceCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FenceCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).finish()
    }
}
impl FenceCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> FenceCreateInfoBuilder<'a> {
        FenceCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceCreateInfo.html) · Builder of [`FenceCreateInfo`]"]
#[repr(transparent)]
pub struct FenceCreateInfoBuilder<'a>(FenceCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> FenceCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> FenceCreateInfoBuilder<'a> {
        FenceCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::FenceCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FenceCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for FenceCreateInfoBuilder<'a> {
    fn default() -> FenceCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FenceCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FenceCreateInfoBuilder<'a> {
    type Target = FenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FenceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceFeatures")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: crate::vk1_0::Bool32,
    pub full_draw_index_uint32: crate::vk1_0::Bool32,
    pub image_cube_array: crate::vk1_0::Bool32,
    pub independent_blend: crate::vk1_0::Bool32,
    pub geometry_shader: crate::vk1_0::Bool32,
    pub tessellation_shader: crate::vk1_0::Bool32,
    pub sample_rate_shading: crate::vk1_0::Bool32,
    pub dual_src_blend: crate::vk1_0::Bool32,
    pub logic_op: crate::vk1_0::Bool32,
    pub multi_draw_indirect: crate::vk1_0::Bool32,
    pub draw_indirect_first_instance: crate::vk1_0::Bool32,
    pub depth_clamp: crate::vk1_0::Bool32,
    pub depth_bias_clamp: crate::vk1_0::Bool32,
    pub fill_mode_non_solid: crate::vk1_0::Bool32,
    pub depth_bounds: crate::vk1_0::Bool32,
    pub wide_lines: crate::vk1_0::Bool32,
    pub large_points: crate::vk1_0::Bool32,
    pub alpha_to_one: crate::vk1_0::Bool32,
    pub multi_viewport: crate::vk1_0::Bool32,
    pub sampler_anisotropy: crate::vk1_0::Bool32,
    pub texture_compression_etc2: crate::vk1_0::Bool32,
    pub texture_compression_astc_ldr: crate::vk1_0::Bool32,
    pub texture_compression_bc: crate::vk1_0::Bool32,
    pub occlusion_query_precise: crate::vk1_0::Bool32,
    pub pipeline_statistics_query: crate::vk1_0::Bool32,
    pub vertex_pipeline_stores_and_atomics: crate::vk1_0::Bool32,
    pub fragment_stores_and_atomics: crate::vk1_0::Bool32,
    pub shader_tessellation_and_geometry_point_size: crate::vk1_0::Bool32,
    pub shader_image_gather_extended: crate::vk1_0::Bool32,
    pub shader_storage_image_extended_formats: crate::vk1_0::Bool32,
    pub shader_storage_image_multisample: crate::vk1_0::Bool32,
    pub shader_storage_image_read_without_format: crate::vk1_0::Bool32,
    pub shader_storage_image_write_without_format: crate::vk1_0::Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_sampled_image_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_storage_image_array_dynamic_indexing: crate::vk1_0::Bool32,
    pub shader_clip_distance: crate::vk1_0::Bool32,
    pub shader_cull_distance: crate::vk1_0::Bool32,
    pub shader_float64: crate::vk1_0::Bool32,
    pub shader_int64: crate::vk1_0::Bool32,
    pub shader_int16: crate::vk1_0::Bool32,
    pub shader_resource_residency: crate::vk1_0::Bool32,
    pub shader_resource_min_lod: crate::vk1_0::Bool32,
    pub sparse_binding: crate::vk1_0::Bool32,
    pub sparse_residency_buffer: crate::vk1_0::Bool32,
    pub sparse_residency_image2_d: crate::vk1_0::Bool32,
    pub sparse_residency_image3_d: crate::vk1_0::Bool32,
    pub sparse_residency2_samples: crate::vk1_0::Bool32,
    pub sparse_residency4_samples: crate::vk1_0::Bool32,
    pub sparse_residency8_samples: crate::vk1_0::Bool32,
    pub sparse_residency16_samples: crate::vk1_0::Bool32,
    pub sparse_residency_aliased: crate::vk1_0::Bool32,
    pub variable_multisample_rate: crate::vk1_0::Bool32,
    pub inherited_queries: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceFeatures {
    fn default() -> Self {
        Self {
            robust_buffer_access: Default::default(),
            full_draw_index_uint32: Default::default(),
            image_cube_array: Default::default(),
            independent_blend: Default::default(),
            geometry_shader: Default::default(),
            tessellation_shader: Default::default(),
            sample_rate_shading: Default::default(),
            dual_src_blend: Default::default(),
            logic_op: Default::default(),
            multi_draw_indirect: Default::default(),
            draw_indirect_first_instance: Default::default(),
            depth_clamp: Default::default(),
            depth_bias_clamp: Default::default(),
            fill_mode_non_solid: Default::default(),
            depth_bounds: Default::default(),
            wide_lines: Default::default(),
            large_points: Default::default(),
            alpha_to_one: Default::default(),
            multi_viewport: Default::default(),
            sampler_anisotropy: Default::default(),
            texture_compression_etc2: Default::default(),
            texture_compression_astc_ldr: Default::default(),
            texture_compression_bc: Default::default(),
            occlusion_query_precise: Default::default(),
            pipeline_statistics_query: Default::default(),
            vertex_pipeline_stores_and_atomics: Default::default(),
            fragment_stores_and_atomics: Default::default(),
            shader_tessellation_and_geometry_point_size: Default::default(),
            shader_image_gather_extended: Default::default(),
            shader_storage_image_extended_formats: Default::default(),
            shader_storage_image_multisample: Default::default(),
            shader_storage_image_read_without_format: Default::default(),
            shader_storage_image_write_without_format: Default::default(),
            shader_uniform_buffer_array_dynamic_indexing: Default::default(),
            shader_sampled_image_array_dynamic_indexing: Default::default(),
            shader_storage_buffer_array_dynamic_indexing: Default::default(),
            shader_storage_image_array_dynamic_indexing: Default::default(),
            shader_clip_distance: Default::default(),
            shader_cull_distance: Default::default(),
            shader_float64: Default::default(),
            shader_int64: Default::default(),
            shader_int16: Default::default(),
            shader_resource_residency: Default::default(),
            shader_resource_min_lod: Default::default(),
            sparse_binding: Default::default(),
            sparse_residency_buffer: Default::default(),
            sparse_residency_image2_d: Default::default(),
            sparse_residency_image3_d: Default::default(),
            sparse_residency2_samples: Default::default(),
            sparse_residency4_samples: Default::default(),
            sparse_residency8_samples: Default::default(),
            sparse_residency16_samples: Default::default(),
            sparse_residency_aliased: Default::default(),
            variable_multisample_rate: Default::default(),
            inherited_queries: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceFeatures").field("robust_buffer_access", &(self.robust_buffer_access != 0)).field("full_draw_index_uint32", &(self.full_draw_index_uint32 != 0)).field("image_cube_array", &(self.image_cube_array != 0)).field("independent_blend", &(self.independent_blend != 0)).field("geometry_shader", &(self.geometry_shader != 0)).field("tessellation_shader", &(self.tessellation_shader != 0)).field("sample_rate_shading", &(self.sample_rate_shading != 0)).field("dual_src_blend", &(self.dual_src_blend != 0)).field("logic_op", &(self.logic_op != 0)).field("multi_draw_indirect", &(self.multi_draw_indirect != 0)).field("draw_indirect_first_instance", &(self.draw_indirect_first_instance != 0)).field("depth_clamp", &(self.depth_clamp != 0)).field("depth_bias_clamp", &(self.depth_bias_clamp != 0)).field("fill_mode_non_solid", &(self.fill_mode_non_solid != 0)).field("depth_bounds", &(self.depth_bounds != 0)).field("wide_lines", &(self.wide_lines != 0)).field("large_points", &(self.large_points != 0)).field("alpha_to_one", &(self.alpha_to_one != 0)).field("multi_viewport", &(self.multi_viewport != 0)).field("sampler_anisotropy", &(self.sampler_anisotropy != 0)).field("texture_compression_etc2", &(self.texture_compression_etc2 != 0)).field("texture_compression_astc_ldr", &(self.texture_compression_astc_ldr != 0)).field("texture_compression_bc", &(self.texture_compression_bc != 0)).field("occlusion_query_precise", &(self.occlusion_query_precise != 0)).field("pipeline_statistics_query", &(self.pipeline_statistics_query != 0)).field("vertex_pipeline_stores_and_atomics", &(self.vertex_pipeline_stores_and_atomics != 0)).field("fragment_stores_and_atomics", &(self.fragment_stores_and_atomics != 0)).field("shader_tessellation_and_geometry_point_size", &(self.shader_tessellation_and_geometry_point_size != 0)).field("shader_image_gather_extended", &(self.shader_image_gather_extended != 0)).field("shader_storage_image_extended_formats", &(self.shader_storage_image_extended_formats != 0)).field("shader_storage_image_multisample", &(self.shader_storage_image_multisample != 0)).field("shader_storage_image_read_without_format", &(self.shader_storage_image_read_without_format != 0)).field("shader_storage_image_write_without_format", &(self.shader_storage_image_write_without_format != 0)).field("shader_uniform_buffer_array_dynamic_indexing", &(self.shader_uniform_buffer_array_dynamic_indexing != 0)).field("shader_sampled_image_array_dynamic_indexing", &(self.shader_sampled_image_array_dynamic_indexing != 0)).field("shader_storage_buffer_array_dynamic_indexing", &(self.shader_storage_buffer_array_dynamic_indexing != 0)).field("shader_storage_image_array_dynamic_indexing", &(self.shader_storage_image_array_dynamic_indexing != 0)).field("shader_clip_distance", &(self.shader_clip_distance != 0)).field("shader_cull_distance", &(self.shader_cull_distance != 0)).field("shader_float64", &(self.shader_float64 != 0)).field("shader_int64", &(self.shader_int64 != 0)).field("shader_int16", &(self.shader_int16 != 0)).field("shader_resource_residency", &(self.shader_resource_residency != 0)).field("shader_resource_min_lod", &(self.shader_resource_min_lod != 0)).field("sparse_binding", &(self.sparse_binding != 0)).field("sparse_residency_buffer", &(self.sparse_residency_buffer != 0)).field("sparse_residency_image2_d", &(self.sparse_residency_image2_d != 0)).field("sparse_residency_image3_d", &(self.sparse_residency_image3_d != 0)).field("sparse_residency2_samples", &(self.sparse_residency2_samples != 0)).field("sparse_residency4_samples", &(self.sparse_residency4_samples != 0)).field("sparse_residency8_samples", &(self.sparse_residency8_samples != 0)).field("sparse_residency16_samples", &(self.sparse_residency16_samples != 0)).field("sparse_residency_aliased", &(self.sparse_residency_aliased != 0)).field("variable_multisample_rate", &(self.variable_multisample_rate != 0)).field("inherited_queries", &(self.inherited_queries != 0)).finish()
    }
}
impl PhysicalDeviceFeatures {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceFeaturesBuilder<'a> {
        PhysicalDeviceFeaturesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceFeatures.html) · Builder of [`PhysicalDeviceFeatures`]"]
#[repr(transparent)]
pub struct PhysicalDeviceFeaturesBuilder<'a>(PhysicalDeviceFeatures, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceFeaturesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceFeaturesBuilder<'a> {
        PhysicalDeviceFeaturesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn robust_buffer_access(mut self, robust_buffer_access: bool) -> Self {
        self.0.robust_buffer_access = robust_buffer_access as _;
        self
    }
    #[inline]
    pub fn full_draw_index_uint32(mut self, full_draw_index_uint32: bool) -> Self {
        self.0.full_draw_index_uint32 = full_draw_index_uint32 as _;
        self
    }
    #[inline]
    pub fn image_cube_array(mut self, image_cube_array: bool) -> Self {
        self.0.image_cube_array = image_cube_array as _;
        self
    }
    #[inline]
    pub fn independent_blend(mut self, independent_blend: bool) -> Self {
        self.0.independent_blend = independent_blend as _;
        self
    }
    #[inline]
    pub fn geometry_shader(mut self, geometry_shader: bool) -> Self {
        self.0.geometry_shader = geometry_shader as _;
        self
    }
    #[inline]
    pub fn tessellation_shader(mut self, tessellation_shader: bool) -> Self {
        self.0.tessellation_shader = tessellation_shader as _;
        self
    }
    #[inline]
    pub fn sample_rate_shading(mut self, sample_rate_shading: bool) -> Self {
        self.0.sample_rate_shading = sample_rate_shading as _;
        self
    }
    #[inline]
    pub fn dual_src_blend(mut self, dual_src_blend: bool) -> Self {
        self.0.dual_src_blend = dual_src_blend as _;
        self
    }
    #[inline]
    pub fn logic_op(mut self, logic_op: bool) -> Self {
        self.0.logic_op = logic_op as _;
        self
    }
    #[inline]
    pub fn multi_draw_indirect(mut self, multi_draw_indirect: bool) -> Self {
        self.0.multi_draw_indirect = multi_draw_indirect as _;
        self
    }
    #[inline]
    pub fn draw_indirect_first_instance(mut self, draw_indirect_first_instance: bool) -> Self {
        self.0.draw_indirect_first_instance = draw_indirect_first_instance as _;
        self
    }
    #[inline]
    pub fn depth_clamp(mut self, depth_clamp: bool) -> Self {
        self.0.depth_clamp = depth_clamp as _;
        self
    }
    #[inline]
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: bool) -> Self {
        self.0.depth_bias_clamp = depth_bias_clamp as _;
        self
    }
    #[inline]
    pub fn fill_mode_non_solid(mut self, fill_mode_non_solid: bool) -> Self {
        self.0.fill_mode_non_solid = fill_mode_non_solid as _;
        self
    }
    #[inline]
    pub fn depth_bounds(mut self, depth_bounds: bool) -> Self {
        self.0.depth_bounds = depth_bounds as _;
        self
    }
    #[inline]
    pub fn wide_lines(mut self, wide_lines: bool) -> Self {
        self.0.wide_lines = wide_lines as _;
        self
    }
    #[inline]
    pub fn large_points(mut self, large_points: bool) -> Self {
        self.0.large_points = large_points as _;
        self
    }
    #[inline]
    pub fn alpha_to_one(mut self, alpha_to_one: bool) -> Self {
        self.0.alpha_to_one = alpha_to_one as _;
        self
    }
    #[inline]
    pub fn multi_viewport(mut self, multi_viewport: bool) -> Self {
        self.0.multi_viewport = multi_viewport as _;
        self
    }
    #[inline]
    pub fn sampler_anisotropy(mut self, sampler_anisotropy: bool) -> Self {
        self.0.sampler_anisotropy = sampler_anisotropy as _;
        self
    }
    #[inline]
    pub fn texture_compression_etc2(mut self, texture_compression_etc2: bool) -> Self {
        self.0.texture_compression_etc2 = texture_compression_etc2 as _;
        self
    }
    #[inline]
    pub fn texture_compression_astc_ldr(mut self, texture_compression_astc_ldr: bool) -> Self {
        self.0.texture_compression_astc_ldr = texture_compression_astc_ldr as _;
        self
    }
    #[inline]
    pub fn texture_compression_bc(mut self, texture_compression_bc: bool) -> Self {
        self.0.texture_compression_bc = texture_compression_bc as _;
        self
    }
    #[inline]
    pub fn occlusion_query_precise(mut self, occlusion_query_precise: bool) -> Self {
        self.0.occlusion_query_precise = occlusion_query_precise as _;
        self
    }
    #[inline]
    pub fn pipeline_statistics_query(mut self, pipeline_statistics_query: bool) -> Self {
        self.0.pipeline_statistics_query = pipeline_statistics_query as _;
        self
    }
    #[inline]
    pub fn vertex_pipeline_stores_and_atomics(mut self, vertex_pipeline_stores_and_atomics: bool) -> Self {
        self.0.vertex_pipeline_stores_and_atomics = vertex_pipeline_stores_and_atomics as _;
        self
    }
    #[inline]
    pub fn fragment_stores_and_atomics(mut self, fragment_stores_and_atomics: bool) -> Self {
        self.0.fragment_stores_and_atomics = fragment_stores_and_atomics as _;
        self
    }
    #[inline]
    pub fn shader_tessellation_and_geometry_point_size(mut self, shader_tessellation_and_geometry_point_size: bool) -> Self {
        self.0.shader_tessellation_and_geometry_point_size = shader_tessellation_and_geometry_point_size as _;
        self
    }
    #[inline]
    pub fn shader_image_gather_extended(mut self, shader_image_gather_extended: bool) -> Self {
        self.0.shader_image_gather_extended = shader_image_gather_extended as _;
        self
    }
    #[inline]
    pub fn shader_storage_image_extended_formats(mut self, shader_storage_image_extended_formats: bool) -> Self {
        self.0.shader_storage_image_extended_formats = shader_storage_image_extended_formats as _;
        self
    }
    #[inline]
    pub fn shader_storage_image_multisample(mut self, shader_storage_image_multisample: bool) -> Self {
        self.0.shader_storage_image_multisample = shader_storage_image_multisample as _;
        self
    }
    #[inline]
    pub fn shader_storage_image_read_without_format(mut self, shader_storage_image_read_without_format: bool) -> Self {
        self.0.shader_storage_image_read_without_format = shader_storage_image_read_without_format as _;
        self
    }
    #[inline]
    pub fn shader_storage_image_write_without_format(mut self, shader_storage_image_write_without_format: bool) -> Self {
        self.0.shader_storage_image_write_without_format = shader_storage_image_write_without_format as _;
        self
    }
    #[inline]
    pub fn shader_uniform_buffer_array_dynamic_indexing(mut self, shader_uniform_buffer_array_dynamic_indexing: bool) -> Self {
        self.0.shader_uniform_buffer_array_dynamic_indexing = shader_uniform_buffer_array_dynamic_indexing as _;
        self
    }
    #[inline]
    pub fn shader_sampled_image_array_dynamic_indexing(mut self, shader_sampled_image_array_dynamic_indexing: bool) -> Self {
        self.0.shader_sampled_image_array_dynamic_indexing = shader_sampled_image_array_dynamic_indexing as _;
        self
    }
    #[inline]
    pub fn shader_storage_buffer_array_dynamic_indexing(mut self, shader_storage_buffer_array_dynamic_indexing: bool) -> Self {
        self.0.shader_storage_buffer_array_dynamic_indexing = shader_storage_buffer_array_dynamic_indexing as _;
        self
    }
    #[inline]
    pub fn shader_storage_image_array_dynamic_indexing(mut self, shader_storage_image_array_dynamic_indexing: bool) -> Self {
        self.0.shader_storage_image_array_dynamic_indexing = shader_storage_image_array_dynamic_indexing as _;
        self
    }
    #[inline]
    pub fn shader_clip_distance(mut self, shader_clip_distance: bool) -> Self {
        self.0.shader_clip_distance = shader_clip_distance as _;
        self
    }
    #[inline]
    pub fn shader_cull_distance(mut self, shader_cull_distance: bool) -> Self {
        self.0.shader_cull_distance = shader_cull_distance as _;
        self
    }
    #[inline]
    pub fn shader_float64(mut self, shader_float64: bool) -> Self {
        self.0.shader_float64 = shader_float64 as _;
        self
    }
    #[inline]
    pub fn shader_int64(mut self, shader_int64: bool) -> Self {
        self.0.shader_int64 = shader_int64 as _;
        self
    }
    #[inline]
    pub fn shader_int16(mut self, shader_int16: bool) -> Self {
        self.0.shader_int16 = shader_int16 as _;
        self
    }
    #[inline]
    pub fn shader_resource_residency(mut self, shader_resource_residency: bool) -> Self {
        self.0.shader_resource_residency = shader_resource_residency as _;
        self
    }
    #[inline]
    pub fn shader_resource_min_lod(mut self, shader_resource_min_lod: bool) -> Self {
        self.0.shader_resource_min_lod = shader_resource_min_lod as _;
        self
    }
    #[inline]
    pub fn sparse_binding(mut self, sparse_binding: bool) -> Self {
        self.0.sparse_binding = sparse_binding as _;
        self
    }
    #[inline]
    pub fn sparse_residency_buffer(mut self, sparse_residency_buffer: bool) -> Self {
        self.0.sparse_residency_buffer = sparse_residency_buffer as _;
        self
    }
    #[inline]
    pub fn sparse_residency_image2_d(mut self, sparse_residency_image2_d: bool) -> Self {
        self.0.sparse_residency_image2_d = sparse_residency_image2_d as _;
        self
    }
    #[inline]
    pub fn sparse_residency_image3_d(mut self, sparse_residency_image3_d: bool) -> Self {
        self.0.sparse_residency_image3_d = sparse_residency_image3_d as _;
        self
    }
    #[inline]
    pub fn sparse_residency2_samples(mut self, sparse_residency2_samples: bool) -> Self {
        self.0.sparse_residency2_samples = sparse_residency2_samples as _;
        self
    }
    #[inline]
    pub fn sparse_residency4_samples(mut self, sparse_residency4_samples: bool) -> Self {
        self.0.sparse_residency4_samples = sparse_residency4_samples as _;
        self
    }
    #[inline]
    pub fn sparse_residency8_samples(mut self, sparse_residency8_samples: bool) -> Self {
        self.0.sparse_residency8_samples = sparse_residency8_samples as _;
        self
    }
    #[inline]
    pub fn sparse_residency16_samples(mut self, sparse_residency16_samples: bool) -> Self {
        self.0.sparse_residency16_samples = sparse_residency16_samples as _;
        self
    }
    #[inline]
    pub fn sparse_residency_aliased(mut self, sparse_residency_aliased: bool) -> Self {
        self.0.sparse_residency_aliased = sparse_residency_aliased as _;
        self
    }
    #[inline]
    pub fn variable_multisample_rate(mut self, variable_multisample_rate: bool) -> Self {
        self.0.variable_multisample_rate = variable_multisample_rate as _;
        self
    }
    #[inline]
    pub fn inherited_queries(mut self, inherited_queries: bool) -> Self {
        self.0.inherited_queries = inherited_queries as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceFeatures {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceFeaturesBuilder<'a> {
    fn default() -> PhysicalDeviceFeaturesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceFeaturesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceFeaturesBuilder<'a> {
    type Target = PhysicalDeviceFeatures;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseProperties.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSparseProperties")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard2_d_block_shape: crate::vk1_0::Bool32,
    pub residency_standard2_d_multisample_block_shape: crate::vk1_0::Bool32,
    pub residency_standard3_d_block_shape: crate::vk1_0::Bool32,
    pub residency_aligned_mip_size: crate::vk1_0::Bool32,
    pub residency_non_resident_strict: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceSparseProperties {
    fn default() -> Self {
        Self { residency_standard2_d_block_shape: Default::default(), residency_standard2_d_multisample_block_shape: Default::default(), residency_standard3_d_block_shape: Default::default(), residency_aligned_mip_size: Default::default(), residency_non_resident_strict: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceSparseProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSparseProperties").field("residency_standard2_d_block_shape", &(self.residency_standard2_d_block_shape != 0)).field("residency_standard2_d_multisample_block_shape", &(self.residency_standard2_d_multisample_block_shape != 0)).field("residency_standard3_d_block_shape", &(self.residency_standard3_d_block_shape != 0)).field("residency_aligned_mip_size", &(self.residency_aligned_mip_size != 0)).field("residency_non_resident_strict", &(self.residency_non_resident_strict != 0)).finish()
    }
}
impl PhysicalDeviceSparseProperties {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSparsePropertiesBuilder<'a> {
        PhysicalDeviceSparsePropertiesBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSparseProperties.html) · Builder of [`PhysicalDeviceSparseProperties`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSparsePropertiesBuilder<'a>(PhysicalDeviceSparseProperties, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSparsePropertiesBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSparsePropertiesBuilder<'a> {
        PhysicalDeviceSparsePropertiesBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn residency_standard2_d_block_shape(mut self, residency_standard2_d_block_shape: bool) -> Self {
        self.0.residency_standard2_d_block_shape = residency_standard2_d_block_shape as _;
        self
    }
    #[inline]
    pub fn residency_standard2_d_multisample_block_shape(mut self, residency_standard2_d_multisample_block_shape: bool) -> Self {
        self.0.residency_standard2_d_multisample_block_shape = residency_standard2_d_multisample_block_shape as _;
        self
    }
    #[inline]
    pub fn residency_standard3_d_block_shape(mut self, residency_standard3_d_block_shape: bool) -> Self {
        self.0.residency_standard3_d_block_shape = residency_standard3_d_block_shape as _;
        self
    }
    #[inline]
    pub fn residency_aligned_mip_size(mut self, residency_aligned_mip_size: bool) -> Self {
        self.0.residency_aligned_mip_size = residency_aligned_mip_size as _;
        self
    }
    #[inline]
    pub fn residency_non_resident_strict(mut self, residency_non_resident_strict: bool) -> Self {
        self.0.residency_non_resident_strict = residency_non_resident_strict as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSparseProperties {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSparsePropertiesBuilder<'a> {
    fn default() -> PhysicalDeviceSparsePropertiesBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSparsePropertiesBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSparsePropertiesBuilder<'a> {
    type Target = PhysicalDeviceSparseProperties;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSparsePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLimits.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceLimits")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension1_d: u32,
    pub max_image_dimension2_d: u32,
    pub max_image_dimension3_d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: crate::vk1_0::DeviceSize,
    pub sparse_address_space_size: crate::vk1_0::DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: std::os::raw::c_float,
    pub max_sampler_anisotropy: std::os::raw::c_float,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [std::os::raw::c_float; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: crate::vk1_0::DeviceSize,
    pub min_uniform_buffer_offset_alignment: crate::vk1_0::DeviceSize,
    pub min_storage_buffer_offset_alignment: crate::vk1_0::DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: std::os::raw::c_float,
    pub max_interpolation_offset: std::os::raw::c_float,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: crate::vk1_0::SampleCountFlags,
    pub framebuffer_depth_sample_counts: crate::vk1_0::SampleCountFlags,
    pub framebuffer_stencil_sample_counts: crate::vk1_0::SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: crate::vk1_0::SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: crate::vk1_0::SampleCountFlags,
    pub sampled_image_integer_sample_counts: crate::vk1_0::SampleCountFlags,
    pub sampled_image_depth_sample_counts: crate::vk1_0::SampleCountFlags,
    pub sampled_image_stencil_sample_counts: crate::vk1_0::SampleCountFlags,
    pub storage_image_sample_counts: crate::vk1_0::SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: crate::vk1_0::Bool32,
    pub timestamp_period: std::os::raw::c_float,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [std::os::raw::c_float; 2],
    pub line_width_range: [std::os::raw::c_float; 2],
    pub point_size_granularity: std::os::raw::c_float,
    pub line_width_granularity: std::os::raw::c_float,
    pub strict_lines: crate::vk1_0::Bool32,
    pub standard_sample_locations: crate::vk1_0::Bool32,
    pub optimal_buffer_copy_offset_alignment: crate::vk1_0::DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: crate::vk1_0::DeviceSize,
    pub non_coherent_atom_size: crate::vk1_0::DeviceSize,
}
impl Default for PhysicalDeviceLimits {
    fn default() -> Self {
        Self {
            max_image_dimension1_d: Default::default(),
            max_image_dimension2_d: Default::default(),
            max_image_dimension3_d: Default::default(),
            max_image_dimension_cube: Default::default(),
            max_image_array_layers: Default::default(),
            max_texel_buffer_elements: Default::default(),
            max_uniform_buffer_range: Default::default(),
            max_storage_buffer_range: Default::default(),
            max_push_constants_size: Default::default(),
            max_memory_allocation_count: Default::default(),
            max_sampler_allocation_count: Default::default(),
            buffer_image_granularity: Default::default(),
            sparse_address_space_size: Default::default(),
            max_bound_descriptor_sets: Default::default(),
            max_per_stage_descriptor_samplers: Default::default(),
            max_per_stage_descriptor_uniform_buffers: Default::default(),
            max_per_stage_descriptor_storage_buffers: Default::default(),
            max_per_stage_descriptor_sampled_images: Default::default(),
            max_per_stage_descriptor_storage_images: Default::default(),
            max_per_stage_descriptor_input_attachments: Default::default(),
            max_per_stage_resources: Default::default(),
            max_descriptor_set_samplers: Default::default(),
            max_descriptor_set_uniform_buffers: Default::default(),
            max_descriptor_set_uniform_buffers_dynamic: Default::default(),
            max_descriptor_set_storage_buffers: Default::default(),
            max_descriptor_set_storage_buffers_dynamic: Default::default(),
            max_descriptor_set_sampled_images: Default::default(),
            max_descriptor_set_storage_images: Default::default(),
            max_descriptor_set_input_attachments: Default::default(),
            max_vertex_input_attributes: Default::default(),
            max_vertex_input_bindings: Default::default(),
            max_vertex_input_attribute_offset: Default::default(),
            max_vertex_input_binding_stride: Default::default(),
            max_vertex_output_components: Default::default(),
            max_tessellation_generation_level: Default::default(),
            max_tessellation_patch_size: Default::default(),
            max_tessellation_control_per_vertex_input_components: Default::default(),
            max_tessellation_control_per_vertex_output_components: Default::default(),
            max_tessellation_control_per_patch_output_components: Default::default(),
            max_tessellation_control_total_output_components: Default::default(),
            max_tessellation_evaluation_input_components: Default::default(),
            max_tessellation_evaluation_output_components: Default::default(),
            max_geometry_shader_invocations: Default::default(),
            max_geometry_input_components: Default::default(),
            max_geometry_output_components: Default::default(),
            max_geometry_output_vertices: Default::default(),
            max_geometry_total_output_components: Default::default(),
            max_fragment_input_components: Default::default(),
            max_fragment_output_attachments: Default::default(),
            max_fragment_dual_src_attachments: Default::default(),
            max_fragment_combined_output_resources: Default::default(),
            max_compute_shared_memory_size: Default::default(),
            max_compute_work_group_count: unsafe { std::mem::zeroed() },
            max_compute_work_group_invocations: Default::default(),
            max_compute_work_group_size: unsafe { std::mem::zeroed() },
            sub_pixel_precision_bits: Default::default(),
            sub_texel_precision_bits: Default::default(),
            mipmap_precision_bits: Default::default(),
            max_draw_indexed_index_value: Default::default(),
            max_draw_indirect_count: Default::default(),
            max_sampler_lod_bias: Default::default(),
            max_sampler_anisotropy: Default::default(),
            max_viewports: Default::default(),
            max_viewport_dimensions: unsafe { std::mem::zeroed() },
            viewport_bounds_range: unsafe { std::mem::zeroed() },
            viewport_sub_pixel_bits: Default::default(),
            min_memory_map_alignment: Default::default(),
            min_texel_buffer_offset_alignment: Default::default(),
            min_uniform_buffer_offset_alignment: Default::default(),
            min_storage_buffer_offset_alignment: Default::default(),
            min_texel_offset: Default::default(),
            max_texel_offset: Default::default(),
            min_texel_gather_offset: Default::default(),
            max_texel_gather_offset: Default::default(),
            min_interpolation_offset: Default::default(),
            max_interpolation_offset: Default::default(),
            sub_pixel_interpolation_offset_bits: Default::default(),
            max_framebuffer_width: Default::default(),
            max_framebuffer_height: Default::default(),
            max_framebuffer_layers: Default::default(),
            framebuffer_color_sample_counts: Default::default(),
            framebuffer_depth_sample_counts: Default::default(),
            framebuffer_stencil_sample_counts: Default::default(),
            framebuffer_no_attachments_sample_counts: Default::default(),
            max_color_attachments: Default::default(),
            sampled_image_color_sample_counts: Default::default(),
            sampled_image_integer_sample_counts: Default::default(),
            sampled_image_depth_sample_counts: Default::default(),
            sampled_image_stencil_sample_counts: Default::default(),
            storage_image_sample_counts: Default::default(),
            max_sample_mask_words: Default::default(),
            timestamp_compute_and_graphics: Default::default(),
            timestamp_period: Default::default(),
            max_clip_distances: Default::default(),
            max_cull_distances: Default::default(),
            max_combined_clip_and_cull_distances: Default::default(),
            discrete_queue_priorities: Default::default(),
            point_size_range: unsafe { std::mem::zeroed() },
            line_width_range: unsafe { std::mem::zeroed() },
            point_size_granularity: Default::default(),
            line_width_granularity: Default::default(),
            strict_lines: Default::default(),
            standard_sample_locations: Default::default(),
            optimal_buffer_copy_offset_alignment: Default::default(),
            optimal_buffer_copy_row_pitch_alignment: Default::default(),
            non_coherent_atom_size: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceLimits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceLimits")
            .field("max_image_dimension1_d", &self.max_image_dimension1_d)
            .field("max_image_dimension2_d", &self.max_image_dimension2_d)
            .field("max_image_dimension3_d", &self.max_image_dimension3_d)
            .field("max_image_dimension_cube", &self.max_image_dimension_cube)
            .field("max_image_array_layers", &self.max_image_array_layers)
            .field("max_texel_buffer_elements", &self.max_texel_buffer_elements)
            .field("max_uniform_buffer_range", &self.max_uniform_buffer_range)
            .field("max_storage_buffer_range", &self.max_storage_buffer_range)
            .field("max_push_constants_size", &self.max_push_constants_size)
            .field("max_memory_allocation_count", &self.max_memory_allocation_count)
            .field("max_sampler_allocation_count", &self.max_sampler_allocation_count)
            .field("buffer_image_granularity", &self.buffer_image_granularity)
            .field("sparse_address_space_size", &self.sparse_address_space_size)
            .field("max_bound_descriptor_sets", &self.max_bound_descriptor_sets)
            .field("max_per_stage_descriptor_samplers", &self.max_per_stage_descriptor_samplers)
            .field("max_per_stage_descriptor_uniform_buffers", &self.max_per_stage_descriptor_uniform_buffers)
            .field("max_per_stage_descriptor_storage_buffers", &self.max_per_stage_descriptor_storage_buffers)
            .field("max_per_stage_descriptor_sampled_images", &self.max_per_stage_descriptor_sampled_images)
            .field("max_per_stage_descriptor_storage_images", &self.max_per_stage_descriptor_storage_images)
            .field("max_per_stage_descriptor_input_attachments", &self.max_per_stage_descriptor_input_attachments)
            .field("max_per_stage_resources", &self.max_per_stage_resources)
            .field("max_descriptor_set_samplers", &self.max_descriptor_set_samplers)
            .field("max_descriptor_set_uniform_buffers", &self.max_descriptor_set_uniform_buffers)
            .field("max_descriptor_set_uniform_buffers_dynamic", &self.max_descriptor_set_uniform_buffers_dynamic)
            .field("max_descriptor_set_storage_buffers", &self.max_descriptor_set_storage_buffers)
            .field("max_descriptor_set_storage_buffers_dynamic", &self.max_descriptor_set_storage_buffers_dynamic)
            .field("max_descriptor_set_sampled_images", &self.max_descriptor_set_sampled_images)
            .field("max_descriptor_set_storage_images", &self.max_descriptor_set_storage_images)
            .field("max_descriptor_set_input_attachments", &self.max_descriptor_set_input_attachments)
            .field("max_vertex_input_attributes", &self.max_vertex_input_attributes)
            .field("max_vertex_input_bindings", &self.max_vertex_input_bindings)
            .field("max_vertex_input_attribute_offset", &self.max_vertex_input_attribute_offset)
            .field("max_vertex_input_binding_stride", &self.max_vertex_input_binding_stride)
            .field("max_vertex_output_components", &self.max_vertex_output_components)
            .field("max_tessellation_generation_level", &self.max_tessellation_generation_level)
            .field("max_tessellation_patch_size", &self.max_tessellation_patch_size)
            .field("max_tessellation_control_per_vertex_input_components", &self.max_tessellation_control_per_vertex_input_components)
            .field("max_tessellation_control_per_vertex_output_components", &self.max_tessellation_control_per_vertex_output_components)
            .field("max_tessellation_control_per_patch_output_components", &self.max_tessellation_control_per_patch_output_components)
            .field("max_tessellation_control_total_output_components", &self.max_tessellation_control_total_output_components)
            .field("max_tessellation_evaluation_input_components", &self.max_tessellation_evaluation_input_components)
            .field("max_tessellation_evaluation_output_components", &self.max_tessellation_evaluation_output_components)
            .field("max_geometry_shader_invocations", &self.max_geometry_shader_invocations)
            .field("max_geometry_input_components", &self.max_geometry_input_components)
            .field("max_geometry_output_components", &self.max_geometry_output_components)
            .field("max_geometry_output_vertices", &self.max_geometry_output_vertices)
            .field("max_geometry_total_output_components", &self.max_geometry_total_output_components)
            .field("max_fragment_input_components", &self.max_fragment_input_components)
            .field("max_fragment_output_attachments", &self.max_fragment_output_attachments)
            .field("max_fragment_dual_src_attachments", &self.max_fragment_dual_src_attachments)
            .field("max_fragment_combined_output_resources", &self.max_fragment_combined_output_resources)
            .field("max_compute_shared_memory_size", &self.max_compute_shared_memory_size)
            .field("max_compute_work_group_count", &self.max_compute_work_group_count)
            .field("max_compute_work_group_invocations", &self.max_compute_work_group_invocations)
            .field("max_compute_work_group_size", &self.max_compute_work_group_size)
            .field("sub_pixel_precision_bits", &self.sub_pixel_precision_bits)
            .field("sub_texel_precision_bits", &self.sub_texel_precision_bits)
            .field("mipmap_precision_bits", &self.mipmap_precision_bits)
            .field("max_draw_indexed_index_value", &self.max_draw_indexed_index_value)
            .field("max_draw_indirect_count", &self.max_draw_indirect_count)
            .field("max_sampler_lod_bias", &self.max_sampler_lod_bias)
            .field("max_sampler_anisotropy", &self.max_sampler_anisotropy)
            .field("max_viewports", &self.max_viewports)
            .field("max_viewport_dimensions", &self.max_viewport_dimensions)
            .field("viewport_bounds_range", &self.viewport_bounds_range)
            .field("viewport_sub_pixel_bits", &self.viewport_sub_pixel_bits)
            .field("min_memory_map_alignment", &self.min_memory_map_alignment)
            .field("min_texel_buffer_offset_alignment", &self.min_texel_buffer_offset_alignment)
            .field("min_uniform_buffer_offset_alignment", &self.min_uniform_buffer_offset_alignment)
            .field("min_storage_buffer_offset_alignment", &self.min_storage_buffer_offset_alignment)
            .field("min_texel_offset", &self.min_texel_offset)
            .field("max_texel_offset", &self.max_texel_offset)
            .field("min_texel_gather_offset", &self.min_texel_gather_offset)
            .field("max_texel_gather_offset", &self.max_texel_gather_offset)
            .field("min_interpolation_offset", &self.min_interpolation_offset)
            .field("max_interpolation_offset", &self.max_interpolation_offset)
            .field("sub_pixel_interpolation_offset_bits", &self.sub_pixel_interpolation_offset_bits)
            .field("max_framebuffer_width", &self.max_framebuffer_width)
            .field("max_framebuffer_height", &self.max_framebuffer_height)
            .field("max_framebuffer_layers", &self.max_framebuffer_layers)
            .field("framebuffer_color_sample_counts", &self.framebuffer_color_sample_counts)
            .field("framebuffer_depth_sample_counts", &self.framebuffer_depth_sample_counts)
            .field("framebuffer_stencil_sample_counts", &self.framebuffer_stencil_sample_counts)
            .field("framebuffer_no_attachments_sample_counts", &self.framebuffer_no_attachments_sample_counts)
            .field("max_color_attachments", &self.max_color_attachments)
            .field("sampled_image_color_sample_counts", &self.sampled_image_color_sample_counts)
            .field("sampled_image_integer_sample_counts", &self.sampled_image_integer_sample_counts)
            .field("sampled_image_depth_sample_counts", &self.sampled_image_depth_sample_counts)
            .field("sampled_image_stencil_sample_counts", &self.sampled_image_stencil_sample_counts)
            .field("storage_image_sample_counts", &self.storage_image_sample_counts)
            .field("max_sample_mask_words", &self.max_sample_mask_words)
            .field("timestamp_compute_and_graphics", &(self.timestamp_compute_and_graphics != 0))
            .field("timestamp_period", &self.timestamp_period)
            .field("max_clip_distances", &self.max_clip_distances)
            .field("max_cull_distances", &self.max_cull_distances)
            .field("max_combined_clip_and_cull_distances", &self.max_combined_clip_and_cull_distances)
            .field("discrete_queue_priorities", &self.discrete_queue_priorities)
            .field("point_size_range", &self.point_size_range)
            .field("line_width_range", &self.line_width_range)
            .field("point_size_granularity", &self.point_size_granularity)
            .field("line_width_granularity", &self.line_width_granularity)
            .field("strict_lines", &(self.strict_lines != 0))
            .field("standard_sample_locations", &(self.standard_sample_locations != 0))
            .field("optimal_buffer_copy_offset_alignment", &self.optimal_buffer_copy_offset_alignment)
            .field("optimal_buffer_copy_row_pitch_alignment", &self.optimal_buffer_copy_row_pitch_alignment)
            .field("non_coherent_atom_size", &self.non_coherent_atom_size)
            .finish()
    }
}
impl PhysicalDeviceLimits {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceLimitsBuilder<'a> {
        PhysicalDeviceLimitsBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceLimits.html) · Builder of [`PhysicalDeviceLimits`]"]
#[repr(transparent)]
pub struct PhysicalDeviceLimitsBuilder<'a>(PhysicalDeviceLimits, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceLimitsBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceLimitsBuilder<'a> {
        PhysicalDeviceLimitsBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_image_dimension1_d(mut self, max_image_dimension1_d: u32) -> Self {
        self.0.max_image_dimension1_d = max_image_dimension1_d as _;
        self
    }
    #[inline]
    pub fn max_image_dimension2_d(mut self, max_image_dimension2_d: u32) -> Self {
        self.0.max_image_dimension2_d = max_image_dimension2_d as _;
        self
    }
    #[inline]
    pub fn max_image_dimension3_d(mut self, max_image_dimension3_d: u32) -> Self {
        self.0.max_image_dimension3_d = max_image_dimension3_d as _;
        self
    }
    #[inline]
    pub fn max_image_dimension_cube(mut self, max_image_dimension_cube: u32) -> Self {
        self.0.max_image_dimension_cube = max_image_dimension_cube as _;
        self
    }
    #[inline]
    pub fn max_image_array_layers(mut self, max_image_array_layers: u32) -> Self {
        self.0.max_image_array_layers = max_image_array_layers as _;
        self
    }
    #[inline]
    pub fn max_texel_buffer_elements(mut self, max_texel_buffer_elements: u32) -> Self {
        self.0.max_texel_buffer_elements = max_texel_buffer_elements as _;
        self
    }
    #[inline]
    pub fn max_uniform_buffer_range(mut self, max_uniform_buffer_range: u32) -> Self {
        self.0.max_uniform_buffer_range = max_uniform_buffer_range as _;
        self
    }
    #[inline]
    pub fn max_storage_buffer_range(mut self, max_storage_buffer_range: u32) -> Self {
        self.0.max_storage_buffer_range = max_storage_buffer_range as _;
        self
    }
    #[inline]
    pub fn max_push_constants_size(mut self, max_push_constants_size: u32) -> Self {
        self.0.max_push_constants_size = max_push_constants_size as _;
        self
    }
    #[inline]
    pub fn max_memory_allocation_count(mut self, max_memory_allocation_count: u32) -> Self {
        self.0.max_memory_allocation_count = max_memory_allocation_count as _;
        self
    }
    #[inline]
    pub fn max_sampler_allocation_count(mut self, max_sampler_allocation_count: u32) -> Self {
        self.0.max_sampler_allocation_count = max_sampler_allocation_count as _;
        self
    }
    #[inline]
    pub fn buffer_image_granularity(mut self, buffer_image_granularity: crate::vk1_0::DeviceSize) -> Self {
        self.0.buffer_image_granularity = buffer_image_granularity as _;
        self
    }
    #[inline]
    pub fn sparse_address_space_size(mut self, sparse_address_space_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.sparse_address_space_size = sparse_address_space_size as _;
        self
    }
    #[inline]
    pub fn max_bound_descriptor_sets(mut self, max_bound_descriptor_sets: u32) -> Self {
        self.0.max_bound_descriptor_sets = max_bound_descriptor_sets as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_samplers(mut self, max_per_stage_descriptor_samplers: u32) -> Self {
        self.0.max_per_stage_descriptor_samplers = max_per_stage_descriptor_samplers as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_uniform_buffers(mut self, max_per_stage_descriptor_uniform_buffers: u32) -> Self {
        self.0.max_per_stage_descriptor_uniform_buffers = max_per_stage_descriptor_uniform_buffers as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_storage_buffers(mut self, max_per_stage_descriptor_storage_buffers: u32) -> Self {
        self.0.max_per_stage_descriptor_storage_buffers = max_per_stage_descriptor_storage_buffers as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_sampled_images(mut self, max_per_stage_descriptor_sampled_images: u32) -> Self {
        self.0.max_per_stage_descriptor_sampled_images = max_per_stage_descriptor_sampled_images as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_storage_images(mut self, max_per_stage_descriptor_storage_images: u32) -> Self {
        self.0.max_per_stage_descriptor_storage_images = max_per_stage_descriptor_storage_images as _;
        self
    }
    #[inline]
    pub fn max_per_stage_descriptor_input_attachments(mut self, max_per_stage_descriptor_input_attachments: u32) -> Self {
        self.0.max_per_stage_descriptor_input_attachments = max_per_stage_descriptor_input_attachments as _;
        self
    }
    #[inline]
    pub fn max_per_stage_resources(mut self, max_per_stage_resources: u32) -> Self {
        self.0.max_per_stage_resources = max_per_stage_resources as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_samplers(mut self, max_descriptor_set_samplers: u32) -> Self {
        self.0.max_descriptor_set_samplers = max_descriptor_set_samplers as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_uniform_buffers(mut self, max_descriptor_set_uniform_buffers: u32) -> Self {
        self.0.max_descriptor_set_uniform_buffers = max_descriptor_set_uniform_buffers as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_uniform_buffers_dynamic(mut self, max_descriptor_set_uniform_buffers_dynamic: u32) -> Self {
        self.0.max_descriptor_set_uniform_buffers_dynamic = max_descriptor_set_uniform_buffers_dynamic as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_storage_buffers(mut self, max_descriptor_set_storage_buffers: u32) -> Self {
        self.0.max_descriptor_set_storage_buffers = max_descriptor_set_storage_buffers as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_storage_buffers_dynamic(mut self, max_descriptor_set_storage_buffers_dynamic: u32) -> Self {
        self.0.max_descriptor_set_storage_buffers_dynamic = max_descriptor_set_storage_buffers_dynamic as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_sampled_images(mut self, max_descriptor_set_sampled_images: u32) -> Self {
        self.0.max_descriptor_set_sampled_images = max_descriptor_set_sampled_images as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_storage_images(mut self, max_descriptor_set_storage_images: u32) -> Self {
        self.0.max_descriptor_set_storage_images = max_descriptor_set_storage_images as _;
        self
    }
    #[inline]
    pub fn max_descriptor_set_input_attachments(mut self, max_descriptor_set_input_attachments: u32) -> Self {
        self.0.max_descriptor_set_input_attachments = max_descriptor_set_input_attachments as _;
        self
    }
    #[inline]
    pub fn max_vertex_input_attributes(mut self, max_vertex_input_attributes: u32) -> Self {
        self.0.max_vertex_input_attributes = max_vertex_input_attributes as _;
        self
    }
    #[inline]
    pub fn max_vertex_input_bindings(mut self, max_vertex_input_bindings: u32) -> Self {
        self.0.max_vertex_input_bindings = max_vertex_input_bindings as _;
        self
    }
    #[inline]
    pub fn max_vertex_input_attribute_offset(mut self, max_vertex_input_attribute_offset: u32) -> Self {
        self.0.max_vertex_input_attribute_offset = max_vertex_input_attribute_offset as _;
        self
    }
    #[inline]
    pub fn max_vertex_input_binding_stride(mut self, max_vertex_input_binding_stride: u32) -> Self {
        self.0.max_vertex_input_binding_stride = max_vertex_input_binding_stride as _;
        self
    }
    #[inline]
    pub fn max_vertex_output_components(mut self, max_vertex_output_components: u32) -> Self {
        self.0.max_vertex_output_components = max_vertex_output_components as _;
        self
    }
    #[inline]
    pub fn max_tessellation_generation_level(mut self, max_tessellation_generation_level: u32) -> Self {
        self.0.max_tessellation_generation_level = max_tessellation_generation_level as _;
        self
    }
    #[inline]
    pub fn max_tessellation_patch_size(mut self, max_tessellation_patch_size: u32) -> Self {
        self.0.max_tessellation_patch_size = max_tessellation_patch_size as _;
        self
    }
    #[inline]
    pub fn max_tessellation_control_per_vertex_input_components(mut self, max_tessellation_control_per_vertex_input_components: u32) -> Self {
        self.0.max_tessellation_control_per_vertex_input_components = max_tessellation_control_per_vertex_input_components as _;
        self
    }
    #[inline]
    pub fn max_tessellation_control_per_vertex_output_components(mut self, max_tessellation_control_per_vertex_output_components: u32) -> Self {
        self.0.max_tessellation_control_per_vertex_output_components = max_tessellation_control_per_vertex_output_components as _;
        self
    }
    #[inline]
    pub fn max_tessellation_control_per_patch_output_components(mut self, max_tessellation_control_per_patch_output_components: u32) -> Self {
        self.0.max_tessellation_control_per_patch_output_components = max_tessellation_control_per_patch_output_components as _;
        self
    }
    #[inline]
    pub fn max_tessellation_control_total_output_components(mut self, max_tessellation_control_total_output_components: u32) -> Self {
        self.0.max_tessellation_control_total_output_components = max_tessellation_control_total_output_components as _;
        self
    }
    #[inline]
    pub fn max_tessellation_evaluation_input_components(mut self, max_tessellation_evaluation_input_components: u32) -> Self {
        self.0.max_tessellation_evaluation_input_components = max_tessellation_evaluation_input_components as _;
        self
    }
    #[inline]
    pub fn max_tessellation_evaluation_output_components(mut self, max_tessellation_evaluation_output_components: u32) -> Self {
        self.0.max_tessellation_evaluation_output_components = max_tessellation_evaluation_output_components as _;
        self
    }
    #[inline]
    pub fn max_geometry_shader_invocations(mut self, max_geometry_shader_invocations: u32) -> Self {
        self.0.max_geometry_shader_invocations = max_geometry_shader_invocations as _;
        self
    }
    #[inline]
    pub fn max_geometry_input_components(mut self, max_geometry_input_components: u32) -> Self {
        self.0.max_geometry_input_components = max_geometry_input_components as _;
        self
    }
    #[inline]
    pub fn max_geometry_output_components(mut self, max_geometry_output_components: u32) -> Self {
        self.0.max_geometry_output_components = max_geometry_output_components as _;
        self
    }
    #[inline]
    pub fn max_geometry_output_vertices(mut self, max_geometry_output_vertices: u32) -> Self {
        self.0.max_geometry_output_vertices = max_geometry_output_vertices as _;
        self
    }
    #[inline]
    pub fn max_geometry_total_output_components(mut self, max_geometry_total_output_components: u32) -> Self {
        self.0.max_geometry_total_output_components = max_geometry_total_output_components as _;
        self
    }
    #[inline]
    pub fn max_fragment_input_components(mut self, max_fragment_input_components: u32) -> Self {
        self.0.max_fragment_input_components = max_fragment_input_components as _;
        self
    }
    #[inline]
    pub fn max_fragment_output_attachments(mut self, max_fragment_output_attachments: u32) -> Self {
        self.0.max_fragment_output_attachments = max_fragment_output_attachments as _;
        self
    }
    #[inline]
    pub fn max_fragment_dual_src_attachments(mut self, max_fragment_dual_src_attachments: u32) -> Self {
        self.0.max_fragment_dual_src_attachments = max_fragment_dual_src_attachments as _;
        self
    }
    #[inline]
    pub fn max_fragment_combined_output_resources(mut self, max_fragment_combined_output_resources: u32) -> Self {
        self.0.max_fragment_combined_output_resources = max_fragment_combined_output_resources as _;
        self
    }
    #[inline]
    pub fn max_compute_shared_memory_size(mut self, max_compute_shared_memory_size: u32) -> Self {
        self.0.max_compute_shared_memory_size = max_compute_shared_memory_size as _;
        self
    }
    #[inline]
    pub fn max_compute_work_group_count(mut self, max_compute_work_group_count: [u32; 3]) -> Self {
        self.0.max_compute_work_group_count = max_compute_work_group_count as _;
        self
    }
    #[inline]
    pub fn max_compute_work_group_invocations(mut self, max_compute_work_group_invocations: u32) -> Self {
        self.0.max_compute_work_group_invocations = max_compute_work_group_invocations as _;
        self
    }
    #[inline]
    pub fn max_compute_work_group_size(mut self, max_compute_work_group_size: [u32; 3]) -> Self {
        self.0.max_compute_work_group_size = max_compute_work_group_size as _;
        self
    }
    #[inline]
    pub fn sub_pixel_precision_bits(mut self, sub_pixel_precision_bits: u32) -> Self {
        self.0.sub_pixel_precision_bits = sub_pixel_precision_bits as _;
        self
    }
    #[inline]
    pub fn sub_texel_precision_bits(mut self, sub_texel_precision_bits: u32) -> Self {
        self.0.sub_texel_precision_bits = sub_texel_precision_bits as _;
        self
    }
    #[inline]
    pub fn mipmap_precision_bits(mut self, mipmap_precision_bits: u32) -> Self {
        self.0.mipmap_precision_bits = mipmap_precision_bits as _;
        self
    }
    #[inline]
    pub fn max_draw_indexed_index_value(mut self, max_draw_indexed_index_value: u32) -> Self {
        self.0.max_draw_indexed_index_value = max_draw_indexed_index_value as _;
        self
    }
    #[inline]
    pub fn max_draw_indirect_count(mut self, max_draw_indirect_count: u32) -> Self {
        self.0.max_draw_indirect_count = max_draw_indirect_count as _;
        self
    }
    #[inline]
    pub fn max_sampler_lod_bias(mut self, max_sampler_lod_bias: std::os::raw::c_float) -> Self {
        self.0.max_sampler_lod_bias = max_sampler_lod_bias as _;
        self
    }
    #[inline]
    pub fn max_sampler_anisotropy(mut self, max_sampler_anisotropy: std::os::raw::c_float) -> Self {
        self.0.max_sampler_anisotropy = max_sampler_anisotropy as _;
        self
    }
    #[inline]
    pub fn max_viewports(mut self, max_viewports: u32) -> Self {
        self.0.max_viewports = max_viewports as _;
        self
    }
    #[inline]
    pub fn max_viewport_dimensions(mut self, max_viewport_dimensions: [u32; 2]) -> Self {
        self.0.max_viewport_dimensions = max_viewport_dimensions as _;
        self
    }
    #[inline]
    pub fn viewport_bounds_range(mut self, viewport_bounds_range: [std::os::raw::c_float; 2]) -> Self {
        self.0.viewport_bounds_range = viewport_bounds_range as _;
        self
    }
    #[inline]
    pub fn viewport_sub_pixel_bits(mut self, viewport_sub_pixel_bits: u32) -> Self {
        self.0.viewport_sub_pixel_bits = viewport_sub_pixel_bits as _;
        self
    }
    #[inline]
    pub fn min_memory_map_alignment(mut self, min_memory_map_alignment: usize) -> Self {
        self.0.min_memory_map_alignment = min_memory_map_alignment as _;
        self
    }
    #[inline]
    pub fn min_texel_buffer_offset_alignment(mut self, min_texel_buffer_offset_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.min_texel_buffer_offset_alignment = min_texel_buffer_offset_alignment as _;
        self
    }
    #[inline]
    pub fn min_uniform_buffer_offset_alignment(mut self, min_uniform_buffer_offset_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.min_uniform_buffer_offset_alignment = min_uniform_buffer_offset_alignment as _;
        self
    }
    #[inline]
    pub fn min_storage_buffer_offset_alignment(mut self, min_storage_buffer_offset_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.min_storage_buffer_offset_alignment = min_storage_buffer_offset_alignment as _;
        self
    }
    #[inline]
    pub fn min_texel_offset(mut self, min_texel_offset: i32) -> Self {
        self.0.min_texel_offset = min_texel_offset as _;
        self
    }
    #[inline]
    pub fn max_texel_offset(mut self, max_texel_offset: u32) -> Self {
        self.0.max_texel_offset = max_texel_offset as _;
        self
    }
    #[inline]
    pub fn min_texel_gather_offset(mut self, min_texel_gather_offset: i32) -> Self {
        self.0.min_texel_gather_offset = min_texel_gather_offset as _;
        self
    }
    #[inline]
    pub fn max_texel_gather_offset(mut self, max_texel_gather_offset: u32) -> Self {
        self.0.max_texel_gather_offset = max_texel_gather_offset as _;
        self
    }
    #[inline]
    pub fn min_interpolation_offset(mut self, min_interpolation_offset: std::os::raw::c_float) -> Self {
        self.0.min_interpolation_offset = min_interpolation_offset as _;
        self
    }
    #[inline]
    pub fn max_interpolation_offset(mut self, max_interpolation_offset: std::os::raw::c_float) -> Self {
        self.0.max_interpolation_offset = max_interpolation_offset as _;
        self
    }
    #[inline]
    pub fn sub_pixel_interpolation_offset_bits(mut self, sub_pixel_interpolation_offset_bits: u32) -> Self {
        self.0.sub_pixel_interpolation_offset_bits = sub_pixel_interpolation_offset_bits as _;
        self
    }
    #[inline]
    pub fn max_framebuffer_width(mut self, max_framebuffer_width: u32) -> Self {
        self.0.max_framebuffer_width = max_framebuffer_width as _;
        self
    }
    #[inline]
    pub fn max_framebuffer_height(mut self, max_framebuffer_height: u32) -> Self {
        self.0.max_framebuffer_height = max_framebuffer_height as _;
        self
    }
    #[inline]
    pub fn max_framebuffer_layers(mut self, max_framebuffer_layers: u32) -> Self {
        self.0.max_framebuffer_layers = max_framebuffer_layers as _;
        self
    }
    #[inline]
    pub fn framebuffer_color_sample_counts(mut self, framebuffer_color_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.framebuffer_color_sample_counts = framebuffer_color_sample_counts as _;
        self
    }
    #[inline]
    pub fn framebuffer_depth_sample_counts(mut self, framebuffer_depth_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.framebuffer_depth_sample_counts = framebuffer_depth_sample_counts as _;
        self
    }
    #[inline]
    pub fn framebuffer_stencil_sample_counts(mut self, framebuffer_stencil_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.framebuffer_stencil_sample_counts = framebuffer_stencil_sample_counts as _;
        self
    }
    #[inline]
    pub fn framebuffer_no_attachments_sample_counts(mut self, framebuffer_no_attachments_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.framebuffer_no_attachments_sample_counts = framebuffer_no_attachments_sample_counts as _;
        self
    }
    #[inline]
    pub fn max_color_attachments(mut self, max_color_attachments: u32) -> Self {
        self.0.max_color_attachments = max_color_attachments as _;
        self
    }
    #[inline]
    pub fn sampled_image_color_sample_counts(mut self, sampled_image_color_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.sampled_image_color_sample_counts = sampled_image_color_sample_counts as _;
        self
    }
    #[inline]
    pub fn sampled_image_integer_sample_counts(mut self, sampled_image_integer_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.sampled_image_integer_sample_counts = sampled_image_integer_sample_counts as _;
        self
    }
    #[inline]
    pub fn sampled_image_depth_sample_counts(mut self, sampled_image_depth_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.sampled_image_depth_sample_counts = sampled_image_depth_sample_counts as _;
        self
    }
    #[inline]
    pub fn sampled_image_stencil_sample_counts(mut self, sampled_image_stencil_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.sampled_image_stencil_sample_counts = sampled_image_stencil_sample_counts as _;
        self
    }
    #[inline]
    pub fn storage_image_sample_counts(mut self, storage_image_sample_counts: crate::vk1_0::SampleCountFlags) -> Self {
        self.0.storage_image_sample_counts = storage_image_sample_counts as _;
        self
    }
    #[inline]
    pub fn max_sample_mask_words(mut self, max_sample_mask_words: u32) -> Self {
        self.0.max_sample_mask_words = max_sample_mask_words as _;
        self
    }
    #[inline]
    pub fn timestamp_compute_and_graphics(mut self, timestamp_compute_and_graphics: bool) -> Self {
        self.0.timestamp_compute_and_graphics = timestamp_compute_and_graphics as _;
        self
    }
    #[inline]
    pub fn timestamp_period(mut self, timestamp_period: std::os::raw::c_float) -> Self {
        self.0.timestamp_period = timestamp_period as _;
        self
    }
    #[inline]
    pub fn max_clip_distances(mut self, max_clip_distances: u32) -> Self {
        self.0.max_clip_distances = max_clip_distances as _;
        self
    }
    #[inline]
    pub fn max_cull_distances(mut self, max_cull_distances: u32) -> Self {
        self.0.max_cull_distances = max_cull_distances as _;
        self
    }
    #[inline]
    pub fn max_combined_clip_and_cull_distances(mut self, max_combined_clip_and_cull_distances: u32) -> Self {
        self.0.max_combined_clip_and_cull_distances = max_combined_clip_and_cull_distances as _;
        self
    }
    #[inline]
    pub fn discrete_queue_priorities(mut self, discrete_queue_priorities: u32) -> Self {
        self.0.discrete_queue_priorities = discrete_queue_priorities as _;
        self
    }
    #[inline]
    pub fn point_size_range(mut self, point_size_range: [std::os::raw::c_float; 2]) -> Self {
        self.0.point_size_range = point_size_range as _;
        self
    }
    #[inline]
    pub fn line_width_range(mut self, line_width_range: [std::os::raw::c_float; 2]) -> Self {
        self.0.line_width_range = line_width_range as _;
        self
    }
    #[inline]
    pub fn point_size_granularity(mut self, point_size_granularity: std::os::raw::c_float) -> Self {
        self.0.point_size_granularity = point_size_granularity as _;
        self
    }
    #[inline]
    pub fn line_width_granularity(mut self, line_width_granularity: std::os::raw::c_float) -> Self {
        self.0.line_width_granularity = line_width_granularity as _;
        self
    }
    #[inline]
    pub fn strict_lines(mut self, strict_lines: bool) -> Self {
        self.0.strict_lines = strict_lines as _;
        self
    }
    #[inline]
    pub fn standard_sample_locations(mut self, standard_sample_locations: bool) -> Self {
        self.0.standard_sample_locations = standard_sample_locations as _;
        self
    }
    #[inline]
    pub fn optimal_buffer_copy_offset_alignment(mut self, optimal_buffer_copy_offset_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.optimal_buffer_copy_offset_alignment = optimal_buffer_copy_offset_alignment as _;
        self
    }
    #[inline]
    pub fn optimal_buffer_copy_row_pitch_alignment(mut self, optimal_buffer_copy_row_pitch_alignment: crate::vk1_0::DeviceSize) -> Self {
        self.0.optimal_buffer_copy_row_pitch_alignment = optimal_buffer_copy_row_pitch_alignment as _;
        self
    }
    #[inline]
    pub fn non_coherent_atom_size(mut self, non_coherent_atom_size: crate::vk1_0::DeviceSize) -> Self {
        self.0.non_coherent_atom_size = non_coherent_atom_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceLimits {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceLimitsBuilder<'a> {
    fn default() -> PhysicalDeviceLimitsBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceLimitsBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceLimitsBuilder<'a> {
    type Target = PhysicalDeviceLimits;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceLimitsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreCreateInfo.html) · Structure"]
#[doc(alias = "VkSemaphoreCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SemaphoreCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::SemaphoreCreateFlags,
}
impl Default for SemaphoreCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::SEMAPHORE_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default() }
    }
}
impl std::fmt::Debug for SemaphoreCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SemaphoreCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).finish()
    }
}
impl SemaphoreCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SemaphoreCreateInfoBuilder<'a> {
        SemaphoreCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSemaphoreCreateInfo.html) · Builder of [`SemaphoreCreateInfo`]"]
#[repr(transparent)]
pub struct SemaphoreCreateInfoBuilder<'a>(SemaphoreCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SemaphoreCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SemaphoreCreateInfoBuilder<'a> {
        SemaphoreCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::SemaphoreCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SemaphoreCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for SemaphoreCreateInfoBuilder<'a> {
    fn default() -> SemaphoreCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SemaphoreCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SemaphoreCreateInfoBuilder<'a> {
    type Target = SemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SemaphoreCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfo.html) · Structure"]
#[doc(alias = "VkQueryPoolCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryPoolCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::QueryPoolCreateFlags,
    pub query_type: crate::vk1_0::QueryType,
    pub query_count: u32,
    pub pipeline_statistics: crate::vk1_0::QueryPipelineStatisticFlags,
}
impl Default for QueryPoolCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::QUERY_POOL_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), query_type: Default::default(), query_count: Default::default(), pipeline_statistics: Default::default() }
    }
}
impl std::fmt::Debug for QueryPoolCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QueryPoolCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("query_type", &self.query_type).field("query_count", &self.query_count).field("pipeline_statistics", &self.pipeline_statistics).finish()
    }
}
impl QueryPoolCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> QueryPoolCreateInfoBuilder<'a> {
        QueryPoolCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkQueryPoolCreateInfo.html) · Builder of [`QueryPoolCreateInfo`]"]
#[repr(transparent)]
pub struct QueryPoolCreateInfoBuilder<'a>(QueryPoolCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> QueryPoolCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> QueryPoolCreateInfoBuilder<'a> {
        QueryPoolCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::QueryPoolCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn query_type(mut self, query_type: crate::vk1_0::QueryType) -> Self {
        self.0.query_type = query_type as _;
        self
    }
    #[inline]
    pub fn query_count(mut self, query_count: u32) -> Self {
        self.0.query_count = query_count as _;
        self
    }
    #[inline]
    pub fn pipeline_statistics(mut self, pipeline_statistics: crate::vk1_0::QueryPipelineStatisticFlags) -> Self {
        self.0.pipeline_statistics = pipeline_statistics as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> QueryPoolCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for QueryPoolCreateInfoBuilder<'a> {
    fn default() -> QueryPoolCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for QueryPoolCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for QueryPoolCreateInfoBuilder<'a> {
    type Target = QueryPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for QueryPoolCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateInfo.html) · Structure"]
#[doc(alias = "VkFramebufferCreateInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FramebufferCreateInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::vk1_0::FramebufferCreateFlags,
    pub render_pass: crate::vk1_0::RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const crate::vk1_0::ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
impl Default for FramebufferCreateInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::FRAMEBUFFER_CREATE_INFO, p_next: std::ptr::null(), flags: Default::default(), render_pass: Default::default(), attachment_count: Default::default(), p_attachments: std::ptr::null(), width: Default::default(), height: Default::default(), layers: Default::default() }
    }
}
impl std::fmt::Debug for FramebufferCreateInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FramebufferCreateInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("render_pass", &self.render_pass).field("attachment_count", &self.attachment_count).field("p_attachments", &self.p_attachments).field("width", &self.width).field("height", &self.height).field("layers", &self.layers).finish()
    }
}
impl FramebufferCreateInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> FramebufferCreateInfoBuilder<'a> {
        FramebufferCreateInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFramebufferCreateInfo.html) · Builder of [`FramebufferCreateInfo`]"]
#[repr(transparent)]
pub struct FramebufferCreateInfoBuilder<'a>(FramebufferCreateInfo, std::marker::PhantomData<&'a ()>);
impl<'a> FramebufferCreateInfoBuilder<'a> {
    #[inline]
    pub fn new() -> FramebufferCreateInfoBuilder<'a> {
        FramebufferCreateInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::vk1_0::FramebufferCreateFlags) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn render_pass(mut self, render_pass: crate::vk1_0::RenderPass) -> Self {
        self.0.render_pass = render_pass as _;
        self
    }
    #[inline]
    pub fn attachments(mut self, attachments: &'a [crate::vk1_0::ImageView]) -> Self {
        self.0.p_attachments = attachments.as_ptr() as _;
        self.0.attachment_count = attachments.len() as _;
        self
    }
    #[inline]
    pub fn width(mut self, width: u32) -> Self {
        self.0.width = width as _;
        self
    }
    #[inline]
    pub fn height(mut self, height: u32) -> Self {
        self.0.height = height as _;
        self
    }
    #[inline]
    pub fn layers(mut self, layers: u32) -> Self {
        self.0.layers = layers as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> FramebufferCreateInfo {
        self.0
    }
}
impl<'a> std::default::Default for FramebufferCreateInfoBuilder<'a> {
    fn default() -> FramebufferCreateInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FramebufferCreateInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FramebufferCreateInfoBuilder<'a> {
    type Target = FramebufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FramebufferCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndirectCommand.html) · Structure"]
#[doc(alias = "VkDrawIndirectCommand")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}
impl Default for DrawIndirectCommand {
    fn default() -> Self {
        Self { vertex_count: Default::default(), instance_count: Default::default(), first_vertex: Default::default(), first_instance: Default::default() }
    }
}
impl std::fmt::Debug for DrawIndirectCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DrawIndirectCommand").field("vertex_count", &self.vertex_count).field("instance_count", &self.instance_count).field("first_vertex", &self.first_vertex).field("first_instance", &self.first_instance).finish()
    }
}
impl DrawIndirectCommand {
    #[inline]
    pub fn into_builder<'a>(self) -> DrawIndirectCommandBuilder<'a> {
        DrawIndirectCommandBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndirectCommand.html) · Builder of [`DrawIndirectCommand`]"]
#[repr(transparent)]
pub struct DrawIndirectCommandBuilder<'a>(DrawIndirectCommand, std::marker::PhantomData<&'a ()>);
impl<'a> DrawIndirectCommandBuilder<'a> {
    #[inline]
    pub fn new() -> DrawIndirectCommandBuilder<'a> {
        DrawIndirectCommandBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.0.vertex_count = vertex_count as _;
        self
    }
    #[inline]
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.0.instance_count = instance_count as _;
        self
    }
    #[inline]
    pub fn first_vertex(mut self, first_vertex: u32) -> Self {
        self.0.first_vertex = first_vertex as _;
        self
    }
    #[inline]
    pub fn first_instance(mut self, first_instance: u32) -> Self {
        self.0.first_instance = first_instance as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DrawIndirectCommand {
        self.0
    }
}
impl<'a> std::default::Default for DrawIndirectCommandBuilder<'a> {
    fn default() -> DrawIndirectCommandBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DrawIndirectCommandBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DrawIndirectCommandBuilder<'a> {
    type Target = DrawIndirectCommand;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DrawIndirectCommandBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndexedIndirectCommand.html) · Structure"]
#[doc(alias = "VkDrawIndexedIndirectCommand")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}
impl Default for DrawIndexedIndirectCommand {
    fn default() -> Self {
        Self { index_count: Default::default(), instance_count: Default::default(), first_index: Default::default(), vertex_offset: Default::default(), first_instance: Default::default() }
    }
}
impl std::fmt::Debug for DrawIndexedIndirectCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DrawIndexedIndirectCommand").field("index_count", &self.index_count).field("instance_count", &self.instance_count).field("first_index", &self.first_index).field("vertex_offset", &self.vertex_offset).field("first_instance", &self.first_instance).finish()
    }
}
impl DrawIndexedIndirectCommand {
    #[inline]
    pub fn into_builder<'a>(self) -> DrawIndexedIndirectCommandBuilder<'a> {
        DrawIndexedIndirectCommandBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDrawIndexedIndirectCommand.html) · Builder of [`DrawIndexedIndirectCommand`]"]
#[repr(transparent)]
pub struct DrawIndexedIndirectCommandBuilder<'a>(DrawIndexedIndirectCommand, std::marker::PhantomData<&'a ()>);
impl<'a> DrawIndexedIndirectCommandBuilder<'a> {
    #[inline]
    pub fn new() -> DrawIndexedIndirectCommandBuilder<'a> {
        DrawIndexedIndirectCommandBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.0.index_count = index_count as _;
        self
    }
    #[inline]
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.0.instance_count = instance_count as _;
        self
    }
    #[inline]
    pub fn first_index(mut self, first_index: u32) -> Self {
        self.0.first_index = first_index as _;
        self
    }
    #[inline]
    pub fn vertex_offset(mut self, vertex_offset: i32) -> Self {
        self.0.vertex_offset = vertex_offset as _;
        self
    }
    #[inline]
    pub fn first_instance(mut self, first_instance: u32) -> Self {
        self.0.first_instance = first_instance as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DrawIndexedIndirectCommand {
        self.0
    }
}
impl<'a> std::default::Default for DrawIndexedIndirectCommandBuilder<'a> {
    fn default() -> DrawIndexedIndirectCommandBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DrawIndexedIndirectCommandBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DrawIndexedIndirectCommandBuilder<'a> {
    type Target = DrawIndexedIndirectCommand;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DrawIndexedIndirectCommandBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDispatchIndirectCommand.html) · Structure"]
#[doc(alias = "VkDispatchIndirectCommand")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl Default for DispatchIndirectCommand {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default(), z: Default::default() }
    }
}
impl std::fmt::Debug for DispatchIndirectCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DispatchIndirectCommand").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl DispatchIndirectCommand {
    #[inline]
    pub fn into_builder<'a>(self) -> DispatchIndirectCommandBuilder<'a> {
        DispatchIndirectCommandBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDispatchIndirectCommand.html) · Builder of [`DispatchIndirectCommand`]"]
#[repr(transparent)]
pub struct DispatchIndirectCommandBuilder<'a>(DispatchIndirectCommand, std::marker::PhantomData<&'a ()>);
impl<'a> DispatchIndirectCommandBuilder<'a> {
    #[inline]
    pub fn new() -> DispatchIndirectCommandBuilder<'a> {
        DispatchIndirectCommandBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn x(mut self, x: u32) -> Self {
        self.0.x = x as _;
        self
    }
    #[inline]
    pub fn y(mut self, y: u32) -> Self {
        self.0.y = y as _;
        self
    }
    #[inline]
    pub fn z(mut self, z: u32) -> Self {
        self.0.z = z as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> DispatchIndirectCommand {
        self.0
    }
}
impl<'a> std::default::Default for DispatchIndirectCommandBuilder<'a> {
    fn default() -> DispatchIndirectCommandBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for DispatchIndirectCommandBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for DispatchIndirectCommandBuilder<'a> {
    type Target = DispatchIndirectCommand;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for DispatchIndirectCommandBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitInfo.html) · Structure"]
#[doc(alias = "VkSubmitInfo")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubmitInfo {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const crate::vk1_0::Semaphore,
    pub p_wait_dst_stage_mask: *const crate::vk1_0::PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const crate::vk1_0::CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const crate::vk1_0::Semaphore,
}
impl Default for SubmitInfo {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::SUBMIT_INFO, p_next: std::ptr::null(), wait_semaphore_count: Default::default(), p_wait_semaphores: std::ptr::null(), p_wait_dst_stage_mask: std::ptr::null(), command_buffer_count: Default::default(), p_command_buffers: std::ptr::null(), signal_semaphore_count: Default::default(), p_signal_semaphores: std::ptr::null() }
    }
}
impl std::fmt::Debug for SubmitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubmitInfo").field("s_type", &self.s_type).field("p_next", &self.p_next).field("wait_semaphore_count", &self.wait_semaphore_count).field("p_wait_semaphores", &self.p_wait_semaphores).field("p_wait_dst_stage_mask", &self.p_wait_dst_stage_mask).field("command_buffer_count", &self.command_buffer_count).field("p_command_buffers", &self.p_command_buffers).field("signal_semaphore_count", &self.signal_semaphore_count).field("p_signal_semaphores", &self.p_signal_semaphores).finish()
    }
}
impl SubmitInfo {
    #[inline]
    pub fn into_builder<'a>(self) -> SubmitInfoBuilder<'a> {
        SubmitInfoBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubmitInfo.html) · Builder of [`SubmitInfo`]"]
#[repr(transparent)]
pub struct SubmitInfoBuilder<'a>(SubmitInfo, std::marker::PhantomData<&'a ()>);
impl<'a> SubmitInfoBuilder<'a> {
    #[inline]
    pub fn new() -> SubmitInfoBuilder<'a> {
        SubmitInfoBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn wait_semaphores(mut self, wait_semaphores: &'a [crate::vk1_0::Semaphore]) -> Self {
        self.0.p_wait_semaphores = wait_semaphores.as_ptr() as _;
        self.0.wait_semaphore_count = wait_semaphores.len() as _;
        self
    }
    #[inline]
    pub fn wait_dst_stage_mask(mut self, wait_dst_stage_mask: &'a [crate::vk1_0::PipelineStageFlags]) -> Self {
        self.0.p_wait_dst_stage_mask = wait_dst_stage_mask.as_ptr() as _;
        self.0.wait_semaphore_count = wait_dst_stage_mask.len() as _;
        self
    }
    #[inline]
    pub fn command_buffers(mut self, command_buffers: &'a [crate::vk1_0::CommandBuffer]) -> Self {
        self.0.p_command_buffers = command_buffers.as_ptr() as _;
        self.0.command_buffer_count = command_buffers.len() as _;
        self
    }
    #[inline]
    pub fn signal_semaphores(mut self, signal_semaphores: &'a [crate::vk1_0::Semaphore]) -> Self {
        self.0.p_signal_semaphores = signal_semaphores.as_ptr() as _;
        self.0.signal_semaphore_count = signal_semaphores.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SubmitInfo {
        self.0
    }
}
impl<'a> std::default::Default for SubmitInfoBuilder<'a> {
    fn default() -> SubmitInfoBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubmitInfoBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubmitInfoBuilder<'a> {
    type Target = SubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubmitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_MAKE_VERSION.html) · Define"]
#[doc(alias = "VK_MAKE_VERSION")]
#[deprecated = "VK_MAKE_API_VERSION should be used instead."]
pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_MAJOR.html) · Define"]
#[doc(alias = "VK_VERSION_MAJOR")]
#[deprecated = "VK_API_VERSION_MAJOR should be used instead."]
pub const fn version_major(version: u32) -> u32 {
    version >> 22
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_MINOR.html) · Define"]
#[doc(alias = "VK_VERSION_MINOR")]
#[deprecated = "VK_API_VERSION_MINOR should be used instead."]
pub const fn version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_PATCH.html) · Define"]
#[doc(alias = "VK_VERSION_PATCH")]
#[deprecated = "VK_API_VERSION_PATCH should be used instead."]
pub const fn version_patch(version: u32) -> u32 {
    version & 0xfff
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_MAKE_API_VERSION.html) · Define"]
#[doc(alias = "VK_MAKE_API_VERSION")]
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_API_VERSION_VARIANT.html) · Define"]
#[doc(alias = "VK_API_VERSION_VARIANT")]
pub const fn api_version_variant(version: u32) -> u32 {
    version >> 29
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_API_VERSION_MAJOR.html) · Define"]
#[doc(alias = "VK_API_VERSION_MAJOR")]
pub const fn api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7f
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_API_VERSION_MINOR.html) · Define"]
#[doc(alias = "VK_API_VERSION_MINOR")]
pub const fn api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_API_VERSION_PATCH.html) · Define"]
#[doc(alias = "VK_API_VERSION_PATCH")]
pub const fn api_version_patch(version: u32) -> u32 {
    version & 0xfff
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_API_VERSION_1_0.html) · Define"]
#[doc(alias = "VK_API_VERSION_1_0")]
pub const API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_API_VERSION_1_1.html) · Define"]
#[doc(alias = "VK_API_VERSION_1_1")]
pub const API_VERSION_1_1: u32 = make_api_version(0, 1, 1, 0);
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_API_VERSION_1_2.html) · Define"]
#[doc(alias = "VK_API_VERSION_1_2")]
pub const API_VERSION_1_2: u32 = make_api_version(0, 1, 2, 0);
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_HEADER_VERSION.html) · Define"]
#[doc(alias = "VK_HEADER_VERSION")]
pub const HEADER_VERSION: u32 = 181u32;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_HEADER_VERSION_COMPLETE.html) · Define"]
#[doc(alias = "VK_HEADER_VERSION_COMPLETE")]
pub const HEADER_VERSION_COMPLETE: u32 = make_api_version(0, 1u32, 2u32, 181u32);
#[doc = "Provided by [`crate::vk1_0`]"]
impl<T> crate::EntryLoader<T> {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html) · Function"]
    #[doc(alias = "vkCreateInstance")]
    pub unsafe fn create_instance(&self, create_info: &crate::vk1_0::InstanceCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Instance> {
        let _function = self.create_instance.expect("tried to call a function that isn't loaded");
        let mut instance = Default::default();
        let _return = _function(
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut instance,
        );
        crate::utils::VulkanResult::new(_return, instance)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceLayerProperties.html) · Function"]
    #[doc(alias = "vkEnumerateInstanceLayerProperties")]
    pub unsafe fn enumerate_instance_layer_properties(&self, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::vk1_0::LayerProperties>> {
        let _function = self.enumerate_instance_layer_properties.expect("tried to call a function that isn't loaded");
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(&mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(&mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html) · Function"]
    #[doc(alias = "vkEnumerateInstanceExtensionProperties")]
    pub unsafe fn enumerate_instance_extension_properties(&self, layer_name: Option<&std::ffi::CStr>, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::vk1_0::ExtensionProperties>> {
        let _function = self.enumerate_instance_extension_properties.expect("tried to call a function that isn't loaded");
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(
                    match layer_name {
                        Some(v) => v.as_ptr(),
                        None => std::ptr::null(),
                    },
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(
            match layer_name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, properties)
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::InstanceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyInstance.html) · Function"]
    #[doc(alias = "vkDestroyInstance")]
    pub unsafe fn destroy_instance(&self, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_instance.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDevices.html) · Function"]
    #[doc(alias = "vkEnumeratePhysicalDevices")]
    pub unsafe fn enumerate_physical_devices(&self, physical_device_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::vk1_0::PhysicalDevice>> {
        let _function = self.enumerate_physical_devices.expect("tried to call a function that isn't loaded");
        let mut physical_device_count = match physical_device_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut physical_devices = vec![Default::default(); physical_device_count as _];
        let _return = _function(self.handle, &mut physical_device_count, physical_devices.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, physical_devices)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html) · Function"]
    #[doc(alias = "vkGetInstanceProcAddr")]
    pub unsafe fn get_instance_proc_addr(&self, name: Option<&std::ffi::CStr>) -> Option<crate::vk1_0::PFN_vkVoidFunction> {
        let _function = self.get_instance_proc_addr.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
        );
        _return
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceProperties")]
    pub unsafe fn get_physical_device_properties(&self, physical_device: crate::vk1_0::PhysicalDevice) -> crate::vk1_0::PhysicalDeviceProperties {
        let _function = self.get_physical_device_properties.expect("tried to call a function that isn't loaded");
        let mut properties = Default::default();
        let _return = _function(physical_device as _, &mut properties);
        properties
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties")]
    pub unsafe fn get_physical_device_queue_family_properties(&self, physical_device: crate::vk1_0::PhysicalDevice, queue_family_property_count: Option<u32>) -> Vec<crate::vk1_0::QueueFamilyProperties> {
        let _function = self.get_physical_device_queue_family_properties.expect("tried to call a function that isn't loaded");
        let mut queue_family_property_count = match queue_family_property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut queue_family_properties = vec![Default::default(); queue_family_property_count as _];
        let _return = _function(physical_device as _, &mut queue_family_property_count, queue_family_properties.as_mut_ptr());
        queue_family_properties
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceMemoryProperties")]
    pub unsafe fn get_physical_device_memory_properties(&self, physical_device: crate::vk1_0::PhysicalDevice) -> crate::vk1_0::PhysicalDeviceMemoryProperties {
        let _function = self.get_physical_device_memory_properties.expect("tried to call a function that isn't loaded");
        let mut memory_properties = Default::default();
        let _return = _function(physical_device as _, &mut memory_properties);
        memory_properties
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceFeatures")]
    pub unsafe fn get_physical_device_features(&self, physical_device: crate::vk1_0::PhysicalDevice) -> crate::vk1_0::PhysicalDeviceFeatures {
        let _function = self.get_physical_device_features.expect("tried to call a function that isn't loaded");
        let mut features = Default::default();
        let _return = _function(physical_device as _, &mut features);
        features
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceFormatProperties")]
    pub unsafe fn get_physical_device_format_properties(&self, physical_device: crate::vk1_0::PhysicalDevice, format: crate::vk1_0::Format) -> crate::vk1_0::FormatProperties {
        let _function = self.get_physical_device_format_properties.expect("tried to call a function that isn't loaded");
        let mut format_properties = Default::default();
        let _return = _function(physical_device as _, format as _, &mut format_properties);
        format_properties
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceImageFormatProperties")]
    pub unsafe fn get_physical_device_image_format_properties(&self, physical_device: crate::vk1_0::PhysicalDevice, format: crate::vk1_0::Format, _type: crate::vk1_0::ImageType, tiling: crate::vk1_0::ImageTiling, usage: crate::vk1_0::ImageUsageFlags, flags: Option<crate::vk1_0::ImageCreateFlags>) -> crate::utils::VulkanResult<crate::vk1_0::ImageFormatProperties> {
        let _function = self.get_physical_device_image_format_properties.expect("tried to call a function that isn't loaded");
        let mut image_format_properties = Default::default();
        let _return = _function(
            physical_device as _,
            format as _,
            _type as _,
            tiling as _,
            usage as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            &mut image_format_properties,
        );
        crate::utils::VulkanResult::new(_return, image_format_properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDevice.html) · Function"]
    #[doc(alias = "vkCreateDevice")]
    pub unsafe fn create_device(&self, physical_device: crate::vk1_0::PhysicalDevice, create_info: &crate::vk1_0::DeviceCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Device> {
        let _function = self.create_device.expect("tried to call a function that isn't loaded");
        let mut device = Default::default();
        let _return = _function(
            physical_device as _,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut device,
        );
        crate::utils::VulkanResult::new(_return, device)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceLayerProperties.html) · Function"]
    #[doc(alias = "vkEnumerateDeviceLayerProperties")]
    pub unsafe fn enumerate_device_layer_properties(&self, physical_device: crate::vk1_0::PhysicalDevice, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::vk1_0::LayerProperties>> {
        let _function = self.enumerate_device_layer_properties.expect("tried to call a function that isn't loaded");
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(physical_device as _, &mut property_count, properties.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceExtensionProperties.html) · Function"]
    #[doc(alias = "vkEnumerateDeviceExtensionProperties")]
    pub unsafe fn enumerate_device_extension_properties(&self, physical_device: crate::vk1_0::PhysicalDevice, layer_name: Option<&std::ffi::CStr>, property_count: Option<u32>) -> crate::utils::VulkanResult<Vec<crate::vk1_0::ExtensionProperties>> {
        let _function = self.enumerate_device_extension_properties.expect("tried to call a function that isn't loaded");
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(
                    physical_device as _,
                    match layer_name {
                        Some(v) => v.as_ptr(),
                        None => std::ptr::null(),
                    },
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(
            physical_device as _,
            match layer_name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
            &mut property_count,
            properties.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, properties)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html) · Function"]
    #[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties")]
    pub unsafe fn get_physical_device_sparse_image_format_properties(&self, physical_device: crate::vk1_0::PhysicalDevice, format: crate::vk1_0::Format, _type: crate::vk1_0::ImageType, samples: crate::vk1_0::SampleCountFlagBits, usage: crate::vk1_0::ImageUsageFlags, tiling: crate::vk1_0::ImageTiling, property_count: Option<u32>) -> Vec<crate::vk1_0::SparseImageFormatProperties> {
        let _function = self.get_physical_device_sparse_image_format_properties.expect("tried to call a function that isn't loaded");
        let mut property_count = match property_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(physical_device as _, format as _, _type as _, samples as _, usage as _, tiling as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut properties = vec![Default::default(); property_count as _];
        let _return = _function(physical_device as _, format as _, _type as _, samples as _, usage as _, tiling as _, &mut property_count, properties.as_mut_ptr());
        properties
    }
}
#[doc = "Provided by [`crate::vk1_0`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceProcAddr.html) · Function"]
    #[doc(alias = "vkGetDeviceProcAddr")]
    pub unsafe fn get_device_proc_addr(&self, name: Option<&std::ffi::CStr>) -> Option<crate::vk1_0::PFN_vkVoidFunction> {
        let _function = self.get_device_proc_addr.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match name {
                Some(v) => v.as_ptr(),
                None => std::ptr::null(),
            },
        );
        _return
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDevice.html) · Function"]
    #[doc(alias = "vkDestroyDevice")]
    pub unsafe fn destroy_device(&self, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_device.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue.html) · Function"]
    #[doc(alias = "vkGetDeviceQueue")]
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> crate::vk1_0::Queue {
        let _function = self.get_device_queue.expect("tried to call a function that isn't loaded");
        let mut queue = Default::default();
        let _return = _function(self.handle, queue_family_index as _, queue_index as _, &mut queue);
        queue
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSubmit.html) · Function"]
    #[doc(alias = "vkQueueSubmit")]
    pub unsafe fn queue_submit(&self, queue: crate::vk1_0::Queue, submits: &[crate::vk1_0::SubmitInfoBuilder], fence: Option<crate::vk1_0::Fence>) -> crate::utils::VulkanResult<()> {
        let _function = self.queue_submit.expect("tried to call a function that isn't loaded");
        let submit_count = submits.len();
        let _return = _function(
            queue as _,
            submit_count as _,
            submits.as_ptr() as _,
            match fence {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueWaitIdle.html) · Function"]
    #[doc(alias = "vkQueueWaitIdle")]
    pub unsafe fn queue_wait_idle(&self, queue: crate::vk1_0::Queue) -> crate::utils::VulkanResult<()> {
        let _function = self.queue_wait_idle.expect("tried to call a function that isn't loaded");
        let _return = _function(queue as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeviceWaitIdle.html) · Function"]
    #[doc(alias = "vkDeviceWaitIdle")]
    pub unsafe fn device_wait_idle(&self) -> crate::utils::VulkanResult<()> {
        let _function = self.device_wait_idle.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateMemory.html) · Function"]
    #[doc(alias = "vkAllocateMemory")]
    pub unsafe fn allocate_memory(&self, allocate_info: &crate::vk1_0::MemoryAllocateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::DeviceMemory> {
        let _function = self.allocate_memory.expect("tried to call a function that isn't loaded");
        let mut memory = Default::default();
        let _return = _function(
            self.handle,
            allocate_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut memory,
        );
        crate::utils::VulkanResult::new(_return, memory)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeMemory.html) · Function"]
    #[doc(alias = "vkFreeMemory")]
    pub unsafe fn free_memory(&self, memory: Option<crate::vk1_0::DeviceMemory>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.free_memory.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match memory {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMapMemory.html) · Function"]
    #[doc(alias = "vkMapMemory")]
    pub unsafe fn map_memory(&self, memory: crate::vk1_0::DeviceMemory, offset: crate::vk1_0::DeviceSize, size: crate::vk1_0::DeviceSize, flags: Option<crate::vk1_0::MemoryMapFlags>, data: *mut *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.map_memory.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            memory as _,
            offset as _,
            size as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
            data,
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUnmapMemory.html) · Function"]
    #[doc(alias = "vkUnmapMemory")]
    pub unsafe fn unmap_memory(&self, memory: crate::vk1_0::DeviceMemory) -> () {
        let _function = self.unmap_memory.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, memory as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFlushMappedMemoryRanges.html) · Function"]
    #[doc(alias = "vkFlushMappedMemoryRanges")]
    pub unsafe fn flush_mapped_memory_ranges(&self, memory_ranges: &[crate::vk1_0::MappedMemoryRangeBuilder]) -> crate::utils::VulkanResult<()> {
        let _function = self.flush_mapped_memory_ranges.expect("tried to call a function that isn't loaded");
        let memory_range_count = memory_ranges.len();
        let _return = _function(self.handle, memory_range_count as _, memory_ranges.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInvalidateMappedMemoryRanges.html) · Function"]
    #[doc(alias = "vkInvalidateMappedMemoryRanges")]
    pub unsafe fn invalidate_mapped_memory_ranges(&self, memory_ranges: &[crate::vk1_0::MappedMemoryRangeBuilder]) -> crate::utils::VulkanResult<()> {
        let _function = self.invalidate_mapped_memory_ranges.expect("tried to call a function that isn't loaded");
        let memory_range_count = memory_ranges.len();
        let _return = _function(self.handle, memory_range_count as _, memory_ranges.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryCommitment.html) · Function"]
    #[doc(alias = "vkGetDeviceMemoryCommitment")]
    pub unsafe fn get_device_memory_commitment(&self, memory: crate::vk1_0::DeviceMemory) -> crate::vk1_0::DeviceSize {
        let _function = self.get_device_memory_commitment.expect("tried to call a function that isn't loaded");
        let mut committed_memory_in_bytes = Default::default();
        let _return = _function(self.handle, memory as _, &mut committed_memory_in_bytes);
        committed_memory_in_bytes
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements.html) · Function"]
    #[doc(alias = "vkGetBufferMemoryRequirements")]
    pub unsafe fn get_buffer_memory_requirements(&self, buffer: crate::vk1_0::Buffer) -> crate::vk1_0::MemoryRequirements {
        let _function = self.get_buffer_memory_requirements.expect("tried to call a function that isn't loaded");
        let mut memory_requirements = Default::default();
        let _return = _function(self.handle, buffer as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory.html) · Function"]
    #[doc(alias = "vkBindBufferMemory")]
    pub unsafe fn bind_buffer_memory(&self, buffer: crate::vk1_0::Buffer, memory: crate::vk1_0::DeviceMemory, memory_offset: crate::vk1_0::DeviceSize) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_buffer_memory.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, buffer as _, memory as _, memory_offset as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements.html) · Function"]
    #[doc(alias = "vkGetImageMemoryRequirements")]
    pub unsafe fn get_image_memory_requirements(&self, image: crate::vk1_0::Image) -> crate::vk1_0::MemoryRequirements {
        let _function = self.get_image_memory_requirements.expect("tried to call a function that isn't loaded");
        let mut memory_requirements = Default::default();
        let _return = _function(self.handle, image as _, &mut memory_requirements);
        memory_requirements
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory.html) · Function"]
    #[doc(alias = "vkBindImageMemory")]
    pub unsafe fn bind_image_memory(&self, image: crate::vk1_0::Image, memory: crate::vk1_0::DeviceMemory, memory_offset: crate::vk1_0::DeviceSize) -> crate::utils::VulkanResult<()> {
        let _function = self.bind_image_memory.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, image as _, memory as _, memory_offset as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements.html) · Function"]
    #[doc(alias = "vkGetImageSparseMemoryRequirements")]
    pub unsafe fn get_image_sparse_memory_requirements(&self, image: crate::vk1_0::Image, sparse_memory_requirement_count: Option<u32>) -> Vec<crate::vk1_0::SparseImageMemoryRequirements> {
        let _function = self.get_image_sparse_memory_requirements.expect("tried to call a function that isn't loaded");
        let mut sparse_memory_requirement_count = match sparse_memory_requirement_count {
            Some(v) => v,
            None => {
                let mut v = Default::default();
                _function(self.handle, image as _, &mut v, std::ptr::null_mut());
                v
            }
        };
        let mut sparse_memory_requirements = vec![Default::default(); sparse_memory_requirement_count as _];
        let _return = _function(self.handle, image as _, &mut sparse_memory_requirement_count, sparse_memory_requirements.as_mut_ptr());
        sparse_memory_requirements
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBindSparse.html) · Function"]
    #[doc(alias = "vkQueueBindSparse")]
    pub unsafe fn queue_bind_sparse(&self, queue: crate::vk1_0::Queue, bind_info: &[crate::vk1_0::BindSparseInfoBuilder], fence: Option<crate::vk1_0::Fence>) -> crate::utils::VulkanResult<()> {
        let _function = self.queue_bind_sparse.expect("tried to call a function that isn't loaded");
        let bind_info_count = bind_info.len();
        let _return = _function(
            queue as _,
            bind_info_count as _,
            bind_info.as_ptr() as _,
            match fence {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFence.html) · Function"]
    #[doc(alias = "vkCreateFence")]
    pub unsafe fn create_fence(&self, create_info: &crate::vk1_0::FenceCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Fence> {
        let _function = self.create_fence.expect("tried to call a function that isn't loaded");
        let mut fence = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut fence,
        );
        crate::utils::VulkanResult::new(_return, fence)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFence.html) · Function"]
    #[doc(alias = "vkDestroyFence")]
    pub unsafe fn destroy_fence(&self, fence: Option<crate::vk1_0::Fence>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_fence.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match fence {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetFences.html) · Function"]
    #[doc(alias = "vkResetFences")]
    pub unsafe fn reset_fences(&self, fences: &[crate::vk1_0::Fence]) -> crate::utils::VulkanResult<()> {
        let _function = self.reset_fences.expect("tried to call a function that isn't loaded");
        let fence_count = fences.len();
        let _return = _function(self.handle, fence_count as _, fences.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceStatus.html) · Function"]
    #[doc(alias = "vkGetFenceStatus")]
    pub unsafe fn get_fence_status(&self, fence: crate::vk1_0::Fence) -> crate::utils::VulkanResult<()> {
        let _function = self.get_fence_status.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, fence as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitForFences.html) · Function"]
    #[doc(alias = "vkWaitForFences")]
    pub unsafe fn wait_for_fences(&self, fences: &[crate::vk1_0::Fence], wait_all: bool, timeout: u64) -> crate::utils::VulkanResult<()> {
        let _function = self.wait_for_fences.expect("tried to call a function that isn't loaded");
        let fence_count = fences.len();
        let _return = _function(self.handle, fence_count as _, fences.as_ptr() as _, wait_all as _, timeout as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSemaphore.html) · Function"]
    #[doc(alias = "vkCreateSemaphore")]
    pub unsafe fn create_semaphore(&self, create_info: &crate::vk1_0::SemaphoreCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Semaphore> {
        let _function = self.create_semaphore.expect("tried to call a function that isn't loaded");
        let mut semaphore = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut semaphore,
        );
        crate::utils::VulkanResult::new(_return, semaphore)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySemaphore.html) · Function"]
    #[doc(alias = "vkDestroySemaphore")]
    pub unsafe fn destroy_semaphore(&self, semaphore: Option<crate::vk1_0::Semaphore>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_semaphore.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match semaphore {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateEvent.html) · Function"]
    #[doc(alias = "vkCreateEvent")]
    pub unsafe fn create_event(&self, create_info: &crate::vk1_0::EventCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Event> {
        let _function = self.create_event.expect("tried to call a function that isn't loaded");
        let mut event = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut event,
        );
        crate::utils::VulkanResult::new(_return, event)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyEvent.html) · Function"]
    #[doc(alias = "vkDestroyEvent")]
    pub unsafe fn destroy_event(&self, event: Option<crate::vk1_0::Event>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_event.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match event {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetEventStatus.html) · Function"]
    #[doc(alias = "vkGetEventStatus")]
    pub unsafe fn get_event_status(&self, event: crate::vk1_0::Event) -> crate::utils::VulkanResult<()> {
        let _function = self.get_event_status.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, event as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetEvent.html) · Function"]
    #[doc(alias = "vkSetEvent")]
    pub unsafe fn set_event(&self, event: crate::vk1_0::Event) -> crate::utils::VulkanResult<()> {
        let _function = self.set_event.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, event as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetEvent.html) · Function"]
    #[doc(alias = "vkResetEvent")]
    pub unsafe fn reset_event(&self, event: crate::vk1_0::Event) -> crate::utils::VulkanResult<()> {
        let _function = self.reset_event.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, event as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateQueryPool.html) · Function"]
    #[doc(alias = "vkCreateQueryPool")]
    pub unsafe fn create_query_pool(&self, create_info: &crate::vk1_0::QueryPoolCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::QueryPool> {
        let _function = self.create_query_pool.expect("tried to call a function that isn't loaded");
        let mut query_pool = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut query_pool,
        );
        crate::utils::VulkanResult::new(_return, query_pool)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyQueryPool.html) · Function"]
    #[doc(alias = "vkDestroyQueryPool")]
    pub unsafe fn destroy_query_pool(&self, query_pool: Option<crate::vk1_0::QueryPool>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_query_pool.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match query_pool {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueryPoolResults.html) · Function"]
    #[doc(alias = "vkGetQueryPoolResults")]
    pub unsafe fn get_query_pool_results(&self, query_pool: crate::vk1_0::QueryPool, first_query: u32, query_count: u32, data_size: usize, data: *mut std::ffi::c_void, stride: crate::vk1_0::DeviceSize, flags: Option<crate::vk1_0::QueryResultFlags>) -> crate::utils::VulkanResult<()> {
        let _function = self.get_query_pool_results.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            query_pool as _,
            first_query as _,
            query_count as _,
            data_size,
            data,
            stride as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBuffer.html) · Function"]
    #[doc(alias = "vkCreateBuffer")]
    pub unsafe fn create_buffer(&self, create_info: &crate::vk1_0::BufferCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Buffer> {
        let _function = self.create_buffer.expect("tried to call a function that isn't loaded");
        let mut buffer = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut buffer,
        );
        crate::utils::VulkanResult::new(_return, buffer)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBuffer.html) · Function"]
    #[doc(alias = "vkDestroyBuffer")]
    pub unsafe fn destroy_buffer(&self, buffer: Option<crate::vk1_0::Buffer>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_buffer.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match buffer {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBufferView.html) · Function"]
    #[doc(alias = "vkCreateBufferView")]
    pub unsafe fn create_buffer_view(&self, create_info: &crate::vk1_0::BufferViewCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::BufferView> {
        let _function = self.create_buffer_view.expect("tried to call a function that isn't loaded");
        let mut view = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut view,
        );
        crate::utils::VulkanResult::new(_return, view)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBufferView.html) · Function"]
    #[doc(alias = "vkDestroyBufferView")]
    pub unsafe fn destroy_buffer_view(&self, buffer_view: Option<crate::vk1_0::BufferView>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_buffer_view.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match buffer_view {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImage.html) · Function"]
    #[doc(alias = "vkCreateImage")]
    pub unsafe fn create_image(&self, create_info: &crate::vk1_0::ImageCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Image> {
        let _function = self.create_image.expect("tried to call a function that isn't loaded");
        let mut image = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut image,
        );
        crate::utils::VulkanResult::new(_return, image)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImage.html) · Function"]
    #[doc(alias = "vkDestroyImage")]
    pub unsafe fn destroy_image(&self, image: Option<crate::vk1_0::Image>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_image.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match image {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSubresourceLayout.html) · Function"]
    #[doc(alias = "vkGetImageSubresourceLayout")]
    pub unsafe fn get_image_subresource_layout(&self, image: crate::vk1_0::Image, subresource: &crate::vk1_0::ImageSubresource) -> crate::vk1_0::SubresourceLayout {
        let _function = self.get_image_subresource_layout.expect("tried to call a function that isn't loaded");
        let mut layout = Default::default();
        let _return = _function(self.handle, image as _, subresource as _, &mut layout);
        layout
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImageView.html) · Function"]
    #[doc(alias = "vkCreateImageView")]
    pub unsafe fn create_image_view(&self, create_info: &crate::vk1_0::ImageViewCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::ImageView> {
        let _function = self.create_image_view.expect("tried to call a function that isn't loaded");
        let mut view = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut view,
        );
        crate::utils::VulkanResult::new(_return, view)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImageView.html) · Function"]
    #[doc(alias = "vkDestroyImageView")]
    pub unsafe fn destroy_image_view(&self, image_view: Option<crate::vk1_0::ImageView>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_image_view.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match image_view {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateShaderModule.html) · Function"]
    #[doc(alias = "vkCreateShaderModule")]
    pub unsafe fn create_shader_module(&self, create_info: &crate::vk1_0::ShaderModuleCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::ShaderModule> {
        let _function = self.create_shader_module.expect("tried to call a function that isn't loaded");
        let mut shader_module = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut shader_module,
        );
        crate::utils::VulkanResult::new(_return, shader_module)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyShaderModule.html) · Function"]
    #[doc(alias = "vkDestroyShaderModule")]
    pub unsafe fn destroy_shader_module(&self, shader_module: Option<crate::vk1_0::ShaderModule>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_shader_module.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match shader_module {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineCache.html) · Function"]
    #[doc(alias = "vkCreatePipelineCache")]
    pub unsafe fn create_pipeline_cache(&self, create_info: &crate::vk1_0::PipelineCacheCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::PipelineCache> {
        let _function = self.create_pipeline_cache.expect("tried to call a function that isn't loaded");
        let mut pipeline_cache = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut pipeline_cache,
        );
        crate::utils::VulkanResult::new(_return, pipeline_cache)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineCache.html) · Function"]
    #[doc(alias = "vkDestroyPipelineCache")]
    pub unsafe fn destroy_pipeline_cache(&self, pipeline_cache: Option<crate::vk1_0::PipelineCache>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_pipeline_cache.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match pipeline_cache {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineCacheData.html) · Function"]
    #[doc(alias = "vkGetPipelineCacheData")]
    pub unsafe fn get_pipeline_cache_data(&self, pipeline_cache: crate::vk1_0::PipelineCache, data_size: *mut usize, data: *mut std::ffi::c_void) -> crate::utils::VulkanResult<()> {
        let _function = self.get_pipeline_cache_data.expect("tried to call a function that isn't loaded");
        let _return = _function(self.handle, pipeline_cache as _, data_size, data);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergePipelineCaches.html) · Function"]
    #[doc(alias = "vkMergePipelineCaches")]
    pub unsafe fn merge_pipeline_caches(&self, dst_cache: crate::vk1_0::PipelineCache, src_caches: &[crate::vk1_0::PipelineCache]) -> crate::utils::VulkanResult<()> {
        let _function = self.merge_pipeline_caches.expect("tried to call a function that isn't loaded");
        let src_cache_count = src_caches.len();
        let _return = _function(self.handle, dst_cache as _, src_cache_count as _, src_caches.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateGraphicsPipelines.html) · Function"]
    #[doc(alias = "vkCreateGraphicsPipelines")]
    pub unsafe fn create_graphics_pipelines(&self, pipeline_cache: Option<crate::vk1_0::PipelineCache>, create_infos: &[crate::vk1_0::GraphicsPipelineCreateInfoBuilder], allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>> {
        let _function = self.create_graphics_pipelines.expect("tried to call a function that isn't loaded");
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as _];
        let _return = _function(
            self.handle,
            match pipeline_cache {
                Some(v) => v,
                None => Default::default(),
            },
            create_info_count as _,
            create_infos.as_ptr() as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            pipelines.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, pipelines)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateComputePipelines.html) · Function"]
    #[doc(alias = "vkCreateComputePipelines")]
    pub unsafe fn create_compute_pipelines(&self, pipeline_cache: Option<crate::vk1_0::PipelineCache>, create_infos: &[crate::vk1_0::ComputePipelineCreateInfoBuilder], allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<Vec<crate::vk1_0::Pipeline>> {
        let _function = self.create_compute_pipelines.expect("tried to call a function that isn't loaded");
        let create_info_count = create_infos.len();
        let mut pipelines = vec![Default::default(); create_info_count as _];
        let _return = _function(
            self.handle,
            match pipeline_cache {
                Some(v) => v,
                None => Default::default(),
            },
            create_info_count as _,
            create_infos.as_ptr() as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            pipelines.as_mut_ptr(),
        );
        crate::utils::VulkanResult::new(_return, pipelines)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipeline.html) · Function"]
    #[doc(alias = "vkDestroyPipeline")]
    pub unsafe fn destroy_pipeline(&self, pipeline: Option<crate::vk1_0::Pipeline>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_pipeline.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match pipeline {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineLayout.html) · Function"]
    #[doc(alias = "vkCreatePipelineLayout")]
    pub unsafe fn create_pipeline_layout(&self, create_info: &crate::vk1_0::PipelineLayoutCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::PipelineLayout> {
        let _function = self.create_pipeline_layout.expect("tried to call a function that isn't loaded");
        let mut pipeline_layout = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut pipeline_layout,
        );
        crate::utils::VulkanResult::new(_return, pipeline_layout)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineLayout.html) · Function"]
    #[doc(alias = "vkDestroyPipelineLayout")]
    pub unsafe fn destroy_pipeline_layout(&self, pipeline_layout: Option<crate::vk1_0::PipelineLayout>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_pipeline_layout.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match pipeline_layout {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSampler.html) · Function"]
    #[doc(alias = "vkCreateSampler")]
    pub unsafe fn create_sampler(&self, create_info: &crate::vk1_0::SamplerCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Sampler> {
        let _function = self.create_sampler.expect("tried to call a function that isn't loaded");
        let mut sampler = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut sampler,
        );
        crate::utils::VulkanResult::new(_return, sampler)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySampler.html) · Function"]
    #[doc(alias = "vkDestroySampler")]
    pub unsafe fn destroy_sampler(&self, sampler: Option<crate::vk1_0::Sampler>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_sampler.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match sampler {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorSetLayout.html) · Function"]
    #[doc(alias = "vkCreateDescriptorSetLayout")]
    pub unsafe fn create_descriptor_set_layout(&self, create_info: &crate::vk1_0::DescriptorSetLayoutCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::DescriptorSetLayout> {
        let _function = self.create_descriptor_set_layout.expect("tried to call a function that isn't loaded");
        let mut set_layout = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut set_layout,
        );
        crate::utils::VulkanResult::new(_return, set_layout)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorSetLayout.html) · Function"]
    #[doc(alias = "vkDestroyDescriptorSetLayout")]
    pub unsafe fn destroy_descriptor_set_layout(&self, descriptor_set_layout: Option<crate::vk1_0::DescriptorSetLayout>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_descriptor_set_layout.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match descriptor_set_layout {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorPool.html) · Function"]
    #[doc(alias = "vkCreateDescriptorPool")]
    pub unsafe fn create_descriptor_pool(&self, create_info: &crate::vk1_0::DescriptorPoolCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::DescriptorPool> {
        let _function = self.create_descriptor_pool.expect("tried to call a function that isn't loaded");
        let mut descriptor_pool = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut descriptor_pool,
        );
        crate::utils::VulkanResult::new(_return, descriptor_pool)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorPool.html) · Function"]
    #[doc(alias = "vkDestroyDescriptorPool")]
    pub unsafe fn destroy_descriptor_pool(&self, descriptor_pool: Option<crate::vk1_0::DescriptorPool>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_descriptor_pool.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match descriptor_pool {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetDescriptorPool.html) · Function"]
    #[doc(alias = "vkResetDescriptorPool")]
    pub unsafe fn reset_descriptor_pool(&self, descriptor_pool: crate::vk1_0::DescriptorPool, flags: Option<crate::vk1_0::DescriptorPoolResetFlags>) -> crate::utils::VulkanResult<()> {
        let _function = self.reset_descriptor_pool.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            descriptor_pool as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateDescriptorSets.html) · Function"]
    #[doc(alias = "vkAllocateDescriptorSets")]
    pub unsafe fn allocate_descriptor_sets(&self, allocate_info: &crate::vk1_0::DescriptorSetAllocateInfo) -> crate::utils::VulkanResult<Vec<crate::vk1_0::DescriptorSet>> {
        let _function = self.allocate_descriptor_sets.expect("tried to call a function that isn't loaded");
        let mut descriptor_sets = vec![Default::default(); allocate_info.descriptor_set_count as _];
        let _return = _function(self.handle, allocate_info as _, descriptor_sets.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, descriptor_sets)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeDescriptorSets.html) · Function"]
    #[doc(alias = "vkFreeDescriptorSets")]
    pub unsafe fn free_descriptor_sets(&self, descriptor_pool: crate::vk1_0::DescriptorPool, descriptor_sets: &[crate::vk1_0::DescriptorSet]) -> crate::utils::VulkanResult<()> {
        let _function = self.free_descriptor_sets.expect("tried to call a function that isn't loaded");
        let descriptor_set_count = descriptor_sets.len();
        let _return = _function(self.handle, descriptor_pool as _, descriptor_set_count as _, descriptor_sets.as_ptr() as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSets.html) · Function"]
    #[doc(alias = "vkUpdateDescriptorSets")]
    pub unsafe fn update_descriptor_sets(&self, descriptor_writes: &[crate::vk1_0::WriteDescriptorSetBuilder], descriptor_copies: &[crate::vk1_0::CopyDescriptorSetBuilder]) -> () {
        let _function = self.update_descriptor_sets.expect("tried to call a function that isn't loaded");
        let descriptor_write_count = descriptor_writes.len();
        let descriptor_copy_count = descriptor_copies.len();
        let _return = _function(self.handle, descriptor_write_count as _, descriptor_writes.as_ptr() as _, descriptor_copy_count as _, descriptor_copies.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFramebuffer.html) · Function"]
    #[doc(alias = "vkCreateFramebuffer")]
    pub unsafe fn create_framebuffer(&self, create_info: &crate::vk1_0::FramebufferCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::Framebuffer> {
        let _function = self.create_framebuffer.expect("tried to call a function that isn't loaded");
        let mut framebuffer = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut framebuffer,
        );
        crate::utils::VulkanResult::new(_return, framebuffer)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFramebuffer.html) · Function"]
    #[doc(alias = "vkDestroyFramebuffer")]
    pub unsafe fn destroy_framebuffer(&self, framebuffer: Option<crate::vk1_0::Framebuffer>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_framebuffer.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match framebuffer {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass.html) · Function"]
    #[doc(alias = "vkCreateRenderPass")]
    pub unsafe fn create_render_pass(&self, create_info: &crate::vk1_0::RenderPassCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::RenderPass> {
        let _function = self.create_render_pass.expect("tried to call a function that isn't loaded");
        let mut render_pass = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut render_pass,
        );
        crate::utils::VulkanResult::new(_return, render_pass)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyRenderPass.html) · Function"]
    #[doc(alias = "vkDestroyRenderPass")]
    pub unsafe fn destroy_render_pass(&self, render_pass: Option<crate::vk1_0::RenderPass>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_render_pass.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match render_pass {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRenderAreaGranularity.html) · Function"]
    #[doc(alias = "vkGetRenderAreaGranularity")]
    pub unsafe fn get_render_area_granularity(&self, render_pass: crate::vk1_0::RenderPass) -> crate::vk1_0::Extent2D {
        let _function = self.get_render_area_granularity.expect("tried to call a function that isn't loaded");
        let mut granularity = Default::default();
        let _return = _function(self.handle, render_pass as _, &mut granularity);
        granularity
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCommandPool.html) · Function"]
    #[doc(alias = "vkCreateCommandPool")]
    pub unsafe fn create_command_pool(&self, create_info: &crate::vk1_0::CommandPoolCreateInfo, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> crate::utils::VulkanResult<crate::vk1_0::CommandPool> {
        let _function = self.create_command_pool.expect("tried to call a function that isn't loaded");
        let mut command_pool = Default::default();
        let _return = _function(
            self.handle,
            create_info as _,
            match allocator {
                Some(v) => v,
                None => std::ptr::null(),
            },
            &mut command_pool,
        );
        crate::utils::VulkanResult::new(_return, command_pool)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCommandPool.html) · Function"]
    #[doc(alias = "vkDestroyCommandPool")]
    pub unsafe fn destroy_command_pool(&self, command_pool: Option<crate::vk1_0::CommandPool>, allocator: Option<&crate::vk1_0::AllocationCallbacks>) -> () {
        let _function = self.destroy_command_pool.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            match command_pool {
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
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandPool.html) · Function"]
    #[doc(alias = "vkResetCommandPool")]
    pub unsafe fn reset_command_pool(&self, command_pool: crate::vk1_0::CommandPool, flags: Option<crate::vk1_0::CommandPoolResetFlags>) -> crate::utils::VulkanResult<()> {
        let _function = self.reset_command_pool.expect("tried to call a function that isn't loaded");
        let _return = _function(
            self.handle,
            command_pool as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateCommandBuffers.html) · Function"]
    #[doc(alias = "vkAllocateCommandBuffers")]
    pub unsafe fn allocate_command_buffers(&self, allocate_info: &crate::vk1_0::CommandBufferAllocateInfo) -> crate::utils::VulkanResult<Vec<crate::vk1_0::CommandBuffer>> {
        let _function = self.allocate_command_buffers.expect("tried to call a function that isn't loaded");
        let mut command_buffers = vec![Default::default(); allocate_info.command_buffer_count as _];
        let _return = _function(self.handle, allocate_info as _, command_buffers.as_mut_ptr());
        crate::utils::VulkanResult::new(_return, command_buffers)
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeCommandBuffers.html) · Function"]
    #[doc(alias = "vkFreeCommandBuffers")]
    pub unsafe fn free_command_buffers(&self, command_pool: crate::vk1_0::CommandPool, command_buffers: &[crate::vk1_0::CommandBuffer]) -> () {
        let _function = self.free_command_buffers.expect("tried to call a function that isn't loaded");
        let command_buffer_count = command_buffers.len();
        let _return = _function(self.handle, command_pool as _, command_buffer_count as _, command_buffers.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBeginCommandBuffer.html) · Function"]
    #[doc(alias = "vkBeginCommandBuffer")]
    pub unsafe fn begin_command_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer, begin_info: &crate::vk1_0::CommandBufferBeginInfo) -> crate::utils::VulkanResult<()> {
        let _function = self.begin_command_buffer.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, begin_info as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEndCommandBuffer.html) · Function"]
    #[doc(alias = "vkEndCommandBuffer")]
    pub unsafe fn end_command_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer) -> crate::utils::VulkanResult<()> {
        let _function = self.end_command_buffer.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandBuffer.html) · Function"]
    #[doc(alias = "vkResetCommandBuffer")]
    pub unsafe fn reset_command_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer, flags: Option<crate::vk1_0::CommandBufferResetFlags>) -> crate::utils::VulkanResult<()> {
        let _function = self.reset_command_buffer.expect("tried to call a function that isn't loaded");
        let _return = _function(
            command_buffer as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipeline.html) · Function"]
    #[doc(alias = "vkCmdBindPipeline")]
    pub unsafe fn cmd_bind_pipeline(&self, command_buffer: crate::vk1_0::CommandBuffer, pipeline_bind_point: crate::vk1_0::PipelineBindPoint, pipeline: crate::vk1_0::Pipeline) -> () {
        let _function = self.cmd_bind_pipeline.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, pipeline_bind_point as _, pipeline as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewport.html) · Function"]
    #[doc(alias = "vkCmdSetViewport")]
    pub unsafe fn cmd_set_viewport(&self, command_buffer: crate::vk1_0::CommandBuffer, first_viewport: u32, viewports: &[crate::vk1_0::ViewportBuilder]) -> () {
        let _function = self.cmd_set_viewport.expect("tried to call a function that isn't loaded");
        let viewport_count = viewports.len();
        let _return = _function(command_buffer as _, first_viewport as _, viewport_count as _, viewports.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissor.html) · Function"]
    #[doc(alias = "vkCmdSetScissor")]
    pub unsafe fn cmd_set_scissor(&self, command_buffer: crate::vk1_0::CommandBuffer, first_scissor: u32, scissors: &[crate::vk1_0::Rect2DBuilder]) -> () {
        let _function = self.cmd_set_scissor.expect("tried to call a function that isn't loaded");
        let scissor_count = scissors.len();
        let _return = _function(command_buffer as _, first_scissor as _, scissor_count as _, scissors.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineWidth.html) · Function"]
    #[doc(alias = "vkCmdSetLineWidth")]
    pub unsafe fn cmd_set_line_width(&self, command_buffer: crate::vk1_0::CommandBuffer, line_width: std::os::raw::c_float) -> () {
        let _function = self.cmd_set_line_width.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, line_width as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBias.html) · Function"]
    #[doc(alias = "vkCmdSetDepthBias")]
    pub unsafe fn cmd_set_depth_bias(&self, command_buffer: crate::vk1_0::CommandBuffer, depth_bias_constant_factor: std::os::raw::c_float, depth_bias_clamp: std::os::raw::c_float, depth_bias_slope_factor: std::os::raw::c_float) -> () {
        let _function = self.cmd_set_depth_bias.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, depth_bias_constant_factor as _, depth_bias_clamp as _, depth_bias_slope_factor as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetBlendConstants.html) · Function"]
    #[doc(alias = "vkCmdSetBlendConstants")]
    pub unsafe fn cmd_set_blend_constants(&self, command_buffer: crate::vk1_0::CommandBuffer, blend_constants: [std::os::raw::c_float; 4]) -> () {
        let _function = self.cmd_set_blend_constants.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, blend_constants as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBounds.html) · Function"]
    #[doc(alias = "vkCmdSetDepthBounds")]
    pub unsafe fn cmd_set_depth_bounds(&self, command_buffer: crate::vk1_0::CommandBuffer, min_depth_bounds: std::os::raw::c_float, max_depth_bounds: std::os::raw::c_float) -> () {
        let _function = self.cmd_set_depth_bounds.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, min_depth_bounds as _, max_depth_bounds as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilCompareMask.html) · Function"]
    #[doc(alias = "vkCmdSetStencilCompareMask")]
    pub unsafe fn cmd_set_stencil_compare_mask(&self, command_buffer: crate::vk1_0::CommandBuffer, face_mask: crate::vk1_0::StencilFaceFlags, compare_mask: u32) -> () {
        let _function = self.cmd_set_stencil_compare_mask.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, face_mask as _, compare_mask as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilWriteMask.html) · Function"]
    #[doc(alias = "vkCmdSetStencilWriteMask")]
    pub unsafe fn cmd_set_stencil_write_mask(&self, command_buffer: crate::vk1_0::CommandBuffer, face_mask: crate::vk1_0::StencilFaceFlags, write_mask: u32) -> () {
        let _function = self.cmd_set_stencil_write_mask.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, face_mask as _, write_mask as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilReference.html) · Function"]
    #[doc(alias = "vkCmdSetStencilReference")]
    pub unsafe fn cmd_set_stencil_reference(&self, command_buffer: crate::vk1_0::CommandBuffer, face_mask: crate::vk1_0::StencilFaceFlags, reference: u32) -> () {
        let _function = self.cmd_set_stencil_reference.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, face_mask as _, reference as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindDescriptorSets.html) · Function"]
    #[doc(alias = "vkCmdBindDescriptorSets")]
    pub unsafe fn cmd_bind_descriptor_sets(&self, command_buffer: crate::vk1_0::CommandBuffer, pipeline_bind_point: crate::vk1_0::PipelineBindPoint, layout: crate::vk1_0::PipelineLayout, first_set: u32, descriptor_sets: &[crate::vk1_0::DescriptorSet], dynamic_offsets: &[u32]) -> () {
        let _function = self.cmd_bind_descriptor_sets.expect("tried to call a function that isn't loaded");
        let descriptor_set_count = descriptor_sets.len();
        let dynamic_offset_count = dynamic_offsets.len();
        let _return = _function(command_buffer as _, pipeline_bind_point as _, layout as _, first_set as _, descriptor_set_count as _, descriptor_sets.as_ptr() as _, dynamic_offset_count as _, dynamic_offsets.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindIndexBuffer.html) · Function"]
    #[doc(alias = "vkCmdBindIndexBuffer")]
    pub unsafe fn cmd_bind_index_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, index_type: crate::vk1_0::IndexType) -> () {
        let _function = self.cmd_bind_index_buffer.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, buffer as _, offset as _, index_type as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers.html) · Function"]
    #[doc(alias = "vkCmdBindVertexBuffers")]
    pub unsafe fn cmd_bind_vertex_buffers(&self, command_buffer: crate::vk1_0::CommandBuffer, first_binding: u32, buffers: &[crate::vk1_0::Buffer], offsets: &[crate::vk1_0::DeviceSize]) -> () {
        let _function = self.cmd_bind_vertex_buffers.expect("tried to call a function that isn't loaded");
        let binding_count = buffers.len().min(offsets.len());
        let _return = _function(command_buffer as _, first_binding as _, binding_count as _, buffers.as_ptr() as _, offsets.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDraw.html) · Function"]
    #[doc(alias = "vkCmdDraw")]
    pub unsafe fn cmd_draw(&self, command_buffer: crate::vk1_0::CommandBuffer, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32) -> () {
        let _function = self.cmd_draw.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, vertex_count as _, instance_count as _, first_vertex as _, first_instance as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexed.html) · Function"]
    #[doc(alias = "vkCmdDrawIndexed")]
    pub unsafe fn cmd_draw_indexed(&self, command_buffer: crate::vk1_0::CommandBuffer, index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32) -> () {
        let _function = self.cmd_draw_indexed.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, index_count as _, instance_count as _, first_index as _, vertex_offset as _, first_instance as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirect.html) · Function"]
    #[doc(alias = "vkCmdDrawIndirect")]
    pub unsafe fn cmd_draw_indirect(&self, command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, draw_count: u32, stride: u32) -> () {
        let _function = self.cmd_draw_indirect.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, buffer as _, offset as _, draw_count as _, stride as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirect.html) · Function"]
    #[doc(alias = "vkCmdDrawIndexedIndirect")]
    pub unsafe fn cmd_draw_indexed_indirect(&self, command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize, draw_count: u32, stride: u32) -> () {
        let _function = self.cmd_draw_indexed_indirect.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, buffer as _, offset as _, draw_count as _, stride as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatch.html) · Function"]
    #[doc(alias = "vkCmdDispatch")]
    pub unsafe fn cmd_dispatch(&self, command_buffer: crate::vk1_0::CommandBuffer, group_count_x: u32, group_count_y: u32, group_count_z: u32) -> () {
        let _function = self.cmd_dispatch.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, group_count_x as _, group_count_y as _, group_count_z as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchIndirect.html) · Function"]
    #[doc(alias = "vkCmdDispatchIndirect")]
    pub unsafe fn cmd_dispatch_indirect(&self, command_buffer: crate::vk1_0::CommandBuffer, buffer: crate::vk1_0::Buffer, offset: crate::vk1_0::DeviceSize) -> () {
        let _function = self.cmd_dispatch_indirect.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, buffer as _, offset as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer.html) · Function"]
    #[doc(alias = "vkCmdCopyBuffer")]
    pub unsafe fn cmd_copy_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer, src_buffer: crate::vk1_0::Buffer, dst_buffer: crate::vk1_0::Buffer, regions: &[crate::vk1_0::BufferCopyBuilder]) -> () {
        let _function = self.cmd_copy_buffer.expect("tried to call a function that isn't loaded");
        let region_count = regions.len();
        let _return = _function(command_buffer as _, src_buffer as _, dst_buffer as _, region_count as _, regions.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage.html) · Function"]
    #[doc(alias = "vkCmdCopyImage")]
    pub unsafe fn cmd_copy_image(&self, command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, regions: &[crate::vk1_0::ImageCopyBuilder]) -> () {
        let _function = self.cmd_copy_image.expect("tried to call a function that isn't loaded");
        let region_count = regions.len();
        let _return = _function(command_buffer as _, src_image as _, src_image_layout as _, dst_image as _, dst_image_layout as _, region_count as _, regions.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage.html) · Function"]
    #[doc(alias = "vkCmdBlitImage")]
    pub unsafe fn cmd_blit_image(&self, command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, regions: &[crate::vk1_0::ImageBlitBuilder], filter: crate::vk1_0::Filter) -> () {
        let _function = self.cmd_blit_image.expect("tried to call a function that isn't loaded");
        let region_count = regions.len();
        let _return = _function(command_buffer as _, src_image as _, src_image_layout as _, dst_image as _, dst_image_layout as _, region_count as _, regions.as_ptr() as _, filter as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage.html) · Function"]
    #[doc(alias = "vkCmdCopyBufferToImage")]
    pub unsafe fn cmd_copy_buffer_to_image(&self, command_buffer: crate::vk1_0::CommandBuffer, src_buffer: crate::vk1_0::Buffer, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, regions: &[crate::vk1_0::BufferImageCopyBuilder]) -> () {
        let _function = self.cmd_copy_buffer_to_image.expect("tried to call a function that isn't loaded");
        let region_count = regions.len();
        let _return = _function(command_buffer as _, src_buffer as _, dst_image as _, dst_image_layout as _, region_count as _, regions.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer.html) · Function"]
    #[doc(alias = "vkCmdCopyImageToBuffer")]
    pub unsafe fn cmd_copy_image_to_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_buffer: crate::vk1_0::Buffer, regions: &[crate::vk1_0::BufferImageCopyBuilder]) -> () {
        let _function = self.cmd_copy_image_to_buffer.expect("tried to call a function that isn't loaded");
        let region_count = regions.len();
        let _return = _function(command_buffer as _, src_image as _, src_image_layout as _, dst_buffer as _, region_count as _, regions.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdUpdateBuffer.html) · Function"]
    #[doc(alias = "vkCmdUpdateBuffer")]
    pub unsafe fn cmd_update_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, data_size: crate::vk1_0::DeviceSize, data: *const std::ffi::c_void) -> () {
        let _function = self.cmd_update_buffer.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, dst_buffer as _, dst_offset as _, data_size, data);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdFillBuffer.html) · Function"]
    #[doc(alias = "vkCmdFillBuffer")]
    pub unsafe fn cmd_fill_buffer(&self, command_buffer: crate::vk1_0::CommandBuffer, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, size: crate::vk1_0::DeviceSize, data: u32) -> () {
        let _function = self.cmd_fill_buffer.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, dst_buffer as _, dst_offset as _, size as _, data as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearColorImage.html) · Function"]
    #[doc(alias = "vkCmdClearColorImage")]
    pub unsafe fn cmd_clear_color_image(&self, command_buffer: crate::vk1_0::CommandBuffer, image: crate::vk1_0::Image, image_layout: crate::vk1_0::ImageLayout, color: &crate::vk1_0::ClearColorValue, ranges: &[crate::vk1_0::ImageSubresourceRangeBuilder]) -> () {
        let _function = self.cmd_clear_color_image.expect("tried to call a function that isn't loaded");
        let range_count = ranges.len();
        let _return = _function(command_buffer as _, image as _, image_layout as _, color as _, range_count as _, ranges.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearDepthStencilImage.html) · Function"]
    #[doc(alias = "vkCmdClearDepthStencilImage")]
    pub unsafe fn cmd_clear_depth_stencil_image(&self, command_buffer: crate::vk1_0::CommandBuffer, image: crate::vk1_0::Image, image_layout: crate::vk1_0::ImageLayout, depth_stencil: &crate::vk1_0::ClearDepthStencilValue, ranges: &[crate::vk1_0::ImageSubresourceRangeBuilder]) -> () {
        let _function = self.cmd_clear_depth_stencil_image.expect("tried to call a function that isn't loaded");
        let range_count = ranges.len();
        let _return = _function(command_buffer as _, image as _, image_layout as _, depth_stencil as _, range_count as _, ranges.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearAttachments.html) · Function"]
    #[doc(alias = "vkCmdClearAttachments")]
    pub unsafe fn cmd_clear_attachments(&self, command_buffer: crate::vk1_0::CommandBuffer, attachments: &[crate::vk1_0::ClearAttachmentBuilder], rects: &[crate::vk1_0::ClearRectBuilder]) -> () {
        let _function = self.cmd_clear_attachments.expect("tried to call a function that isn't loaded");
        let attachment_count = attachments.len();
        let rect_count = rects.len();
        let _return = _function(command_buffer as _, attachment_count as _, attachments.as_ptr() as _, rect_count as _, rects.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage.html) · Function"]
    #[doc(alias = "vkCmdResolveImage")]
    pub unsafe fn cmd_resolve_image(&self, command_buffer: crate::vk1_0::CommandBuffer, src_image: crate::vk1_0::Image, src_image_layout: crate::vk1_0::ImageLayout, dst_image: crate::vk1_0::Image, dst_image_layout: crate::vk1_0::ImageLayout, regions: &[crate::vk1_0::ImageResolveBuilder]) -> () {
        let _function = self.cmd_resolve_image.expect("tried to call a function that isn't loaded");
        let region_count = regions.len();
        let _return = _function(command_buffer as _, src_image as _, src_image_layout as _, dst_image as _, dst_image_layout as _, region_count as _, regions.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetEvent.html) · Function"]
    #[doc(alias = "vkCmdSetEvent")]
    pub unsafe fn cmd_set_event(&self, command_buffer: crate::vk1_0::CommandBuffer, event: crate::vk1_0::Event, stage_mask: crate::vk1_0::PipelineStageFlags) -> () {
        let _function = self.cmd_set_event.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, event as _, stage_mask as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetEvent.html) · Function"]
    #[doc(alias = "vkCmdResetEvent")]
    pub unsafe fn cmd_reset_event(&self, command_buffer: crate::vk1_0::CommandBuffer, event: crate::vk1_0::Event, stage_mask: crate::vk1_0::PipelineStageFlags) -> () {
        let _function = self.cmd_reset_event.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, event as _, stage_mask as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWaitEvents.html) · Function"]
    #[doc(alias = "vkCmdWaitEvents")]
    pub unsafe fn cmd_wait_events(&self, command_buffer: crate::vk1_0::CommandBuffer, events: &[crate::vk1_0::Event], src_stage_mask: Option<crate::vk1_0::PipelineStageFlags>, dst_stage_mask: Option<crate::vk1_0::PipelineStageFlags>, memory_barriers: &[crate::vk1_0::MemoryBarrierBuilder], buffer_memory_barriers: &[crate::vk1_0::BufferMemoryBarrierBuilder], image_memory_barriers: &[crate::vk1_0::ImageMemoryBarrierBuilder]) -> () {
        let _function = self.cmd_wait_events.expect("tried to call a function that isn't loaded");
        let event_count = events.len();
        let memory_barrier_count = memory_barriers.len();
        let buffer_memory_barrier_count = buffer_memory_barriers.len();
        let image_memory_barrier_count = image_memory_barriers.len();
        let _return = _function(
            command_buffer as _,
            event_count as _,
            events.as_ptr() as _,
            match src_stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
            match dst_stage_mask {
                Some(v) => v,
                None => Default::default(),
            },
            memory_barrier_count as _,
            memory_barriers.as_ptr() as _,
            buffer_memory_barrier_count as _,
            buffer_memory_barriers.as_ptr() as _,
            image_memory_barrier_count as _,
            image_memory_barriers.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier.html) · Function"]
    #[doc(alias = "vkCmdPipelineBarrier")]
    pub unsafe fn cmd_pipeline_barrier(&self, command_buffer: crate::vk1_0::CommandBuffer, src_stage_mask: crate::vk1_0::PipelineStageFlags, dst_stage_mask: crate::vk1_0::PipelineStageFlags, dependency_flags: Option<crate::vk1_0::DependencyFlags>, memory_barriers: &[crate::vk1_0::MemoryBarrierBuilder], buffer_memory_barriers: &[crate::vk1_0::BufferMemoryBarrierBuilder], image_memory_barriers: &[crate::vk1_0::ImageMemoryBarrierBuilder]) -> () {
        let _function = self.cmd_pipeline_barrier.expect("tried to call a function that isn't loaded");
        let memory_barrier_count = memory_barriers.len();
        let buffer_memory_barrier_count = buffer_memory_barriers.len();
        let image_memory_barrier_count = image_memory_barriers.len();
        let _return = _function(
            command_buffer as _,
            src_stage_mask as _,
            dst_stage_mask as _,
            match dependency_flags {
                Some(v) => v,
                None => Default::default(),
            },
            memory_barrier_count as _,
            memory_barriers.as_ptr() as _,
            buffer_memory_barrier_count as _,
            buffer_memory_barriers.as_ptr() as _,
            image_memory_barrier_count as _,
            image_memory_barriers.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQuery.html) · Function"]
    #[doc(alias = "vkCmdBeginQuery")]
    pub unsafe fn cmd_begin_query(&self, command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, query: u32, flags: Option<crate::vk1_0::QueryControlFlags>) -> () {
        let _function = self.cmd_begin_query.expect("tried to call a function that isn't loaded");
        let _return = _function(
            command_buffer as _,
            query_pool as _,
            query as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQuery.html) · Function"]
    #[doc(alias = "vkCmdEndQuery")]
    pub unsafe fn cmd_end_query(&self, command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, query: u32) -> () {
        let _function = self.cmd_end_query.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, query_pool as _, query as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetQueryPool.html) · Function"]
    #[doc(alias = "vkCmdResetQueryPool")]
    pub unsafe fn cmd_reset_query_pool(&self, command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, first_query: u32, query_count: u32) -> () {
        let _function = self.cmd_reset_query_pool.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, query_pool as _, first_query as _, query_count as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteTimestamp.html) · Function"]
    #[doc(alias = "vkCmdWriteTimestamp")]
    pub unsafe fn cmd_write_timestamp(&self, command_buffer: crate::vk1_0::CommandBuffer, pipeline_stage: crate::vk1_0::PipelineStageFlagBits, query_pool: crate::vk1_0::QueryPool, query: u32) -> () {
        let _function = self.cmd_write_timestamp.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, pipeline_stage as _, query_pool as _, query as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyQueryPoolResults.html) · Function"]
    #[doc(alias = "vkCmdCopyQueryPoolResults")]
    pub unsafe fn cmd_copy_query_pool_results(&self, command_buffer: crate::vk1_0::CommandBuffer, query_pool: crate::vk1_0::QueryPool, first_query: u32, query_count: u32, dst_buffer: crate::vk1_0::Buffer, dst_offset: crate::vk1_0::DeviceSize, stride: crate::vk1_0::DeviceSize, flags: Option<crate::vk1_0::QueryResultFlags>) -> () {
        let _function = self.cmd_copy_query_pool_results.expect("tried to call a function that isn't loaded");
        let _return = _function(
            command_buffer as _,
            query_pool as _,
            first_query as _,
            query_count as _,
            dst_buffer as _,
            dst_offset as _,
            stride as _,
            match flags {
                Some(v) => v,
                None => Default::default(),
            },
        );
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushConstants.html) · Function"]
    #[doc(alias = "vkCmdPushConstants")]
    pub unsafe fn cmd_push_constants(&self, command_buffer: crate::vk1_0::CommandBuffer, layout: crate::vk1_0::PipelineLayout, stage_flags: crate::vk1_0::ShaderStageFlags, offset: u32, size: u32, values: *const std::ffi::c_void) -> () {
        let _function = self.cmd_push_constants.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, layout as _, stage_flags as _, offset as _, size, values);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass.html) · Function"]
    #[doc(alias = "vkCmdBeginRenderPass")]
    pub unsafe fn cmd_begin_render_pass(&self, command_buffer: crate::vk1_0::CommandBuffer, render_pass_begin: &crate::vk1_0::RenderPassBeginInfo, contents: crate::vk1_0::SubpassContents) -> () {
        let _function = self.cmd_begin_render_pass.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, render_pass_begin as _, contents as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass.html) · Function"]
    #[doc(alias = "vkCmdNextSubpass")]
    pub unsafe fn cmd_next_subpass(&self, command_buffer: crate::vk1_0::CommandBuffer, contents: crate::vk1_0::SubpassContents) -> () {
        let _function = self.cmd_next_subpass.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, contents as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass.html) · Function"]
    #[doc(alias = "vkCmdEndRenderPass")]
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: crate::vk1_0::CommandBuffer) -> () {
        let _function = self.cmd_end_render_pass.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteCommands.html) · Function"]
    #[doc(alias = "vkCmdExecuteCommands")]
    pub unsafe fn cmd_execute_commands(&self, command_buffer: crate::vk1_0::CommandBuffer, command_buffers: &[crate::vk1_0::CommandBuffer]) -> () {
        let _function = self.cmd_execute_commands.expect("tried to call a function that isn't loaded");
        let command_buffer_count = command_buffers.len();
        let _return = _function(command_buffer as _, command_buffer_count as _, command_buffers.as_ptr() as _);
        ()
    }
}
