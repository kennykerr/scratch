trait RuntimeType {
    type ValueType;
    type AbiType;
}

impl RuntimeType for i32 {
    type ValueType = Self;
    type AbiType = Self;
}

pub trait QueryType {
    fn type_guid() -> i32;
}

struct Rc<T: RuntimeType + ?Sized> {
    placeholder: std::marker::PhantomData<T>,
}

impl<T: RuntimeType + ?Sized> Rc<T> {
    fn new() -> Rc<T> {
        Rc { placeholder: std::marker::PhantomData }
    }
}

impl<T: RuntimeType + ?Sized> RuntimeType for Rc<T> {
    type ValueType = Self;
    type AbiType = usize;
}

#[derive(PartialEq)]
struct Button {}
impl RuntimeType for Button {
    type ValueType = Self;
    type AbiType = usize;
}

trait IClosable {
    fn close(&self);
}

trait IStringable: IClosable {
    fn to_string(&self) -> String;
}

impl QueryType for IStringable {
    fn type_guid() -> i32 {
        1
    }
}

impl RuntimeType for dyn IClosable {
    type ValueType = Rc<Self>;
    type AbiType = usize;
}

impl RuntimeType for dyn IStringable {
    type ValueType = Rc<Self>;
    type AbiType = usize;
}

trait IVector<T: RuntimeType> {
    fn get_at(&self, index: u32) -> T::ValueType;
}

impl<T: RuntimeType + ?Sized> RuntimeType for dyn IVector<T> {
    type ValueType = Rc<Self>;
    type AbiType = usize;
}

impl<T: RuntimeType, I: IVector<T> + RuntimeType + ?Sized> IVector<T> for Rc<I> {
    fn get_at(&self, index: u32) -> T::ValueType {
        panic!("call abi");
    }
}

impl Rc<dyn IClosable> {
    fn close(&self) {
        panic!("IClosable: call abi directly");
    }
}

impl<I> IClosable for Rc<I>
where
    I: IClosable + RuntimeType + ?Sized,
{
    fn close(&self) {
        panic!("IClosable: call abi via QI");
    }
}

impl<I: IStringable + RuntimeType + ?Sized> IStringable for Rc<I> {
    fn to_string(&self) -> String {
        panic!("call abi");
    }
}

impl QueryType for IVector<i32> {
    fn type_guid() -> i32 {
        2
    }
}

impl QueryType for IVector<Button> {
    fn type_guid() -> i32 {
        3
    }
}

impl QueryType for IVector<Rc<dyn IStringable>> {
    fn type_guid() -> i32 {
        4
    }
}

fn call1(v: &Rc<dyn IVector<i32>>) {
    assert!(v.get_at(0) == 0);
}

fn call2(v: &Rc<dyn IVector<Button>>) {
    assert!(v.get_at(0) == Button {});
}

fn call3(v: &Rc<dyn IVector<Rc<dyn IStringable>>>) {
    v.get_at(0);
}

fn call4(s: &Rc<dyn IStringable>) {
    s.to_string();
    s.close();
}

fn call5(s: &Rc<dyn IClosable>) {
    s.close();
}

fn main() {
    assert!(<IStringable as QueryType>::type_guid() == 1);
    assert!(<IVector<i32> as QueryType>::type_guid() == 2);
    assert!(<IVector<Button> as QueryType>::type_guid() == 3);
    assert!(<IVector<Rc<IStringable>> as QueryType>::type_guid() == 4);

    call5(&Rc::new());
}
