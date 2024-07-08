// cast struct to &[u8], and cast [u8] to struct

use std::mem;

enum MaybeOwned<'a, T> {
    Ref(&'a T),
    Boxed(Box<T>)
}

impl<'a,T> std::ops::Deref for MaybeOwned<'a, T> {
    type Target=T;
    fn deref(&self)->&T {
        match self {
            Self::Ref(r) => r,
            Self::Boxed(ref x) => &*x
        }
    }
}

#[cfg(target_endian = "little")]
unsafe trait FromLEByteSlice: Sized {
    fn cast_aligned<'a>(bytes: &'a [u8]) -> Option<&'a Self> {
        assert_eq!(bytes.len(), mem::size_of::<Self>());
        let ptr: *const u8 = bytes.as_ptr();
        match ptr.align_offset(mem::align_of::<Self>()) {
            0 => unsafe { ptr.cast::<Self>().as_ref() },
            _ => None
        }
    }

    fn read_unaligned(bytes: & [u8]) -> Self {
        assert_eq!(bytes.len(), mem::size_of::<Self>());
        unsafe { bytes.as_ptr().cast::<Self>().read_unaligned() }
    }

    fn cast<'a>(bytes: &'a [u8]) -> MaybeOwned<'a, Self> {
        match Self::cast_aligned(bytes) {
            Some(x) => MaybeOwned::Ref(x),
            None => MaybeOwned::Boxed(Box::new(Self::read_unaligned(bytes)))
        }
    }
}


unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::core::mem::size_of::<T>(),
    )
}

unsafe fn cast_to_struct_ref<'a, T>(bytes: &'a [u8]) -> &'a T {
    // assert correct endianness somehow
    assert_eq!(bytes.len(), std::mem::size_of::<T>());
    let ptr: *const u8 = bytes.as_ptr();
    assert_eq!(ptr.align_offset(std::mem::align_of::<T>()), 0);

    ptr.cast::<T>().as_ref().unwrap()
}

#[repr(C)]
struct P6L_S {
    pub nCmd : u32,
    pub nPar : i32,
    pub A9F  : [f64;9]
}

fn main() {
    let p6l = P6L_S {
        nCmd: 1,
        nPar: 2,
        A9F: [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    };
    // 1) cast data to [u8]
    let u8d = unsafe { any_as_u8_slice(&p6l) };
    println!("u0={},u1={}", u8d[0], u8d[1]);

    // let s = unsafe { std::mem::transmute::<[u8;80], P6L_S>(u8d[0..79]) };
    let ss: &P6L_S = unsafe { cast_to_struct_ref(u8d) };
    println!("Cmd={},Par={},AF9={}", ss.nCmd, ss.nPar, ss.A9F[8]);

    println!("p0={:p}, p1={:p}, p3={:p}",&p6l, u8d, ss);
}