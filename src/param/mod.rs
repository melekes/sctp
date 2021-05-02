#[cfg(test)]
mod param_test;

pub(crate) mod param_chunk_list;
pub(crate) mod param_forward_tsn_supported;
pub(crate) mod param_header;
pub(crate) mod param_heartbeat_info;
pub(crate) mod param_outgoing_reset_request;
pub(crate) mod param_random;
pub(crate) mod param_reconfig_response;
pub(crate) mod param_requested_hmac_algorithm;
pub(crate) mod param_state_cookie;
pub(crate) mod param_supported_extensions;
pub(crate) mod param_type;

use crate::error::Error;
use crate::param::{
    param_chunk_list::ParamChunkList, param_forward_tsn_supported::ParamForwardTsnSupported,
    param_heartbeat_info::ParamHeartbeatInfo,
    param_outgoing_reset_request::ParamOutgoingResetRequest, param_random::ParamRandom,
    param_reconfig_response::ParamReconfigResponse,
    param_requested_hmac_algorithm::ParamRequestedHmacAlgorithm,
    param_state_cookie::ParamStateCookie, param_supported_extensions::ParamSupportedExtensions,
};
use param_header::*;
use param_type::*;

use bytes::{Buf, Bytes, BytesMut};

pub(crate) trait Param {
    fn unmarshal(raw: &Bytes) -> Result<Self, Error>
    where
        Self: Sized;
    fn marshal_to(&self, buf: &mut BytesMut) -> Result<usize, Error>;
    fn value_length(&self) -> usize;

    fn marshal(&self) -> Result<Bytes, Error> {
        let capacity = PARAM_HEADER_LENGTH + self.value_length();
        let mut buf = BytesMut::with_capacity(capacity);
        self.marshal_to(&mut buf)?;
        Ok(buf.freeze())
    }
}

pub(crate) fn build_param(raw_param: &Bytes) -> Result<Box<dyn Param>, Error> {
    if raw_param.len() < PARAM_HEADER_LENGTH {
        return Err(Error::ErrParamHeaderTooShort);
    }
    let reader = &mut raw_param.slice(..2);
    let t: ParamType = reader.get_u16().into();
    match t {
        ParamType::ForwardTsnSupp => Ok(Box::new(ParamForwardTsnSupported::unmarshal(raw_param)?)),
        ParamType::SupportedExt => Ok(Box::new(ParamSupportedExtensions::unmarshal(raw_param)?)),
        ParamType::Random => Ok(Box::new(ParamRandom::unmarshal(raw_param)?)),
        ParamType::ReqHmacAlgo => Ok(Box::new(ParamRequestedHmacAlgorithm::unmarshal(raw_param)?)),
        ParamType::ChunkList => Ok(Box::new(ParamChunkList::unmarshal(raw_param)?)),
        ParamType::StateCookie => Ok(Box::new(ParamStateCookie::unmarshal(raw_param)?)),
        ParamType::HeartbeatInfo => Ok(Box::new(ParamHeartbeatInfo::unmarshal(raw_param)?)),
        ParamType::OutSsnResetReq => Ok(Box::new(ParamOutgoingResetRequest::unmarshal(raw_param)?)),
        ParamType::ReconfigResp => Ok(Box::new(ParamReconfigResponse::unmarshal(raw_param)?)),
        _ => Err(Error::ErrParamTypeUnhandled),
    }
}
