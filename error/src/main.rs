static mut ERROR: i32 = 0;

fn main() {
    let mut f = File::new("something.txt");
    
    read(f, buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while reading the file")
        }
    }
    close(f);

    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing the file")
        }
    }
}
