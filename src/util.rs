use super::*;

pub fn report_err<T, E: Display>(res: Result<T, E>) -> Result<T, ()> {
    match res {
        Ok(value) => Ok(value),
        Err(error) => {
            log::error!("Error: {error}");
            Err(())
        }
    }
}
