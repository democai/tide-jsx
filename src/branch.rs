use crate::Render;
use crate::Fragment;

/// Conditionally render one of two components based on a boolean condition.
///
/// This function helps simplify conditional rendering.
/// It takes a boolean condition and two closures, executing only one based on the condition.
///
/// # Example
///
/// ```
/// let fragment = branch(
///     notifications.is_empty(),
///     || {
///         rsx! {
///             <p>{"No notifications at this time"}</p>
///         }
///     },
///     || {
///         rsx! {
///             <p>{"Notifications"}</p>
///             <NotificationWidget />
///         }
///     },
/// );
/// ```
///
/// In this example, if `notifications.is_empty()` is true, it will render a paragraph
/// saying "No notifications at this time". Otherwise, it will render a paragraph
/// saying "Notifications" followed by a `NotificationWidget` component.
pub fn branch<F1, F2, R1, R2>(condition: bool, f1: F1, f2: F2) -> Fragment<(Option<R1>, Option<R2>)>
where
    F1: FnOnce() -> R1,
    F2: FnOnce() -> R2,
    R1: Render + Clone,
    R2: Render + Clone,
{
    if condition {
        Fragment { children: (Some(f1()), None) }
    } else {
        Fragment { children: (None, Some(f2())) }
    }
}
