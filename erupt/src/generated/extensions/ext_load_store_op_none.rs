#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION")]
pub const EXT_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME")]
pub const EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_load_store_op_none");
#[doc = "Provided by [`crate::extensions::ext_load_store_op_none`]"]
impl crate::vk1_0::AttachmentLoadOp {
    pub const NONE_EXT: Self = Self(1000400000);
}
#[doc = "Provided by [`crate::extensions::ext_load_store_op_none`]"]
impl crate::vk1_0::AttachmentStoreOp {
    pub const NONE_EXT: Self = Self(1000301000);
}
