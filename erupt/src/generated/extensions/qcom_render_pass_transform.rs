#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_QCOM_render_pass_transform");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html) · Structure"]
#[doc(alias = "VkRenderPassTransformBeginInfoQCOM")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassTransformBeginInfoQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
}
impl Default for RenderPassTransformBeginInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
            p_next: std::ptr::null_mut(),
            transform: Default::default(),
        }
    }
}
impl std::fmt::Debug for RenderPassTransformBeginInfoQCOM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderPassTransformBeginInfoQCOM")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("transform", &self.transform)
            .finish()
    }
}
impl RenderPassTransformBeginInfoQCOM {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderPassTransformBeginInfoQCOMBuilder<'a> {
        RenderPassTransformBeginInfoQCOMBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html) · Builder of [`RenderPassTransformBeginInfoQCOM`]"]
#[repr(transparent)]
pub struct RenderPassTransformBeginInfoQCOMBuilder<'a>(RenderPassTransformBeginInfoQCOM, std::marker::PhantomData<&'a ()>);
impl<'a> RenderPassTransformBeginInfoQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassTransformBeginInfoQCOMBuilder<'a> {
        RenderPassTransformBeginInfoQCOMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn transform(mut self, transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.0.transform = transform as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RenderPassTransformBeginInfoQCOM {
        self.0
    }
}
impl<'a> std::default::Default for RenderPassTransformBeginInfoQCOMBuilder<'a> {
    fn default() -> RenderPassTransformBeginInfoQCOMBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderPassTransformBeginInfoQCOMBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderPassTransformBeginInfoQCOMBuilder<'a> {
    type Target = RenderPassTransformBeginInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassTransformBeginInfoQCOMBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html) · Structure"]
#[doc(alias = "VkCommandBufferInheritanceRenderPassTransformInfoQCOM")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    pub render_area: crate::vk1_0::Rect2D,
}
impl Default for CommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM,
            p_next: std::ptr::null_mut(),
            transform: Default::default(),
            render_area: Default::default(),
        }
    }
}
impl std::fmt::Debug for CommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CommandBufferInheritanceRenderPassTransformInfoQCOM")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("transform", &self.transform)
            .field("render_area", &self.render_area)
            .finish()
    }
}
impl CommandBufferInheritanceRenderPassTransformInfoQCOM {
    #[inline]
    pub fn into_builder<'a>(self) -> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
        CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html) · Builder of [`CommandBufferInheritanceRenderPassTransformInfoQCOM`]"]
#[repr(transparent)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a>(CommandBufferInheritanceRenderPassTransformInfoQCOM, std::marker::PhantomData<&'a ()>);
impl<'a> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
        CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn transform(mut self, transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR) -> Self {
        self.0.transform = transform as _;
        self
    }
    #[inline]
    pub fn render_area(mut self, render_area: crate::vk1_0::Rect2D) -> Self {
        self.0.render_area = render_area as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CommandBufferInheritanceRenderPassTransformInfoQCOM {
        self.0
    }
}
impl<'a> std::default::Default for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    fn default() -> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    type Target = CommandBufferInheritanceRenderPassTransformInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
