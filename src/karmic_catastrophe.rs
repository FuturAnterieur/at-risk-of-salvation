use std::error::Error;

pub struct KarmicCatastrophe
{
    pub message: String
}

impl Error for KarmicCatastrophe {   }

impl std::fmt::Debug for KarmicCatastrophe 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({})", self.message)
    }
}

impl std::fmt::Display for KarmicCatastrophe
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({})", self.message)
    }
}
