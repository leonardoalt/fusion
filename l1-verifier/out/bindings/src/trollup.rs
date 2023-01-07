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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"root\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Verifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256[]\",\"name\":\"input\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBlock\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static TROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TROLLUP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061131f806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c80634655038b1461003b578063ebf0c71714610050575b600080fd5b61004e610049366004611185565b61006b565b005b61005960005481565b60405190815260200160405180910390f35b805160081461007957600080fd5b8060008151811061008c5761008c611245565b6020026020010151600054146100a157600080fd5b6100ab81836100da565b156100b557600080fd5b806005815181106100c8576100c8611245565b60200260200101516000819055505050565b60007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000018161010661025f565b90508060800151518551600161011c9190611271565b1461012657600080fd5b604080518082019091526000808252602082018190525b86518110156101d4578387828151811061015957610159611245565b60200260200101511061016b57600080fd5b6101c0826101bb85608001518460016101849190611271565b8151811061019457610194611245565b60200260200101518a85815181106101ae576101ae611245565b602002602001015161093c565b61099a565b9150806101cc81611284565b91505061013d565b506101fd8183608001516000815181106101f0576101f0611245565b602002602001015161099a565b905061024185600001518660200151610215846109f0565b85604001516102278a604001516109f0565b60608801518851610237906109f0565b8960200151610a8f565b6102515760019350505050610259565b600093505050505b92915050565b610267610efb565b6040805180820182527f1e19a8a58ad52243374aeded373b7e89656ea339b9fa8ace98dd5fb221885ea281527f2e66a9a67f1a9060a51da039c91c3402d1f46b71bbf10c7348ac4f13c39067366020808301919091529083528151608080820184527f1ca3e556290187c64a1057061f419a078dc71353f6af1066c03d7e1448bbc1198285019081527f2bbc1b80e59743b489ec811b4ebf30a1ff540c2c37ced63d360b94f92f0a41fb606080850191909152908352845180860186527f07eceb98d2fb10fa7363b45f51aa3d3ef3d511b482790645039a2562e2070f3081527f1c3e076d2aaf914abd6a49b72c4205669d3d1cbe4a4bf97b9ee49ac0fbbdbda9818601528385015285840192909252835180820185527f222c0019521d3e52881431be17cacaf8a7379398dd0833f60a2ac45f1c3fcd428186019081527f1018dbb94cd920bd55af4e2b12a9f740c6b38748a163b5dbd37c5ef6cf74708f828501528152845180860186527f18bf34dc86b549a92f316f7a32070a3ce45a0f38fa45dda1162c4b6498baf28681527f12848d5a670b6102d5bd45d2b8250d50361001ea335ff6a1405a52504c22b8ac818601528185015285850152835190810184527f13b8e16c40a6a299ea42107a97f881f9fa89986dcd5234ecb6919caf756ac1cb8185019081527f25b64e4978690cd7cb531dbab0119148c96f5fc0c984c0cafb290bb75f033a09828401528152835180850185527f1758eaa970929deff5e96e5852d21790c32591dbb13bc63e3df046f0271479a481527f14d0b4222ad1710c6330e4bd8ad8f0d7b8f4cff0a37793d53001800e49f41192818501528184015290840152815160098082526101408201909352919082015b60408051808201909152600080825260208201528152602001906001900390816104e357505060808201908152604080518082019091527f0925dc800d3a577859439a049f8ed0ae7a37dcd36652de478d662c08907a762681527f1f7f76e299220ebf3da17bb415d25e6574e142391972dbd1513cf81341975cb560208201529051805160009061057657610576611245565b602002602001018190525060405180604001604052807f0e67dddfd91adad72376c56cbd98d5cfa4df5217d6115ff26ec741d0154f0bd881526020017f0b97b2ddfdf4c31916d98e384bee3b24bfa0fc59a21ab489153f4dcd1a9a48ca81525081608001516001815181106105ed576105ed611245565b602002602001018190525060405180604001604052807f1416b354665883cbbc5f5541012d1f8dd87ebb4415b3ee431be0804fff290bbf81526020017f1a284dd2eff43e6cb5aaea43dd9bee022ef0c91d90d0803cf5f7e4677e94a271815250816080015160028151811061066457610664611245565b602002602001018190525060405180604001604052807f2526852e7f009b4afa1fe0e1d30334c6e516fac223866b81a830b472164bdfe781526020017f28adf6fbe54ba40afa91555c18477ad3a2f0a460f68d55a15b4e0c264b9c11c281525081608001516003815181106106db576106db611245565b602002602001018190525060405180604001604052807f22feae4a12bfb751638cd76b2373e84884ba4adef575ea14ed50c5954d31d41081526020017f108c3da0ffd7eda1fe7789e41693146beb979bd1644b19bbd517742ca3841348815250816080015160048151811061075257610752611245565b602002602001018190525060405180604001604052807f081477a5c52f41533cf6ca4f778ab922d59ba44b5a5e3fbdbf34ed8dc1a47a8d81526020017f0f2624780bd75b9f6c47f7bee582d02f1f983529b8aa9493ca848e38f2ec844781525081608001516005815181106107c9576107c9611245565b602002602001018190525060405180604001604052807f0603e7413c605d1e9b9352a62f0208e2bbd247d3cf3b3721f72c3a9407d679b981526020017f0ce2d2dca8ae14ac4fd2f3fd89e602cdb45de815cf3ba183a25a47d877d9f6f5815250816080015160068151811061084057610840611245565b602002602001018190525060405180604001604052807f05bbe6b58285021fb843123971f8e2cfdd207b02c0aef5923ffe7ac841ee0cc981526020017f2da1d3c2049546a7b46aaf89a8a7de493470087ce9af8ba37673f5ee8c35eb1b81525081608001516007815181106108b7576108b7611245565b602002602001018190525060405180604001604052807f2c721270df9ba8884d309140f3a4b150a8e53a6c9d09bd8fc7c9aa3c4901aa8d81526020017f0de2cb1684759e693e855711fa1c381ae737e463447c3817df507a02064b470f815250816080015160088151811061092e5761092e611245565b602002602001018190525090565b6040805180820190915260008082526020820152610958610f4c565b835181526020808501519082015260408101839052600060608360808460076107d05a03fa9050808061098757fe5b508061099257600080fd5b505092915050565b60408051808201909152600080825260208201526109b6610f6a565b8351815260208085015181830152835160408301528301516060808301919091526000908360c08460066107d05a03fa9050808061098757fe5b604080518082019091526000808252602082015281517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4790158015610a3757506020830151155b15610a575750506040805180820190915260008082526020820152919050565b604051806040016040528084600001518152602001828560200151610a7c919061129d565b610a8690846112bf565b90529392505050565b60408051600480825260a08201909252600091829190816020015b6040805180820190915260008082526020820152815260200190600190039081610aaa57505060408051600480825260a0820190925291925060009190602082015b610af4610f88565b815260200190600190039081610aec5790505090508a82600081518110610b1d57610b1d611245565b60200260200101819052508882600181518110610b3c57610b3c611245565b60200260200101819052508682600281518110610b5b57610b5b611245565b60200260200101819052508482600381518110610b7a57610b7a611245565b60200260200101819052508981600081518110610b9957610b99611245565b60200260200101819052508781600181518110610bb857610bb8611245565b60200260200101819052508581600281518110610bd757610bd7611245565b60200260200101819052508381600381518110610bf657610bf6611245565b6020026020010181905250610c0b8282610c1a565b9b9a5050505050505050505050565b60008151835114610c2a57600080fd5b82516000610c398260066112d2565b905060008167ffffffffffffffff811115610c5657610c56610fe9565b604051908082528060200260200182016040528015610c7f578160200160208202803683370190505b50905060005b83811015610eba57868181518110610c9f57610c9f611245565b60200260200101516000015182826006610cb991906112d2565b610cc4906000611271565b81518110610cd457610cd4611245565b602002602001018181525050868181518110610cf257610cf2611245565b60200260200101516020015182826006610d0c91906112d2565b610d17906001611271565b81518110610d2757610d27611245565b602002602001018181525050858181518110610d4557610d45611245565b60209081029190910181015151015182610d608360066112d2565b610d6b906002611271565b81518110610d7b57610d7b611245565b602002602001018181525050858181518110610d9957610d99611245565b6020908102919091010151515182610db28360066112d2565b610dbd906003611271565b81518110610dcd57610dcd611245565b602002602001018181525050858181518110610deb57610deb611245565b602002602001015160200151600160028110610e0957610e09611245565b602002015182610e1a8360066112d2565b610e25906004611271565b81518110610e3557610e35611245565b602002602001018181525050858181518110610e5357610e53611245565b602002602001015160200151600060028110610e7157610e71611245565b602002015182610e828360066112d2565b610e8d906005611271565b81518110610e9d57610e9d611245565b602090810291909101015280610eb281611284565b915050610c85565b50610ec3610fad565b6000602082602086026020860160086107d05a03fa90508080610ee257fe5b5080610eed57600080fd5b505115159695505050505050565b6040805160e08101909152600060a0820181815260c0830191909152815260208101610f25610f88565b8152602001610f32610f88565b8152602001610f3f610f88565b8152602001606081525090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280610f9b610fcb565b8152602001610fa8610fcb565b905290565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff8111828210171561102257611022610fe9565b60405290565b6040516060810167ffffffffffffffff8111828210171561102257611022610fe9565b60006040828403121561105d57600080fd5b6040516040810181811067ffffffffffffffff8211171561108057611080610fe9565b604052823581526020928301359281019290925250919050565b600082601f8301126110ab57600080fd5b6110b3610fff565b8060408401858111156110c557600080fd5b845b818110156110df5780358452602093840193016110c7565b509095945050505050565b600082601f8301126110fb57600080fd5b8135602067ffffffffffffffff8083111561111857611118610fe9565b8260051b604051601f19603f8301168101818110848211171561113d5761113d610fe9565b60405293845285810183019383810192508785111561115b57600080fd5b83870191505b8482101561117a57813583529183019190830190611161565b979650505050505050565b60008082840361012081121561119a57600080fd5b610100808212156111aa57600080fd5b6111b2611028565b6111bc878761104b565b81526080603f19840112156111d057600080fd5b6111d8610fff565b92506111e7876040880161109a565b83526111f6876080880161109a565b602084015282602082015261120e8760c0880161104b565b60408201529350840135905067ffffffffffffffff81111561122f57600080fd5b61123b858286016110ea565b9150509250929050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b808201808211156102595761025961125b565b6000600182016112965761129661125b565b5060010190565b6000826112ba57634e487b7160e01b600052601260045260246000fd5b500690565b818103818111156102595761025961125b565b80820281158282048414176102595761025961125b56fea2646970667358221220304194e74a2dc657e1ebd55edfa83b3e22a3ab18ce9e21b28182442ab12c73df64736f6c63430008110033" . parse () . expect ("invalid bytecode")
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
        #[doc = "Calls the contract's `submitBlock` (0x4655038b) function"]
        pub fn submit_block(
            &self,
            proof: Proof,
            input: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 85, 3, 139], (proof, input))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Trollup<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[doc = "Container type for all input parameters for the `submitBlock` function with signature `submitBlock(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])` and selector `[70, 85, 3, 139]`"]
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
        abi = "submitBlock(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[])"
    )]
    pub struct SubmitBlockCall {
        pub proof: Proof,
        pub input: ::std::vec::Vec<ethers::core::types::U256>,
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
