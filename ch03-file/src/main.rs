#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: name.into(),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: Vec<u8>) -> File {
        File {
            name: name.into(),
            data,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        save_to.reserve(self.data.len());
        save_to.append(self.data.clone().as_mut());
        self.data.len()
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let mut f1 = File::new_with_data("f1.txt", vec![114, 117, 115, 116, 33]);
    let mut _f2 = File::new("f2.txt");

    let mut buf: Vec<u8> = vec![];

    open(&mut f1);
    let f1_length = f1.read(&mut buf);
    close(&mut f1);

    // use buffer
    let text = String::from_utf8_lossy(&buf);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    println!("{}", text);
}
