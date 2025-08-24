use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Error, Read, Write};
use std::path::Path;

#[allow(dead_code)]
struct Chunk {
    length: u32,
    c_type: [u8; 4],
    data: Box<Vec<u8>>,
    crc: u32,
    required: bool,
}

impl Chunk {
    // Removes unwanted chunks.
    fn destroy(&mut self, counter: &mut usize) {
        if !self.required {
            let mut ammount: usize = self.length.try_into().unwrap();
            ammount += 8; // won't fit on previous line?? why??

            *counter += ammount;
        }
    }

    // Adds required chunks to file.
    fn add_to(&mut self, new_file: &mut File) -> Result<(), Error> {
        new_file.write_all(&self.length.to_be_bytes())?;
        new_file.write_all(&self.c_type)?;
        Ok(())
    }

    fn inform(&mut self) {
        let ctype_ext = String::from_utf8_lossy(&self.c_type);

        println!(
            "Chunk size: {} | type: {} | required: {}",
            self.length, ctype_ext, self.required
        );
    }
}

// This will all also be in pixtools later
fn main() -> Result<(), Error> {
    let file_path = Path::new("/home/chud/testimg.png");
    let file = File::open(file_path)?;
    let length_file = file.metadata().unwrap().len();
    let rm_unwanted = remove_chunks_prompt();
    let mut file_new = create_replacement(rm_unwanted).unwrap(); // Removing chunks can't be done in place

    let mut reader = BufReader::new(file);
    is_png(&mut reader).unwrap();

    let mut bytes_read: u64 = 8;
    let mut chunks_removed: usize = 0;

    // Performs actions while keeping bytes_read updated. Its fast and important.
    while bytes_read < length_file {
        let dat_size: u32 = read_4_to_u32(&mut reader, &mut bytes_read)?;
        let chunktype: [u8; 4] = read_4(&mut reader, &mut bytes_read)?;

        let mut chunk = Chunk {
            length: dat_size,
            c_type: chunktype,
            data: Box::new(read_dat(&mut reader, dat_size, &mut bytes_read)?),
            crc: read_4_to_u32(&mut reader, &mut bytes_read)?,
            required: is_required(chunktype),
        };
        chunk.inform();

        if rm_unwanted {
            chunk.destroy(&mut chunks_removed);
            chunk.add_to(&mut file_new)?;
        }
    }

    println!("\n{}/{} bytes read from png.", bytes_read, length_file);
    Ok(())
}

fn is_png(reader: &mut BufReader<File>) -> Result<(), Error> {
    let mut first_8: [u8; 8] = [0; 8];
    reader.read_exact(&mut first_8)?;

    match first_8 {
        [137, 80, 78, 71, 13, 10, 26, 10] => Ok(()),
        _ => Err(Error::new(std::io::ErrorKind::InvalidInput, "Not a PNG.")),
    }
}

fn read_4(reader: &mut BufReader<File>, counter: &mut u64) -> Result<[u8; 4], Error> {
    let mut four_buf: [u8; 4] = [0; 4];
    reader.read_exact(&mut four_buf)?;
    *counter += 4;
    Ok(four_buf)
}

// Asks user if they want to remove chunks (write only needed chunks to new file).
fn remove_chunks_prompt() -> bool {
    print!("Remove all unwanted chunks? [Y/n]: ");
    let mut rm = String::new();
    io::stdin().read_line(&mut rm).ok();

    // If its unable to read input, or an invalid input is chosen,
    let choice = match rm.trim().to_lowercase().as_str() {
        "y" => true,
        "n" => false,
        _ => {
            print!(
                "Input either invalid or unable to be read. Continuing with default, no chunks will be removed."
            );
            false
        }
    };

    return choice;
}

// Checks if the chunk being read is required for the image to load (we don't want it if it isnt).
fn is_required(bytes: [u8; 4]) -> bool {
    let firstbit: u8 = (bytes[0] >> 5) & 1;
    let required = match firstbit {
        0 => true,
        1 => false,
        _ => {
            println!("ERR: bit is not 1 or 0 (?????? how is this even possible ????)");
            true
        }
    };

    return required;
}

// Basically wraps the result of read_4 (an arr of 4 bytes) into a u32.
fn read_4_to_u32(reader: &mut BufReader<File>, counter: &mut u64) -> Result<u32, Error> {
    let num: u32 = u32::from_be_bytes(read_4(reader, counter).unwrap());
    Ok(num)
}

// Reads exactly the ammount of data outlined in the length bytes of the chunk.
fn read_dat(reader: &mut BufReader<File>, size: u32, counter: &mut u64) -> Result<Vec<u8>, Error> {
    let mut data = vec![0; size as usize];
    reader.read_exact(&mut data)?;
    *counter += size as u64;
    Ok(data)
}

fn create_replacement(remove: bool) -> Option<File> {
    match remove {
        true => {
            let path = Path::new("/home/chud/cleaned.png");
            let rep_file = OpenOptions::new().create(true).append(true).open(path);
            return Some(rep_file.unwrap());
        }
        false => None,
    }
<<<<<<< HEAD:src/main.rs
}
=======
}
>>>>>>> 54765055607a92c19599fb6ecb663ae521ce2003:main.rs
