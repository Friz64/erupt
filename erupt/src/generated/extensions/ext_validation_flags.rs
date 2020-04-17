# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_validation_flags.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VALIDATION_FLAGS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_validation_flags");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFlagsEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValidationFlagsEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub disabled_validation_check_count: u32,
    pub p_disabled_validation_checks:
        *const crate::extensions::ext_validation_flags::ValidationCheckEXT,
}
impl ValidationFlagsEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByValidationFlagsEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> ValidationFlagsEXTBuilder<'a> {
        ValidationFlagsEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ValidationFlagsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ValidationFlagsEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "disabled_validation_check_count",
                &self.disabled_validation_check_count,
            )
            .field(
                "p_disabled_validation_checks",
                &self.p_disabled_validation_checks,
            )
            .finish()
    }
}
impl Default for ValidationFlagsEXT {
    fn default() -> ValidationFlagsEXT {
        ValidationFlagsEXT {
            s_type: crate::vk1_0::StructureType::VALIDATION_FLAGS_EXT,
            p_next: std::ptr::null(),
            disabled_validation_check_count: Default::default(),
            p_disabled_validation_checks: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`ValidationFlagsEXT::extend`](struct.ValidationFlagsEXT.html#method.extend)"]
pub trait ExtendableByValidationFlagsEXT {}
impl ExtendableByValidationFlagsEXT for crate::vk1_0::InstanceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ValidationFlagsEXT`](struct.ValidationFlagsEXT.html)"]
#[repr(transparent)]
pub struct ValidationFlagsEXTBuilder<'a>(ValidationFlagsEXT, std::marker::PhantomData<&'a ()>);
impl<'a> ValidationFlagsEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ValidationFlagsEXTBuilder<'a> {
        ValidationFlagsEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn disabled_validation_checks(
        mut self,
        disabled_validation_checks : &'a [ crate :: extensions :: ext_validation_flags :: ValidationCheckEXT ],
    ) -> Self {
        self.0.disabled_validation_check_count = disabled_validation_checks.len() as _;
        self.0.p_disabled_validation_checks = disabled_validation_checks.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ValidationFlagsEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ValidationFlagsEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ValidationFlagsEXTBuilder<'a> {
    type Target = ValidationFlagsEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ValidationFlagsEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationCheckEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCheckEXT(pub i32);
#[doc = "[Part of `extensions::ext_validation_flags`](../../extensions/ext_validation_flags/index.html)"]
impl ValidationCheckEXT {
    pub const ALL_EXT: Self = Self(0);
    pub const SHADERS_EXT: Self = Self(1);
}
impl std::fmt::Debug for ValidationCheckEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ALL_EXT => "ALL_EXT",
            &Self::SHADERS_EXT => "SHADERS_EXT",
            _ => "Unknown enum variant",
        })
    }
}
