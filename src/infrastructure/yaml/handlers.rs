use std::{collections::HashMap, any::Any, future::Future};
use configs::config::Config;

use crate::configs;

struct ApiEndpoint {
    pub name: String,
}
impl ApiEndpoint {

}
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, body::BoxBody};
type Handle = Box<(dyn Fn(HttpRequest) -> HttpResponse)>;
struct Handler {
    pub method: String,
    pub handle: Handle
}


#[get("/")]
pub async fn get_loja() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
struct Proxy;
impl Proxy {
    pub fn build_proxy(path: String, method: String) -> Box<(dyn Fn(HttpRequest) -> HttpResponse)> {
        fn proxy(info: HttpRequest) -> HttpResponse {
            // do request with method: path
            HttpResponse::Ok().body("info")
        }
        return Box::new(proxy);
    }
}

pub fn register_handlers(cf: Config) -> HashMap<String, HashMap<String, HashMap<String, Handle>>>{
    let mut hosts: HashMap<String, HashMap<String, HashMap<String, Handle>>> = HashMap::new();
    for key in cf.pipelines {
        for endpoint_key in key.1.api_endpoints {
            for endpointVec in cf.api_endpoints.get(&endpoint_key) {
                for endpoint in endpointVec {
                    for method in &endpoint.methods {
                        for path in endpoint.paths {
                            if hosts.get(&endpoint.host).is_none(){
                                print!("{:?} {:?} {:?}", endpoint.host, path, method);
                                let mut path_map = HashMap::new();
                                let mut method_map = HashMap::new();
                                let mut handle = 
                                method_map.insert(method.to_string(), Proxy::build_proxy(path, method.to_string()));
                                path_map.insert(path.to_string(), method_map);
                                hosts.insert(endpoint.host, path_map);
                            }
                        }
                    }
                }
            }
        }
        /*for policy in key.1.policies{
            for key in policy.keys() {
                if key == "proxy" {
                    policy[key]
                }
            }
        }*/
    }
    return hosts;
    /*for key in cf.api_endpoints {
        api_endpoints.push(ApiEndpoint::new(key));
        add_endpoint.add_endpoint()
        
    }*/
}