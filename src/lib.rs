include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut lib_array = [0i8, 0, 0, 0];
        unsafe {HH_GetLibraryVersion(lib_array.as_mut_ptr());}
    }
}
