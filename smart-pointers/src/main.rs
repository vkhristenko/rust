fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}

fn test0() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn test1() {
    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));

    println!("list is {:?}", list);
}

fn test2() {
    let x = 5;
    let y = &x;

    println!("x = {}", x);
    println!("y = {}", *y);
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn test3() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let lmbda = |name: &str| { println!("hello, {}!", name); };
    let m = MyBox::new(String::from("rust"));
    lmbda(&m);
    lmbda(&(*m)[..])
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn test4() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created");
}

fn test5() {
    let x = CustomSmartPointer { data: String::from("here we go") };
    std::mem::drop(x);
//    x.drop();
}

enum ConsList {
    Cons(i32, Rc<ConsList>),
    Nil,
}

use std::rc::Rc;

fn test6() {
    let a = Rc::new(ConsList::Cons(5,
        Rc::new(ConsList::Cons(10,
            Rc::new(ConsList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsList::Cons(4, Rc::clone(&a));
        println!("coutn after creating c = {}", Rc::strong_count(&a));
    }
    println!("coutn after c goes out of scope = {}", Rc::strong_count(&a));
}
