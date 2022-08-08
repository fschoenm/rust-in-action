#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &mut File, save_to: &mut Vec<u8>) -> usize {
    save_to.reserve(f.data.len());
    save_to.append(f.data.clone().as_mut());
    f.data.len()
}

fn main() {
    let mut f1 = File {
        name: String::from("f1.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buf: Vec<u8> = vec![];

    open(&mut f1);
    let f1_length = read(&mut f1, &mut buf);
    close(&mut f1);

    // use buffer
    let text = String::from_utf8_lossy(&buf);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    println!("{}", text);
}
