use std::{collections::HashMap, any::Any, future::Future};
use configs::config::Config;

use crate::configs;

struct ApiEndpoint {
    pub name: String,
}
impl ApiEndpoint {

}
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
type Handle = fn () -> dyn Responder<Body=dyn Any>;
struct Handler {
    pub method: String,
    pub handle: Handle
}


#[get("/")]
pub async fn get_loja() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
/*struct Proxy;
impl Proxy {
    pub fn build_proxy() -> Box<dyn Fn(HttpRequest) -> dyn Future<Output = HttpResponse>> {
        async fn proxy(info: HttpRequest) -> HttpResponse {
            HttpResponse::Ok().body("body")
        }
        return Box::new(proxy);
    }
}*/

pub fn register_handlers(cf: Config) {
    let hosts: HashMap<String, HashMap<String, HashMap<String, Handle>>> = HashMap::new();
    let handlers_endpoints:  = HashMap::new();
    for key in cf.pipelines {
        for endpoint_key in cf.api_endpoints {
            for endpoint in endpoint_key.1 {
                for method in endpoint.methods {
                    for path in endpoint.paths {
                        if hosts.get(&endpoint.host).is_none(){
                            let path_map = HashMap::new();
                            let method_map = HashMap::new();
                            path_map.insert(method, method_map);
                            hosts.insert(endpoint.host, path_map);
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
    /*for key in cf.api_endpoints {
        api_endpoints.push(ApiEndpoint::new(key));
        add_endpoint.add_endpoint()
        
    }*/
}