use crate::vk;
use std::{
    error::Error,
    fmt::{self, Debug, Display},
};

/// Idiomatic wrapper around a Vulkan Result.
#[must_use = "this `VulkanResult` may be an error, which should be handled"]
#[derive(Copy, Clone, Default)]
pub struct VulkanResult<T> {
    /// The raw result from Vulkan.
    pub raw: vk::Result,
    /// The value this wrapper type may be holding.
    pub value: Option<T>,
}

impl<T> VulkanResult<T> {
    /// Construct a new `VulkanResult` from `raw` and `value`.
    ///
    /// This will not populate `self.value`
    /// if the raw result is negative (Error).
    #[inline]
    pub fn new(raw: vk::Result, value: T) -> VulkanResult<T> {
        let value = if raw.0.is_negative() { None } else { Some(value) };

        VulkanResult { raw, value }
    }

    /// Returns the contained value, consuming `self`.
    ///
    /// Panics with the name of `self.raw` if `self.value` is `None`.
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}", self.raw),
        }
    }

    /// Returns the contained value, consuming `self`.
    ///
    /// Panics with `msg` and the name of `self.raw` if `self.value` is `None`.
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: impl Display) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}: {}", self.raw, msg),
        }
    }

    /// Converts from `&VulkanResult<T>` to `VulkanResult<&T>`.
    ///
    /// Clones `self.raw`.
    #[inline]
    pub fn as_ref(&self) -> VulkanResult<&T> {
        VulkanResult { raw: self.raw.clone(), value: self.value.as_ref() }
    }

    /// Converts from `&mut VulkanResult<T>` to `VulkanResult<&mut T>`.
    ///
    /// Clones `self.raw`.
    #[inline]
    pub fn as_mut(&mut self) -> VulkanResult<&mut T> {
        VulkanResult { raw: self.raw.clone(), value: self.value.as_mut() }
    }

    /// Constructs a new `VulkanResult` from `value`.
    ///
    /// This will always set `self.raw` to `vk::Result::SUCCESS`.
    #[inline]
    pub fn new_ok(value: T) -> VulkanResult<T> {
        VulkanResult::new(vk::Result::SUCCESS, value)
    }

    /// Constructs a new `VulkanResult` from `raw`.
    ///
    /// This will always set `self.value` to `None`.
    #[inline]
    pub fn new_err(raw: vk::Result) -> VulkanResult<T> {
        VulkanResult { raw, value: None }
    }

    /// Returns `self.value`, consuming `self` and dropping `self.raw`.
    #[inline]
    pub fn ok(self) -> Option<T> {
        self.value
    }

    /// Maps `Some(v)` of `self.value` to `Ok(v)` and `None` of `self.value` to `Err(self.raw)`.
    #[inline]
    pub fn result(self) -> Result<T, vk::Result> {
        self.value.ok_or(self.raw)
    }

    /// Maps `Some(v)` of `self.value` to `Ok(v)` and `None` of `self.value` to `Err(op(self.raw))`.
    #[inline]
    pub fn map_err<F, O: FnOnce(vk::Result) -> F>(self, op: O) -> Result<T, F> {
        let raw = self.raw;
        self.value.ok_or_else(move || op(raw))
    }

    /// Returns `true` if `self.value` is `Some`.
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.value.is_some()
    }

    /// Returns `true` if `self.raw` is positive.
    #[inline]
    pub fn raw_is_ok(&self) -> bool {
        self.raw.0.is_positive()
    }

    /// Returns `true` if `self.value` is `None`.
    #[inline]
    pub fn is_err(&self) -> bool {
        self.value.is_none()
    }

    /// Returns `true` if `self.raw` is negative.
    #[inline]
    pub fn raw_is_err(&self) -> bool {
        self.raw.0.is_negative()
    }
}

impl Display for vk::Result {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, fmt)
    }
}

impl Error for vk::Result {}

impl<T> Debug for VulkanResult<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => write!(fmt, "{:?}: {:?}", self.raw, value),
            None => write!(fmt, "{:?}: (no value)", self.raw),
        }
    }
}

impl<T> Display for VulkanResult<T>
where
    T: Display,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => write!(fmt, "{:?}: {}", self.raw, value),
            None => write!(fmt, "{:?}: (no value)", self.raw),
        }
    }
}
