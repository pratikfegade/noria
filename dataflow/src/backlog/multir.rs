use basics::DataType;
use srmap;
use std::collections::HashMap;
use fnv::FnvBuildHasher;

#[derive(Clone)]
pub(super) enum Handle {
    SingleSR(srmap::srmap::Handle<DataType, Vec<DataType>, i64>),
    DoubleSR(srmap::srmap::Handle<(DataType, DataType), Vec<DataType>, i64>),
    ManySR(srmap::srmap::Handle<Vec<DataType>, Vec<DataType>, i64>),
    // Single(evmap::ReadHandle<DataType, Vec<DataType>, i64, FnvBuildHasher>),
    // Double(evmap::ReadHandle<(DataType, DataType), Vec<DataType>, i64, FnvBuildHasher>),
    // Many(evmap::ReadHandle<Vec<DataType>, Vec<DataType>, i64, FnvBuildHasher>),
}

impl Handle {
    pub fn len(&self) -> usize {
        match *self {
            // Handle::Single(ref h) => h.len(),
            Handle::SingleSR(ref h) => h.len(),
            // Handle::Double(ref h) => h.len(),
            Handle::DoubleSR(ref h) => h.len(),
            // Handle::Many(ref h) => h.len(),
            Handle::ManySR(ref h) => h.len(),
        }
    }

    pub fn for_each<F>(&self, mut f: F, uid: usize)
    where
        F: FnMut(&[Vec<DataType>]),
    {
        match *self {
            // Handle::Single(ref h) => h.for_each(|_, v| f(v)),
            // Handle::Double(ref h) => h.for_each(|_, v| f(v)),
            // Handle::Many(ref h) => h.for_each(|_, v| f(v)),
            Handle::SingleSR(ref h) => h.for_each(|_, v| f(v), uid),
            Handle::DoubleSR(ref h) => h.for_each(|_, v| f(v), uid),
            Handle::ManySR(ref h) => h.for_each(|_, v| f(v), uid),
        }
    }

    pub fn meta_get_and<F, T>(&self, key: &[DataType], then: F, uid: usize) -> Option<(Option<T>, i64)>
    where
        F: FnOnce(&[Vec<DataType>]) -> T,
    {
        match *self {
            // Handle::Single(ref h) => {
            //     assert_eq!(key.len(), 1);
            //     h.meta_get_and(&key[0], then)
            // },
            Handle::SingleSR(ref h) => {
                assert_eq!(key.len(), 1);
                // println!("call to meta get and");
                h.meta_get_and(&key[0], then, uid)
            },
            // Handle::Double(ref h) => {
            //     assert_eq!(key.len(), 2);
            //     // we want to transmute &[T; 2] to &(T, T), but that's not actually safe
            //     // we're not guaranteed that they have the same memory layout
            //     // we *could* just clone DataType, but that would mean dealing with string refcounts
            //     // so instead, we play a trick where we memcopy onto the stack and then forget!
            //     //
            //     // h/t https://gist.github.com/mitsuhiko/f6478a0dd1ef174b33c63d905babc89a
            //     use std::mem;
            //     use std::ptr;
            //     unsafe {
            //         let mut stack_key: (DataType, DataType) = mem::uninitialized();
            //         ptr::copy_nonoverlapping(
            //             &key[0] as *const DataType,
            //             &mut stack_key.0 as *mut DataType,
            //             1,
            //         );
            //         ptr::copy_nonoverlapping(
            //             &key[1] as *const DataType,
            //             &mut stack_key.1 as *mut DataType,
            //             1,
            //         );
            //
            //         let v = h.meta_get_and(&stack_key, then);
            //         mem::forget(stack_key);
            //         v
            //     }
            // },
            Handle::DoubleSR(ref h) => {
                assert_eq!(key.len(), 2);
                // we want to transmute &[T; 2] to &(T, T), but that's not actually safe
                // we're not guaranteed that they have the same memory layout
                // we *could* just clone DataType, but that would mean dealing with string refcounts
                // so instead, we play a trick where we memcopy onto the stack and then forget!
                //
                // h/t https://gist.github.com/mitsuhiko/f6478a0dd1ef174b33c63d905babc89a
                use std::mem;
                use std::ptr;
                unsafe {
                    let mut stack_key: (DataType, DataType) = mem::uninitialized();
                    ptr::copy_nonoverlapping(
                        &key[0] as *const DataType,
                        &mut stack_key.0 as *mut DataType,
                        1,
                    );
                    ptr::copy_nonoverlapping(
                        &key[1] as *const DataType,
                        &mut stack_key.1 as *mut DataType,
                        1,
                    );

                    let v = h.meta_get_and(&stack_key, then, uid);
                    mem::forget(stack_key);
                    v
                }
            },
            // Handle::Many(ref h) => {
            //     h.meta_get_and(&key.to_vec(), then)
            //  },
             Handle::ManySR(ref h) => {
                 h.meta_get_and(&key.to_vec(), then, uid)
              },
        }
    }
}
