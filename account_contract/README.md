
class hash :
starkli class-hash target/dev/vanity_address_aa_Account.contract_class.json
0x04a090fc6d10de4f8d9565e7b2d7f6e8367f79e0b8397877fcbc798f8b996187

update ~/.starkli/wallets/aa_wallet/account.json with the hash class

source ~/.starkli/wallets/aa_wallet/envars.sh


declare via wallet test :
starkli declare --account ~/.starkli/wallets/test/account.json --keystore ~/.starkli/wallets/test/keystore.json target/dev/vanity_address_aa_Account.contract_class.json 


deploy :
starkli -v account deploy ~/.starkli/wallets/aa_wallet/account.json 

will deployed at 0x02e1e2faf2a3983dba5fe98fe21af52b73b1feba1d5063b6b00e752bc6434526

