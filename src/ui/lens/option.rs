use druid::Lens;
use std::marker::PhantomData;
use druid::lens::Constant;

pub struct OptionLens<TIn, TOut: Clone, LSome: Lens<TIn, TOut>>
{
    pub some_lens: LSome,
    pub none_lens: Constant<TOut>,
    phantom_in: PhantomData<TIn>
}

impl<TIn, TOut: Clone, LSome: Lens<TIn, TOut>> OptionLens<TIn, TOut, LSome> {
    pub fn new(some_lens: LSome, none_value: TOut) -> OptionLens<TIn, TOut, LSome>
    {
        return OptionLens {
            some_lens,
            none_lens: Constant(none_value),
            phantom_in: Default::default()
        };
    }
}

impl<TIn, TOut: Clone, LSome: Lens<TIn, TOut>> Lens<Option<TIn>, TOut> for OptionLens<TIn, TOut, LSome> {
    fn with<R, F: FnOnce(&TOut) -> R>(&self, data: &Option<TIn>, f: F) -> R {
        return match data {
            Some(d) => self.some_lens.with(d, f),
            None => self.none_lens.with(&(), f)
        };
    }

    fn with_mut<R, F: FnOnce(&mut TOut) -> R>(&self, data: &mut Option<TIn>, f: F) -> R {
        return match data {
            Some(d) => self.some_lens.with_mut(d, f),
            None => self.none_lens.with_mut(& mut(), f)
        };
    }
}