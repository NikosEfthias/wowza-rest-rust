use super::prelude::*;
use isahc::http::Request;
pub fn start_video_recording(
    api_serv_url: &str,
    cfg: &StreamRecorderConfig,
) -> Result<super::types::ResponseData, super::types::Error>
{
    let addr = format!(
        "{}/v2/servers/{}/vhosts/{}/applications/{}/instances/{}/streamrecorders",
        api_serv_url, cfg.serverName, cfg.vhostName, cfg.applicationName, cfg.instanceName
    );
    let mut req = Request::post(addr);
    default_headers().iter().for_each(|&(k, v)| {
        req.header(k, v);
    });
    let req = req
        .body(serde_json::to_string(cfg).unwrap())
        .map_err(|e| Error::HttpError(e))?;
    let mut resp = isahc::send(req).map_err(|e| Error::IsahcError(e))?;
    resp.body_mut().json().map_err(|e| Error::JsonError(e))
}
pub fn stop_video_recording(
    api_serv_url: &str,
    cfg: &StreamRecorderConfig,
) -> Result<super::types::ResponseData, Error>
{
    let addr = format!(
        "{}/v2/servers/{}/vhosts/{}/applications/{}/instances/{}/streamrecorders/{}/actions/stopRecording",
        api_serv_url, cfg.serverName, cfg.vhostName, cfg.applicationName, cfg.instanceName,cfg.recorderName
    );
    let req = Request::put(addr)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(())
        .map_err(|e| Error::HttpError(e))?;
    isahc::send(req)
        .map_err(|e| Error::IsahcError(e))?
        .body_mut()
        .json()
        .map_err(|e| Error::JsonError(e))
}
