//! GAT-based lending render backend.

/// A renderer whose completed frame may borrow backend-owned storage.
///
/// The generic associated frame family supports allocation-free frame reuse.
/// Backend selection remains static, so calls monomorphize without a vtable.
pub trait RenderBackend<V: ?Sized> {
    /// Backend-specific failure type.
    type Error;

    /// Frame representation borrowed for the duration of a backend borrow.
    type Frame<'a>
    where
        Self: 'a;

    /// Render `view` into backend-owned frame storage.
    ///
    /// # Errors
    ///
    /// Returns the backend's typed error when rendering cannot complete.
    fn render<'a>(&'a mut self, view: &V) -> Result<Self::Frame<'a>, Self::Error>;
}
