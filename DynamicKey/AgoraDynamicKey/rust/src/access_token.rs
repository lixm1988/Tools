use crate::utils;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::any::Any;
use std::collections::HashMap;
use std::io::{Error, Read, Write};
use std::panic;

pub const VERSION: &str = "007";
pub const VERSION_LENGTH: usize = 3;

// Service type
pub const SERVICE_TYPE_RTC: u16 = 1;
pub const SERVICE_TYPE_RTM: u16 = 2;
pub const SERVICE_TYPE_FPA: u16 = 4;
pub const SERVICE_TYPE_CHAT: u16 = 5;
pub const SERVICE_TYPE_APAAS: u16 = 7;

// Rtc
pub const PRIVILEGE_JOIN_CHANNEL: u16 = 1;
pub const PRIVILEGE_PUBLISH_AUDIO_STREAM: u16 = 2;
pub const PRIVILEGE_PUBLISH_VIDEO_STREAM: u16 = 3;
pub const PRIVILEGE_PUBLISH_DATA_STREAM: u16 = 4;

// Rtm
// Fpa
pub const PRIVILEGE_LOGIN: u16 = 1;

// Chat
pub const PRIVILEGE_CHAT_USER: u16 = 1;
pub const PRIVILEGE_CHAT_APP: u16 = 2;

// Apaas
pub const PRIVILEGE_APAAS_ROOM_USER: u16 = 1;
pub const PRIVILEGE_APAAS_USER: u16 = 2;
pub const PRIVILEGE_APAAS_APP: u16 = 3;

pub trait IService {
    fn as_any(&self) -> &dyn Any;
    fn get_service_type(&self) -> u16;
    fn pack(&self, writer: &mut dyn Write) -> Result<(), Error>;
    fn unpack(&mut self, reader: &mut dyn Read) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct Service {
    privileges: HashMap<u16, u32>,
    service_type: u16,
}

pub fn new_service(service_type: u16) -> Service {
    Service {
        privileges: HashMap::new(),
        service_type,
    }
}

impl Service {
    pub fn add_privilege(&mut self, privilege: u16, expire: u32) {
        self.privileges.insert(privilege, expire);
    }

    pub fn pack_privileges(&self, writer: &mut dyn Write) -> Result<(), Error> {
        utils::pack_map_uint32(writer, &self.privileges)
    }

    pub fn pack_type(&self, writer: &mut dyn Write) -> Result<(), Error> {
        utils::pack_uint16(writer, self.service_type)
    }
}

impl IService for Service {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_type(&self) -> u16 {
        self.service_type
    }

    fn pack(&self, writer: &mut dyn Write) -> Result<(), Error> {
        self.pack_type(writer)?;
        self.pack_privileges(writer)
    }

    fn unpack(&mut self, reader: &mut dyn Read) -> Result<(), Error> {
        self.privileges = utils::unpack_map_uint32(reader)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ServiceRtc {
    pub service: Service,
    channel_name: String,
    uid: String,
}

pub fn new_service_rtc(channel_name: &str, uid: &str) -> ServiceRtc {
    ServiceRtc {
        service: new_service(SERVICE_TYPE_RTC),
        channel_name: channel_name.to_string(),
        uid: uid.to_string(),
    }
}

impl IService for ServiceRtc {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_type(&self) -> u16 {
        self.service.service_type
    }

    fn pack(&self, w: &mut dyn Write) -> Result<(), Error> {
        self.service.pack(w)?;
        utils::pack_string(w, &self.channel_name)?;
        utils::pack_string(w, &self.uid)
    }

    fn unpack(&mut self, r: &mut dyn Read) -> Result<(), Error> {
        self.service.unpack(r)?;
        self.channel_name = utils::unpack_string(r)?;
        self.uid = utils::unpack_string(r)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ServiceRtm {
    pub service: Service,
    user_id: String,
}

pub fn new_service_rtm(user_id: &str) -> ServiceRtm {
    ServiceRtm {
        service: new_service(SERVICE_TYPE_RTM),
        user_id: user_id.to_string(),
    }
}

impl IService for ServiceRtm {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_type(&self) -> u16 {
        self.service.service_type
    }

    fn pack(&self, w: &mut dyn Write) -> Result<(), Error> {
        self.service.pack(w)?;
        utils::pack_string(w, &self.user_id)
    }

    fn unpack(&mut self, r: &mut dyn Read) -> Result<(), Error> {
        self.service.unpack(r)?;
        self.user_id = utils::unpack_string(r)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ServiceFpa {
    pub service: Service,
}

pub fn new_service_fpa() -> ServiceFpa {
    ServiceFpa {
        service: new_service(SERVICE_TYPE_FPA),
    }
}

impl IService for ServiceFpa {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_type(&self) -> u16 {
        self.service.service_type
    }

    fn pack(&self, w: &mut dyn Write) -> Result<(), Error> {
        self.service.pack(w)
    }

    fn unpack(&mut self, r: &mut dyn Read) -> Result<(), Error> {
        self.service.unpack(r)
    }
}

#[derive(Debug)]
pub struct ServiceChat {
    pub service: Service,
    user_id: String,
}

pub fn new_service_chat(user_id: &str) -> ServiceChat {
    ServiceChat {
        service: new_service(SERVICE_TYPE_CHAT),
        user_id: user_id.to_string(),
    }
}

impl IService for ServiceChat {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_type(&self) -> u16 {
        self.service.service_type
    }

    fn pack(&self, w: &mut dyn Write) -> Result<(), Error> {
        self.service.pack(w)?;
        utils::pack_string(w, &self.user_id)
    }

    fn unpack(&mut self, r: &mut dyn Read) -> Result<(), Error> {
        self.service.unpack(r)?;
        self.user_id = utils::unpack_string(r)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ServiceApaas {
    pub service: Service,
    room_uuid: String,
    user_uuid: String,
    role: i16,
}

pub fn new_service_apaas(room_uuid: &str, user_uuid: &str, role: i16) -> ServiceApaas {
    ServiceApaas {
        service: new_service(SERVICE_TYPE_APAAS),
        room_uuid: room_uuid.to_string(),
        user_uuid: user_uuid.to_string(),
        role,
    }
}

impl IService for ServiceApaas {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_service_type(&self) -> u16 {
        self.service.service_type
    }

    fn pack(&self, w: &mut dyn Write) -> Result<(), Error> {
        self.service.pack(w)?;
        utils::pack_string(w, &self.room_uuid)?;
        utils::pack_string(w, &self.user_uuid)?;
        utils::pack_int16(w, self.role)
    }

    fn unpack(&mut self, r: &mut dyn Read) -> Result<(), Error> {
        self.service.unpack(r)?;
        self.room_uuid = utils::unpack_string(r)?;
        self.user_uuid = utils::unpack_string(r)?;
        self.role = utils::unpack_int16(r)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AccessToken {
    app_cert: String,
    app_id: String,
    expire: u32,
    issue_ts: u32,
    salt: u32,
    services: HashMap<u16, Box<dyn IService>>,
}

pub fn new_access_token(app_id: &str, app_cert: &str, expire: u32) -> AccessToken {
    let issue_ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32;
    let salt = utils::get_rand(1, 99999999) as u32;

    return AccessToken {
        app_cert: app_cert.to_string(),
        app_id: app_id.to_string(),
        expire,
        issue_ts,
        salt,
        services: HashMap::new(),
    };
}

pub fn create_access_token() -> AccessToken {
    new_access_token("", "", 900)
}

impl std::fmt::Debug for dyn IService + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "service_type:{:?}", self.get_service_type())
    }
}

impl AccessToken {
    pub fn add_service(&mut self, service: Box<dyn IService>) {
        self.services.insert(service.get_service_type(), service);
    }

    pub fn build(&self) -> Result<String, Box<dyn std::error::Error>> {
        if !is_uuid(&self.app_id) || !is_uuid(&self.app_cert) {
            return Err("check appId or appCertificate".to_string().into());
        }

        let mut buf = Vec::new();
        utils::pack_string(&mut buf, &self.app_id)?;
        utils::pack_uint32(&mut buf, self.issue_ts)?;
        utils::pack_uint32(&mut buf, self.expire)?;
        utils::pack_uint32(&mut buf, self.salt)?;
        utils::pack_uint16(&mut buf, self.services.len() as u16)?;

        // Sign
        let sign = self.get_sign()?;

        // Pack services in definite order
        let service_types = vec![SERVICE_TYPE_RTC, SERVICE_TYPE_RTM, SERVICE_TYPE_FPA, SERVICE_TYPE_CHAT, SERVICE_TYPE_APAAS];
        for service_type in service_types {
            if let Some(service) = self.services.get(&service_type) {
                service.pack(&mut buf)?;
            }
        }

        // Signature
        let mut h_sign = Hmac::<Sha256>::new_from_slice(&sign)?;
        h_sign.update(&buf);
        let signature = h_sign.finalize().into_bytes();

        let mut buf_content = Vec::new();
        utils::pack_string(&mut buf_content, unsafe { &String::from_utf8_unchecked(signature.to_vec()) })?;

        buf_content.extend(&buf);

        let res = get_version() + &utils::base64_encode_str(&utils::compress_zlib(&buf_content));
        Ok(res)
    }

    pub fn parse(&mut self, token: &str) -> Result<bool, Error> {
        let version = &token[0..VERSION_LENGTH];
        if version != get_version() {
            return Ok(false);
        }

        let decode_byte = utils::base64_decode_str(&token[VERSION_LENGTH..]);
        let mut buffer = std::io::Cursor::new(utils::decompress_zlib(&decode_byte.unwrap()));

        // signature
        utils::unpack_string(&mut buffer).unwrap();

        self.app_id = utils::unpack_string(&mut buffer).unwrap();
        self.issue_ts = utils::unpack_uint32(&mut buffer).unwrap();
        self.expire = utils::unpack_uint32(&mut buffer).unwrap();
        self.salt = utils::unpack_uint32(&mut buffer).unwrap();

        let service_num: u16 = utils::unpack_uint16(&mut buffer).unwrap();
        let mut service_type: u16;

        for _ in 0..service_num {
            service_type = utils::unpack_uint16(&mut buffer).unwrap();
            let mut service = self.new_service(service_type);

            if let Err(e) = service.unpack(&mut buffer) {
                return Err(e);
            }
            self.services.insert(service_type, service);
        }

        Ok(true)
    }

    pub fn get_sign(&self) -> Result<Vec<u8>, Error> {
        // IssueTs
        let mut buf_issue_ts = Vec::new();
        utils::pack_uint32(&mut buf_issue_ts, self.issue_ts)?;
        let mut h_issue_ts = Hmac::<Sha256>::new_from_slice(&buf_issue_ts).expect("HMAC issue_ts error");
        h_issue_ts.update(self.app_cert.as_bytes());

        // Salt
        let mut buf_salt = Vec::new();
        utils::pack_uint32(&mut buf_salt, self.salt)?;
        let mut h_salt = Hmac::<Sha256>::new_from_slice(&buf_salt).expect("HMAC salt error");
        h_salt.update(h_issue_ts.finalize().into_bytes().as_slice());

        Ok(h_salt.finalize().into_bytes().to_vec())
    }

    pub fn new_service(&mut self, service_type: u16) -> Box<dyn IService> {
        match service_type {
            SERVICE_TYPE_RTC => Box::new(new_service_rtc("", "")),
            SERVICE_TYPE_RTM => Box::new(new_service_rtm("")),
            SERVICE_TYPE_FPA => Box::new(new_service_fpa()),
            SERVICE_TYPE_CHAT => Box::new(new_service_chat("")),
            SERVICE_TYPE_APAAS => Box::new(new_service_apaas("", "", -1)),
            _ => panic!("new service failed: unknown service type `{}`", service_type),
        }
    }
}

pub fn get_uid_str(uid: u32) -> String {
    if uid == 0 {
        return String::from("");
    }
    uid.to_string()
}

pub fn get_version() -> String {
    String::from(VERSION)
}

pub fn is_uuid(s: &str) -> bool {
    if s.len() != 32 {
        return false;
    }

    return s.chars().all(|c| c.is_digit(16));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_rtc() {
        let app_id: &str = "970CA35de60c44645bbae8a215061b33";
        let app_cert: &str = "5CFd2fd1755d40ecb72977518be15d3b";
        let channel_name: &str = "7d72365eb983485397e3e3f9d460bdda";
        let uid_str: &str = "2882341273";
        let expire: u32 = 600;
        let salt: u32 = 1;
        let ts: u32 = 1111111;

        let mut access_token = new_access_token(app_id, app_cert, expire);
        access_token.issue_ts = ts;
        access_token.salt = salt;

        let mut service_rtc = new_service_rtc(channel_name, uid_str);
        service_rtc.service.add_privilege(PRIVILEGE_JOIN_CHANNEL, expire);

        access_token.add_service(Box::new(service_rtc));

        assert_eq!("007eJwBigB1/yAAFqC4TFpegsv3T7gT0J9ZxUvaycBhIFgFOayXV46VixogADk3MENBMzVkZTYwYzQ0NjQ1YmJhZThhMjE1MDYxYjMzR/QQAFgCAAABAAAAAQABAAEAAQBYAgAAIAA3ZDcyMzY1ZWI5ODM0ODUzOTdlM2UzZjlkNDYwYmRkYQoAMjg4MjM0MTI3M8JqJOM=", access_token.build().unwrap());
    }

    #[test]
    fn test_parse() {
        let app_id: &str = "970CA35de60c44645bbae8a215061b33";
        let channel_name: &str = "7d72365eb983485397e3e3f9d460bdda";
        let uid_str: &str = "2882341273";
        let expire: u32 = 600;
        let salt: u32 = 1;
        let ts: u32 = 1111111;

        let mut access_token = create_access_token();
        let token = "007eJwBigB1/yAAFqC4TFpegsv3T7gT0J9ZxUvaycBhIFgFOayXV46VixogADk3MENBMzVkZTYwYzQ0NjQ1YmJhZThhMjE1MDYxYjMzR/QQAFgCAAABAAAAAQABAAEAAQBYAgAAIAA3ZDcyMzY1ZWI5ODM0ODUzOTdlM2UzZjlkNDYwYmRkYQoAMjg4MjM0MTI3M8JqJOM=";

        let res = access_token.parse(token).unwrap();
        let service_rtc: &Box<dyn IService> = access_token.services.get(&SERVICE_TYPE_RTC).unwrap();

        assert_eq!(true, res);
        assert_eq!(app_id, access_token.app_id);
        assert_eq!(expire, access_token.expire);
        assert_eq!(salt, access_token.salt);
        assert_eq!(ts, access_token.issue_ts);
        assert_eq!(SERVICE_TYPE_RTC, service_rtc.get_service_type());

        if let Some(service_rtc_downcast) = service_rtc.as_any().downcast_ref::<ServiceRtc>() {
            assert_eq!(channel_name, service_rtc_downcast.channel_name);
            assert_eq!(uid_str, service_rtc_downcast.uid);
        }
    }
}
