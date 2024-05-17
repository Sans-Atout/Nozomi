use std::path::Path;

use crate::error::Error;
#[cfg(not(feature="error-stack"))]
use crate::Result;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq,Eq))]
pub struct SecureDelete{
    path : String,
    pattern: Option<[u8; 3]>,
    byte: Option<u8>,
}

#[cfg(not(feature="error-stack"))]
impl SecureDelete{
    pub fn new(path : &str) -> Result<Self>{
        if !Path::new(&path).exists(){
            return Err(crate::Error::FileNotFound(path.to_string()))
        }
        Ok(SecureDelete {
            path : path.to_string(),
            pattern : None,
            byte : None
        })
    }

    pub fn delete(&self) -> Result<()>{

        Ok(())
    }

    pub fn rename(&mut self) -> Result<()>{

        Ok(())
    }

    pub fn overwrite(&mut self) -> Result<()>{

        Ok(())
    }

    fn zero_name(&self) -> Result<String> {
        let name = Path::new(&self.path).file_name().ok_or(Error::NoFileName(self.clone().to_owned()))?;
        let new_name = (0..name.len()).map(|_| "0").collect::<String>();
        Ok(new_name)
    }

}

#[cfg(feature = "error-stack")]
impl SecureDelete{
    pub fn new(path : &str) -> Result<Self>{
        if !Path::new(&path).exists(){
            return Err(crate::Error::FileNotFound(path.to_string()))
        }
        Ok(SecureDelete {
            path : path.to_string(),
            pattern : None,
            byte : None
        })
    }
}

impl SecureDelete{
    pub fn byte(&mut self, byte: &u8) -> &mut Self {
        self.byte = Some(*byte);
        self.pattern = None;
        self
    }

    pub fn pattern(&mut self, pattern : &[u8;3]) -> &mut Self{
        self.pattern = Some(*pattern);
        self.byte = None;
        self
    }

}


#[cfg(test)]
#[cfg(not(feature="error-stack"))]
mod std_test {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use crate::Result;

    use super::SecureDelete;

    #[test]
    fn creation() -> Result<()>{
        let mut basic_creation = SecureDelete::new("README.md")?;
        assert_eq!(basic_creation, SecureDelete {
            path : "README.md".to_string(),
            byte : None, 
            pattern : None
        });
        basic_creation.pattern(&[0x00_u8, 0x00_u8, 0x00_u8]);
        assert_eq!(basic_creation, SecureDelete {
            path : "README.md".to_string(),
            byte : None, 
            pattern : Some([0x00_u8, 0x00_u8, 0x00_u8])
        });
        basic_creation.byte(&0x00_u8);
        assert_eq!(basic_creation, SecureDelete {
            path : "README.md".to_string(),
            byte : Some(0x00_u8), 
            pattern : None
        });
        Ok(())
    }

    #[test]
    fn zero_string() -> Result<()>{
        let tested = SecureDelete::new("README.md")?.zero_name()?;
        assert_eq!("000000000", &tested );
        assert_ne!("0000000", &tested);

        let folder_test = SecureDelete::new("images/AFSSI_5020.png")?.zero_name()?;
        assert_eq!("00000000000000", &folder_test );
        Ok(())
    }
}