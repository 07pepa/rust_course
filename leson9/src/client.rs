// i runned out of time 
// but idea was just to copy deseralization stuff and for each incoming connection spawn thread and then handle incoming msges in folowing matter
// wait/read long (u64) figure out lenght and then read msg
// it would be better to use some standard like websocket/grpc something higher level but required serialization forced my hand

fn main() {
    let args = Args::parse();
    let address = format!("{}:{}",args.hostname,args.port);
    loop  {
        
    }
    
}
