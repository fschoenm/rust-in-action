use rand::Rng;

fn one_in(n: u32) -> bool {
    rand::thread_rng().gen_bool(1.0 / n as f64)
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: name.into(),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        File {
            name: name.into(),
            data: data.clone(),
            state: FileState::Closed,
        }
    }
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(format!("File {} is not open", self.name));
        }

        save_to.reserve(self.data.len());
        save_to.append(self.data.clone().as_mut());

        Ok(self.data.len())
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("permission denied");
        return Err(err_msg);
    }

    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("interrupted by signal");
        return Err(err_msg);
    }

    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f1 = File::new_with_data("f1.txt", &vec![114, 117, 115, 116, 33]);
    let mut _f2 = File::new("f2.txt");

    let mut buf: Vec<u8> = vec![];

    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buf).unwrap();
    f1 = close(f1).unwrap();

    // use buffer
    let text = String::from_utf8_lossy(&buf);

    println!("{:?}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    println!("{}", text);
}
