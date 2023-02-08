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
            "0x608060405234801561001057600080fd5b506118a6806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063798574621461003b578063ebf0c71714610050575b600080fd5b61004e610049366004611685565b61006b565b005b61005960005481565b60405190815260200160405180910390f35b8051602001518051600090610082576100826117cc565b6020026020010151600054146100ab57604051633ac8d0b760e21b815260040160405180910390fd5b60005b60018110156100e6576100d68282600181106100cc576100cc6117cc565b602002015161012f565b6100df816117f8565b90506100ae565b50806100f3600180611811565b60018110610103576101036117cc565b60200201516020015160018151811061011e5761011e6117cc565b602002602001015160008190555050565b80602001515160111461015557604051637db491eb60e01b815260040160405180910390fd5b61016781602001518260000151610188565b15610185576040516332d7ac2d60e21b815260040160405180910390fd5b50565b60007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001816101b461030d565b9050806080015151855160016101ca9190611824565b146101d457600080fd5b604080518082019091526000808252602082018190525b86518110156102825783878281518110610207576102076117cc565b60200260200101511061021957600080fd5b61026e8261026985608001518460016102329190611824565b81518110610242576102426117cc565b60200260200101518a858151811061025c5761025c6117cc565b6020026020010151610e18565b610e76565b91508061027a816117f8565b9150506101eb565b506102ab81836080015160008151811061029e5761029e6117cc565b6020026020010151610e76565b90506102ef856000015186602001516102c384610ecc565b85604001516102d58a60400151610ecc565b606088015188516102e590610ecc565b8960200151610f6b565b6102ff5760019350505050610307565b600093505050505b92915050565b6103156113d7565b6040805180820182527f297cdac9e35d8077895472d2c2d2a71676c0e001d0126a9d12319ee7914d1fe881527f0a2bb82bf13e43d23f3145840815cf57396d55a41f4438e143cca1dc74dc99856020808301919091529083528151608080820184527f1f1ab9a233e2acf747b2c9e5bd5fb13b31253a475da017b4f811b2759e5995a48285019081527e58b360a694dc2e72711e66c2edeed5652dca26c4c639e8836d97902022b0d8606080850191909152908352845180860186527f28ca7483534e1069a9426e0d7f9fbc45cacb34620aaae58f3d4ca581ad811d8f81527f21059cd087d8134f9a42da67ff7de052c769eff87b143a94b9bc92fa675dbf83818601528385015285840192909252835180820185527f09207fcac5afdef4bca08d2beffca91dd634a685eb15488afc799e1133f101ae8186019081527f1c5a50ef0dce16b4a4eecb01ff51d3fcc25f399104388c39637f997bb337d4c6828501528152845180860186527f23f91a6decb7094c49edb7f698c1cae9c79c25c98a50f5560026e477fc107cd181527f1f02038245039abd176629079cc0492412f667a93285f37fcff26e3bdea337be818601528185015285850152835190810184527f2741788bafc49742152727741cf364aa0e8da40f449bd2ac0a95ea198b5b5e518185019081527f025860ea884c46cf5b910aff15a07696236642fdce0a59063a368f3caeed7256828401528152835180850185527f25392edc68241931a4fdd042140bdc670b391aeb404ed9ec2552e286c4b9083d81527f17c26bce7e7348f9527a6e7bf872a7a26d19d755b4b350063c20ecd77687f2c8818501528184015290840152815160128082526102608201909352919082015b604080518082019091526000808252602082015281526020019060019003908161059057505060808201908152604080518082019091527f04b263b6923f302c6840bdab7496dc6247f9b2c2700222b5a1ff61aeb9c6d1c481527f0c23723df6bec79a3c35267bdb555dc1735fff9ab374b08a25871bcfa840d686602082015290518051600090610623576106236117cc565b602002602001018190525060405180604001604052807f0ffe3f0dc36c328b0b20d891658af5930b5905e1aa10fae755fffd5bcf0e95ef81526020017f27e7316183ae5180512eeffdb9b4ffa79c29c57ef8265d32228691463c0706df815250816080015160018151811061069a5761069a6117cc565b602002602001018190525060405180604001604052807f2a168eaef0e4cf47689bd94fffe11452fa15e1eef58ca62b27a09d51ef8fc7a081526020017f11ae5b682c67d5b5ed790d46314dba711aa727ec45135bcfc9da05d0b51c6b6b8152508160800151600281518110610711576107116117cc565b602002602001018190525060405180604001604052807f0707d93413ed09d749a7fffbfa7c22a18c96eec4762c2270b8065f7eac5ef23481526020017f1e2831087765b510277554f952d2b9012c2d13d1544a79cf9e67dbce7fc1395a8152508160800151600381518110610788576107886117cc565b602002602001018190525060405180604001604052807f187eaec08942e5d08d81be1e49126a0e398b0bf80c973abfd0e937291ea9a15781526020017f3017d14ed48be6cdfe9b308b2a1883a2fb1fc4fa811f598eb97ec90d73c1a65b81525081608001516004815181106107ff576107ff6117cc565b602002602001018190525060405180604001604052807f275b91198404a8e1dc0243c74da1a38b4cbbf429e688a5d1805c58b2970469d381526020017f2461a1403c14cef48af28b3a0bd0172f06b2106a88298adb39d9102ad0e5e3ee8152508160800151600581518110610876576108766117cc565b602002602001018190525060405180604001604052807f0c186dd5df32e5ab28318e94caac5dfa697c6be7d5198c0b41429c4e6bce683e81526020017f10a598c549b5df5474f2b3278afaa50b32e71a4b1870c4b6ab2e0a51308cc66b81525081608001516006815181106108ed576108ed6117cc565b602002602001018190525060405180604001604052807f20d7b6d6aea2508a9a0016e408e5cd123ccf31ee34f33611fc2dc62e4933062381526020017f066d0df216ad76d329bfdef5a0ff416c4d1bc751c4f684a72e560af7c0b0283c8152508160800151600781518110610964576109646117cc565b602002602001018190525060405180604001604052807f1b4f18325c40f140097eac77eb71e10f9c007ef405a2d9918ab98f0477e313eb81526020017f1347398cb9d3e1f3a13a7920fc00b051f5d941ca52088fdff8f51f704b11687081525081608001516008815181106109db576109db6117cc565b602002602001018190525060405180604001604052807f055017edbdd67f08d91c98bcf683ca74054f11f9764453a8df8925cfa077fa6981526020017f127f018e8532275be50812b163d2ca93037586919094366cf7f550f7a463eae48152508160800151600981518110610a5257610a526117cc565b602002602001018190525060405180604001604052807f1f81a9a159bb031f48d3f308223ea0d278d457650eae16513f64e929fc7b315281526020017f281ff653d8a0290ba935ebeb036b056ac44bc336decf3466ff33dc7404969d778152508160800151600a81518110610ac957610ac96117cc565b602002602001018190525060405180604001604052807f05b1fa83bbc356719e34a0ffbc0b4295a815e7da10874be81ebdfb33f21a504d81526020017f22631a1ba19074cb4c95d9f4fa14c8187d62f1d748d736ffed3dccf31b5c53728152508160800151600b81518110610b4057610b406117cc565b602002602001018190525060405180604001604052807f065d807a33f57b717ef604906cb7d2901e7bf26b5b55aeb735b0fe35090631d381526020017f1adfb2c5be32a220db9dae812752cb4a4402193ec76fa71f525d5ba1fda38a6a8152508160800151600c81518110610bb757610bb76117cc565b602002602001018190525060405180604001604052807f067f4e49864eb1ff5f9e87ba75dccad2c6bd1b8b7eb385306641c6ffb514df7181526020017f2cb2ba297d60ac2567d67cb06eb51e1024b65130de11a8a6b4112526834fab898152508160800151600d81518110610c2e57610c2e6117cc565b602002602001018190525060405180604001604052807f2bf4858ff70fce0a8eb0f53ef51507595bf7701f5afea66f80b732532452aabb81526020017f085a7f67908a25f8e2601701317a878d962ab8127d9cac85e83586e9c7c794998152508160800151600e81518110610ca557610ca56117cc565b602002602001018190525060405180604001604052807f0a021e370a4a8ae92d4785190ea82b26bff638f8765a99d2e47e428efb12caec81526020017f10dfaaa8cd0c07b247b017cc0ea569d0232f982560bd272fe5d7d01033a6e9e18152508160800151600f81518110610d1c57610d1c6117cc565b602002602001018190525060405180604001604052807f2cca0b50b117b438dcd4010bef5c78fc986ecfe2c3eb980f143bac34674cc6da81526020017f1dde7191c707eefd3de59bad089ccb3ab974f5f596ecf4e9c331cb5108cf10268152508160800151601081518110610d9357610d936117cc565b602002602001018190525060405180604001604052807f153459f819be372a364af8167623e1dcb9d61e91b100cc9129ba496ef5f75d6f81526020017f2bfc60f71d0e309d7ef610085da9ce53d9c99a61b4d969143bace06b58a6998c8152508160800151601181518110610e0a57610e0a6117cc565b602002602001018190525090565b6040805180820190915260008082526020820152610e34611428565b835181526020808501519082015260408101839052600060608360808460076107d05a03fa90508080610e6357fe5b5080610e6e57600080fd5b505092915050565b6040805180820190915260008082526020820152610e92611446565b8351815260208085015181830152835160408301528301516060808301919091526000908360c08460066107d05a03fa90508080610e6357fe5b604080518082019091526000808252602082015281517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd4790158015610f1357506020830151155b15610f335750506040805180820190915260008082526020820152919050565b604051806040016040528084600001518152602001828560200151610f589190611837565b610f629084611811565b90529392505050565b60408051600480825260a08201909252600091829190816020015b6040805180820190915260008082526020820152815260200190600190039081610f8657505060408051600480825260a0820190925291925060009190602082015b610fd0611464565b815260200190600190039081610fc85790505090508a82600081518110610ff957610ff96117cc565b60200260200101819052508882600181518110611018576110186117cc565b60200260200101819052508682600281518110611037576110376117cc565b60200260200101819052508482600381518110611056576110566117cc565b60200260200101819052508981600081518110611075576110756117cc565b60200260200101819052508781600181518110611094576110946117cc565b602002602001018190525085816002815181106110b3576110b36117cc565b602002602001018190525083816003815181106110d2576110d26117cc565b60200260200101819052506110e782826110f6565b9b9a5050505050505050505050565b6000815183511461110657600080fd5b82516000611115826006611859565b905060008167ffffffffffffffff811115611132576111326114c5565b60405190808252806020026020018201604052801561115b578160200160208202803683370190505b50905060005b838110156113965786818151811061117b5761117b6117cc565b602002602001015160000151828260066111959190611859565b6111a0906000611824565b815181106111b0576111b06117cc565b6020026020010181815250508681815181106111ce576111ce6117cc565b602002602001015160200151828260066111e89190611859565b6111f3906001611824565b81518110611203576112036117cc565b602002602001018181525050858181518110611221576112216117cc565b6020908102919091018101515101518261123c836006611859565b611247906002611824565b81518110611257576112576117cc565b602002602001018181525050858181518110611275576112756117cc565b602090810291909101015151518261128e836006611859565b611299906003611824565b815181106112a9576112a96117cc565b6020026020010181815250508581815181106112c7576112c76117cc565b6020026020010151602001516001600281106112e5576112e56117cc565b6020020151826112f6836006611859565b611301906004611824565b81518110611311576113116117cc565b60200260200101818152505085818151811061132f5761132f6117cc565b60200260200101516020015160006002811061134d5761134d6117cc565b60200201518261135e836006611859565b611369906005611824565b81518110611379576113796117cc565b60209081029190910101528061138e816117f8565b915050611161565b5061139f611489565b6000602082602086026020860160086107d05a03fa905080806113be57fe5b50806113c957600080fd5b505115159695505050505050565b6040805160e08101909152600060a0820181815260c0830191909152815260208101611401611464565b815260200161140e611464565b815260200161141b611464565b8152602001606081525090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806114776114a7565b81526020016114846114a7565b905290565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff811182821017156114fe576114fe6114c5565b60405290565b6040516060810167ffffffffffffffff811182821017156114fe576114fe6114c5565b6040516020810167ffffffffffffffff811182821017156114fe576114fe6114c5565b60006040828403121561155c57600080fd5b6115646114db565b9050813581526020820135602082015292915050565b600082601f83011261158b57600080fd5b6040516040810181811067ffffffffffffffff821117156115ae576115ae6114c5565b80604052508060408401858111156115c557600080fd5b845b818110156115df5780358352602092830192016115c7565b509195945050505050565b600082601f8301126115fb57600080fd5b8135602067ffffffffffffffff80831115611618576116186114c5565b8260051b604051601f19603f8301168101818110848211171561163d5761163d6114c5565b60405293845285810183019383810192508785111561165b57600080fd5b83870191505b8482101561167a57813583529183019190830190611661565b979650505050505050565b6000602080838503121561169857600080fd5b823567ffffffffffffffff808211156116b057600080fd5b818501915085601f8301126116c457600080fd5b6116cc611527565b80848401888111156116dd57600080fd5b845b818110156117be578035858111156116f657600080fd5b8601808b0361012081121561170a57600080fd5b6117126114db565b6101008083121561172257600080fd5b61172a611504565b6117348f8661154a565b81526080603f19850112156117495760008081fd5b6117516114db565b93506117608f6040870161157a565b845261176f8f6080870161157a565b8c850152838c8201526117858f60c0870161154a565b6040820152825283013591508782111561179f5760008081fd5b6117ab8d8385016115ea565b818b0152865250509286019286016116df565b509098975050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60006001820161180a5761180a6117e2565b5060010190565b81810381811115610307576103076117e2565b80820180821115610307576103076117e2565b60008261185457634e487b7160e01b600052601260045260246000fd5b500690565b8082028115828204841417610307576103076117e256fea26469706673582212204c9b6c1c91cdef032599aa7c835218877afb775636de09df244e8eb803e8480364736f6c63430008110033" . parse () . expect ("invalid bytecode")
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
