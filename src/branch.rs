use crate::Render;
use crate::Fragment;

// Help with the annoying case of doing an if statement.
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
