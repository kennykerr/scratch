// winrt::import!(
//     dependencies:
//         "os"
//     modules:
//         "windows.foundation.collections"
// );

struct Thing {}

trait IStringable {
    // This works fine:
    fn to_string1(&self);

    // This does not:
    fn to_string2<'a, T: Into<Thing>>(&self, t: T);
}

struct Rc<T: ?Sized> {
    placeholder: std::marker::PhantomData<T>,
}

fn call(thing: Rc<dyn IStringable>) {
}

fn main() {
}
