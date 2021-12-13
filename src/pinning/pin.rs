use std::pin::Pin;
use std::marker::PhantomPinned;

async fn read_into_buf(x: &mut [i32; 128]) {}

fn tolumide() {
    async {
        let mut x = [0; 128];
        let read_into_buf_fut = read_into_buf(&mut x);
        read_into_buf_fut.await;
        println!("{:?}", x);
    };
}



#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test { 
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned // this makes our type `!Unpin`
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.get_ref().a;
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b called wuthout Test::init being called first");
        unsafe { &*(self.b) }
    }
}



// pinning to the heap
#[derive(Debug)]
struct HTest {
    a: String,
    b: *const String,
    _marker: PhantomPinned
}

impl HTest {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = HTest {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned
        };

        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe {boxed.as_mut().get_unchecked_mut().b = self_ptr};
        boxed
    }

    fn a<'a>
}