///////////////////////////////////////////////////////////////////////////////
/// FFI for C/C++
/////////////////////////////////////////////////////////////////////////////// 
#[repr(C)]
pub struct MyStruct {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}

#[unsafe(no_mangle)]
pub extern "C" fn process(s: *mut MyStruct) {
    if s.is_null() {
        return;
    }
    unsafe {
        (*s).result = (*s).a + (*s).b;
    }
}

///////////////////////////////////////////////////////////////////////////////
/// rust sample
/////////////////////////////////////////////////////////////////////////////// 
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
