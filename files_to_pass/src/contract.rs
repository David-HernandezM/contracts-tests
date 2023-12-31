use core::ops::Index;

use gstd::{
    prelude::*, 
    msg 
};
use program_io::*;

include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
extern "C" fn init() {
    let config: InitContractData = msg::load()
        .expect("Error in decoding init message 'InitContractData'");
    unsafe {
        CONTRACT = Some(Contract {
            owner: msg::source(),
            nft_contract: config.nft_contract,
            tokens_metadata_default: config.tokens_metadata_default
                .into_iter()
                .enumerate()
                .map(|(index, data)| (index as u8, data))
                .collect(),
            ..Default::default()
        });
    };
}

#[gstd::async_main]
async fn main() {
    let action = msg::load().expect("Error in loading message");
    let state = state_mut();
    
    let mut set_nft_contract = false;
    if let RutzoAction::SetNFTAddress(_) = action {
        set_nft_contract = true;
    }
    
    if !set_nft_contract && state.nft_contract.is_none() {
        panic!("Nft contract does not exists!");
    }
    
    match action {
        RutzoAction::PlayGame {
            token_id,
            power
        } => {
            let message = state.play_game(msg::source(), token_id, power.parse::<u8>().expect("Error parsing")).await;
            msg::reply(message, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::MintCard {
            token_id
        } => {
            msg::reply(state.mint_card(token_id).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::SetNFTAddress(address) => {
            let user_id = msg::source();
            if user_id != state.owner {
                msg::reply(RutzoEvent::UserIsNotApproved(user_id), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.nft_contract = Some(address);
            
            msg::reply(RutzoEvent::NFTContractSaved, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::Register => {
            msg::reply(state.register_user(msg::source()), 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::AddNftForSale { 
            token_metadata 
        } => {
            msg::reply(state.mint_nft_to_sale(msg::source(), token_metadata, msg::value()).await, 0)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::BuyNFT(token_id) => {
            let (message, value_to_return) = state.buy_nft(msg::source(), token_id, msg::value()).await;
            msg::reply(message, value_to_return)
                .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::ApproveMinter(user_id) => {
            let caller = msg::source();
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            state.approved_minters.push(user_id);
            
            msg::reply(RutzoEvent::Approved(user_id), 0)
                 .expect("Error in reply a message 'RutzoEvent'");
        },
        RutzoAction::DelegateApprovedUser(user_id) => {
            let caller = msg::source();
            if caller != state.owner {
                msg::reply(RutzoEvent::UserIsNotTheOwner(caller), 0)
                    .expect("Error in reply a message 'RutzoEvent'");
                return;
            }
            
            let index = match state.approved_minters
                .iter()
                .enumerate()
                .find(|&(_, approved_user)| *approved_user == user_id) {
                    Some((index, _)) => {
                        index
                    },
                    None => {
                        msg::reply(RutzoEvent::UserApprovedNotExists(user_id), 0)
                            .expect("Error in reply a message 'RutzoEvent'");
                        return;
                    }
                };
                
            state.approved_minters.swap_remove(index);
            
            msg::reply(RutzoEvent::ApprovedUserDeleted(user_id), 0) 
                .expect("Error in reply a message 'RutzoEvent'");
        }
    }
}

/*
#[no_mangle]
unsafe extern "C" fn state() {
    let contract = CONTRACT.take().expect("Unexpected error in taking state");
    msg::reply::<ContractState>(contract.into(), 0)
        .expect("Error in sending reply state");
}
*/

#[no_mangle]
unsafe extern "C" fn state() {
    let message = msg::load()
        .expect("Error in decode 'RutzoStateAction'");
    match message {
        RutzoStateAction::GetId => {
            msg::reply(RutzoStateEvent::Id(msg::source()), 0)
                .expect("Error in reply message 'RutzoStateEvent'");
        },
        RutzoStateAction::Story => {
            msg::reply(RutzoStateEvent::StoryEvent(String::from("Story")), 0)
                .expect("Error in reply message 'RutzoStateEvent'");
        },
        RutzoStateAction::All => {
             let contract = CONTRACT.take().expect("Unexpected error in taking state");
            msg::reply(RutzoStateEvent::AllEvent(contract.into()), 0)
                .expect("Error in sending reply state");
        }
    }
}


pub fn state_mut() -> &'static mut Contract {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}


/*
                {
                    "name": "Death City Earth",
                    "description": "Rock",
                    "media": "https://home.rutzo.studio/NFT/death_city_earth.jpg",
                    "reference": "20"
                },
                {
                    "name": "Chinampa",
                    "description": "Water",
                    "media": "https://home.rutzo.studio/NFT/chinampa_water.jpg",
                    "reference": "25"
                },
                {
                    "name": "Chile",
                    "description": "Fire",
                    "media": "https://home.rutzo.studio/NFT/chile_fire.jpg",
                    "reference": "55"
                },
                {
                    "name": "peaceful axolotl",
                    "description": "Water",
                    "media": "https://home.rutzo.studio/NFT/peaceful_axolotl_water.jpg",
                    "reference": "33"
                },
                {
                    "name": "ixchel",
                    "description": "Rock",
                    "media": "https://home.rutzo.studio/NFT/ixchel_wind.jpg",
                    "reference": "33"
                },
                {
                    "name": "tlaloc",
                    "description": "Water",
                    "media": "https://home.rutzo.studio/NFT/tlaloc_water.jpg",
                    "reference": "75"
                }



{
    "owner": "Address of owner",
    "nftContract": "address of nft contract",
    "games": [
        {
            "user1": {
                "userId": "address of player1",
                "chosenNft": "Number of chosen nft",
                "power": "Number of power"
            },
            "user2": {
                "userId": "address of player2",
                "chosenNft": "Number of chosen nft",
                "power": "Number of power"
            },
            "matchState": {
                "Finished": {
                    "winner": "address of winner",
                    "loser": "address of loser"
                }
            }
        },
        {
            "user1": {
                "userId": "address of player1",
                "chosenNft": "Number of chosen nft",
                "power": "Number of power"
            },
            "user2": null,
            "matchState": "InProgress"
        }
    ],
    "gamesWaiting": [
        "Number of index in the games vector"
    ],
    "gamesInformationByUser": [
        [
            "Address of user",
            {
                "currentGame": null,
                "pastGames": [
                    "Number of index in games vector with finished game",
                    "Number of index in games vector with finished game"
                ]
            }
        ],
        [
            "Address of user",
            {
                "currentGame": "Index Number in games vector where the user is playing",
                "pastGames": [
                    "Number of index in games vector with finished game"
                ]
            }
        ],
    ],
    "gameId": "Actual index for new games",
    "tokensMetadataDefault": [
        [
            "0",
            {
                "name": "Death City Earth",
                "description": "Rock",
                "media": "https://home.rutzo.studio/NFT/death_city_earth.jpg",
                "reference": "20"
            }
        ],
        [
            "1",
            {
                "name": "Chinampa",
                "description": "Water",
                "media": "https://home.rutzo.studio/NFT/chinampa_water.jpg",
                "reference": "25"
            }
        ],
        [
            "2",
            {
                "name": "Chile",
                "description": "Fire",
                "media": "https://home.rutzo.studio/NFT/chile_fire.jpg",
                "reference": "55"
            }
        ],
        [
            "3",
            {
                "name": "peaceful axolotl",
                "description": "Water",
                "media": "https://home.rutzo.studio/NFT/peaceful_axolotl_water.jpg",
                "reference": "33"
            }
        ],
        [
            "4",
            {
                "name": "ixchel",
                "description": "Rock",
                "media": "https://home.rutzo.studio/NFT/ixchel_wind.jpg",
                "reference": "33"
            }
        ],
        [
            "5",
            {
                "name": "tlaloc",
                "description": "Water",
                "media": "https://home.rutzo.studio/NFT/tlaloc_water.jpg",
                "reference": "75"
            }
        ]
    ],
    "defaultTokensMintedById": [
        [
            "Address of user",
            "Number of already minted default nft"
        ],
        [
            "Address of user",
            "Number of already minted default nft"
        ]
    ],
    "approvedMinters": [],
    "transactionId": "Number of actual transaction id of the main contract",
    "pendingTransfers": []
}




*/



/*
Sistemas lineaes e invariantes en el tiempo (LIT) 

(Suma ponderada de impulsos unitarios desplazados en el tiempo - muestreo)
La convolucion es una doble suma ponderada de pulsos unitarios desplazados en el tiempo, y luego cada impulso generado
afecta a la otra señal ccon la que se esta convolucionando y tanto en amplitud como en desplazamiento.
*/