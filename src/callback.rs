use std::marker::PhantomData;

pub trait Callback {
    type Arg;

    fn call(&self, arg: Self::Arg);
}

pub struct Callback1<A, M, CB> 
    where M: ComponentMsg,
          CB: Fn(A) -> M
{
    cb: CB,
    _m1: PhantomData<A>,
    _m2: PhantomData<M>
}

impl<A, M, CB> Callback for Callback1<A, M, CB>
    where M: ComponentMsg,
          CB: Fn(A) -> M
{
    type Arg = A;

    fn call(&self, arg: Self::Arg) {
        
    }
}

impl<A, M, CB> Callback1<A, M, CB>
    where M: ComponentMsg,
          CB: Fn(A) -> M
{
    pub fn new(cb: CB) -> Self {
        Self {
            cb,
            _m1: PhantomData,
            _m2: PhantomData
        }
    }
}