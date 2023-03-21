/// Describes whether or not a component should be rendered after state changes.
/// For uses see:
///
/// * [`Hook::bind`](crate::stateful::Hook::bind)
/// * [`IntoState::update`](crate::stateful::IntoState::update)
pub trait ShouldRender {
    fn should_render(self) -> bool;
}

/// Closures without return type always update their view.
impl ShouldRender for () {
    fn should_render(self) -> bool {
        true
    }
}

/// An enum that implements the [`ShouldRender`](ShouldRender) trait.
/// See:
///
/// * [`Hook::bind`](crate::stateful::Hook::bind)
/// * [`IntoState::update`](crate::stateful::IntoState::update)
pub enum Then {
    /// This is a silent update
    Stop,
    /// Render the view after this update
    Render,
}

impl ShouldRender for Then {
    fn should_render(self) -> bool {
        match self {
            Then::Stop => false,
            Then::Render => true,
        }
    }
}
