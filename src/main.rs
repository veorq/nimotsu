use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{
    fs::File,
    io::{self, Write},
};

use curve25519::{exchange, gen_keypair};
use rand::rngs::OsRng;
use structopt::StructOpt;

mod blake3;
mod chacha20;
mod curve25519;

#[derive(StructOpt, Debug)]
#[structopt(name = "nimotsu")]
enum Args {
    /// Generate a new keypair
    ///
    /// The public key will be printed out, the private key will be saved to a file
    Generate {
        /// The file to write the private key into
        #[structopt(short = "o", long = "out", parse(from_os_str))]
        out_file: PathBuf,
    },
    /// Encrypt a file for a given recipient
    Encrypt {
        /// The recipient to encrypt to
        #[structopt(short = "r", long = "recipient")]
        recipient: String,
        /// The file to write the encrypted data to
        #[structopt(short = "o", long = "out", parse(from_os_str))]
        out_file: PathBuf,
        /// The file contained the data to encrypt
        #[structopt(name = "INPUT_FILE", parse(from_os_str))]
        in_file: PathBuf,
    },
    /// Decrypt a file encrypted for you
    Decrypt {
        /// A path to your private key file
        #[structopt(short = "k", long = "key", parse(from_os_str))]
        key_file: PathBuf,
        /// The file contained the data to decrypt
        #[structopt(name = "INPUT_FILE", parse(from_os_str))]
        in_file: PathBuf,
        /// The file to write the decrypted to, or directly to the console
        #[structopt(short = "o", long = "out", parse(from_os_str))]
        out_file: Option<PathBuf>,
    },
}

fn bytes_to_hex<W: io::Write>(data: &[u8], writer: &mut W) -> io::Result<()> {
    for byte in data {
        write!(writer, "{:02X}", byte)?;
    }
    Ok(())
}

fn generate(out_file: &Path) -> io::Result<()> {
    let (pub_key, priv_key) = curve25519::gen_keypair(&mut OsRng);
    print!("Public Key:\n荷物の公開鍵");
    bytes_to_hex(&pub_key.bytes, &mut io::stdout())?;
    println!();
    let mut file = File::create(out_file)?;
    write!(file, "# Public Key: 荷物の公開鍵")?;
    bytes_to_hex(&pub_key.bytes, &mut file)?;
    write!(file, "\n荷物の秘密鍵")?;
    bytes_to_hex(&priv_key.bytes, &mut file)?;
    Ok(())
}

fn encrypt(recipient: &str, out_file: &Path, in_file: &Path) -> io::Result<()> {
    unimplemented!()
}

fn decrypt(key_file: &Path, in_file: &Path, out_file: Option<&Path>) -> io::Result<()> {
    unimplemented!()
}

fn main() -> io::Result<()> {
    let args = Args::from_args();
    match args {
        Args::Generate { out_file } => generate(&out_file),
        Args::Encrypt {
            recipient,
            out_file,
            in_file,
        } => encrypt(&recipient, &out_file, &in_file),
        Args::Decrypt {
            key_file,
            in_file,
            out_file,
        } => decrypt(&key_file, &in_file, out_file.as_ref().map(|x| &*x as &Path)),
    }
}
