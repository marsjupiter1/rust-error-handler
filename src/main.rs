
mod door;
mod handler;
mod errors;
use crate::handler::Handler;

#[derive(Copy, Clone)]
struct EventHandler<'a>{
    handler_name: &'a str,
}

impl EventHandler<'_>{
    pub fn new(id:& str) ->EventHandler{
        EventHandler{handler_name: id}
    }
}

impl<'a> handler::Handler<'a> for EventHandler<'a>{

    fn name(&self)-> &str{
        self.handler_name
    }

    fn handle(&self,level:u32,err: Option<handler::HandlerError>,message:String) ->  Option<handler::HandlerError>{

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

    let  myhandler = EventHandler::new("main");

    let mut err = door::check_door(&myhandler);
 
    err = myhandler.handle(handler::ERROR,err,"main check door failed".to_string());

    if err.is_some(){
        println!("unhandled error");
        assert!(false);
    }
  
    err = door::open_door();
    err = myhandler.handle(handler::ERROR,err,"main cannot open door".to_string());
    if err.is_some(){
        println!("unhandled error");
        assert!(false);
    }
    err = door::close_door();

    err = myhandler.handle(handler::ERROR,err,"main cannot close door".to_string());
    if err.is_some(){
        println!("unhandled error");
        assert!(false);
    }
    
}