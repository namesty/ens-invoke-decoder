pub mod wrap;
pub use wrap::*;

impl ModuleTrait for Module {
    fn register_domain_and_subdomains_recursively(args: ArgsRegisterDomainAndSubdomainsRecursively) -> Result<Option<String>, String> {
        let domain = args.domain;
        let resolver = args.resolver_address;
        let ttl = args.ttl;
        Ok(Some(format!("Recursively register {domain} domain with resolver '{resolver}' and TTL {ttl}")))
    }
}

#[cfg(test)]
mod tests {
  use std::{fs, path::PathBuf, sync::Arc};

use polywrap_client::{builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}, core::{uri::Uri, file_reader::SimpleFileReader, invoker::Invoker}, wasm::wasm_wrapper::WasmWrapper, client::PolywrapClient};

use crate::ArgsRegisterDomainAndSubdomainsRecursively;

  #[test]
    fn it_works() {
      let bytes = fs::read(PathBuf::from("./build/wrap.wasm")).unwrap();
      let wrapper_uri = Uri::try_from("embed/test").unwrap();

      let mut config = PolywrapClientConfig::new();
      
      config.add_wrapper(wrapper_uri.clone(), Arc::new(
        WasmWrapper::try_from_bytecode(&bytes, Arc::new(SimpleFileReader::new())).unwrap()
      ));

      let client = PolywrapClient::new(config.into());

      let result = client.invoke_raw(&wrapper_uri,
        "registerDomainAndSubdomainsRecursively",
        Some(&polywrap_client::msgpack::to_vec(&ArgsRegisterDomainAndSubdomainsRecursively {
          domain: "foo".to_string(),
          owner: "foo".to_string(),
          resolver_address: "foo".to_string(),
          ttl: "foo".to_string(),
          registrar_address: "foo".to_string(),
          registry_address: "foo".to_string(),
        }).unwrap()),
        None,
        None
      ).unwrap();

      println!("{:?}", result)
    }
}