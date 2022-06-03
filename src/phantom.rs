use std::fmt::Debug;
use std::marker::PhantomData;
struct Deserializer<T> {
    _t: PhantomData<T>, //generic over a type T, but doesn't contain any data
                        //is considering dropping the T
}
struct Deserializer2<T> {
    //can't drop a T, but can drop a fn() -> T
    _t1: PhantomData<fn() -> T>, //covariant
    //or with raw pointer
    _t2: PhantomData<*const T>, // also covariant
}
struct Deserializer3<T> {
    _t: PhantomData<fn(T)>, //contravariant
}
struct Deserializer4<T> {
    //covariant and contravariant -> invariant in T
    _t: PhantomData<fn(T) -> T>,
}

struct TouchDrop<T: Debug>(T);
//impl Drop
impl<T: Debug> Drop for TouchDrop<T> {
    fn drop(&mut self) {
        println!("Dropping {:?}", self.0); //access the inner reference(T) when is dropped
    }
}

#[test]
fn foo() {
    let x = String::from("hello");
    // let z = vec![TouchDrop(&x)];
    let z = vec![&x]; //dropping the reference do not access the inner type of drop -> DROP CHECK
                      //z is never access again
    drop(x);
}
