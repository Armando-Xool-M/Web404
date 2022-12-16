#![no_std]
use gstd::{msg,ActorId, Decode, Encode, TypeInfo, prelude::*};



gstd::metadata! {
    title: "SCTransact",
    handle:
        input: SCTAction,
        output:TransactEvent,
    state:
    output: u128,
}

#[derive(Default)]

pub struct InitSCT {
    pub user_a: ActorId,
    pub user_b: ActorId,
    pub state: TransactState, 
    pub price:u128,
}


#[derive(Debug,Eq,PartialEq)]

pub enum TransactState {
    AwaitingTransaq,
    AwaitingValidation,
    AwaitingDelivery,
    AwaitingUpdate,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum SCTAction{
    ExecutionTransact,
    ConfirmValidation,
    ConfirmDelivery,
    VerifyUpdate,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TransactEvent {
    TransactExec,
    Valid,
    DeliveryC,
    UpdateV,
}

impl InitSCT {
    pub fn transact(&mut self){
        assert_eq!(
            self.state,
            TransactState::AwaitingTransaq,
            "Se está esperando que inicie la transacción"
        );
        assert_eq!(
            msg::source(),
            self.user_a,
            "El Usuario A solicitará la transacción con el Usuario B"           
        );
        assert_eq!(
            msg::source(),
            self.user_b,
            "El Usuario B aceptará la transacción con el Usuario A"   
        );
        assert_eq!(
            msg::value(),
            self.price,
            "El monto que se manejará entre los usuarios"
        );
        self.state = TransactState::AwaitingTransaq;
        msg::reply(TransactEvent::TransactExec, 0).expect("Error in reply `TransactEvent::TransactExec");
      
    }
    fn delivery(&mut self){
        assert_eq!(
            self.state,
            TransactState::AwaitingDelivery,
            "Esperando la entrega del monto acordado"
        );
        assert_eq!(
            msg::source(),
            self.user_a,
            "El Usuario A envie el monto para el Usuario B"           
        );
        assert_eq!(
            msg::source(),
            self.user_b,
            "El Usuario B esperará la transacción del Usuario A"   
        );

        self.state = TransactState::AwaitingDelivery;
        msg::reply(TransactEvent::DeliveryC, 0).expect("Error in reply `TransactEvent::DeliveryC");
      
    }

    fn validation(&mut self){
        assert_eq!(
            self.state,
            TransactState::AwaitingValidation,
            "Esperando la entrega del monto acordado"
        );
        assert_eq!(
            msg::source(),
            self.user_a,
            "El Usuario A esperará la validación del Usuario B sobre el monto llegado"           
        );
        assert_eq!(
            msg::source(),
            self.user_b,
            "El Usuario B notificará que la transacción se ha concluido con el Usuario A"   
        );

        self.state = TransactState::AwaitingValidation;
        msg::reply(TransactEvent::Valid, 0).expect("Error in reply `TransactEvent::Valid");
      
    }
    
    fn update(&mut self){
        assert_eq!(
            self.state,
            TransactState::AwaitingUpdate,
            "Esperando la entrega del monto acordado"
        );
        assert_eq!(
            msg::source(),
            self.user_a,
            "El Usuario A visualizará la actualización de su balance"           
        );
        assert_eq!(
            msg::source(),
            self.user_b,
            "El Usuario B visualizará la actualización de su balance"   
        );


        self.state = TransactState::AwaitingUpdate;
        msg::reply(TransactEvent::UpdateV, 0).expect("Error in reply `TransactEvent::UpdateV");
      
    }
    }

 impl Default for TransactState {
     fn default() -> Self {
         Self::AwaitingTransaq 
     }
} 
 
    static mut InitSCT: Option<InitSCT> = None; 

 #[no_mangle]
 unsafe extern "C" fn handle() {
    let action: SCTAction = msg::load().unwrap_or_else(|_|{
        panic!(
            "Unable to decode SCTACtion"
        )
    });
    let init_sct: &mut InitSCT = InitSCT.get_or_insert(Default::default());
    match action {
        SCTAction::ExecutionTransact => init_sct.transact(),
        SCTAction:: ConfirmDelivery => init_sct.delivery(),
        SCTAction:: ConfirmValidation => init_sct.validation(),
        SCTAction:: VerifyUpdate => init_sct.update(),
    }
 }

 #[no_mangle]
unsafe extern "C" fn init () {
    let x: ActorId =msg::source();
     let mut init_sct: InitSCT = Default::default();
    init_sct = InitSCT {
         user_a: x.clone(),
         user_b: x.clone(),
         price: 500 as u128,
         state:TransactState::AwaitingTransaq

     };
     unsafe {InitSCT= Some(init_sct)}
    }
