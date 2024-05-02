#[derive(PartialEq, Eq)]
pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
     match security_level {
        Security::Unknown => server.unwrap(),
        Security::High => match  server{
            Ok(value) => value,  
            Err(_) => panic!("ERROR: program stops")   
        },
            
        Security::Medium => server.unwrap_or("WARNING: check the server".to_string()),
        
        Security::Low => server.unwrap_or_else(|err| format!("Not found: {}", err)),
            
        Security::BlockServer => match server{
            Ok(value) => panic!("{}", value),
            Err(_) => format!("Breukh") 
        }
    }
}


