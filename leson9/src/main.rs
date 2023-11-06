use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::io::Write;
use std::fs;
use std::path::Path;
use std::net::TcpStream;
use std::result::Result;
use std::error::Error;

use bincode;

use serde::{Deserialize, Serialize};
use image::io::Reader as ImageReader;
use clap::Parser;
use std::io;




#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg( long,default_value_t=String::from("localhost"))]
    hostname: String,

    /// Number of times to greet
    #[arg( long, default_value_t = 11111)]
    port: u16,
}

#[derive(Serialize, Deserialize)]
enum Message {
    Text(String),
    Image(String, Vec<u8>),
    File(String, Vec<u8>), 
}

enum MessageType {
    Text,
    Image,
    File,
}


fn convert_to_png(path: &Path)->Result<Vec<u8>,Box<dyn Error>>{
    let img = ImageReader::open(path)?.decode()?;
    
let mut bytes: Vec<u8> = Vec::new();
img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    Ok(bytes)
}


fn get_file_as_byte_vec(filename: &Path) -> Result<Vec<u8>,Box<dyn Error>> {
    let mut f = File::open(&filename)?;
    let metadata = fs::metadata(&filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer)?;

    Ok(buffer)
}

fn generic_file_procesor(message: &MessageType, path: &Path)->Result<Message,Box<dyn Error>>{
 if ! path.is_file()
{
   return Err("file does not exist".into())
}
let to_encode:Result<Vec<u8>,Box<dyn Error>> ;
let mut filename:String=path.file_name().unwrap().to_str().unwrap().to_string();


match  message{
    MessageType::Image=>{
        to_encode=convert_to_png(&path);
        filename=filename.replace(path.extension().unwrap().to_str().unwrap(), "png")
    }
    MessageType::File =>{
        to_encode=get_file_as_byte_vec(path);
    }
    _ =>{ to_encode=  Err("forgot to implement mesage type".into()); }// Add handling for other message types (Image and File) here

}
Ok(match message {
    MessageType::Image => Message::Image(filename, to_encode?),
    MessageType::File => Message::File(filename, to_encode?),
    _ => Message::Text("".to_string()),
})
}




fn serialize_message(message: &MessageType, location_or_msg: &str) -> Result<Vec<u8>,Box<dyn Error>> {
    let  msg:Result<Message,Box<dyn Error>>;
     
    match message {
        MessageType::Text => {
             msg = Ok(Message::Text(location_or_msg.to_string()));

        }
        MessageType::File | MessageType::Image=>{
            let path=Path::new(location_or_msg);
            msg=generic_file_procesor(message,path);
        }
    }

    let encoded: Vec<u8> = bincode::serialize(&msg?)?;

    Ok(encoded)
}

fn send_message(address: &str, message: &MessageType, location_or_msg: &str) {
    let serialized = serialize_message(message, location_or_msg);
    if let Err(err) = serialized {
        eprintln!("Error serializing message: {:?}", err);
        return;
    }

    let data = serialized.unwrap();
    let mut stream = TcpStream::connect(address).unwrap();

    let len = data.len() as u64;
    stream.write_all(&len.to_be_bytes()).unwrap();
    stream.write_all(&data).unwrap();

    // Ensure that the message is sent by flushing the stream
    stream.flush().unwrap();
}

fn main() {
    let args = Args::parse();
    let address = format!("{}:{}",args.hostname,args.port);
    loop  {
        let mut input = String::new();
        let _=io::stdin().read_line(&mut input);
        if input==".quit"{
            break;
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        let message_type :MessageType;
        let message_content:String;
        if parts.len()!=2{
           message_type=MessageType::Text;
           message_content=input
           
        } else if parts[0]==".file" ||parts[0]==".image"{
            match parts[0] {
                ".file"=>{message_type=MessageType::File;
                    message_content=parts[1].into();}
                ".image"=>{message_type=MessageType::Image;
                    message_content=parts[1].into();}
                _=>{
                    message_type=MessageType::Text;
                    message_content=input
                }
            }
        }else {
            message_type=MessageType::Text;
            message_content=input
        }
        send_message(address.as_str(), &message_type, message_content.as_str());
    }
    
}
