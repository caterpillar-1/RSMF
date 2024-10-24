use rsmf::bcos::Sdk;

fn main() {
    let config = r#"
[common]
    ; if ssl connection is disabled, default: false
    ; disable_ssl = true
    ; thread pool size for network message sending receiving handing
    thread_pool_size = 8
    ; send message timeout(ms)
    message_timeout_ms = 10000

; ssl cert config items,  
[cert]
    ; ssl_type: ssl or sm_ssl, default: ssl
    ssl_type = ssl
    ; directory the certificates located in, default: ./conf
    ca_path=./conf
    ; the ca certificate file
    ca_cert=ca.crt
    ; the node private key file
    sdk_key=sdk.key
    ; the node certificate file
    sdk_cert=sdk.crt

[peers]
# supported ipv4 and ipv6 
    node.0=127.0.0.1:20200
    node.1=127.0.0.1:20201
    "#;
    let mut sdk = Sdk::new(config);
    println!("{}", sdk.version());
}
