//LOGGERS
//Group of different development loggers


//DefaultLogger: a logger for all general uses 
pub struct DefaultLogger {
    message: String
}
 
impl DefaultLogger {
    pub fn new() -> Self {
        Self { message: String::from("") }
    }

    pub fn log_info(&mut self, msg: &str) {
        DefaultLogger::set_message(self, msg);
        println!("INFO: {}", self.message)
    }

    fn set_message(&mut self, message: &str) {
        self.message = message.to_string()
    }
}

