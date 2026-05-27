use crate::config;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::{process::exit, time::Duration};
use tokio::time;
use url::Url;

pub async fn fetch_portal_url() -> anyhow::Result<Url> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    // 在未认证的情况下通过外部域名自动重定向到认证页面（可能会失败）

    // let mut stream = TcpStream::connect("detectportal.firefox.com:80").await?;
    // let request = "GET /canonical.html HTTP/1.1\r\n\
    //                Host: detectportal.firefox.com\r\n\
    //                User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0\r\n\
    //                Connection: close\r\n\r\n";

    let mut stream = time::timeout(
        Duration::from_secs(3),
        TcpStream::connect("10.255.254.2:8080"),
    )
    .await?
    .expect("登录超时,请检查网络连接");
    let request = "GET /zportal/notice/html HTTP/1.1\r\n\
                   Host: 10.255.254.2:8080\r\n\
                   User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0\r\n\
                   Connection: close\r\n\r\n";

    stream.write_all(request.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let re = regex::Regex::new(r#"href="(http://10\.255\.254\.2[^"]*)"#)?;

    if let Some(caps) = re.captures(&response) {
        let url = caps[1].to_string();
        let url = Url::parse(url.clone().as_ref())?;

        Ok(url)
    } else {
        error!("没有找到URL, 请检查本地是否开启了代理服务器或者已经登录过了.");
        exit(1)
    }
}

// 网络认证结构
#[derive(Debug, Serialize, Deserialize)]
pub struct NetAuthorization {
    pub wlanuserip: String,
    pub wlanacname: String,
    pub ssid: String,
    pub nasip: String,
    pub snmpagentip: String,
    pub mac: String,
    pub t: String,
    pub url: String,
    pub apmac: String,
    pub nasid: String,
    pub vid: String,
    pub port: String,
    pub nasportid: String,
}

impl NetAuthorization {
    pub async fn new() -> anyhow::Result<Self> {
        
        let url = fetch_portal_url().await?;
        let query: NetAuthorization = serde_urlencoded::from_str(url.query().unwrap())?;
        Ok(query)
    }
}

// 构造post请求体
#[derive(Debug, Serialize)]
pub struct LoginPostPaylod {
    #[serde(rename = "qrCodeId")]
    pub qr_code_id: String,
    pub username: String,
    pub pwd: String,
    #[serde(rename = "validCode")]
    pub valid_code: String,
    #[serde(rename = "validCodeFlag")]
    pub valid_code_flag: bool,
    pub ssid: String,
    pub mac: String,
    pub t: String,
    pub wlanacname: String,
    pub url: String,
    pub nasip: String,
    pub wlanuserip: String,
}

impl LoginPostPaylod {
    pub fn new(net_authorization: NetAuthorization, username: String, pwd: Option<String>) -> Self {
        Self {
            qr_code_id: "%E8%AF%B7%E8%BE%93%E5%85%A5%E7%BC%96%E5%8F%B7".to_string(),
            username: username.to_string(),
            pwd: pwd.unwrap().to_string(),
            valid_code: "%E9%AA%8C%E8%AF%81%E7%A0%81".to_string(),
            valid_code_flag: false,
            ssid: net_authorization.ssid,
            mac: net_authorization.mac,
            t: net_authorization.t,
            wlanacname: net_authorization.wlanacname,
            url: net_authorization.url,
            nasip: net_authorization.nasip,
            wlanuserip: net_authorization.wlanuserip,
        }
    }

    pub async fn get_login_payload() -> anyhow::Result<String> {
        let query: NetAuthorization = NetAuthorization::new().await?;
        debug!("{:#?}", query);
        let app_config = config::get().credentials();
        let username = app_config.username.clone().unwrap();
        let pwd = app_config.password.clone().unwrap();

        let b = LoginPostPaylod::new(query, username, pwd.into());

        let post_body = serde_urlencoded::to_string(&b)?;

        debug!("Login post payload: {}", post_body);

        Ok(post_body)
    }
}

// #[derive(Debug, Serialize)]
// pub struct LogoutPostPayload {
//     #[serde(rename = "userName")]
//     pub username: String,
//     #[serde(rename = "userIp")]
//     pub userip: String,
//     #[serde(rename = "deviceIp")]
//     pub device_ip: String,
//     #[serde(rename = "service.id")]
//     pub service_id: String,
//     #[serde(rename = "autoLoginFlag")]
//     pub auto_login_flag: bool,
//     #[serde(rename = "userMac")]
//     pub user_mac: String,
//     #[serde(rename = "operationType")]
//     pub operation_type: String,
//     #[serde(rename = "isMacFastAuth")]
//     pub is_mac_fast_auth: bool,
// }

// impl LogoutPostPayload {
//     fn new() -> Self {
//         Self {
//             username: ,
//             userip: "".to_string(),
//             device_ip: "".to_string(),
//             service_id: "".to_string(),
//             auto_login_flag: false,
//             user_mac: "".to_string(),
//             operation_type: "".to_string(),
//             is_mac_fast_auth: false,
//         }
