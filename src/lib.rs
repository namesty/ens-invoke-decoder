pub mod wrap;
pub use wrap::*;

impl ModuleTrait for Module {
    fn get_resolver(args: ArgsGetResolver) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Get resolver address for {domain}")))
    }

    fn get_owner(args: ArgsGetOwner) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Get owner of {domain}")))
    }

    fn check_if_record_exists(args: ArgsCheckIfRecordExists) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Check if {domain} exists")))
    }

    fn get_address(args: ArgsGetAddress) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Get address of {domain} owner")))
    }

    fn get_address_from_domain(args: ArgsGetAddressFromDomain) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Get address of {domain} owner")))
    }

    fn get_content_hash(args: ArgsGetContentHash) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Get {domain}'s content hash")))
    }

    fn get_content_hash_from_domain(args: ArgsGetContentHashFromDomain) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Get {domain}'s content hash")))
    }

    fn get_expiry_times(args: ArgsGetExpiryTimes) -> Result<Option<String>, String> {
        let domain = args.domain;
        Ok(Some(format!("Get expiry time for {domain}")))
    }

    fn get_reverse_resolver(args: ArgsGetReverseResolver) -> Result<Option<String>, String> {
        let address = args.address;
        Ok(Some(format!("Get reverse resolver address for {address}")))
    }

    fn get_name_from_reverse_resolver(args: ArgsGetNameFromReverseResolver) -> Result<Option<String>, String> {
        let address = args.address;
        Ok(Some(format!("Get ENS domain registered to {address}")))
    }

    fn get_name_from_address(args: ArgsGetNameFromAddress) -> Result<Option<String>, String> {
        let address = args.address;
        Ok(Some(format!("Get ENS domain registered to {address}")))
    }

    fn get_text_record(args: ArgsGetTextRecord) -> Result<Option<String>, String> {
        let domain = args.domain;
        let key = args.key;
        Ok(Some(format!("Get value of '{key}' in {domain}")))
    }

    fn set_resolver(args: ArgsSetResolver) -> Result<Option<String>, String> {
        let domain = args.domain;
        let resolver = args.resolver_address;
        Ok(Some(format!("Set {domain}'s resolver to '{resolver}'")))
    }

    fn register_domain(args: ArgsRegisterDomain) -> Result<Option<String>, String> {
        let domain = args.domain;
        let registrar = args.registrar_address;
        let owner = args.owner;
        Ok(Some(format!("Register {domain} with registrar '{registrar}' for address '{owner}'")))
    }

    fn reverse_register_domain(args: ArgsReverseRegisterDomain) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn set_name(args: ArgsSetName) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn set_address(args: ArgsSetAddress) -> Result<Option<String>, String> {
        let domain = args.domain;
        let address = args.address;
        Ok(Some(format!("Set {domain}'s address to {address}")))
    }

    fn set_owner(args: ArgsSetOwner) -> Result<Option<String>, String> {
        let domain = args.domain;
        let owner = args.new_owner;
        Ok(Some(format!("Set {domain}'s owner to '{owner}'")))
    }

    fn set_subdomain_owner(args: ArgsSetSubdomainOwner) -> Result<Option<String>, String> {
        let subdomain = args.subdomain;
        let owner = args.owner;
        Ok(Some(format!("Set {subdomain}'s owner to '{owner}'")))
    }

    fn set_record(args: ArgsSetRecord) -> Result<Option<String>, String> {
        let domain = args.domain;
        let resolver = args.resolver_address;
        let ttl = args.ttl;
        Ok(Some(format!("Register {domain} with resolver '{resolver}' and TTL {ttl}")))
    }

    fn set_subdomain_record(args: ArgsSetSubdomainRecord) -> Result<Option<String>, String> {
        let label = args.label;
        let domain = args.domain;
        let resolver = args.resolver_address;
        let ttl = args.ttl;
        Ok(Some(format!("Register subdomain {label}.{domain} with resolver '{resolver}' and TTL {ttl}")))
    }

    fn register_subdomains_recursively(args: ArgsRegisterSubdomainsRecursively) -> Result<Option<String>, String> {
        let domain = args.domain;
        let resolver = args.resolver_address;
        let ttl = args.ttl;
        Ok(Some(format!("Recursively register {domain} subdomain with resolver '{resolver}' and TTL {ttl}")))
    }

    fn register_domain_and_subdomains_recursively(args: ArgsRegisterDomainAndSubdomainsRecursively) -> Result<Option<String>, String> {
        let domain = args.domain;
        let resolver = args.resolver_address;
        let ttl = args.ttl;
        Ok(Some(format!("Recursively register {domain} domain with resolver '{resolver}' and TTL {ttl}")))
    }

    fn set_content_hash(args: ArgsSetContentHash) -> Result<Option<String>, String> {
        let domain = args.domain;
        let content = args.cid;
        Ok(Some(format!("Set {domain}'s content hash to '{content}'")))
    }

    fn set_address_from_domain(args: ArgsSetAddressFromDomain) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn set_content_hash_from_domain(args: ArgsSetContentHashFromDomain) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn deploy_f_i_f_s_registrar(args: ArgsDeployFIFSRegistrar) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn register_subnode_owner_with_f_i_f_s_registrar(args: ArgsRegisterSubnodeOwnerWithFIFSRegistrar) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn set_text_record(args: ArgsSetTextRecord) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn configure_open_domain(args: ArgsConfigureOpenDomain) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn create_subdomain_in_open_domain(args: ArgsCreateSubdomainInOpenDomain) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }

    fn create_subdomain_in_open_domain_and_set_content_hash(args: ArgsCreateSubdomainInOpenDomainAndSetContentHash) -> Result<Option<String>, String> {
        // TODO
        Ok(None)
    }
}
