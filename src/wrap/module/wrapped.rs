use polywrap_client::msgpack::{from_slice, to_vec};
use serde::{Deserialize, Serialize};

use crate::module::{ModuleTrait, Module};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRegisterDomainAndSubdomainsRecursively {
    pub domain: String,
    pub owner: String,
    pub resolver_address: String,
    pub ttl: String,
    pub registrar_address: String,
    pub registry_address: String,
}

pub fn register_domain_and_subdomains_recursively_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsRegisterDomainAndSubdomainsRecursively>(args) {
        Ok(args) => {
            let result = Module::register_domain_and_subdomains_recursively(ArgsRegisterDomainAndSubdomainsRecursively {
                domain: args.domain,
                owner: args.owner,
                resolver_address: args.resolver_address,
                ttl: args.ttl,
                registrar_address: args.registrar_address,
                registry_address: args.registry_address,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
