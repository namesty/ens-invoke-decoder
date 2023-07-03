use crate::{
    ArgsRegisterDomainAndSubdomainsRecursively,
};

pub struct Module;

pub trait ModuleTrait {
  fn register_domain_and_subdomains_recursively(args: ArgsRegisterDomainAndSubdomainsRecursively) -> Result<Option<String>, String>;
}
