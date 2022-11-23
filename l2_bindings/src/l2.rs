pub use l2::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod l2 {
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
    #[doc = "L2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSignature\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidStateRoot\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"currentState\",\"outputs\":[{\"internalType\":\"uint256[2]\",\"name\":\"\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"root\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Tx[]\",\"name\":\"_tx\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"_newRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBlock\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amt\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static L2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static L2_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b5061001961001e565b6100b5565b600060208181527f7028a0ec50eccd63e0e5375e137bd936cb02c6de8c1e141753e6eeb1c22d74fe5473419978a8729ed2c3b1048b5bba49f8599ed8f7c19092527fd7f372c1652071b9369261dfb776c225fb305102b25c5645f4c47914557535b454604051610098939201918252602082015260400190565b60408051601f198184030181529190528051602090910120600155565b610acd806100c46000396000f3fe60806040526004361061004a5760003560e01c80630c3f6acf1461004f5780632e1a7d4d1461007a578063d0e30db01461009c578063ebf0c717146100a4578063f74aa465146100c8575b600080fd5b34801561005b57600080fd5b506100646100e8565b60405161007191906107bb565b60405180910390f35b34801561008657600080fd5b5061009a6100953660046107ec565b610167565b005b61009a610210565b3480156100b057600080fd5b506100ba60015481565b604051908152602001610071565b3480156100d457600080fd5b5061009a6100e3366004610891565b6102c8565b6100f0610757565b50604080518082019091527f7028a0ec50eccd63e0e5375e137bd936cb02c6de8c1e141753e6eeb1c22d74fe54815273419978a8729ed2c3b1048b5bba49f8599ed8f7c1600090815260209081527fd7f372c1652071b9369261dfb776c225fb305102b25c5645f4c47914557535b4549082015290565b6040805160608101825260018082523360208301529181018390526002805480840182556000829052825191027f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace01805492939092839160ff199091169083818111156101d6576101d6610a16565b0217905550602082015181546001600160a01b0390911661010002610100600160a81b031990911617815560409091015160019091015550565b6000341161021d57600080fd5b6040805160608101825260008082523360208301523492820192909252600280546001808201835593829052825191027f405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace01805492939092839160ff1990911690838181111561028f5761028f610a16565b0217905550602082015181546001600160a01b0390911661010002610100600160a81b0319909116178155604090910151600190910155565b60005b8251811015610345576102f68382815181106102e9576102e9610a2c565b602002602001015161037c565b61031357604051638baa579f60e01b815260040160405180910390fd5b61033583828151811061032857610328610a2c565b60200260200101516103bf565b61033e81610a58565b90506102cb565b5061034e61042d565b600154811461037057604051630b6fac0360e41b815260040160405180910390fd5b6103786104c4565b5050565b60008061039061038b84610521565b610596565b905082600001516001600160a01b03166103ae8285608001516105d1565b6001600160a01b0316149392505050565b60408082015182516001600160a01b0316600090815260208190529182208054919290916103ee908490610a71565b90915550506040808201516020808401516001600160a01b0316600090815290819052918220805491929091610425908490610a84565b909155505050565b600060208181527f7028a0ec50eccd63e0e5375e137bd936cb02c6de8c1e141753e6eeb1c22d74fe5473419978a8729ed2c3b1048b5bba49f8599ed8f7c19092527fd7f372c1652071b9369261dfb776c225fb305102b25c5645f4c47914557535b4546040516104a7939201918252602082015260400190565b60408051601f198184030181529190528051602090910120600155565b60005b60025481101561050a576104fa600282815481106104e7576104e7610a2c565b9060005260206000209060020201610651565b61050381610a58565b90506104c7565b5061051760026000610775565b61051f61042d565b565b600081600001518260200151836040015184606001516040516020016105799493929190606094851b6bffffffffffffffffffffffff1990811682529390941b90921660148401526028830152604882015260680190565b604051602081830303815290604052805190602001209050919050565b6040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c8101829052600090605c01610579565b6000806000806105e085610728565b6040805160008152602081018083528b905260ff8516918101919091526060810183905260808101829052929550909350915060019060a0016020604051602081039080840390855afa15801561063b573d6000803e3d6000fd5b5050506020604051035193505050505b92915050565b6000815460ff16600181111561066957610669610a16565b036106ac576001810154815461010090046001600160a01b0316600090815260208190526040812080549091906106a1908490610a84565b909155506107259050565b6001810154815461010090046001600160a01b0316600090815260208190526040812080549091906106df908490610a71565b9091555050805460018201546040516101009092046001600160a01b0316916108fc82150291906000818181858888f19350505050158015610378573d6000803e3d6000fd5b50565b6000806000835160411461073b57600080fd5b5050506020810151604082015160609092015160001a92909190565b60405180604001604052806002906020820280368337509192915050565b508054600082556002029060005260206000209081019061072591905b808211156107b75780546001600160a81b031916815560006001820155600201610792565b5090565b60408101818360005b60028110156107e35781518352602092830192909101906001016107c4565b50505092915050565b6000602082840312156107fe57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b60405160a0810167ffffffffffffffff8111828210171561083e5761083e610805565b60405290565b604051601f8201601f1916810167ffffffffffffffff8111828210171561086d5761086d610805565b604052919050565b80356001600160a01b038116811461088c57600080fd5b919050565b600080604083850312156108a457600080fd5b823567ffffffffffffffff808211156108bc57600080fd5b818501915085601f8301126108d057600080fd5b81356020828211156108e4576108e4610805565b8160051b6108f3828201610844565b928352848101820192828101908a85111561090d57600080fd5b83870192505b84831015610a055782358681111561092a57600080fd5b8701601f1960a0828e038201121561094157600080fd5b61094961081b565b610954878401610875565b815261096260408401610875565b87820152606083013560408201526080830135606082015260a08301358981111561098c57600080fd5b8084019350508d603f8401126109a157600080fd5b86830135898111156109b5576109b5610805565b6109c58884601f84011601610844565b92508083528e60408286010111156109dc57600080fd5b806040850189850137600090830188015260808101919091528352509183019190830190610913565b9a9890920135985050505050505050565b634e487b7160e01b600052602160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060018201610a6a57610a6a610a42565b5060010190565b8181038181111561064b5761064b610a42565b8082018082111561064b5761064b610a4256fea264697066735822122090a90a798a5890c88737b1b3030c70efe8934b58513baec22db155ebb74e8a0164736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct L2<M>(ethers::contract::Contract<M>);
    impl<M> Clone for L2<M> {
        fn clone(&self) -> Self {
            L2(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for L2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for L2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(L2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> L2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), L2_ABI.clone(), client).into()
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
                L2_ABI.clone(),
                L2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `currentState` (0x0c3f6acf) function"]
        pub fn current_state(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [ethers::core::types::U256; 2usize]>
        {
            self.0
                .method_hash([12, 63, 106, 207], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xd0e30db0) function"]
        pub fn deposit(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `root` (0xebf0c717) function"]
        pub fn root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([235, 240, 199, 23], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitBlock` (0xf74aa465) function"]
        pub fn submit_block(
            &self,
            tx: ::std::vec::Vec<Tx>,
            new_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 74, 164, 101], (tx, new_root))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw(
            &self,
            amt: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amt)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for L2<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `[139, 170, 87, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    #[doc = "Custom Error type `InvalidStateRoot` with signature `InvalidStateRoot()` and selector `[182, 250, 192, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidStateRoot", abi = "InvalidStateRoot()")]
    pub struct InvalidStateRoot;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum L2Errors {
        InvalidSignature(InvalidSignature),
        InvalidStateRoot(InvalidStateRoot),
    }
    impl ethers::core::abi::AbiDecode for L2Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <InvalidSignature as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2Errors::InvalidSignature(decoded));
            }
            if let Ok(decoded) =
                <InvalidStateRoot as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2Errors::InvalidStateRoot(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for L2Errors {
        fn encode(self) -> Vec<u8> {
            match self {
                L2Errors::InvalidSignature(element) => element.encode(),
                L2Errors::InvalidStateRoot(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for L2Errors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                L2Errors::InvalidSignature(element) => element.fmt(f),
                L2Errors::InvalidStateRoot(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<InvalidSignature> for L2Errors {
        fn from(var: InvalidSignature) -> Self {
            L2Errors::InvalidSignature(var)
        }
    }
    impl ::std::convert::From<InvalidStateRoot> for L2Errors {
        fn from(var: InvalidStateRoot) -> Self {
            L2Errors::InvalidStateRoot(var)
        }
    }
    #[doc = "Container type for all input parameters for the `currentState` function with signature `currentState()` and selector `[12, 63, 106, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "currentState", abi = "currentState()")]
    pub struct CurrentStateCall;
    #[doc = "Container type for all input parameters for the `deposit` function with signature `deposit()` and selector `[208, 227, 13, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
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
    #[doc = "Container type for all input parameters for the `submitBlock` function with signature `submitBlock((address,address,uint256,uint256,bytes)[],bytes32)` and selector `[247, 74, 164, 101]`"]
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
        abi = "submitBlock((address,address,uint256,uint256,bytes)[],bytes32)"
    )]
    pub struct SubmitBlockCall {
        pub tx: ::std::vec::Vec<Tx>,
        pub new_root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `[46, 26, 125, 77]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amt: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum L2Calls {
        CurrentState(CurrentStateCall),
        Deposit(DepositCall),
        Root(RootCall),
        SubmitBlock(SubmitBlockCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for L2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CurrentStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2Calls::CurrentState(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2Calls::Deposit(decoded));
            }
            if let Ok(decoded) = <RootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(L2Calls::Root(decoded));
            }
            if let Ok(decoded) =
                <SubmitBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2Calls::SubmitBlock(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(L2Calls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for L2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                L2Calls::CurrentState(element) => element.encode(),
                L2Calls::Deposit(element) => element.encode(),
                L2Calls::Root(element) => element.encode(),
                L2Calls::SubmitBlock(element) => element.encode(),
                L2Calls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for L2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                L2Calls::CurrentState(element) => element.fmt(f),
                L2Calls::Deposit(element) => element.fmt(f),
                L2Calls::Root(element) => element.fmt(f),
                L2Calls::SubmitBlock(element) => element.fmt(f),
                L2Calls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CurrentStateCall> for L2Calls {
        fn from(var: CurrentStateCall) -> Self {
            L2Calls::CurrentState(var)
        }
    }
    impl ::std::convert::From<DepositCall> for L2Calls {
        fn from(var: DepositCall) -> Self {
            L2Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<RootCall> for L2Calls {
        fn from(var: RootCall) -> Self {
            L2Calls::Root(var)
        }
    }
    impl ::std::convert::From<SubmitBlockCall> for L2Calls {
        fn from(var: SubmitBlockCall) -> Self {
            L2Calls::SubmitBlock(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for L2Calls {
        fn from(var: WithdrawCall) -> Self {
            L2Calls::Withdraw(var)
        }
    }
    #[doc = "Container type for all return fields from the `currentState` function with signature `currentState()` and selector `[12, 63, 106, 207]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CurrentStateReturn(pub [ethers::core::types::U256; 2usize]);
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
    pub struct RootReturn(pub [u8; 32]);
    #[doc = "`Tx(address,address,uint256,uint256,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Tx {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amt: ethers::core::types::U256,
        pub nonce: ethers::core::types::U256,
        pub signature: ethers::core::types::Bytes,
    }
}
