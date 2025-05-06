use super::types::Order;
use rust2go;

#[repr(C)]
#[derive(rust2go::R2G,Clone,Debug)]
pub struct Exchange {
    pub name: String,
    pub id: String,
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
    pub api_rate_limit: u64
}

#[rust2go::r2g]
trait ExchangeAPI { 
    // fn watch_order(e:&Exchange,symbol:String)-> impl std::future::Future<Output = Vec<Order>>;
    fn fetch_order(e:&Exchange,symbol:String)-> Order;
    fn new(name: String, id: String, api_key: String, api_secret: String, api_passphrase: String,api_rate_limit:u64) -> Exchange;
}

// impl Exchange {
//     pub fn new(name: String, id: String, api_key: String, api_secret: String, api_passphrase: Option< String>,api_rate_limit:u64) -> Exchange {
//         Exchange {
//             name,
//             id,
//             api_key,
//             api_secret,
//             api_passphrase,
//             api_rate_limit
//         }
//     }
    
// }