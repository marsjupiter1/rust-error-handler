use super::handler;
use super::errors;
use crate::handler::Handler;

struct EventHandler<'a>{
    handler_name: &'a str,
    chained_handler: &'a Box<dyn handler::Handler>,
    error_count: u32,
}

impl  <'a>EventHandler<'a>{
    pub fn chained(id:&'a str,h: &'a mut Box<dyn handler::Handler>) -> Box<dyn handler::Handler+ 'a>{
        Box::new(EventHandler{handler_name: id,chained_handler:h,error_count:0})
    }

}

impl  handler::Handler for EventHandler<'_>{
    fn name(&self)->  &str{
        self.handler_name
    }

    fn handle(&mut self,level:u32,mut err: Option<handler::HandlerError>,message:String) ->  Option<handler::HandlerError>{

        println!("door handler {}",err.as_ref().unwrap().err);
        if err.is_some(){
            self.error_count+=1;
            let code = err.as_ref().unwrap().err;
            match code{
                errors::DOOR_WONT_OPEN =>{
                    println!("trap door won't open");
                    return None;
                }
                 _ => {
                    let mut detail: Option<String> = None;
                    if err.as_ref().unwrap().detail.is_some(){
                        detail = Some(err.as_ref().unwrap().detail.as_ref().unwrap().clone());
                    }
                    let newerr = Some(handler::HandlerError{err:code,detail: detail,origin:self.handler_name.to_string()});
                    println!("pass ownership from {} to {}",err.unwrap().origin, self.handler_name);
                    err = newerr;
               }

            }
        }
        self.chained_handler.handle(level,err,message)
  
    }

}

pub fn check_door(mut h:&mut Box<dyn  handler::Handler>) -> Option<handler::HandlerError>{
        
    println!("check door");
    let mut ch = EventHandler::chained("check_door",h);
    let mut err = open_door();

    if err.is_some(){
         err = ch.handle(handler::ERROR,err,"check_door open failed".to_string());
    }else{
        err = close_door();
        err = ch.handle(handler::ERROR,err,"check_door close failed".to_string());
    }
    err
}

pub fn open_door() ->Option<handler::HandlerError>{

    //Some(handler::HandlerError{err:errors::DOOR_WONT_OPEN,detail:None,origin:"open_door".to_string()})
    None
}

pub fn close_door() ->Option<handler::HandlerError>{

    Some(handler::HandlerError{err:errors::DOOR_WONT_CLOSE,detail:None,origin:"close_door".to_string()})
}