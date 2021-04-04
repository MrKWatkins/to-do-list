use druid::{Lens, LensExt};
use druid::lens::Then;

pub struct IsSomeLens;

impl IsSomeLens {
    pub fn new() -> IsSomeLens {
        return IsSomeLens;
    }
}

impl<TIn> Lens<Option<TIn>, bool> for IsSomeLens {
    fn with<R, F: FnOnce(&bool) -> R>(&self, data: &Option<TIn>, f: F) -> R {
        return f(&data.is_some());
    }

    fn with_mut<R, F: FnOnce(&mut bool) -> R>(&self, data: &mut Option<TIn>, f: F) -> R {
        return f(&mut data.is_some());
    }
}

pub trait IsSomeLensExt<TIn: ?Sized, TOut: Sized>: Lens<TIn, Option<TOut>> + Sized {
    fn then_is_some(self) -> Then<Self, IsSomeLens, Option<TOut>>
    {
        return self.then(IsSomeLens);
    }
}