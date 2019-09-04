pub mod current
{
    use super::super::prelude::*;
    pub fn get(api_serv_url: &str) -> Result<CurrentMachineStatistics, Error>
    {
        let req =
            isahc::http::Request::get(format!("{}/v2/machine/monitoring/current", api_serv_url))
                .header("Accept", "application/json")
                .body(())
                .map_err(|e| Error::HttpError(e))?;
        isahc::send(req)
            .map_err(|e| Error::IsahcError(e))?
            .body_mut()
            .json()
            .map_err(|e| Error::JsonError(e))
    }
}
