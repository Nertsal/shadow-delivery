use super::*;

pub fn report_err<T, E: Display>(res: Result<T, E>) -> Result<T, ()> {
    match res {
        Ok(value) => Ok(value),
        Err(error) => {
            log::error!("{error}");
            Err(())
        }
    }
}

pub fn report_warn<T, E: Display>(res: Result<T, E>, msg: impl Display) -> Result<T, ()> {
    match res {
        Ok(value) => Ok(value),
        Err(error) => {
            log::warn!("{msg}: {error}");
            Err(())
        }
    }
}

pub fn smooth_step<F: Float>(t: F) -> F {
    let two = F::ONE + F::ONE;
    let three = two + F::ONE;
    three * t * t - two * t * t * t
}
