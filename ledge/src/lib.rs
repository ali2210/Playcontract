// external rust modules 
use near_sdk::{near_bindgen, env};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::init;
use std::time::{Instant,Duration};


// contract is a specialize object which will transfer when "Bob" or "Alice" buy assets 
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[near_bindgen]
#[derive(Debug, PartialEq, Clone)]
pub struct Contract{
    signed_account : String,
    ledge_account : String,
    timestamp : u64,
    collect: bool,
    duration: u64,
    gas : f64,

}

// static variables for the contract
pub static COLLECT_CON : bool = true;
pub static LEGAL_BIND : u64 = Duration::from_secs(24*60*60).as_secs();

#[derive(BorshSerialize, BorshDeserialize)]
#[derive(PartialEq, Debug,Clone, Copy)]
// Contract Players Either Bob owned asset legally or Alice will.
pub enum Characters{
    Bob,
    Alice,
}

#[derive(BorshSerialize, BorshDeserialize)]
#[derive(PartialEq,Debug,Clone)]
// Bob and Alice Balance sheet for asset
pub struct Contractor{
    signed_account : String,
    balance : f64,
    signed : bool,
    nick : Characters,
    votes : i64,
}

impl Contract {

    #[init]
    pub fn init(c : Contractor, owner : String) -> Self {
        Self{
                signed_account : c.signed_account.clone(), 
                gas : c.balance * 0.2,
                timestamp : Instant::now().elapsed().as_secs(),
                collect : false,
                duration : LEGAL_BIND,
                ledge_account : owner.clone(),    
        }
    }
    // "Alice" & "Bob" apply for legal business
    pub fn new(self, c : Contractor, owner : String) -> Contract{
        Contract {
            signed_account : c.signed_account.clone(),
            gas : c.balance * 0.2,
            timestamp : self.get_timestamp(),
            collect : false,
            duration : LEGAL_BIND,
            ledge_account : owner.clone(),
        }
    }
    
    // When will legal buiness bind created 
    fn get_timestamp(self) -> u64 {
        Instant::now().elapsed().as_secs()
    }

}

impl Contractor{

    // "Alice" & "Bob" create balance sheet in BLOCK-CHAIN
    pub fn new_contractor(&self, signed_account : String, choice: Characters) -> Contractor {
        
        Contractor{
            signed_account: signed_account.clone(),
            balance: 100.0,
            signed : false,
            nick : self.get_nick(choice),
            votes: 1,
        }
    }

    // GET NICKNAME BECAUSE OF BLOCKCHAIN NATURE
    fn get_nick(&self, choice: Characters) -> Characters{
        let nickname : Characters;
        match choice {
            Characters::Bob =>{ nickname = Characters::Bob;},
            Characters::Alice=>{ nickname = Characters::Alice;}
        }
        nickname
    }

    // DOES BOTH HAVE SAME STATUSQ;
    fn get_max(&self, a : i64, b :i64) -> i64 {
        let mut max : i64 = 0;
        if a >= b { max = a; } 
        else if a < b { max = b; }
        max
    }

    // MONOPOLY GAME
    fn list_voter(self, monopoly : Characters, bid : Vec::<i64>, value: i64) -> (bool, Self){
   
        let mut trival : bool = false;
        let mut logs ;
        match monopoly{
            Characters::Bob =>{

                logs = format!("Both parties are willing to buy art : {:?}", bid);
                env::log(logs.as_bytes());
                if self.get_max(bid[bid.iter().count()- bid.len()], bid[bid.iter().count()- bid.len()+1]) == value{
                    trival = true;
                }
                logs = format!("State Bob : {:#?}", self);
                env::log(logs.as_bytes());
                (trival, self)
            }, Characters::Alice => {
                if self.get_max(bid[bid.iter().count()- bid.len()], bid[bid.iter().count()- bid.len()+1]) == value{
                    trival = false;
                }
                logs = format!("State Alice : {:#?}", self);
                env::log(logs.as_bytes());
                (trival, self)
            }
        }
        
    }
    
    // Art transfer to one of the party based on Monopoly game
    fn change_owership(&mut self, a: &mut Artifact, contractor : String) -> Contract{
        
        #[warn(unused_mut)]
        let mut logs;
        
        // pay the amount if you win
        self.balance -= a.bought; 


        // report party balance 
        logs = format!("Your new balance is : {:?}", self.balance);
        env::log(logs.as_bytes());
        
        // finalize the contract & transfer to win party
        Contract{
            signed_account : self.signed_account.clone() ,
            gas : self.balance * 0.2,
            timestamp : Instant::now().elapsed().as_secs(),
            collect : COLLECT_CON,
            duration : LEGAL_BIND,
            ledge_account : contractor.clone(),
        }
    }
    
}

// Art information that are publicly available or shared between clients
pub static COLLECTION_NAME: &'static str = "GENESIS_COIN";
pub static COLLECTION_CREATED : &'static str = "2009";
pub static ARTIST_NAME : &'static str = "Satoshi";
pub static BID_VALUE : u64 = 30;

// Buy with Aution or without aution
#[derive(Debug, PartialEq, Clone)]
pub enum BuyingStatus{ Auction, None}


// art object
#[derive(Debug, PartialEq, Clone)]
pub struct Artifact{
    name : String,
    options : BuyingStatus,
    bought : f64,
    who_owned : String,
    started : u64,
    timestamp : u64,
    created : String,
    artist : String,
}

impl Artifact{

    // create art with public information 
    pub fn new_collections(&self) -> Artifact{
        Artifact{
            name : COLLECTION_NAME.to_string(),
            options : BuyingStatus::None,
            bought : 0.00,
            who_owned : "None".to_string(),
            started : BID_VALUE,
            timestamp : Instant::now().elapsed().as_secs(),
            created: COLLECTION_CREATED.to_string(),
            artist : ARTIST_NAME.to_string(),
        }
    }

    // Aution started 
    fn auction(&mut self, party : Vec::<Contractor>, price : Vec::<f64>) -> &mut Self{

        for i in party.iter() {
            for j in 0..price.len() {
                if price[j] > (i.balance - (i.balance * 0.2)) || price[j] == 0.00 {
                    env::panic(" You don't have enough money".as_bytes());
                }else{
                    let (offer, parties) = self.get_max(price[j], price[price.iter().count() -price.len()+1]);
                    if offer > 0.00 && parties != 1{
                        self.bought = price[0];
                        self.options = BuyingStatus::Auction; 
                        self.timestamp = Instant::now().elapsed().as_secs();
                        self.artist = ARTIST_NAME.to_string();
                        self.created = COLLECTION_CREATED.to_string();
                        self.started = BID_VALUE;
                        self.who_owned = i.signed_account.clone();
                        self.name = COLLECTION_NAME.to_string();
                    }else{
                        self.bought = price[1];
                        self.options = BuyingStatus::Auction;
                        self.timestamp = Instant::now().elapsed().as_secs();
                        self.artist = ARTIST_NAME.to_string();
                        self.created = COLLECTION_NAME.to_string();
                        self.started = BID_VALUE;
                        self.who_owned = i.signed_account.clone();
                        self.name = COLLECTION_CREATED.to_string();
                    }
                }
            }
        }
        self
    }

    // Who are biggest capitalist 
    fn get_max(&self, price : f64, price_1 : f64) -> (f64, i64) {
        let mut max : f64 = 0.00;
        if price >= price_1 { max = price; (max, 0) } 
        else if price < price_1 { max = price_1; (max, 1) }
        else{
            env::panic("Invalid price".as_bytes());
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs,VMContextBuilder};
    use near_sdk::{testing_env};
    // use near_sdk::json_types::{ValidAccountId};
    use near_sdk::MockedBlockchain;

    #[test]
    // Execute contract
    fn get_contractor(){

        testing_env!(VMContextBuilder::new().build());
        
        #[warn(unused_mut)]
        let mut logs;
        let mut contract = Vec::<Contractor>::new();

        let ledge = Contractor{signed_account: "".to_string(), balance : 0.00, nick : Characters::Bob,signed:false, votes : 0,};
        contract.push(ledge.new_contractor("mxhHjc8g2xeYgnqqACiF7DEQegZHyLJEon".to_string(), Characters::Bob));
        contract.push(ledge.new_contractor("n2Fhga9AsZ2XmEd6bqbKfLPd8D4ySG4uY".to_string(), Characters::Alice));
        assert_ne!(contract.iter().count(), 0);
        assert_ne!(contract[0],ledge);
        assert_ne!(contract[0].signed_account, contract[1].signed_account);

        let bind = Contract{
            signed_account : "".to_string(),
            ledge_account : "".to_string(),
            timestamp : Instant::now().elapsed().as_secs(),
            gas : 0.0000,
            collect : false,
            duration : Duration::from_secs(60).as_secs(),
        };
        
        assert_ne!(bind.new(ledge.new_contractor("mxhHjc8g2xeYgnqqACiF7DEQegZHyLJEon".to_string(), Characters::Bob)
        , contract[1].signed_account.clone()), Contract{
            signed_account : "".to_string(),
            ledge_account : "".to_string(),
            timestamp : Instant::now().elapsed().as_secs(),
            gas : 0.0000,
            collect : false,
            duration : Duration::from_secs(60).as_secs(),
        });
        
        let mut created_art = Artifact{
           name : "".to_string(),
           options : BuyingStatus::None,
           bought : 0.00,
           who_owned : "None".to_string(),
           started : 0,
           timestamp : Instant::now().elapsed().as_secs(),
           created : "None".to_string(),
           artist : "None".to_string(),
        };

        logs = format!("Art in aution: {:#?}", created_art.new_collections());
        env::log(logs.as_bytes());

        let mut votes : Vec<i64> = Vec::<i64>::new();
        for v in contract.iter() {
            votes.push(v.votes);
        }
        
        assert_ne!(votes.len(),0);
        
        for c in contract.iter(){
            let (ok, con) = c.clone().list_voter(c.nick,votes.clone(),20);    
            assert_eq!(c, &con);
            assert_ne!(ok, true);
        }     

        let mut offer = Vec::<f64>::new();
        offer.push(70.00);
        offer.push(60.00);
         
        let own = created_art.auction(contract.clone(), offer);
        logs = format!("Art owed: {:#?}", own);
        env::log(logs.as_bytes());


        for c in contract.iter() {
           if own.who_owned == c.signed_account{
              logs = format!("Ledger: {:#?}", c.clone().change_owership(own, own.who_owned.clone()));
              env::log(logs.as_bytes());
           }
           
        }

        println!("Logs: {:#?}", get_logs());
        
    }
    
  }

