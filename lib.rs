//! Host-level SAC conformance tests. No custom token contract is deployed.
#[cfg(test)]
mod tests {
 use soroban_sdk::{Env,Address,token::{TokenClient,StellarAssetClient},testutils::Address as _};
 fn setup()->(Env,Address,Address,Address){let e=Env::default();e.mock_all_auths();let admin=Address::generate(&e);let sac=e.register_stellar_asset_contract_v2(admin);let a=Address::generate(&e);let b=Address::generate(&e);(e,sac.address(),a,b)}
 #[test]fn transfer_exact_amount(){let(e,id,a,b)=setup();let admin=StellarAssetClient::new(&e,&id);let token=TokenClient::new(&e,&id);admin.mint(&a,&200000000);token.transfer(&a,&b,&12345678);assert_eq!(token.balance(&a),187654322);}
 #[test]fn one_stroop(){let(e,id,a,b)=setup();let admin=StellarAssetClient::new(&e,&id);let token=TokenClient::new(&e,&id);admin.mint(&a,&2);token.transfer(&a,&b,&1);assert_eq!(token.balance(&a),1);assert_eq!(token.balance(&b),1);}
 #[test]#[should_panic]fn insufficient_balance(){let(e,id,a,b)=setup();let token=TokenClient::new(&e,&id);token.transfer(&a,&b,&1);}
 #[test]#[should_panic]fn transfer_requires_authorization(){let(e,id,a,b)=setup();StellarAssetClient::new(&e,&id).mint(&a,&2);e.mock_auths(&[]);TokenClient::new(&e,&id).transfer(&a,&b,&1);}
}
