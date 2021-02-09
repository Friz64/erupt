#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_SPEC_VERSION")]
pub const KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME")]
pub const KHR_COPY_COMMANDS_2_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_copy_commands2");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_BUFFER2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCmdCopyBuffer2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_IMAGE2_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdCopyImage2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BLIT_IMAGE2_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdBlitImage2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_BUFFER_TO_IMAGE2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCmdCopyBufferToImage2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_COPY_IMAGE_TO_BUFFER2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCmdCopyImageToBuffer2KHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_RESOLVE_IMAGE2_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkCmdResolveImage2KHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_copy_buffer_info: *const crate::extensions::khr_copy_commands2::CopyBufferInfo2KHR,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_copy_image_info: *const crate::extensions::khr_copy_commands2::CopyImageInfo2KHR,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_blit_image_info: *const crate::extensions::khr_copy_commands2::BlitImageInfo2KHR,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage2KHR = unsafe extern "system" fn (command_buffer : crate :: vk1_0 :: CommandBuffer , p_copy_buffer_to_image_info : * const crate :: extensions :: khr_copy_commands2 :: CopyBufferToImageInfo2KHR) -> () ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer2KHR = unsafe extern "system" fn (command_buffer : crate :: vk1_0 :: CommandBuffer , p_copy_image_to_buffer_info : * const crate :: extensions :: khr_copy_commands2 :: CopyImageToBufferInfo2KHR) -> () ;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage2KHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage2KHR = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_resolve_image_info: *const crate::extensions::khr_copy_commands2::ResolveImageInfo2KHR,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy2KHR.html) · Structure"]
#[doc(alias = "VkBufferCopy2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferCopy2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_offset: crate::vk1_0::DeviceSize,
    pub dst_offset: crate::vk1_0::DeviceSize,
    pub size: crate::vk1_0::DeviceSize,
}
impl Default for BufferCopy2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BUFFER_COPY_2_KHR,
            p_next: std::ptr::null(),
            src_offset: Default::default(),
            dst_offset: Default::default(),
            size: Default::default(),
        }
    }
}
impl std::fmt::Debug for BufferCopy2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferCopy2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_offset", &self.src_offset)
            .field("dst_offset", &self.dst_offset)
            .field("size", &self.size)
            .finish()
    }
}
impl BufferCopy2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferCopy2KHRBuilder<'a> {
        BufferCopy2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferCopy2KHR.html) · Builder of [`BufferCopy2KHR`]"]
#[repr(transparent)]
pub struct BufferCopy2KHRBuilder<'a>(BufferCopy2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> BufferCopy2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> BufferCopy2KHRBuilder<'a> {
        BufferCopy2KHRBuilder(Default::default(), std::marker::PhantomData)
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferCopy2KHR {
        self.0
    }
}
impl<'a> std::default::Default for BufferCopy2KHRBuilder<'a> {
    fn default() -> BufferCopy2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferCopy2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferCopy2KHRBuilder<'a> {
    type Target = BufferCopy2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferCopy2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCopy2KHR.html) · Structure"]
#[doc(alias = "VkImageCopy2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageCopy2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub src_offset: crate::vk1_0::Offset3D,
    pub dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub dst_offset: crate::vk1_0::Offset3D,
    pub extent: crate::vk1_0::Extent3D,
}
impl Default for ImageCopy2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_COPY_2_KHR,
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageCopy2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageCopy2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_subresource", &self.src_subresource)
            .field("src_offset", &self.src_offset)
            .field("dst_subresource", &self.dst_subresource)
            .field("dst_offset", &self.dst_offset)
            .field("extent", &self.extent)
            .finish()
    }
}
impl ImageCopy2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageCopy2KHRBuilder<'a> {
        ImageCopy2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageCopy2KHR.html) · Builder of [`ImageCopy2KHR`]"]
#[repr(transparent)]
pub struct ImageCopy2KHRBuilder<'a>(ImageCopy2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImageCopy2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImageCopy2KHRBuilder<'a> {
        ImageCopy2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_subresource(
        mut self,
        src_subresource: crate::vk1_0::ImageSubresourceLayers,
    ) -> Self {
        self.0.src_subresource = src_subresource as _;
        self
    }
    #[inline]
    pub fn src_offset(mut self, src_offset: crate::vk1_0::Offset3D) -> Self {
        self.0.src_offset = src_offset as _;
        self
    }
    #[inline]
    pub fn dst_subresource(
        mut self,
        dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    ) -> Self {
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageCopy2KHR {
        self.0
    }
}
impl<'a> std::default::Default for ImageCopy2KHRBuilder<'a> {
    fn default() -> ImageCopy2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageCopy2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageCopy2KHRBuilder<'a> {
    type Target = ImageCopy2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageCopy2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit2KHR.html) · Structure"]
#[doc(alias = "VkImageBlit2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageBlit2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub src_offsets: [crate::vk1_0::Offset3D; 2],
    pub dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub dst_offsets: [crate::vk1_0::Offset3D; 2],
}
impl Default for ImageBlit2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_BLIT_2_KHR,
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offsets: unsafe { std::mem::zeroed() },
            dst_subresource: Default::default(),
            dst_offsets: unsafe { std::mem::zeroed() },
        }
    }
}
impl std::fmt::Debug for ImageBlit2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageBlit2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_subresource", &self.src_subresource)
            .field("src_offsets", &self.src_offsets)
            .field("dst_subresource", &self.dst_subresource)
            .field("dst_offsets", &self.dst_offsets)
            .finish()
    }
}
impl ImageBlit2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageBlit2KHRBuilder<'a> {
        ImageBlit2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a>
    crate::ExtendableFrom<
        'a,
        crate::extensions::qcom_rotated_copy_commands::CopyCommandTransformInfoQCOM,
    > for ImageBlit2KHRBuilder<'a>
{
}
impl<'a>
    crate::ExtendableFrom<
        'a,
        crate::extensions::qcom_rotated_copy_commands::CopyCommandTransformInfoQCOMBuilder<'_>,
    > for ImageBlit2KHRBuilder<'a>
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageBlit2KHR.html) · Builder of [`ImageBlit2KHR`]"]
#[repr(transparent)]
pub struct ImageBlit2KHRBuilder<'a>(ImageBlit2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImageBlit2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImageBlit2KHRBuilder<'a> {
        ImageBlit2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_subresource(
        mut self,
        src_subresource: crate::vk1_0::ImageSubresourceLayers,
    ) -> Self {
        self.0.src_subresource = src_subresource as _;
        self
    }
    #[inline]
    pub fn src_offsets(mut self, src_offsets: [crate::vk1_0::Offset3D; 2]) -> Self {
        self.0.src_offsets = src_offsets as _;
        self
    }
    #[inline]
    pub fn dst_subresource(
        mut self,
        dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    ) -> Self {
        self.0.dst_subresource = dst_subresource as _;
        self
    }
    #[inline]
    pub fn dst_offsets(mut self, dst_offsets: [crate::vk1_0::Offset3D; 2]) -> Self {
        self.0.dst_offsets = dst_offsets as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageBlit2KHR {
        self.0
    }
}
impl<'a> std::default::Default for ImageBlit2KHRBuilder<'a> {
    fn default() -> ImageBlit2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageBlit2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageBlit2KHRBuilder<'a> {
    type Target = ImageBlit2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageBlit2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy2KHR.html) · Structure"]
#[doc(alias = "VkBufferImageCopy2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BufferImageCopy2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub buffer_offset: crate::vk1_0::DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub image_offset: crate::vk1_0::Offset3D,
    pub image_extent: crate::vk1_0::Extent3D,
}
impl Default for BufferImageCopy2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BUFFER_IMAGE_COPY_2_KHR,
            p_next: std::ptr::null(),
            buffer_offset: Default::default(),
            buffer_row_length: Default::default(),
            buffer_image_height: Default::default(),
            image_subresource: Default::default(),
            image_offset: Default::default(),
            image_extent: Default::default(),
        }
    }
}
impl std::fmt::Debug for BufferImageCopy2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BufferImageCopy2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("buffer_offset", &self.buffer_offset)
            .field("buffer_row_length", &self.buffer_row_length)
            .field("buffer_image_height", &self.buffer_image_height)
            .field("image_subresource", &self.image_subresource)
            .field("image_offset", &self.image_offset)
            .field("image_extent", &self.image_extent)
            .finish()
    }
}
impl BufferImageCopy2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> BufferImageCopy2KHRBuilder<'a> {
        BufferImageCopy2KHRBuilder(self, std::marker::PhantomData)
    }
}
impl<'a>
    crate::ExtendableFrom<
        'a,
        crate::extensions::qcom_rotated_copy_commands::CopyCommandTransformInfoQCOM,
    > for BufferImageCopy2KHRBuilder<'a>
{
}
impl<'a>
    crate::ExtendableFrom<
        'a,
        crate::extensions::qcom_rotated_copy_commands::CopyCommandTransformInfoQCOMBuilder<'_>,
    > for BufferImageCopy2KHRBuilder<'a>
{
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBufferImageCopy2KHR.html) · Builder of [`BufferImageCopy2KHR`]"]
#[repr(transparent)]
pub struct BufferImageCopy2KHRBuilder<'a>(BufferImageCopy2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> BufferImageCopy2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> BufferImageCopy2KHRBuilder<'a> {
        BufferImageCopy2KHRBuilder(Default::default(), std::marker::PhantomData)
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
    pub fn image_subresource(
        mut self,
        image_subresource: crate::vk1_0::ImageSubresourceLayers,
    ) -> Self {
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BufferImageCopy2KHR {
        self.0
    }
}
impl<'a> std::default::Default for BufferImageCopy2KHRBuilder<'a> {
    fn default() -> BufferImageCopy2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BufferImageCopy2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BufferImageCopy2KHRBuilder<'a> {
    type Target = BufferImageCopy2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BufferImageCopy2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageResolve2KHR.html) · Structure"]
#[doc(alias = "VkImageResolve2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImageResolve2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub src_offset: crate::vk1_0::Offset3D,
    pub dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    pub dst_offset: crate::vk1_0::Offset3D,
    pub extent: crate::vk1_0::Extent3D,
}
impl Default for ImageResolve2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMAGE_RESOLVE_2_KHR,
            p_next: std::ptr::null(),
            src_subresource: Default::default(),
            src_offset: Default::default(),
            dst_subresource: Default::default(),
            dst_offset: Default::default(),
            extent: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImageResolve2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImageResolve2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_subresource", &self.src_subresource)
            .field("src_offset", &self.src_offset)
            .field("dst_subresource", &self.dst_subresource)
            .field("dst_offset", &self.dst_offset)
            .field("extent", &self.extent)
            .finish()
    }
}
impl ImageResolve2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImageResolve2KHRBuilder<'a> {
        ImageResolve2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImageResolve2KHR.html) · Builder of [`ImageResolve2KHR`]"]
#[repr(transparent)]
pub struct ImageResolve2KHRBuilder<'a>(ImageResolve2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> ImageResolve2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImageResolve2KHRBuilder<'a> {
        ImageResolve2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_subresource(
        mut self,
        src_subresource: crate::vk1_0::ImageSubresourceLayers,
    ) -> Self {
        self.0.src_subresource = src_subresource as _;
        self
    }
    #[inline]
    pub fn src_offset(mut self, src_offset: crate::vk1_0::Offset3D) -> Self {
        self.0.src_offset = src_offset as _;
        self
    }
    #[inline]
    pub fn dst_subresource(
        mut self,
        dst_subresource: crate::vk1_0::ImageSubresourceLayers,
    ) -> Self {
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
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImageResolve2KHR {
        self.0
    }
}
impl<'a> std::default::Default for ImageResolve2KHRBuilder<'a> {
    fn default() -> ImageResolve2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImageResolve2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImageResolve2KHRBuilder<'a> {
    type Target = ImageResolve2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImageResolve2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyBufferInfo2KHR.html) · Structure"]
#[doc(alias = "VkCopyBufferInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyBufferInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_buffer: crate::vk1_0::Buffer,
    pub dst_buffer: crate::vk1_0::Buffer,
    pub region_count: u32,
    pub p_regions: *const crate::extensions::khr_copy_commands2::BufferCopy2KHR,
}
impl Default for CopyBufferInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::COPY_BUFFER_INFO_2_KHR,
            p_next: std::ptr::null(),
            src_buffer: Default::default(),
            dst_buffer: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for CopyBufferInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyBufferInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_buffer", &self.src_buffer)
            .field("dst_buffer", &self.dst_buffer)
            .field("region_count", &self.region_count)
            .field("p_regions", &self.p_regions)
            .finish()
    }
}
impl CopyBufferInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyBufferInfo2KHRBuilder<'a> {
        CopyBufferInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyBufferInfo2KHR.html) · Builder of [`CopyBufferInfo2KHR`]"]
#[repr(transparent)]
pub struct CopyBufferInfo2KHRBuilder<'a>(CopyBufferInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> CopyBufferInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyBufferInfo2KHRBuilder<'a> {
        CopyBufferInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_buffer(mut self, src_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.src_buffer = src_buffer as _;
        self
    }
    #[inline]
    pub fn dst_buffer(mut self, dst_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.dst_buffer = dst_buffer as _;
        self
    }
    #[inline]
    pub fn regions(
        mut self,
        regions: &'a [crate::extensions::khr_copy_commands2::BufferCopy2KHRBuilder],
    ) -> Self {
        self.0.p_regions = regions.as_ptr() as _;
        self.0.region_count = regions.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyBufferInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for CopyBufferInfo2KHRBuilder<'a> {
    fn default() -> CopyBufferInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyBufferInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyBufferInfo2KHRBuilder<'a> {
    type Target = CopyBufferInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyBufferInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyImageInfo2KHR.html) · Structure"]
#[doc(alias = "VkCopyImageInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyImageInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_image: crate::vk1_0::Image,
    pub src_image_layout: crate::vk1_0::ImageLayout,
    pub dst_image: crate::vk1_0::Image,
    pub dst_image_layout: crate::vk1_0::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const crate::extensions::khr_copy_commands2::ImageCopy2KHR,
}
impl Default for CopyImageInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::COPY_IMAGE_INFO_2_KHR,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for CopyImageInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyImageInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_image", &self.src_image)
            .field("src_image_layout", &self.src_image_layout)
            .field("dst_image", &self.dst_image)
            .field("dst_image_layout", &self.dst_image_layout)
            .field("region_count", &self.region_count)
            .field("p_regions", &self.p_regions)
            .finish()
    }
}
impl CopyImageInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyImageInfo2KHRBuilder<'a> {
        CopyImageInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyImageInfo2KHR.html) · Builder of [`CopyImageInfo2KHR`]"]
#[repr(transparent)]
pub struct CopyImageInfo2KHRBuilder<'a>(CopyImageInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> CopyImageInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyImageInfo2KHRBuilder<'a> {
        CopyImageInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_image(mut self, src_image: crate::vk1_0::Image) -> Self {
        self.0.src_image = src_image as _;
        self
    }
    #[inline]
    pub fn src_image_layout(mut self, src_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.src_image_layout = src_image_layout as _;
        self
    }
    #[inline]
    pub fn dst_image(mut self, dst_image: crate::vk1_0::Image) -> Self {
        self.0.dst_image = dst_image as _;
        self
    }
    #[inline]
    pub fn dst_image_layout(mut self, dst_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.dst_image_layout = dst_image_layout as _;
        self
    }
    #[inline]
    pub fn regions(
        mut self,
        regions: &'a [crate::extensions::khr_copy_commands2::ImageCopy2KHRBuilder],
    ) -> Self {
        self.0.p_regions = regions.as_ptr() as _;
        self.0.region_count = regions.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyImageInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for CopyImageInfo2KHRBuilder<'a> {
    fn default() -> CopyImageInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyImageInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyImageInfo2KHRBuilder<'a> {
    type Target = CopyImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyImageInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlitImageInfo2KHR.html) · Structure"]
#[doc(alias = "VkBlitImageInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlitImageInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_image: crate::vk1_0::Image,
    pub src_image_layout: crate::vk1_0::ImageLayout,
    pub dst_image: crate::vk1_0::Image,
    pub dst_image_layout: crate::vk1_0::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const crate::extensions::khr_copy_commands2::ImageBlit2KHR,
    pub filter: crate::vk1_0::Filter,
}
impl Default for BlitImageInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::BLIT_IMAGE_INFO_2_KHR,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
            filter: Default::default(),
        }
    }
}
impl std::fmt::Debug for BlitImageInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BlitImageInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_image", &self.src_image)
            .field("src_image_layout", &self.src_image_layout)
            .field("dst_image", &self.dst_image)
            .field("dst_image_layout", &self.dst_image_layout)
            .field("region_count", &self.region_count)
            .field("p_regions", &self.p_regions)
            .field("filter", &self.filter)
            .finish()
    }
}
impl BlitImageInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> BlitImageInfo2KHRBuilder<'a> {
        BlitImageInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkBlitImageInfo2KHR.html) · Builder of [`BlitImageInfo2KHR`]"]
#[repr(transparent)]
pub struct BlitImageInfo2KHRBuilder<'a>(BlitImageInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> BlitImageInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> BlitImageInfo2KHRBuilder<'a> {
        BlitImageInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_image(mut self, src_image: crate::vk1_0::Image) -> Self {
        self.0.src_image = src_image as _;
        self
    }
    #[inline]
    pub fn src_image_layout(mut self, src_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.src_image_layout = src_image_layout as _;
        self
    }
    #[inline]
    pub fn dst_image(mut self, dst_image: crate::vk1_0::Image) -> Self {
        self.0.dst_image = dst_image as _;
        self
    }
    #[inline]
    pub fn dst_image_layout(mut self, dst_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.dst_image_layout = dst_image_layout as _;
        self
    }
    #[inline]
    pub fn regions(
        mut self,
        regions: &'a [crate::extensions::khr_copy_commands2::ImageBlit2KHRBuilder],
    ) -> Self {
        self.0.p_regions = regions.as_ptr() as _;
        self.0.region_count = regions.len() as _;
        self
    }
    #[inline]
    pub fn filter(mut self, filter: crate::vk1_0::Filter) -> Self {
        self.0.filter = filter as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> BlitImageInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for BlitImageInfo2KHRBuilder<'a> {
    fn default() -> BlitImageInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for BlitImageInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for BlitImageInfo2KHRBuilder<'a> {
    type Target = BlitImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for BlitImageInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyBufferToImageInfo2KHR.html) · Structure"]
#[doc(alias = "VkCopyBufferToImageInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyBufferToImageInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_buffer: crate::vk1_0::Buffer,
    pub dst_image: crate::vk1_0::Image,
    pub dst_image_layout: crate::vk1_0::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const crate::extensions::khr_copy_commands2::BufferImageCopy2KHR,
}
impl Default for CopyBufferToImageInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::COPY_BUFFER_TO_IMAGE_INFO_2_KHR,
            p_next: std::ptr::null(),
            src_buffer: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for CopyBufferToImageInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyBufferToImageInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_buffer", &self.src_buffer)
            .field("dst_image", &self.dst_image)
            .field("dst_image_layout", &self.dst_image_layout)
            .field("region_count", &self.region_count)
            .field("p_regions", &self.p_regions)
            .finish()
    }
}
impl CopyBufferToImageInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyBufferToImageInfo2KHRBuilder<'a> {
        CopyBufferToImageInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyBufferToImageInfo2KHR.html) · Builder of [`CopyBufferToImageInfo2KHR`]"]
#[repr(transparent)]
pub struct CopyBufferToImageInfo2KHRBuilder<'a>(
    CopyBufferToImageInfo2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CopyBufferToImageInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyBufferToImageInfo2KHRBuilder<'a> {
        CopyBufferToImageInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_buffer(mut self, src_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.src_buffer = src_buffer as _;
        self
    }
    #[inline]
    pub fn dst_image(mut self, dst_image: crate::vk1_0::Image) -> Self {
        self.0.dst_image = dst_image as _;
        self
    }
    #[inline]
    pub fn dst_image_layout(mut self, dst_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.dst_image_layout = dst_image_layout as _;
        self
    }
    #[inline]
    pub fn regions(
        mut self,
        regions: &'a [crate::extensions::khr_copy_commands2::BufferImageCopy2KHRBuilder],
    ) -> Self {
        self.0.p_regions = regions.as_ptr() as _;
        self.0.region_count = regions.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyBufferToImageInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for CopyBufferToImageInfo2KHRBuilder<'a> {
    fn default() -> CopyBufferToImageInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyBufferToImageInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyBufferToImageInfo2KHRBuilder<'a> {
    type Target = CopyBufferToImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyBufferToImageInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyImageToBufferInfo2KHR.html) · Structure"]
#[doc(alias = "VkCopyImageToBufferInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyImageToBufferInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_image: crate::vk1_0::Image,
    pub src_image_layout: crate::vk1_0::ImageLayout,
    pub dst_buffer: crate::vk1_0::Buffer,
    pub region_count: u32,
    pub p_regions: *const crate::extensions::khr_copy_commands2::BufferImageCopy2KHR,
}
impl Default for CopyImageToBufferInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::COPY_IMAGE_TO_BUFFER_INFO_2_KHR,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_buffer: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for CopyImageToBufferInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyImageToBufferInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_image", &self.src_image)
            .field("src_image_layout", &self.src_image_layout)
            .field("dst_buffer", &self.dst_buffer)
            .field("region_count", &self.region_count)
            .field("p_regions", &self.p_regions)
            .finish()
    }
}
impl CopyImageToBufferInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyImageToBufferInfo2KHRBuilder<'a> {
        CopyImageToBufferInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyImageToBufferInfo2KHR.html) · Builder of [`CopyImageToBufferInfo2KHR`]"]
#[repr(transparent)]
pub struct CopyImageToBufferInfo2KHRBuilder<'a>(
    CopyImageToBufferInfo2KHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CopyImageToBufferInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> CopyImageToBufferInfo2KHRBuilder<'a> {
        CopyImageToBufferInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_image(mut self, src_image: crate::vk1_0::Image) -> Self {
        self.0.src_image = src_image as _;
        self
    }
    #[inline]
    pub fn src_image_layout(mut self, src_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.src_image_layout = src_image_layout as _;
        self
    }
    #[inline]
    pub fn dst_buffer(mut self, dst_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.dst_buffer = dst_buffer as _;
        self
    }
    #[inline]
    pub fn regions(
        mut self,
        regions: &'a [crate::extensions::khr_copy_commands2::BufferImageCopy2KHRBuilder],
    ) -> Self {
        self.0.p_regions = regions.as_ptr() as _;
        self.0.region_count = regions.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyImageToBufferInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for CopyImageToBufferInfo2KHRBuilder<'a> {
    fn default() -> CopyImageToBufferInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyImageToBufferInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyImageToBufferInfo2KHRBuilder<'a> {
    type Target = CopyImageToBufferInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyImageToBufferInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveImageInfo2KHR.html) · Structure"]
#[doc(alias = "VkResolveImageInfo2KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ResolveImageInfo2KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub src_image: crate::vk1_0::Image,
    pub src_image_layout: crate::vk1_0::ImageLayout,
    pub dst_image: crate::vk1_0::Image,
    pub dst_image_layout: crate::vk1_0::ImageLayout,
    pub region_count: u32,
    pub p_regions: *const crate::extensions::khr_copy_commands2::ImageResolve2KHR,
}
impl Default for ResolveImageInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RESOLVE_IMAGE_INFO_2_KHR,
            p_next: std::ptr::null(),
            src_image: Default::default(),
            src_image_layout: Default::default(),
            dst_image: Default::default(),
            dst_image_layout: Default::default(),
            region_count: Default::default(),
            p_regions: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for ResolveImageInfo2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ResolveImageInfo2KHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("src_image", &self.src_image)
            .field("src_image_layout", &self.src_image_layout)
            .field("dst_image", &self.dst_image)
            .field("dst_image_layout", &self.dst_image_layout)
            .field("region_count", &self.region_count)
            .field("p_regions", &self.p_regions)
            .finish()
    }
}
impl ResolveImageInfo2KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ResolveImageInfo2KHRBuilder<'a> {
        ResolveImageInfo2KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkResolveImageInfo2KHR.html) · Builder of [`ResolveImageInfo2KHR`]"]
#[repr(transparent)]
pub struct ResolveImageInfo2KHRBuilder<'a>(ResolveImageInfo2KHR, std::marker::PhantomData<&'a ()>);
impl<'a> ResolveImageInfo2KHRBuilder<'a> {
    #[inline]
    pub fn new() -> ResolveImageInfo2KHRBuilder<'a> {
        ResolveImageInfo2KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn src_image(mut self, src_image: crate::vk1_0::Image) -> Self {
        self.0.src_image = src_image as _;
        self
    }
    #[inline]
    pub fn src_image_layout(mut self, src_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.src_image_layout = src_image_layout as _;
        self
    }
    #[inline]
    pub fn dst_image(mut self, dst_image: crate::vk1_0::Image) -> Self {
        self.0.dst_image = dst_image as _;
        self
    }
    #[inline]
    pub fn dst_image_layout(mut self, dst_image_layout: crate::vk1_0::ImageLayout) -> Self {
        self.0.dst_image_layout = dst_image_layout as _;
        self
    }
    #[inline]
    pub fn regions(
        mut self,
        regions: &'a [crate::extensions::khr_copy_commands2::ImageResolve2KHRBuilder],
    ) -> Self {
        self.0.p_regions = regions.as_ptr() as _;
        self.0.region_count = regions.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ResolveImageInfo2KHR {
        self.0
    }
}
impl<'a> std::default::Default for ResolveImageInfo2KHRBuilder<'a> {
    fn default() -> ResolveImageInfo2KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ResolveImageInfo2KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ResolveImageInfo2KHRBuilder<'a> {
    type Target = ResolveImageInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ResolveImageInfo2KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_copy_commands2`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer2KHR.html) · Function"]
    #[doc(alias = "vkCmdCopyBuffer2KHR")]
    pub unsafe fn cmd_copy_buffer2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        copy_buffer_info: &crate::extensions::khr_copy_commands2::CopyBufferInfo2KHR,
    ) -> () {
        let _function = self
            .cmd_copy_buffer2_khr
            .expect("`cmd_copy_buffer2_khr` is not loaded");
        let _return = _function(command_buffer as _, copy_buffer_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage2KHR.html) · Function"]
    #[doc(alias = "vkCmdCopyImage2KHR")]
    pub unsafe fn cmd_copy_image2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        copy_image_info: &crate::extensions::khr_copy_commands2::CopyImageInfo2KHR,
    ) -> () {
        let _function = self
            .cmd_copy_image2_khr
            .expect("`cmd_copy_image2_khr` is not loaded");
        let _return = _function(command_buffer as _, copy_image_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage2KHR.html) · Function"]
    #[doc(alias = "vkCmdBlitImage2KHR")]
    pub unsafe fn cmd_blit_image2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        blit_image_info: &crate::extensions::khr_copy_commands2::BlitImageInfo2KHR,
    ) -> () {
        let _function = self
            .cmd_blit_image2_khr
            .expect("`cmd_blit_image2_khr` is not loaded");
        let _return = _function(command_buffer as _, blit_image_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage2KHR.html) · Function"]
    #[doc(alias = "vkCmdCopyBufferToImage2KHR")]
    pub unsafe fn cmd_copy_buffer_to_image2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        copy_buffer_to_image_info : & crate :: extensions :: khr_copy_commands2 :: CopyBufferToImageInfo2KHR,
    ) -> () {
        let _function = self
            .cmd_copy_buffer_to_image2_khr
            .expect("`cmd_copy_buffer_to_image2_khr` is not loaded");
        let _return = _function(command_buffer as _, copy_buffer_to_image_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer2KHR.html) · Function"]
    #[doc(alias = "vkCmdCopyImageToBuffer2KHR")]
    pub unsafe fn cmd_copy_image_to_buffer2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        copy_image_to_buffer_info : & crate :: extensions :: khr_copy_commands2 :: CopyImageToBufferInfo2KHR,
    ) -> () {
        let _function = self
            .cmd_copy_image_to_buffer2_khr
            .expect("`cmd_copy_image_to_buffer2_khr` is not loaded");
        let _return = _function(command_buffer as _, copy_image_to_buffer_info as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage2KHR.html) · Function"]
    #[doc(alias = "vkCmdResolveImage2KHR")]
    pub unsafe fn cmd_resolve_image2_khr(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        resolve_image_info: &crate::extensions::khr_copy_commands2::ResolveImageInfo2KHR,
    ) -> () {
        let _function = self
            .cmd_resolve_image2_khr
            .expect("`cmd_resolve_image2_khr` is not loaded");
        let _return = _function(command_buffer as _, resolve_image_info as _);
        ()
    }
}
