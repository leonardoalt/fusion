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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"DepositAmountNotAvailable\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DepositAmountTooLow\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidInputLength\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidL1Address\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidPreRoot\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSNARK\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidTransactionType\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"l2Recipient\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"deposits\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"root\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Trollup.TxProof[1]\",\"name\":\"l2Block\",\"type\":\"tuple[1]\",\"components\":[{\"internalType\":\"struct Verifier.Proof\",\"name\":\"proof\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G2Point\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256[2]\",\"name\":\"X\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"Y\",\"type\":\"uint256[2]\",\"components\":[]}]},{\"internalType\":\"struct Pairing.G1Point\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"X\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"Y\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256[]\",\"name\":\"input\",\"type\":\"uint256[]\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBlock\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static TROLLUP_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static TROLLUP_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50611b88806100206000396000f3fe60806040526004361061003f5760003560e01c80637985746214610044578063b02c43d014610066578063b6b55f25146100a5578063ebf0c717146100b8575b600080fd5b34801561005057600080fd5b5061006461005f36600461194e565b6100ce565b005b34801561007257600080fd5b50610093610081366004611a95565b60016020526000908152604090205481565b60405190815260200160405180910390f35b6100646100b3366004611a95565b610192565b3480156100c457600080fd5b5061009360005481565b80516020015180516000906100e5576100e5611aae565b60200260200101516000541461010e57604051633ac8d0b760e21b815260040160405180910390fd5b60005b60018110156101495761013982826001811061012f5761012f611aae565b60200201516101d9565b61014281611ada565b9050610111565b5080610156600180611af3565b6001811061016657610166611aae565b60200201516020015160018151811061018157610181611aae565b602002602001015160008190555050565b346000036101b3576040516355fcd02760e01b815260040160405180910390fd5b600081815260016020526040812080543492906101d1908490611b06565b909155505050565b8060200151516012146101ff57604051637db491eb60e01b815260040160405180910390fd5b6102088161023b565b61021a816020015182600001516103da565b15610238576040516332d7ac2d60e21b815260040160405180910390fd5b50565b6000816020015160028151811061025457610254611aae565b6020026020010151905080600003156103d65780600103610303576000826020015160088151811061028857610288611aae565b6020026020010151905060008360200151600c815181106102ab576102ab611aae565b6020026020010151905081600160008381526020019081526020016000205410156102e95760405163113fd29f60e21b815260040160405180910390fd5b600090815260016020526040902080549190910390555050565b806002036103bd576000826020015160088151811061032457610324611aae565b6020026020010151905060008360200151600f8151811061034757610347611aae565b602002602001015190506000819050806001600160a01b0316821461037f5760405163c2c0fefd60e01b815260040160405180910390fd5b6040516001600160a01b0382169084156108fc029085906000818181858888f193505050501580156103b5573d6000803e3d6000fd5b505050505050565b604051637513b90360e01b815260040160405180910390fd5b5050565b60007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000018161040661055f565b90508060800151518551600161041c9190611b06565b1461042657600080fd5b604080518082019091526000808252602082018190525b86518110156104d4578387828151811061045957610459611aae565b60200260200101511061046b57600080fd5b6104c0826104bb85608001518460016104849190611b06565b8151811061049457610494611aae565b60200260200101518a85815181106104ae576104ae611aae565b60200260200101516110e1565b61113f565b9150806104cc81611ada565b91505061043d565b506104fd8183608001516000815181106104f0576104f0611aae565b602002602001015161113f565b90506105418560000151866020015161051584611195565b85604001516105278a60400151611195565b6060880151885161053790611195565b8960200151611234565b6105515760019350505050610559565b600093505050505b92915050565b6105676116a0565b6040805180820182527f22004253f00cd1a64cd6f1303d3ebb3cd12c7c0941c4c09ecf5d3e1008204ab281527f2b9031138d9403cfaed8bc0282eb1ec39f13f3a5b2f55a2e7e4f9ea1cbc4f5126020808301919091529083528151608080820184527f0f30544ddb798f22c9e557137337d58441ae9ad7530faf64647605807b86d03f8285019081527f1b3419c58f9fbe1e58eba7fb69fc115018909e1a45b9592368d758d0c09bddef606080850191909152908352845180860186527f13dcf5a823d85482f929e0a9e9e6a63d066437995334f6c290aec0728b25ed9681527f25fd9e0c09cf68ea4cf9acbe625797c17f1f6ca976bf88de349c0602c6cbe747818601528385015285840192909252835180820185527f067696285961bfa5ab748f3359f08bcf592a5b0cae7508950bf0c4db973b8f678186019081527f058e3d28f22f3710a62cbcf0e0609d54349984db61f82a6b6bf7d164e999dbf9828501528152845180860186527f1bdc33eae4a7cea4e94b634a2a50548e5acc466033b9045d0892927bd5afc2df81527f159f43c482b48ab07356b2a8214ecd6c2ffdcc56bb46a32ee3dc9a9a832792eb818601528185015285850152835190810184527f0772102e0f3278f04cff738427f7df080987dc31369809973d388cc9830d83568185019081527f0d8fa6e12f53bb176b9688ec8036baf460e855705ae0ad50b0599ad8088fc80b828401528152835180850185527f2411774933b5b1ac93501e7ba912597d4db01daed7b9dd9642593537c798feeb81527f2027602d325876d6c7b1a3a659cee0e1e8474c4e9ec77a5d56bbc552b5309769818501528184015290840152815160138082526102808201909352919082015b60408051808201909152600080825260208201528152602001906001900390816107e357505060808201908152604080518082019091527f2c88936d73d99c9d64914bb2f93dbd32d574792c66967cdd9e0de58fa8f8c7e981527e69fbadbcf9a9baf393e2e7cfbcc2812af395ab923945954b3692e7f9e3b5cc60208201529051805160009061087557610875611aae565b602002602001018190525060405180604001604052807f0b8cd1da0a614550d33202a5dae797038ad3a0fbc89cdc0162d10c07186a77d181526020017f1b00d8bcd9ad98464c7043bfdd07ab00e53a11bd382a41f192bd5c9dea7deda681525081608001516001815181106108ec576108ec611aae565b602002602001018190525060405180604001604052807f1e4e46dbf59952f6cda0c16523b17bea95ad3d1002a12bb34bffad406ccf13d381526020017f0d91697fdc114778e7101a96be2233c1b1b17732adbe95783e63aa35591be008815250816080015160028151811061096357610963611aae565b602002602001018190525060405180604001604052807f2c2e462dae75f075d8bbd5859811ef0304bf4337cc61c8604c4202a1dd19984481526020017f12224c6fa30fa2755edaf4ef31de1b8d3a67f81711002351d9c4e5a0a566026d81525081608001516003815181106109da576109da611aae565b602002602001018190525060405180604001604052807f0b34dae17aef7c6469d275d750c5c505f2dd7c3055aab8eb316989d658e1196481526020017f2e803697ac18a886809fede2001fb4ae5f8c4053d979acf7441f7591b5599c8e8152508160800151600481518110610a5157610a51611aae565b602002602001018190525060405180604001604052807f11e4c0d14c50417c36e08142a60269573a5881eda0b067f5e3bb121f258b5cf481526020017f291abc843c5bb2eca7bca2ef084e84c386ed43a978bb169226759423302b28988152508160800151600581518110610ac857610ac8611aae565b602002602001018190525060405180604001604052807f2bb2c4e14f5d055e7496e44d0d3b8bc426b2079e67ac6039c8914fb68dc1710781526020017f21413ca0ce6ae8dc235fa3b2a69606b754e0e56ac9ee10eb606f7576ac9bc52c8152508160800151600681518110610b3f57610b3f611aae565b602002602001018190525060405180604001604052807f0d0aac7ca66d17b7ff012908ab02d9a0f137832ad949a2292f51afa8fa344bbc81526020017f1180af50bf11532f56308e5f63e3626d89882af62fc588d8e7b4b873b6bd13898152508160800151600781518110610bb657610bb6611aae565b602002602001018190525060405180604001604052807f158ce71db852340569ffe52087236dcafc30e5cb5605cb8572fce6c0c7ce385c81526020017f0a648d89b3d35a3067705ab4c297030c0006c0c6af8f1684a604a6b541013c4e8152508160800151600881518110610c2d57610c2d611aae565b602002602001018190525060405180604001604052807f305e9daeb32ea7134b54614b7c89514bf7f4a4ccaa0818fd05b47e16bf8aa93381526020017f2d7a44921597fa4b5f311fc555704dd3a395e47672fbc30fa322e84e7d574ea98152508160800151600981518110610ca457610ca4611aae565b602002602001018190525060405180604001604052807f146b4e9bf5494ecfe968978c466e71ed8f12410891562d964ad6b1d1362d08aa81526020017f205e30e7e33b72c11cc0df6f913239234de6b8a77bee7e95f2f88679adb500358152508160800151600a81518110610d1b57610d1b611aae565b602002602001018190525060405180604001604052807f1a39ceee6a31a4fc196718617afe6bcde246b7a11b537a9553e360750255cf1081526020017f176fa44ff29799357cd5f351b4de825bac18bca4e5b2038e33ff7b7ba8be45e58152508160800151600b81518110610d9257610d92611aae565b602002602001018190525060405180604001604052807f0efa01abf163385f4e09925f8ac959a81f78684bd2b80d1655f151996030b4ca81526020017f128b228af95e4bd98c18bd663fc9fa79e953b6c848d17ef87cfff9a05a4bc1a08152508160800151600c81518110610e0957610e09611aae565b602002602001018190525060405180604001604052807f2419d0749e24e80ccc0c9acab5f50016a3ae5606bce213174cf5bdaf2c855d9b81526020017f2862b77b695ca85ee701978bc45538dc5142f02368dcb5084c32839e477b72f78152508160800151600d81518110610e8057610e80611aae565b602002602001018190525060405180604001604052807f02379d696e3f6d3e589b2c00bab9d0e693e07a3cff1ffe42b3031ce473c1da1981526020017f10da37a17079d6a9d61dd5ec05c1c4c118b31052987eb7fd95ed9567a96c58a98152508160800151600e81518110610ef757610ef7611aae565b602002602001018190525060405180604001604052807f1f03cf33a8759b8ac81b4b4aba5bc971aac5d2612272828ff458b11294eb5d0781526020017f29a41c7ba2c28a1170676f4e159c05cf0b70e24e706bf83ea49cdced8ef864068152508160800151600f81518110610f6e57610f6e611aae565b602002602001018190525060405180604001604052807f133ae86f06c0ddb1cfc0e282849e06571d2e3b8a5692e5e1e60b4b87e81f5b4681526020017f29c2611ca28b649f7a273a1cc20d48991aef5a5828678030122d96aba25426e58152508160800151601081518110610fe557610fe5611aae565b602002602001018190525060405180604001604052807f1a4eead82ed7c5319979266bdfec366392aa367cf2888a2057704364b3b2d90781526020017f0fecfb80df7095b471dddd87a3a53946fa8bebba80947db44cd5fdfb8056ca2a815250816080015160118151811061105c5761105c611aae565b602002602001018190525060405180604001604052807f1a11379ec03d661bb0a82cb882e8ae5d7596413a29b249820b1fd95bbe68dbd881526020017f2acc5a3fd18a1b63d6308eb90e54fbcce75b2194ae50ad2ab786fab092da205e81525081608001516012815181106110d3576110d3611aae565b602002602001018190525090565b60408051808201909152600080825260208201526110fd6116f1565b835181526020808501519082015260408101839052600060608360808460076107d05a03fa9050808061112c57fe5b508061113757600080fd5b505092915050565b604080518082019091526000808252602082015261115b61170f565b8351815260208085015181830152835160408301528301516060808301919091526000908360c08460066107d05a03fa9050808061112c57fe5b604080518082019091526000808252602082015281517f30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47901580156111dc57506020830151155b156111fc5750506040805180820190915260008082526020820152919050565b6040518060400160405280846000015181526020018285602001516112219190611b19565b61122b9084611af3565b90529392505050565b60408051600480825260a08201909252600091829190816020015b604080518082019091526000808252602082015281526020019060019003908161124f57505060408051600480825260a0820190925291925060009190602082015b61129961172d565b8152602001906001900390816112915790505090508a826000815181106112c2576112c2611aae565b602002602001018190525088826001815181106112e1576112e1611aae565b6020026020010181905250868260028151811061130057611300611aae565b6020026020010181905250848260038151811061131f5761131f611aae565b6020026020010181905250898160008151811061133e5761133e611aae565b6020026020010181905250878160018151811061135d5761135d611aae565b6020026020010181905250858160028151811061137c5761137c611aae565b6020026020010181905250838160038151811061139b5761139b611aae565b60200260200101819052506113b082826113bf565b9b9a5050505050505050505050565b600081518351146113cf57600080fd5b825160006113de826006611b3b565b905060008167ffffffffffffffff8111156113fb576113fb61178e565b604051908082528060200260200182016040528015611424578160200160208202803683370190505b50905060005b8381101561165f5786818151811061144457611444611aae565b6020026020010151600001518282600661145e9190611b3b565b611469906000611b06565b8151811061147957611479611aae565b60200260200101818152505086818151811061149757611497611aae565b602002602001015160200151828260066114b19190611b3b565b6114bc906001611b06565b815181106114cc576114cc611aae565b6020026020010181815250508581815181106114ea576114ea611aae565b60209081029190910181015151015182611505836006611b3b565b611510906002611b06565b8151811061152057611520611aae565b60200260200101818152505085818151811061153e5761153e611aae565b6020908102919091010151515182611557836006611b3b565b611562906003611b06565b8151811061157257611572611aae565b60200260200101818152505085818151811061159057611590611aae565b6020026020010151602001516001600281106115ae576115ae611aae565b6020020151826115bf836006611b3b565b6115ca906004611b06565b815181106115da576115da611aae565b6020026020010181815250508581815181106115f8576115f8611aae565b60200260200101516020015160006002811061161657611616611aae565b602002015182611627836006611b3b565b611632906005611b06565b8151811061164257611642611aae565b60209081029190910101528061165781611ada565b91505061142a565b50611668611752565b6000602082602086026020860160086107d05a03fa9050808061168757fe5b508061169257600080fd5b505115159695505050505050565b6040805160e08101909152600060a0820181815260c08301919091528152602081016116ca61172d565b81526020016116d761172d565b81526020016116e461172d565b8152602001606081525090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280611740611770565b815260200161174d611770565b905290565b60405180602001604052806001906020820280368337509192915050565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff811182821017156117c7576117c761178e565b60405290565b6040516060810167ffffffffffffffff811182821017156117c7576117c761178e565b6040516020810167ffffffffffffffff811182821017156117c7576117c761178e565b60006040828403121561182557600080fd5b61182d6117a4565b9050813581526020820135602082015292915050565b600082601f83011261185457600080fd5b6040516040810181811067ffffffffffffffff821117156118775761187761178e565b806040525080604084018581111561188e57600080fd5b845b818110156118a8578035835260209283019201611890565b509195945050505050565b600082601f8301126118c457600080fd5b8135602067ffffffffffffffff808311156118e1576118e161178e565b8260051b604051601f19603f830116810181811084821117156119065761190661178e565b60405293845285810183019383810192508785111561192457600080fd5b83870191505b848210156119435781358352918301919083019061192a565b979650505050505050565b6000602080838503121561196157600080fd5b823567ffffffffffffffff8082111561197957600080fd5b818501915085601f83011261198d57600080fd5b6119956117f0565b80848401888111156119a657600080fd5b845b81811015611a87578035858111156119bf57600080fd5b8601808b036101208112156119d357600080fd5b6119db6117a4565b610100808312156119eb57600080fd5b6119f36117cd565b6119fd8f86611813565b81526080603f1985011215611a125760008081fd5b611a1a6117a4565b9350611a298f60408701611843565b8452611a388f60808701611843565b8c850152838c820152611a4e8f60c08701611813565b60408201528252830135915087821115611a685760008081fd5b611a748d8385016118b3565b818b0152865250509286019286016119a8565b509098975050505050505050565b600060208284031215611aa757600080fd5b5035919050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060018201611aec57611aec611ac4565b5060010190565b8181038181111561055957610559611ac4565b8082018082111561055957610559611ac4565b600082611b3657634e487b7160e01b600052601260045260246000fd5b500690565b808202811582820484141761055957610559611ac456fea2646970667358221220bc60136741f2d729ac95bd11beb10b5ab08a00cea68880fe251dad4c219d246464736f6c63430008110033" . parse () . expect ("invalid bytecode")
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
    #[doc = "Custom Error type `InvalidL1Address` with signature `InvalidL1Address()` and selector `[194, 192, 254, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidL1Address", abi = "InvalidL1Address()")]
    pub struct InvalidL1Address;
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
        InvalidL1Address(InvalidL1Address),
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
                <InvalidL1Address as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TrollupErrors::InvalidL1Address(decoded));
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
                TrollupErrors::InvalidL1Address(element) => element.encode(),
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
                TrollupErrors::InvalidL1Address(element) => element.fmt(f),
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
    impl ::std::convert::From<InvalidL1Address> for TrollupErrors {
        fn from(var: InvalidL1Address) -> Self {
            TrollupErrors::InvalidL1Address(var)
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
