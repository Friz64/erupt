# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_QCOM_render_pass_transform.html)\n\n## Extends\n- [`RenderPassCreateFlagBits`](../../vk1_0/struct.RenderPassCreateFlagBits.html)\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_QCOM_render_pass_transform");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassTransformBeginInfoQCOM.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassTransformBeginInfoQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
}
impl RenderPassTransformBeginInfoQCOM {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByRenderPassTransformBeginInfoQCOM,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> RenderPassTransformBeginInfoQCOMBuilder<'a> {
        RenderPassTransformBeginInfoQCOMBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for RenderPassTransformBeginInfoQCOM {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("RenderPassTransformBeginInfoQCOM")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("transform", &self.transform)
            .finish()
    }
}
impl Default for RenderPassTransformBeginInfoQCOM {
    fn default() -> RenderPassTransformBeginInfoQCOM {
        RenderPassTransformBeginInfoQCOM {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
            p_next: std::ptr::null_mut(),
            transform: Default::default(),
        }
    }
}
#[doc = "Used by [`RenderPassTransformBeginInfoQCOM::extend`](struct.RenderPassTransformBeginInfoQCOM.html#method.extend)"]
pub trait ExtendableByRenderPassTransformBeginInfoQCOM {}
impl ExtendableByRenderPassTransformBeginInfoQCOM for crate::vk1_0::RenderPassBeginInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`RenderPassTransformBeginInfoQCOM`](struct.RenderPassTransformBeginInfoQCOM.html)"]
#[repr(transparent)]
pub struct RenderPassTransformBeginInfoQCOMBuilder<'a>(
    RenderPassTransformBeginInfoQCOM,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassTransformBeginInfoQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassTransformBeginInfoQCOMBuilder<'a> {
        RenderPassTransformBeginInfoQCOMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn transform(
        mut self,
        transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> Self {
        self.0.transform = transform;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> RenderPassTransformBeginInfoQCOM {
        self.0
    }
}
impl<'a> std::fmt::Debug for RenderPassTransformBeginInfoQCOMBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    pub render_area: crate::vk1_0::Rect2D,
}
impl CommandBufferInheritanceRenderPassTransformInfoQCOM {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByCommandBufferInheritanceRenderPassTransformInfoQCOM,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
        CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for CommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("CommandBufferInheritanceRenderPassTransformInfoQCOM")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("transform", &self.transform)
            .field("render_area", &self.render_area)
            .finish()
    }
}
impl Default for CommandBufferInheritanceRenderPassTransformInfoQCOM {
    fn default() -> CommandBufferInheritanceRenderPassTransformInfoQCOM {
        CommandBufferInheritanceRenderPassTransformInfoQCOM { s_type : crate :: vk1_0 :: StructureType :: COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM , p_next : std :: ptr :: null_mut ( ) , transform : Default :: default ( ) , render_area : Default :: default ( ) , }
    }
}
#[doc = "Used by [`CommandBufferInheritanceRenderPassTransformInfoQCOM::extend`](struct.CommandBufferInheritanceRenderPassTransformInfoQCOM.html#method.extend)"]
pub trait ExtendableByCommandBufferInheritanceRenderPassTransformInfoQCOM {}
impl ExtendableByCommandBufferInheritanceRenderPassTransformInfoQCOM
    for crate::vk1_0::CommandBufferInheritanceInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`CommandBufferInheritanceRenderPassTransformInfoQCOM`](struct.CommandBufferInheritanceRenderPassTransformInfoQCOM.html)"]
#[repr(transparent)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a>(
    CommandBufferInheritanceRenderPassTransformInfoQCOM,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
        CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn transform(
        mut self,
        transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> Self {
        self.0.transform = transform;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn render_area(mut self, render_area: crate::vk1_0::Rect2D) -> Self {
        self.0.render_area = render_area;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> CommandBufferInheritanceRenderPassTransformInfoQCOM {
        self.0
    }
}
impl<'a> std::fmt::Debug for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
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
