fn main() {
    let b = unsafe { std::mem::transmute::<u8, bool>(2) };
    let _x = b == true; //~ ERROR invalid boolean value read
}
