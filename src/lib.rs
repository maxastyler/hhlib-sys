#[macro_use]
extern crate num_derive;

mod bindings {include!(concat!(env!("OUT_DIR"), "/bindings.rs"));};

mod types;

#[cfg(test)]
mod tests {
    use super::bindings::*;
    #[test]
    fn it_works() {
        let mut lib_array = [0i8, 0, 0, 0];
        unsafe {HH_GetLibraryVersion(lib_array.as_mut_ptr());}
        assert_eq!(lib_array, [51i8, 46, 48, 0]);
    }
}
