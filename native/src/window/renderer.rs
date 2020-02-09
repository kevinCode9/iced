use crate::MouseCursor;

use raw_window_handle::HasRawWindowHandle;

/// A graphics backend that can render to windows.
pub trait Backend: Sized {
    /// The settings of the backend.
    type Settings: Default;

    /// The iced renderer of the backend.
    type Renderer: crate::Renderer;

    /// The surface of the backend.
    type Surface;

    /// The target of the backend.
    type Target;

    /// Creates a new [`Gpu`] and an associated iced renderer.
    ///
    /// [`Gpu`]: trait.Gpu.html
    fn new(settings: Self::Settings) -> (Self, Self::Renderer);

    /// Crates a new [`Surface`] for the given window.
    ///
    /// [`Surface`]: #associatedtype.Surface
    fn create_surface<W: HasRawWindowHandle>(
        &mut self,
        window: &W,
    ) -> Self::Surface;

    /// Crates a new [`Target`] for the given [`Surface`].
    ///
    /// [`Target`]: #associatedtype.Target
    /// [`Surface`]: #associatedtype.Surface
    fn create_target(
        &mut self,
        surface: &Self::Surface,
        width: u32,
        height: u32,
        scale_factor: f64,
    ) -> Self::Target;

    /// Draws the output primitives to the given [`Target`].
    ///
    /// [`Target`]: #associatedtype.Target
    /// [`Surface`]: #associatedtype.Surface
    fn draw<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        target: &mut Self::Target,
        output: &<Self::Renderer as crate::Renderer>::Output,
        overlay: &[T],
    ) -> MouseCursor;
}
