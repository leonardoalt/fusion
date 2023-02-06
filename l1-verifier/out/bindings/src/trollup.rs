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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidInputLength\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidPreRoot\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSNARK\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"root\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Trollup.TxProof[1]\",\"name\":\"l2Block\",\"type\":\"tuple[1]\",\"components\":[{\"internalType\":\"struct Verifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256[]\",\"name\":\"input\",\"type\":\"uint256[]\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBlock\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static TROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TROLLUP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50611a0a806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063798574621461003b578063ebf0c71714610050575b600080fd5b61004e6100493660046117e9565b61006b565b005b61005960005481565b60405190815260200160405180910390f35b805160200151805160009061008257610082611930565b6020026020010151600054146100ab57604051633ac8d0b760e21b815260040160405180910390fd5b60005b60018110156100e6576100d68282600181106100cc576100cc611930565b602002015161012f565b6100df8161195c565b90506100ae565b50806100f3600180611975565b6001811061010357610103611930565b60200201516020015160018151811061011e5761011e611930565b602002602001015160008190555050565b80602001515160141461015557604051637db491eb60e01b815260040160405180910390fd5b61016781602001518260000151610188565b15610185576040516332d7ac2d60e21b815260040160405180910390fd5b50565b60007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001816101b461030d565b9050806080015151855160016101ca9190611988565b146101d457600080fd5b604080518082019091526000808252602082018190525b8651811015610282578387828151811061020757610207611930565b60200260200101511061021957600080fd5b61026e8261026985608001518460016102329190611988565b8151811061024257610242611930565b60200260200101518a858151811061025c5761025c611930565b6020026020010151610f7c565b610fda565b91508061027a8161195c565b9150506101eb565b506102ab81836080015160008151811061029e5761029e611930565b6020026020010151610fda565b90506102ef856000015186602001516102c384611030565b85604001516102d58a60400151611030565b606088015188516102e590611030565b89602001516110cf565b6102ff5760019350505050610307565b600093505050505b92915050565b61031561153b565b6040805180820182527f290f8bb5d79d6d0dbe1c03ca1011fa9b0b7c6ecda13f37cfba658a4907183e8281527f01bdd72ab2502e44350a2c689656a181d6b4296b8d5166e8985cf791214cb5dd6020808301919091529083528151608080820184527f2097d33038a83c026f6368e2a92256591f4c2e1e4f434bc9bfb78a4f63e8d6da8285019081527f288c98376de73cd6fc6244cd2f922dd5d67389b6d98ecc5a8678cec63fa33699606080850191909152908352845180860186527f0cadbda639bd8c91b3da3d95a0ad0d16c5ed6bbefe4239e10df63a44fe9dc7fa81527f0a20619d0ebf2d01922725e4636dc59f6399f23bd92f641ccac6495f4707c59a818601528385015285840192909252835180820185527f1699b936b058af6b03c0875a819c79f7efe5cc124e27e9b1795e0ae99f0eb17b8186019081527f2d8e4ab909054c94d10faf0a9838d101307578438eff38336640ce8292297be1828501528152845180860186527f0ceadc89b0d21f6881e47d23c19a89d4ddf3bfd00d0a1f70cc97d35779b37c8681527f0335b618b04cfeffae49902bfd90b27473f5185b0875facaa446ec96c4c427f6818601528185015285850152835190810184527f20c7bd11eb26046af5aaba82e57671bc5a96381e89577d120ff7e83cdeffa5528185019081527f2b2a2bf9ee96069bb2d09f01ddab62728b65c37e932d5a6f5fd6b488c99785b7828401528152835180850185527f15d299ea897bbe8777e50a221570e48bedc74c76f0378f6a4ec5856cc9f36ec581527f0368a9236c577309e63b0581be044d231a934e37c9ef3ed339e7cd398e0ea776818501528184015290840152815160158082526102c08201909352919082015b604080518082019091526000808252602082015281526020019060019003908161059157505060808201908152604080518082019091527f26428f1cc3497817783af1b2bbf9307742be492cbc3f39ef8c4639844b62eb2081527f2b7ceba4863591b34957f4a286ecb47d4917d4991789835431f0dcb16bac093060208201529051805160009061062457610624611930565b602002602001018190525060405180604001604052807f0b876906484105f0ba685cf120171e4e6f1b8087b6ab097de676f441b073de8681526020017f2b8ab2b876a2ca288e6cc97e5c396d69504ffd4e716d39e140dad48acfe3bf9b815250816080015160018151811061069b5761069b611930565b602002602001018190525060405180604001604052807f0ecb31ec1f0a6f5b085babcc0921e5c4683af0b9dc5e16f1d7761de291d48d6881526020017f28d1202dff883363d90ef693bf1f5672e43de0dcb364a0ca0d6c7c2d5e4e735a815250816080015160028151811061071257610712611930565b602002602001018190525060405180604001604052807f0c162682fa1477599be6334337ddc979c28d8a3a1dd18edbb5731699ce96237281526020017f2c922f58408b2b1d04306262ecd018810272a9abda833e3e9e64e71185bca297815250816080015160038151811061078957610789611930565b602002602001018190525060405180604001604052807f1b97c31415496279a71c1ee0c1c4a5f5a30b9818e95447b60c9b452a1dec5eb781526020017f21ff09cece580082abb3699979c6f40dc05ed3437218e026f43962cc88e8f43a815250816080015160048151811061080057610800611930565b602002602001018190525060405180604001604052807f1fd2d912092d81e9b234417ff59392701a1296764bcdee079f8d144a120ffbd681526020017f13c6f6a981ec443978ca844e396bbed8065aac15a75c2fd02a2f96e47fbf24bb815250816080015160058151811061087757610877611930565b602002602001018190525060405180604001604052807e43e53e8692422e2e44f142ff4aaf54e53db43e55804e397e022ac3c5cecf0381526020017ee79de0cc7688636e59a8fbd0e67b138fbb84df19513e74675ddec279b18fb481525081608001516006815181106108ec576108ec611930565b602002602001018190525060405180604001604052807f1ac610f7bfa3fd301f668401c39e4ed65f0cfb7c7726cb1fd95aadf7e84b434881526020017f2115d4a0c0aa5ebfc44c4a42b549b5262374037f8c9f76333429639f52577a31815250816080015160078151811061096357610963611930565b602002602001018190525060405180604001604052807f2f6aa182bc65cac0c5e5f21f03f474ce44cbb5fad370c4b584063b3fc517c5fe81526020017f1b0c97e9c968beab6a66c9b81188328a286b3858b345186af759a17cde8e7c3f81525081608001516008815181106109da576109da611930565b602002602001018190525060405180604001604052807f0740f4076838085a6183a7cdebd811f24932c6dc041eaf7481fbe23d8166768781526020017f25e52414c179e1c77b62b5681f402e301a1c120faf66bfbc5d97e10a963cdc118152508160800151600981518110610a5157610a51611930565b602002602001018190525060405180604001604052807f0844d8c2856eb58c5058fac08689f57716655e6b36346d6ea137e27b1aafc0c481526020017f2d05eaa47be44f41c3b645882e640ca4453b1b2e58ff03910498086af64488528152508160800151600a81518110610ac857610ac8611930565b602002602001018190525060405180604001604052807f28cd6042b707cc8f32f2a1fb2e4bdbc7ff17cb5468a12ebadb71af1c27a5fbfe81526020017f28c4395b18c54f664dad47398b8913580192ae06fdd01ec0e63fd146ce26b2698152508160800151600b81518110610b3f57610b3f611930565b602002602001018190525060405180604001604052807f186d5236f22221b2656360e6ff5fb420f54ffa539b4df86161bc11919924f96d81526020017f2dacbcdabd151b9076c34b5d63dddce80c36a34dc013f6bd17ec2f29987782838152508160800151600c81518110610bb657610bb6611930565b602002602001018190525060405180604001604052807f236f5fa8d339fd32b71b44b88b596c1013e4f527700b566429c81c625392ed7481526020017f0413e46bab46a6e337c8b91e4a516705dd48bba14eb6cdc91573684e57b55cdc8152508160800151600d81518110610c2d57610c2d611930565b602002602001018190525060405180604001604052807f162e6a98a3aef486b657224a2734a9dce051ca497571b306baa916752a07496181526020017f1f184e0c901decb679cf320bcdc27be65c3897c0d0f34c81828a28a81007c44a8152508160800151600e81518110610ca457610ca4611930565b602002602001018190525060405180604001604052807f267e86e0864ab82d3d4f1b942ba1b4463dca76095f63b33f88d18ceb2a2f752e81526020017f21e125ba61a642342157a41c4aa077a9ce60a64a584bddf25bce720aac72e3018152508160800151600f81518110610d1b57610d1b611930565b602002602001018190525060405180604001604052807f2145f45f569d56390aa4579420c41727a29cbc8bdefd9f0fc9baf07472f6f9de81526020017f254546e2d86527ce3a10f8df73f2e44b61085b7831bd776f68ef234c2649300a8152508160800151601081518110610d9257610d92611930565b602002602001018190525060405180604001604052807f2671263de6756f7c6d714a75480b9d060b4efd3fccc5d2229830396410f384a181526020017f24f452bc36bbadc749643279a25c9dde72a5d95a1efc2e677a1738549ca284988152508160800151601181518110610e0957610e09611930565b602002602001018190525060405180604001604052807f2dfc8e1ed39f5d65e80298d10a65d154d5f0621368eddd81a0cdeeb419d084fc81526020017f25117c0d88687d8057e88d89f2c53aa2185795daf5025e6737dd6d81e687ede78152508160800151601281518110610e8057610e80611930565b602002602001018190525060405180604001604052807f181593ed30e38be05aac78d84dad1bb15ac222c234b505fa524bf605fcfa43ae81526020017f0ce9a2e123a9101f22b063678459d44e58574889348caaf8815fa1d146c531e68152508160800151601381518110610ef757610ef7611930565b602002602001018190525060405180604001604052807f11f37bf7bb920da5c220da0c865052d021e4bfc35ae5c46ebf26aea6952fbe4381526020017f2df9ac6050451c357616fb3964fedbc796dab1dc40fb565f8de5976dea5fa93d8152508160800151601481518110610f6e57610f6e611930565b602002602001018190525090565b6040805180820190915260008082526020820152610f9861158c565b835181526020808501519082015260408101839052600060608360808460076107d05a03fa90508080610fc757fe5b5080610fd257600080fd5b505092915050565b6040805180820190915260008082526020820152610ff66115aa565b8351815260208085015181830152835160408301528301516060808301919091526000908360c08460066107d05a03fa90508080610fc757fe5b604080518082019091526000808252602082015281517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd479015801561107757506020830151155b156110975750506040805180820190915260008082526020820152919050565b6040518060400160405280846000015181526020018285602001516110bc919061199b565b6110c69084611975565b90529392505050565b60408051600480825260a08201909252600091829190816020015b60408051808201909152600080825260208201528152602001906001900390816110ea57505060408051600480825260a0820190925291925060009190602082015b6111346115c8565b81526020019060019003908161112c5790505090508a8260008151811061115d5761115d611930565b6020026020010181905250888260018151811061117c5761117c611930565b6020026020010181905250868260028151811061119b5761119b611930565b602002602001018190525084826003815181106111ba576111ba611930565b602002602001018190525089816000815181106111d9576111d9611930565b602002602001018190525087816001815181106111f8576111f8611930565b6020026020010181905250858160028151811061121757611217611930565b6020026020010181905250838160038151811061123657611236611930565b602002602001018190525061124b828261125a565b9b9a5050505050505050505050565b6000815183511461126a57600080fd5b825160006112798260066119bd565b905060008167ffffffffffffffff81111561129657611296611629565b6040519080825280602002602001820160405280156112bf578160200160208202803683370190505b50905060005b838110156114fa578681815181106112df576112df611930565b602002602001015160000151828260066112f991906119bd565b611304906000611988565b8151811061131457611314611930565b60200260200101818152505086818151811061133257611332611930565b6020026020010151602001518282600661134c91906119bd565b611357906001611988565b8151811061136757611367611930565b60200260200101818152505085818151811061138557611385611930565b602090810291909101810151510151826113a08360066119bd565b6113ab906002611988565b815181106113bb576113bb611930565b6020026020010181815250508581815181106113d9576113d9611930565b60209081029190910101515151826113f28360066119bd565b6113fd906003611988565b8151811061140d5761140d611930565b60200260200101818152505085818151811061142b5761142b611930565b60200260200101516020015160016002811061144957611449611930565b60200201518261145a8360066119bd565b611465906004611988565b8151811061147557611475611930565b60200260200101818152505085818151811061149357611493611930565b6020026020010151602001516000600281106114b1576114b1611930565b6020020151826114c28360066119bd565b6114cd906005611988565b815181106114dd576114dd611930565b6020908102919091010152806114f28161195c565b9150506112c5565b506115036115ed565b6000602082602086026020860160086107d05a03fa9050808061152257fe5b508061152d57600080fd5b505115159695505050505050565b6040805160e08101909152600060a0820181815260c08301919091528152602081016115656115c8565b81526020016115726115c8565b815260200161157f6115c8565b8152602001606081525090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806115db61160b565b81526020016115e861160b565b905290565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff8111828210171561166257611662611629565b60405290565b6040516060810167ffffffffffffffff8111828210171561166257611662611629565b6040516020810167ffffffffffffffff8111828210171561166257611662611629565b6000604082840312156116c057600080fd5b6116c861163f565b9050813581526020820135602082015292915050565b600082601f8301126116ef57600080fd5b6040516040810181811067ffffffffffffffff8211171561171257611712611629565b806040525080604084018581111561172957600080fd5b845b8181101561174357803583526020928301920161172b565b509195945050505050565b600082601f83011261175f57600080fd5b8135602067ffffffffffffffff8083111561177c5761177c611629565b8260051b604051601f19603f830116810181811084821117156117a1576117a1611629565b6040529384528581018301938381019250878511156117bf57600080fd5b83870191505b848210156117de578135835291830191908301906117c5565b979650505050505050565b600060208083850312156117fc57600080fd5b823567ffffffffffffffff8082111561181457600080fd5b818501915085601f83011261182857600080fd5b61183061168b565b808484018881111561184157600080fd5b845b818110156119225780358581111561185a57600080fd5b8601808b0361012081121561186e57600080fd5b61187661163f565b6101008083121561188657600080fd5b61188e611668565b6118988f866116ae565b81526080603f19850112156118ad5760008081fd5b6118b561163f565b93506118c48f604087016116de565b84526118d38f608087016116de565b8c850152838c8201526118e98f60c087016116ae565b604082015282528301359150878211156119035760008081fd5b61190f8d83850161174e565b818b015286525050928601928601611843565b509098975050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60006001820161196e5761196e611946565b5060010190565b8181038181111561030757610307611946565b8082018082111561030757610307611946565b6000826119b857634e487b7160e01b600052601260045260246000fd5b500690565b80820281158282048414176103075761030761194656fea2646970667358221220708c3beed0a471b9ebef2f76df63678a68b14ee026b98a46e67cd1b954b4dc6d64736f6c63430008110033" . parse () . expect ("invalid bytecode")
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TrollupErrors {
        InvalidInputLength(InvalidInputLength),
        InvalidPreRoot(InvalidPreRoot),
        InvalidSNARK(InvalidSNARK),
    }
    impl ethers::core::abi::AbiDecode for TrollupErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
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
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TrollupErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                TrollupErrors::InvalidInputLength(element) => element.encode(),
                TrollupErrors::InvalidPreRoot(element) => element.encode(),
                TrollupErrors::InvalidSNARK(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TrollupErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TrollupErrors::InvalidInputLength(element) => element.fmt(f),
                TrollupErrors::InvalidPreRoot(element) => element.fmt(f),
                TrollupErrors::InvalidSNARK(element) => element.fmt(f),
            }
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
        Root(RootCall),
        SubmitBlock(SubmitBlockCall),
    }
    impl ethers::core::abi::AbiDecode for TrollupCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
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
                TrollupCalls::Root(element) => element.encode(),
                TrollupCalls::SubmitBlock(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TrollupCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TrollupCalls::Root(element) => element.fmt(f),
                TrollupCalls::SubmitBlock(element) => element.fmt(f),
            }
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
