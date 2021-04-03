/*use druid::Lens;

struct IsSome<T>;

impl Lens<T, bool> for IsSome<T> {
    fn with<R, F: FnOnce(Option<T>) -> R>(&self, option: Option<T>, f: F) -> R {
        return option.is_some();
    }

    fn with_mut<R, F: FnOnce(&mut f64) -> R>(&self, data: &mut f64, f: F) -> R {
        panic!("Not supported");
    }
}*/