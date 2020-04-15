# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_validation_features.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_VALIDATION_FEATURES_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_validation_features");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ValidationFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub enabled_validation_feature_count: u32,
    pub p_enabled_validation_features:
        *const crate::extensions::ext_validation_features::ValidationFeatureEnableEXT,
    pub disabled_validation_feature_count: u32,
    pub p_disabled_validation_features:
        *const crate::extensions::ext_validation_features::ValidationFeatureDisableEXT,
}
impl ValidationFeaturesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByValidationFeaturesEXT,
    {
        unsafe {
            crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
        }
    }
    #[inline]
    pub fn builder<'a>(self) -> ValidationFeaturesEXTBuilder<'a> {
        ValidationFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for ValidationFeaturesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ValidationFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "enabled_validation_feature_count",
                &self.enabled_validation_feature_count,
            )
            .field(
                "p_enabled_validation_features",
                &self.p_enabled_validation_features,
            )
            .field(
                "disabled_validation_feature_count",
                &self.disabled_validation_feature_count,
            )
            .field(
                "p_disabled_validation_features",
                &self.p_disabled_validation_features,
            )
            .finish()
    }
}
impl Default for ValidationFeaturesEXT {
    fn default() -> ValidationFeaturesEXT {
        ValidationFeaturesEXT {
            s_type: crate::vk1_0::StructureType::VALIDATION_FEATURES_EXT,
            p_next: std::ptr::null(),
            enabled_validation_feature_count: Default::default(),
            p_enabled_validation_features: std::ptr::null(),
            disabled_validation_feature_count: Default::default(),
            p_disabled_validation_features: std::ptr::null(),
        }
    }
}
#[doc = "Used by [`ValidationFeaturesEXT::extend`](struct.ValidationFeaturesEXT.html#method.extend)"]
pub trait ExtendableByValidationFeaturesEXT {}
impl ExtendableByValidationFeaturesEXT for crate::vk1_0::InstanceCreateInfo {}
#[derive(Copy, Clone)]
#[doc = "Builder of [`ValidationFeaturesEXT`](struct.ValidationFeaturesEXT.html)"]
#[repr(transparent)]
pub struct ValidationFeaturesEXTBuilder<'a>(
    ValidationFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ValidationFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> ValidationFeaturesEXTBuilder<'a> {
        ValidationFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn enabled_validation_features(
        mut self,
        enabled_validation_features : &'a [ crate :: extensions :: ext_validation_features :: ValidationFeatureEnableEXT ],
    ) -> Self {
        self.0.enabled_validation_feature_count = enabled_validation_features.len() as _;
        self.0.p_enabled_validation_features = enabled_validation_features.as_ptr() as _;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn disabled_validation_features(
        mut self,
        disabled_validation_features : &'a [ crate :: extensions :: ext_validation_features :: ValidationFeatureDisableEXT ],
    ) -> Self {
        self.0.disabled_validation_feature_count = disabled_validation_features.len() as _;
        self.0.p_disabled_validation_features = disabled_validation_features.as_ptr() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> ValidationFeaturesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for ValidationFeaturesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for ValidationFeaturesEXTBuilder<'a> {
    type Target = ValidationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ValidationFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeatureEnableEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationFeatureEnableEXT(pub i32);
#[doc = "[Part of `extensions::ext_validation_features`](../../extensions/ext_validation_features/index.html)"]
impl ValidationFeatureEnableEXT {
    pub const GPU_ASSISTED_EXT: Self = Self(0);
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: Self = Self(1);
    pub const BEST_PRACTICES_EXT: Self = Self(2);
    pub const DEBUG_PRINTF_EXT: Self = Self(3);
}
impl std::fmt::Debug for ValidationFeatureEnableEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::GPU_ASSISTED_EXT => "GPU_ASSISTED_EXT",
            &Self::GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT => "GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT",
            &Self::BEST_PRACTICES_EXT => "BEST_PRACTICES_EXT",
            &Self::DEBUG_PRINTF_EXT => "DEBUG_PRINTF_EXT",
            _ => "Unknown enum variant",
        })
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkValidationFeatureDisableEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ValidationFeatureDisableEXT(pub i32);
#[doc = "[Part of `extensions::ext_validation_features`](../../extensions/ext_validation_features/index.html)"]
impl ValidationFeatureDisableEXT {
    pub const ALL_EXT: Self = Self(0);
    pub const SHADERS_EXT: Self = Self(1);
    pub const THREAD_SAFETY_EXT: Self = Self(2);
    pub const API_PARAMETERS_EXT: Self = Self(3);
    pub const OBJECT_LIFETIMES_EXT: Self = Self(4);
    pub const CORE_CHECKS_EXT: Self = Self(5);
    pub const UNIQUE_HANDLES_EXT: Self = Self(6);
}
impl std::fmt::Debug for ValidationFeatureDisableEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::ALL_EXT => "ALL_EXT",
            &Self::SHADERS_EXT => "SHADERS_EXT",
            &Self::THREAD_SAFETY_EXT => "THREAD_SAFETY_EXT",
            &Self::API_PARAMETERS_EXT => "API_PARAMETERS_EXT",
            &Self::OBJECT_LIFETIMES_EXT => "OBJECT_LIFETIMES_EXT",
            &Self::CORE_CHECKS_EXT => "CORE_CHECKS_EXT",
            &Self::UNIQUE_HANDLES_EXT => "UNIQUE_HANDLES_EXT",
            _ => "Unknown enum variant",
        })
    }
}
