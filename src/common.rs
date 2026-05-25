use crate::config;
use anyhow::Ok;
use serde::{Deserialize, Serialize};
use url::Url;

pub async fn fetch_portal_url() -> anyhow::Result<Url> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let mut stream = TcpStream::connect("detectportal.firefox.com:80").await?;
    let request = "GET /canonical.html HTTP/1.1\r\n\
                   Host: detectportal.firefox.com\r\n\
                   User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0\r\n\
                   Connection: close\r\n\r\n";

    stream.write_all(request.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let re = regex::Regex::new(r"location\.href='([^']+)'")?;

    if let Some(caps) = re.captures(&response) {
        let url = caps[1].to_string();
        // let url = "http://10.255.254.2:8080/zportal/login?wlanuserip=cd4cb040cfc7f60d3c6684baa3f57550&wlanacname=5afa3fbba6198b4477a72818670bfb91&ssid=4bda2ee3911dfdd27bb5284aff515f5b&nasip=f390ae1967a8978d716c593f746d5fd1&snmpagentip=&mac=75d9adcafa44f3372ac8ad1c5bc70dee&t=wireless-v2&url=317d944be3fb27f02c85ea8993d39cd900b09c560c1b3889822434da2757a7e4b71c1f057a68fd2d0765a2b223ad6f23&apmac=&nasid=5afa3fbba6198b4477a72818670bfb91&vid=3ee93e9a72dfb2f3&port=bb9627c50cc1ec53&nasportid=fdbb2897d851dd8d4c71d05528f4ff82c2f7b805c17418959d1704e67071ee78";
        let url = Url::parse(url.clone().as_ref())?;
        Ok(url)
    } else {
        anyhow::bail!("未能解析到 URL:\n{}", response);
        // panic!("未解析到url")
    }

    // Ok(url)
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

        //  反序列化 query 参数到结构体
        let query: NetAuthorization = serde_urlencoded::from_str(url.query().unwrap())?;

        Ok(query)
        // query
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
    fn new(net_authorization: NetAuthorization, username: String, pwd: String) -> Self {
        Self {
            qr_code_id: "%E8%AF%B7%E8%BE%93%E5%85%A5%E7%BC%96%E5%8F%B7".to_string(),
            username: username.to_string(),
            pwd: pwd.to_string(),
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

    pub fn get_login_post_body() -> anyhow::Result<String> {
        let refer = "http://10.255.254.2:8080/zportal/login?wlanuserip=cd4cb040cfc7f60d3c6684baa3f57550&wlanacname=5afa3fbba6198b4477a72818670bfb91&ssid=4bda2ee3911dfdd27bb5284aff515f5b&nasip=f390ae1967a8978d716c593f746d5fd1&snmpagentip=&mac=75d9adcafa44f3372ac8ad1c5bc70dee&t=wireless-v2&url=317d944be3fb27f02c85ea8993d39cd900b09c560c1b3889822434da2757a7e4b71c1f057a68fd2d0765a2b223ad6f23&apmac=&nasid=5afa3fbba6198b4477a72818670bfb91&vid=3ee93e9a72dfb2f3&port=bb9627c50cc1ec53&nasportid=fdbb2897d851dd8d4c71d05528f4ff82c2f7b805c17418959d1704e67071ee78";
        // let refer = fetch_portal_url().await?;

        println!("{refer}");

        let url = Url::parse(refer.clone().as_ref()).unwrap();

        let host = format!("{}:{}", url.host().unwrap(), url.port().unwrap());
        println!("{host}");

        let path = format! {"{}/do",{url.path()}};

        println!("{path}");

        //  反序列化 query 参数到结构体
        let query: NetAuthorization = serde_urlencoded::from_str(url.query().unwrap()).unwrap();
        println!("{:#?}", query);

        let app_config = config::get().credentials();
        let username = app_config.username.clone().unwrap();
        let pwd = app_config.password.clone().unwrap();

        let b = LoginPostPaylod::new(query, username, pwd);

        let post_body = serde_urlencoded::to_string(&b)?;
        println!("{post_body}");

        Ok(post_body)
    }
}

#[derive(Debug, Serialize)]
pub struct LogoutPostPayload {
    #[serde(rename = "userName")]
    pub username: String,
    #[serde(rename = "userIp")]
    pub userip: String,
    #[serde(rename = "deviceIp")]
    pub device_ip: String,
    #[serde(rename = "service.id")]
    pub service_id: String,
    #[serde(rename = "autoLoginFlag")]
    pub auto_login_flag: bool,
    #[serde(rename = "userMac")]
    pub user_mac: String,
    #[serde(rename = "operationType")]
    pub operation_type: String,
    #[serde(rename = "isMacFastAuth")]
    pub is_mac_fast_auth: bool,
}

impl LogoutPostPayload {
    fn new(login_post_paylod: LoginPostPaylod) -> Self {
        Self {
            username: login_post_paylod.username,
            userip: "10.243.192.157".to_string(),
            device_ip: "10.255.254.254".to_string(),
            service_id: "".to_string(),
            auto_login_flag: false,
            user_mac: "f6901cdeecc7".to_string(),
            operation_type: "".to_string(),
            is_mac_fast_auth: false,
        }
    }
}
