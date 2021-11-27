pub mod Proxy {
    use reqwest::*;
    

    pub enum ProxyTypes {
        http,
        https,
        socks4,
        socks5
    }

    pub struct Proxy {
        ip: String,
        port: i32,
        proxy_type: ProxyTypes
    }

    pub struct Reqeust {
        url: String,
        client: Client,
        proxy: Proxy
    }


    trait ProxyTrait {
        pub fn Create(ip: String, port: i32, proxy_type: ProxyTypes) -> Proxy;
    }

    impl ProxyTrait for Proxy {
        fn Create() -> Proxy {
            let proxy = Proxy {
                ip: String = ip,
                port: i32 = port,
                proxy_type: ProxyTypes = proxy_type
            };
            return proxy;
        }
    }
}

pub mod Web {
    use reqwest;

    pub fn create_client() -> reqwest::Client {
        
    }
}