pub use trollup::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod trollup {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Trollup was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"DepositAmountNotAvailable\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DepositAmountTooLow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidInputLength\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidPreRoot\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSNARK\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidTransactionType\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"l2Recipient\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"deposits\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"root\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Trollup.TxProof[1]\",\"name\":\"l2Block\",\"type\":\"tuple[1]\",\"components\":[{\"internalType\":\"struct Verifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256[]\",\"name\":\"input\",\"type\":\"uint256[]\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBlock\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static TROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TROLLUP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50611ad6806100206000396000f3fe60806040526004361061003f5760003560e01c80637985746214610044578063b02c43d014610066578063b6b55f25146100a5578063ebf0c717146100b8575b600080fd5b34801561005057600080fd5b5061006461005f36600461189c565b6100ce565b005b34801561007257600080fd5b506100936100813660046119e3565b60016020526000908152604090205481565b60405190815260200160405180910390f35b6100646100b33660046119e3565b610192565b3480156100c457600080fd5b5061009360005481565b80516020015180516000906100e5576100e56119fc565b60200260200101516000541461010e57604051633ac8d0b760e21b815260040160405180910390fd5b60005b60018110156101495761013982826001811061012f5761012f6119fc565b60200201516101d9565b61014281611a28565b9050610111565b5080610156600180611a41565b60018110610166576101666119fc565b602002015160200151600181518110610181576101816119fc565b602002602001015160008190555050565b346000036101b3576040516355fcd02760e01b815260040160405180910390fd5b600081815260016020526040812080543492906101d1908490611a54565b909155505050565b8060200151516012146101ff57604051637db491eb60e01b815260040160405180910390fd5b6102088161023b565b61021a81602001518260000151610328565b15610238576040516332d7ac2d60e21b815260040160405180910390fd5b50565b60008160200151600281518110610254576102546119fc565b60200260200101519050806000031561032457806001036103035760008260200151600881518110610288576102886119fc565b6020026020010151905060008360200151600f815181106102ab576102ab6119fc565b6020026020010151905081600160008381526020019081526020016000205410156102e95760405163113fd29f60e21b815260040160405180910390fd5b600090815260016020526040902080549190910390555050565b6002811461032457604051637513b90360e01b815260040160405180910390fd5b5050565b60007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001816103546104ad565b90508060800151518551600161036a9190611a54565b1461037457600080fd5b604080518082019091526000808252602082018190525b865181101561042257838782815181106103a7576103a76119fc565b6020026020010151106103b957600080fd5b61040e8261040985608001518460016103d29190611a54565b815181106103e2576103e26119fc565b60200260200101518a85815181106103fc576103fc6119fc565b602002602001015161102f565b61108d565b91508061041a81611a28565b91505061038b565b5061044b81836080015160008151811061043e5761043e6119fc565b602002602001015161108d565b905061048f85600001518660200151610463846110e3565b85604001516104758a604001516110e3565b60608801518851610485906110e3565b8960200151611182565b61049f57600193505050506104a7565b600093505050505b92915050565b6104b56115ee565b6040805180820182527f156f4298431641d9c52f53242c20b411123e08e6b8678f092e901c2ee2747d5281527f0403de71aef0e283a4a2128fa6519a656f7fd604d2153f116d65715a2ec1b42f6020808301919091529083528151608080820184527f228690f5dbdc1f9a1389e38a02c3054d0976d01e87f95e1cda3ddd4b53a763988285019081527f1162dcaaedd7b1fa6a7822dce599cc62e5cc4cd6e43aa7988081ba49794df20e606080850191909152908352845180860186527f154f08b947550c80539915fe7cf42bfbde1cbf056ac7e27314fc2ae89e378cd581527f28edd304a2ee9688a4429dee675256cfa725f1915f39dcf1e1043314f1a89b4c818601528385015285840192909252835180820185527f16d7fbb83e6cbc1bdce7f60e046c58ffd678e783a56c4dfeef4014791020e06c8186019081527f0b495e18f6d04bb15b9845798ad1cb578733e4d8cb892f1d8461a1e65f44c763828501528152845180860186527f2033883ae4dc5f7a600d4a3641f3c756720dcfe36d04383a9517f41602e696e781527f1d4cc5c6ba0ec0fa503da4660d64b780c0445d96cf4ffb0ffd2f98f1592094a9818601528185015285850152835190810184527f0b16fc9838be6ecac15f599d66f2b10fe3ec1a80365e721f51773421959120998185019081527f1f870ede5d50e264a09b41c8c40df0548732c62e023c0838a101b064183e7be1828401528152835180850185527f02d64f5084c92ac60c25079c09d8f17687ec562d83565c407e44621f593058ad81527f1c6296917688c7602626e243afe23a91843cfb65e3d3c220a9c3ffaf2f6d74e8818501528184015290840152815160138082526102808201909352919082015b604080518082019091526000808252602082015281526020019060019003908161073157505060808201908152604080518082019091527f0a6b8b07cdb84a2fd822cfa9ae18d7c2a166658e61a917870f52f319ea34a92d81527f0b686a8fe0587d599e80963ffe20c78117e62cc19d1a7f9ffb9dcd829fc6f3a16020820152905180516000906107c4576107c46119fc565b602002602001018190525060405180604001604052807f11835b5d167aea2c5ffc1278e11b9e0b298d65c502db7e3a42416792e6eb6baf81526020017f2c125b5b400d13a1ae962c62dfeab32ee4da5f4508a47c77943ee7a7180a72b9815250816080015160018151811061083b5761083b6119fc565b602002602001018190525060405180604001604052807f2d964b9b791d6912cc6c854a681e13ccbc07d115d40844ff2554032aabea8bea81526020017f0babc469cb59cb3baf39b74ebcc3b5cc7cbad7b81ecf7dd985a0b4d0b09dbef281525081608001516002815181106108b2576108b26119fc565b602002602001018190525060405180604001604052807f0174991f1df5a0dfbcd23ba615968dd0d059c569d005e03a5bc83d59c47d455e81526020017f169c91eacfaf94ee47a3a3e248d74483680731dba738e29b84dec3421978a1c28152508160800151600381518110610929576109296119fc565b602002602001018190525060405180604001604052807f1be49a20d32e3ba1020738d42ffa86b583b16c59c6556f935039e7720773796681526020017f19a28033d8aae78567a00392f3a0122365766582e0f8e166922bcd7998ca7d9881525081608001516004815181106109a0576109a06119fc565b602002602001018190525060405180604001604052807f0fe6fc03fd19cc1b241cfcb742debb9b3fb8bfba302085aba414d5881ad2348081526020017f10eedf52938864957c333b81497e0977ae36c9cbaa0c988b2060675ced1655dc8152508160800151600581518110610a1757610a176119fc565b602002602001018190525060405180604001604052807ec4c78c0edcdd4f7bdbf5370e5cf0ab59a093af259b338bba9c6edb131d432c81526020017f2d665f139808ee420ef2434d67067bcc88a4ff235edf9afdf405c809ff28735f8152508160800151600681518110610a8d57610a8d6119fc565b602002602001018190525060405180604001604052807f2d16301ea2fc1b2468ba07b57e3a8fa74de7d02a35c0ce8b72328af1db7dfe4581526020017f1cb877d36020f03e00779f28ba089682703143101e71094f5827ffbdec2eadf08152508160800151600781518110610b0457610b046119fc565b602002602001018190525060405180604001604052807f26c88202d23245ec9cb387a4e28291fec0c3f786e0db9365c13f9daab20643f681526020017f305b5a532f40afc0ca09b34f5c10b7d4c00193a68a48409a49ed1673ffba0ae68152508160800151600881518110610b7b57610b7b6119fc565b602002602001018190525060405180604001604052807f2db36a3b796555945beb07be239e6386b38001f71f5f9d26fe4398937886e74181526020017f0a63a7c2e66acaeb97d5be7a6faa4873451ba6e8a35a97755b7e7d96e9b41bd28152508160800151600981518110610bf257610bf26119fc565b602002602001018190525060405180604001604052807f0922e322e208e48c635ae0a6e638a69d05add5fcfea3f6c2129757c47f4ebf1581526020017f0c3970515345b10b468f1fe3b6733da7b6cb726f497ac4c7a881051ea3d310908152508160800151600a81518110610c6957610c696119fc565b602002602001018190525060405180604001604052807f1a5e374bdaa40089a7ff80396850be23d280296472adaf53a0e6402ee4b9759881526020017f24016c75e6f6edf206f8fff8bcbbf0de517d93cd3ab1c0c13ce6fef089ed2b298152508160800151600b81518110610ce057610ce06119fc565b602002602001018190525060405180604001604052807f2e66295003ef5d6d962753a45c6b7bef3eae89e4eb7fba8024127d0cfb7b88ba81526020017f130813dac324f864b4db90cf64520ff26ebbf95224a7446888c0d122abcb4a878152508160800151600c81518110610d5757610d576119fc565b602002602001018190525060405180604001604052807f22dd64d9aa3fee8c9bca3d1fae9bae94e972d4d917ce881aed4f1b1501f5920381526020017f1d61d5a654e0f77aec9a81742e06e2ccec99fcd7ed05ba42c7ab2f7bccc7d1208152508160800151600d81518110610dce57610dce6119fc565b602002602001018190525060405180604001604052807f1fc614b9f0f4e81c5f59698a5fcd76c01a4f68b332f4abcf9e02aed1f3bc846281526020017f0d57b19e9b3ab0ffb165d0feee04b7fbc342ceb82993660a211d976cc88bbf5e8152508160800151600e81518110610e4557610e456119fc565b602002602001018190525060405180604001604052807f25baa1c8840bc2235b217a53d166639800f6a44150c7a9be27a2ca7ca8bfe14181526020017f2c7fe519bff0a1be8c702889e1029e80da177dfb88420cc2ae7e128009c3425b8152508160800151600f81518110610ebc57610ebc6119fc565b602002602001018190525060405180604001604052807f1701ae96d1e30a5586a39ab124ed37012a208f55b807cfd1c3e29725fb213fb781526020017f214c349ef6e4961f0eea62c20706c32295ff8a921334be8b99422f63011c75c48152508160800151601081518110610f3357610f336119fc565b602002602001018190525060405180604001604052807f2272b2c144ce5d3dda8fedcf172eb9a347592efc5b26c638b7e52db3db854b9081526020017f0934ebc85ae366e003d4e331cb2f4553d739ab8e55014f982d650a37327a6a778152508160800151601181518110610faa57610faa6119fc565b602002602001018190525060405180604001604052807f26bbd937eef4975b5f873ddfc0b11e857a5a3e6dffb5b4550dc9b98f398c4a4481526020017f0c0fa5ec7892a758cc8f34dc57e2a5e0ba3c98f5168c801feeda09e60eed54578152508160800151601281518110611021576110216119fc565b602002602001018190525090565b604080518082019091526000808252602082015261104b61163f565b835181526020808501519082015260408101839052600060608360808460076107d05a03fa9050808061107a57fe5b508061108557600080fd5b505092915050565b60408051808201909152600080825260208201526110a961165d565b8351815260208085015181830152835160408301528301516060808301919091526000908360c08460066107d05a03fa9050808061107a57fe5b604080518082019091526000808252602082015281517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd479015801561112a57506020830151155b1561114a5750506040805180820190915260008082526020820152919050565b60405180604001604052808460000151815260200182856020015161116f9190611a67565b6111799084611a41565b90529392505050565b60408051600480825260a08201909252600091829190816020015b604080518082019091526000808252602082015281526020019060019003908161119d57505060408051600480825260a0820190925291925060009190602082015b6111e761167b565b8152602001906001900390816111df5790505090508a82600081518110611210576112106119fc565b6020026020010181905250888260018151811061122f5761122f6119fc565b6020026020010181905250868260028151811061124e5761124e6119fc565b6020026020010181905250848260038151811061126d5761126d6119fc565b6020026020010181905250898160008151811061128c5761128c6119fc565b602002602001018190525087816001815181106112ab576112ab6119fc565b602002602001018190525085816002815181106112ca576112ca6119fc565b602002602001018190525083816003815181106112e9576112e96119fc565b60200260200101819052506112fe828261130d565b9b9a5050505050505050505050565b6000815183511461131d57600080fd5b8251600061132c826006611a89565b905060008167ffffffffffffffff811115611349576113496116dc565b604051908082528060200260200182016040528015611372578160200160208202803683370190505b50905060005b838110156115ad57868181518110611392576113926119fc565b602002602001015160000151828260066113ac9190611a89565b6113b7906000611a54565b815181106113c7576113c76119fc565b6020026020010181815250508681815181106113e5576113e56119fc565b602002602001015160200151828260066113ff9190611a89565b61140a906001611a54565b8151811061141a5761141a6119fc565b602002602001018181525050858181518110611438576114386119fc565b60209081029190910181015151015182611453836006611a89565b61145e906002611a54565b8151811061146e5761146e6119fc565b60200260200101818152505085818151811061148c5761148c6119fc565b60209081029190910101515151826114a5836006611a89565b6114b0906003611a54565b815181106114c0576114c06119fc565b6020026020010181815250508581815181106114de576114de6119fc565b6020026020010151602001516001600281106114fc576114fc6119fc565b60200201518261150d836006611a89565b611518906004611a54565b81518110611528576115286119fc565b602002602001018181525050858181518110611546576115466119fc565b602002602001015160200151600060028110611564576115646119fc565b602002015182611575836006611a89565b611580906005611a54565b81518110611590576115906119fc565b6020908102919091010152806115a581611a28565b915050611378565b506115b66116a0565b6000602082602086026020860160086107d05a03fa905080806115d557fe5b50806115e057600080fd5b505115159695505050505050565b6040805160e08101909152600060a0820181815260c083019190915281526020810161161861167b565b815260200161162561167b565b815260200161163261167b565b8152602001606081525090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b604051806040016040528061168e6116be565b815260200161169b6116be565b905290565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715611715576117156116dc565b60405290565b6040516060810167ffffffffffffffff81118282101715611715576117156116dc565b6040516020810167ffffffffffffffff81118282101715611715576117156116dc565b60006040828403121561177357600080fd5b61177b6116f2565b9050813581526020820135602082015292915050565b600082601f8301126117a257600080fd5b6040516040810181811067ffffffffffffffff821117156117c5576117c56116dc565b80604052508060408401858111156117dc57600080fd5b845b818110156117f65780358352602092830192016117de565b509195945050505050565b600082601f83011261181257600080fd5b8135602067ffffffffffffffff8083111561182f5761182f6116dc565b8260051b604051601f19603f83011681018181108482111715611854576118546116dc565b60405293845285810183019383810192508785111561187257600080fd5b83870191505b8482101561189157813583529183019190830190611878565b979650505050505050565b600060208083850312156118af57600080fd5b823567ffffffffffffffff808211156118c757600080fd5b818501915085601f8301126118db57600080fd5b6118e361173e565b80848401888111156118f457600080fd5b845b818110156119d55780358581111561190d57600080fd5b8601808b0361012081121561192157600080fd5b6119296116f2565b6101008083121561193957600080fd5b61194161171b565b61194b8f86611761565b81526080603f19850112156119605760008081fd5b6119686116f2565b93506119778f60408701611791565b84526119868f60808701611791565b8c850152838c82015261199c8f60c08701611761565b604082015282528301359150878211156119b65760008081fd5b6119c28d838501611801565b818b0152865250509286019286016118f6565b509098975050505050505050565b6000602082840312156119f557600080fd5b5035919050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060018201611a3a57611a3a611a12565b5060010190565b818103818111156104a7576104a7611a12565b808201808211156104a7576104a7611a12565b600082611a8457634e487b7160e01b600052601260045260246000fd5b500690565b80820281158282048414176104a7576104a7611a1256fea2646970667358221220e6c9aa52489dced404855fcfa888b79a93f10768900c91d98b7b8eed31a4c6ab64736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct Trollup<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Trollup<M> {
        fn clone(&self) -> Self {
            Trollup(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Trollup<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Trollup<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Trollup))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Trollup<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), TROLLUP_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                TROLLUP_ABI.clone(),
                TROLLUP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `deposit` (0xb6b55f25) function"]
        pub fn deposit(
            &self,
            l_2_recipient: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 181, 95, 37], l_2_recipient)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposits` (0xb02c43d0) function"]
        pub fn deposits(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([176, 44, 67, 208], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `root` (0xebf0c717) function"]
        pub fn root(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([235, 240, 199, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitBlock` (0x79857462) function"]
        pub fn submit_block(
            &self,
            l_2_block: [TxProof; 1usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 133, 116, 98], l_2_block)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Trollup<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `DepositAmountNotAvailable` with signature `DepositAmountNotAvailable()` and selector `[68, 255, 74, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "DepositAmountNotAvailable",
        abi = "DepositAmountNotAvailable()"
    )]
    pub struct DepositAmountNotAvailable;
    #[doc = "Custom Error type `DepositAmountTooLow` with signature `DepositAmountTooLow()` and selector `[85, 252, 208, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "DepositAmountTooLow", abi = "DepositAmountTooLow()")]
    pub struct DepositAmountTooLow;
    #[doc = "Custom Error type `InvalidInputLength` with signature `InvalidInputLength()` and selector `[125, 180, 145, 235]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidInputLength", abi = "InvalidInputLength()")]
    pub struct InvalidInputLength;
    #[doc = "Custom Error type `InvalidPreRoot` with signature `InvalidPreRoot()` and selector `[235, 35, 66, 220]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidPreRoot", abi = "InvalidPreRoot()")]
    pub struct InvalidPreRoot;
    #[doc = "Custom Error type `InvalidSNARK` with signature `InvalidSNARK()` and selector `[203, 94, 176, 180]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidSNARK", abi = "InvalidSNARK()")]
    pub struct InvalidSNARK;
    #[doc = "Custom Error type `InvalidTransactionType` with signature `InvalidTransactionType()` and selector `[117, 19, 185, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidTransactionType", abi = "InvalidTransactionType()")]
    pub struct InvalidTransactionType;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TrollupErrors {
        DepositAmountNotAvailable(DepositAmountNotAvailable),
        DepositAmountTooLow(DepositAmountTooLow),
        InvalidInputLength(InvalidInputLength),
        InvalidPreRoot(InvalidPreRoot),
        InvalidSNARK(InvalidSNARK),
        InvalidTransactionType(InvalidTransactionType),
    }
    impl ethers::core::abi::AbiDecode for TrollupErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DepositAmountNotAvailable as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupErrors::DepositAmountNotAvailable(decoded));
            }
            if let Ok(decoded) =
                <DepositAmountTooLow as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupErrors::DepositAmountTooLow(decoded));
            }
            if let Ok(decoded) =
                <InvalidInputLength as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupErrors::InvalidInputLength(decoded));
            }
            if let Ok(decoded) =
                <InvalidPreRoot as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupErrors::InvalidPreRoot(decoded));
            }
            if let Ok(decoded) =
                <InvalidSNARK as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupErrors::InvalidSNARK(decoded));
            }
            if let Ok(decoded) =
                <InvalidTransactionType as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupErrors::InvalidTransactionType(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TrollupErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                TrollupErrors::DepositAmountNotAvailable(element) => element.encode(),
                TrollupErrors::DepositAmountTooLow(element) => element.encode(),
                TrollupErrors::InvalidInputLength(element) => element.encode(),
                TrollupErrors::InvalidPreRoot(element) => element.encode(),
                TrollupErrors::InvalidSNARK(element) => element.encode(),
                TrollupErrors::InvalidTransactionType(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TrollupErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TrollupErrors::DepositAmountNotAvailable(element) => element.fmt(f),
                TrollupErrors::DepositAmountTooLow(element) => element.fmt(f),
                TrollupErrors::InvalidInputLength(element) => element.fmt(f),
                TrollupErrors::InvalidPreRoot(element) => element.fmt(f),
                TrollupErrors::InvalidSNARK(element) => element.fmt(f),
                TrollupErrors::InvalidTransactionType(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DepositAmountNotAvailable> for TrollupErrors {
        fn from(var: DepositAmountNotAvailable) -> Self {
            TrollupErrors::DepositAmountNotAvailable(var)
        }
    }
    impl ::std::convert::From<DepositAmountTooLow> for TrollupErrors {
        fn from(var: DepositAmountTooLow) -> Self {
            TrollupErrors::DepositAmountTooLow(var)
        }
    }
    impl ::std::convert::From<InvalidInputLength> for TrollupErrors {
        fn from(var: InvalidInputLength) -> Self {
            TrollupErrors::InvalidInputLength(var)
        }
    }
    impl ::std::convert::From<InvalidPreRoot> for TrollupErrors {
        fn from(var: InvalidPreRoot) -> Self {
            TrollupErrors::InvalidPreRoot(var)
        }
    }
    impl ::std::convert::From<InvalidSNARK> for TrollupErrors {
        fn from(var: InvalidSNARK) -> Self {
            TrollupErrors::InvalidSNARK(var)
        }
    }
    impl ::std::convert::From<InvalidTransactionType> for TrollupErrors {
        fn from(var: InvalidTransactionType) -> Self {
            TrollupErrors::InvalidTransactionType(var)
        }
    }
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit(uint256)` and selector `[182, 181, 95, 37]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256)")]
    pub struct DepositCall {
        pub l_2_recipient: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deposits` function with signature `deposits(uint256)` and selector `[176, 44, 67, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deposits", abi = "deposits(uint256)")]
    pub struct DepositsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `root` function with signature `root()` and selector `[235, 240, 199, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "root", abi = "root()")]
    pub struct RootCall;
    #[doc = "Container type for all input parameters for the `submitBlock` function with signature `submitBlock((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])[1])` and selector `[121, 133, 116, 98]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "submitBlock",
        abi = "submitBlock((((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])[1])"
    )]
    pub struct SubmitBlockCall {
        pub l_2_block: [TxProof; 1usize],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TrollupCalls {
        Deposit(DepositCall),
        Deposits(DepositsCall),
        Root(RootCall),
        SubmitBlock(SubmitBlockCall),
    }
    impl ethers::core::abi::AbiDecode for TrollupCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupCalls::Deposits(decoded));
            }
            if let Ok(decoded) = <RootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(TrollupCalls::Root(decoded));
            }
            if let Ok(decoded) =
                <SubmitBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupCalls::SubmitBlock(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TrollupCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TrollupCalls::Deposit(element) => element.encode(),
                TrollupCalls::Deposits(element) => element.encode(),
                TrollupCalls::Root(element) => element.encode(),
                TrollupCalls::SubmitBlock(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TrollupCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TrollupCalls::Deposit(element) => element.fmt(f),
                TrollupCalls::Deposits(element) => element.fmt(f),
                TrollupCalls::Root(element) => element.fmt(f),
                TrollupCalls::SubmitBlock(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DepositCall> for TrollupCalls {
        fn from(var: DepositCall) -> Self {
            TrollupCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositsCall> for TrollupCalls {
        fn from(var: DepositsCall) -> Self {
            TrollupCalls::Deposits(var)
        }
    }
    impl ::std::convert::From<RootCall> for TrollupCalls {
        fn from(var: RootCall) -> Self {
            TrollupCalls::Root(var)
        }
    }
    impl ::std::convert::From<SubmitBlockCall> for TrollupCalls {
        fn from(var: SubmitBlockCall) -> Self {
            TrollupCalls::SubmitBlock(var)
        }
    }
    #[doc = "Container type for all return fields from the `deposits` function with signature `deposits(uint256)` and selector `[176, 44, 67, 208]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DepositsReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `root` function with signature `root()` and selector `[235, 240, 199, 23]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RootReturn(pub ethers::core::types::U256);
    #[doc = "`G1Point(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct G1Point {
        pub x: ethers::core::types::U256,
        pub y: ethers::core::types::U256,
    }
    #[doc = "`G2Point(uint256[2],uint256[2])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct G2Point {
        pub x: [ethers::core::types::U256; 2],
        pub y: [ethers::core::types::U256; 2],
    }
    #[doc = "`TxProof(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct TxProof {
        pub proof: Proof,
        pub input: Vec<ethers::core::types::U256>,
    }
    #[doc = "`Proof((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Proof {
        pub a: G1Point,
        pub b: G2Point,
        pub c: G1Point,
    }
}
