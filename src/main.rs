
mod door;
mod handler;
mod errors;
use crate::handler::Handler;

pub struct EventHandler{
    pub handler_name: String,
}

impl  EventHandler{
     pub fn new(id:String) ->  dyn handler::Handler{

        EventHandler{handler_name: id}
     }
}

impl<'a> handler::Handler<'a> for EventHandler{

    fn name(&self)-> String{
        self.handler_name
    }

    fn handle(&mut self,level:u32,err: Option<handler::HandlerError>,message:String) ->  Option<handler::HandlerError>{
        if err.is_none(){
            return None;
        }
        let code = err.as_ref().unwrap().err;
        let origin = &err.as_ref().unwrap().origin;

        match err.as_ref().unwrap().origin.as_ref(){

            "check_door" =>{
                match code{
                    errors::DOOR_WONT_CLOSE =>{
                        println!("main handler: trap door won't close {}", origin);
                        return None;
                    }
                     _ => {}    
                }
            }
            _ => {
                match code{
                     errors::DOOR_WONT_CLOSE =>{

                         println!("Fatal: main handler: trap door won't close {}", origin); 
                        
                    }
                    _ => {}

                 }
            }
        }
        println!("MainHandler {} {} in {}",level,message,self.name());
        if level == handler::FATAL{
            assert!(false);
        }
         err
    }
 }

fn main(){

    let mut h = EventHandler::new("main".to_string())  ;
    let &mut  myhandler: dyn handler::Handler = h;

    run(&myhandler);

}

fn run(myhandler: &mut dyn handler::Handler){
}

// fn run(myhandler: &mut dyn handler::Handler){

//     let mut err = door::check_door(myhandler);
 
//     err = myhandler.handle(handler::ERROR,err,"main check door failed".to_string());

//     if err.is_some(){
//         println!("unhandled error");
//         assert!(false);
//     }
  
//     err = door::open_door();
//     err = myhandler.handle(handler::ERROR,err,"main cannot open door".to_string());
//     if err.is_some(){
//         println!("unhandled error");
//         assert!(false);
//     }
//     err = door::close_door();

//     err = myhandler.handle(handler::ERROR,err,"main cannot close door".to_string());
//     if err.is_some(){
//         println!("unhandled error");
//         assert!(false);
//     }
    
// }