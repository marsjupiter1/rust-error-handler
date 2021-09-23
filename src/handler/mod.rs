pub const ERROR:u32=0;
pub const FATAL:u32=0;

pub struct HandlerError{
    pub err: u32, 
    pub detail: Option<String> ,
    pub  origin: String, 
}   

pub trait Handler<'a>{
    fn name(&self) ->String;
    fn handle(&mut self,level:u32,err: Option<HandlerError>,message:String) 
        -> Option<HandlerError>{
        println!("BaseHandler {} {} in {}",level,message,self.name());

        if level == FATAL{
            assert!(false);
        }
        err
    }
}