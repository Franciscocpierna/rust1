mod it;
use it::{Router, Network};

fn main() {
    let router = Router {
        ip: "192.168.1.1".to_string(),
    };
    
    let is_alive = router.ping("google.com");
    let trace = router.traceroute("google.com");
    let ip = router.nslookup("google.com");

    println!("O host está online: {}", is_alive);
    println!("Trace de rota: {:?}", trace);
    println!("Endereço IP: {}", ip);
}

