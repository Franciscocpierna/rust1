// Define uma trait chamada Network com os métodos ping, traceroute e nslookup
pub struct Router {
    pub ip: String,
}

pub trait Network {
    fn ping(&self, host: &str) -> bool;
    fn traceroute(&self, host: &str) -> Vec<String>;
    fn nslookup(&self, host: &str) -> String;
}

impl Network for Router {
    // Implementação do método ping
    fn ping(&self, host: &str) -> bool {
        // Simulando uma verificação de ping
        println!("Verificando conexão com {} através do IP {}", host, self.ip);
        true
    }

    // Implementação do método traceroute
    fn traceroute(&self, host: &str) -> Vec<String> {
        // Simulando um trace de rota
        println!("Executando trace de rota para {} a partir do IP {}", host, self.ip);
        vec!["192.168.1.1".to_string(), "192.168.1.2".to_string(), "192.168.1.3".to_string()]
    }

    // Implementação do método nslookup
    fn nslookup(&self, host: &str) -> String {
        // Simulando uma consulta de DNS
        println!("Executando consulta de DNS para {} a partir do IP {}", host, self.ip);
        "192.168.1.100".to_string()
    }
}