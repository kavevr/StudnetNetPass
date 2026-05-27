use crate::config;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;
use url::Url;

pub async fn fetch_portal_url() -> anyhow::Result<Url> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let mut stream = time::timeout(
        Duration::from_secs(3),
        TcpStream::connect("10.255.254.2:8080"),
    )
    .await?
    .map_err(|_| anyhow::anyhow!("登录超时, 请检查网络连接"))?;

    let request = "GET /zportal/notice/html HTTP/1.1\r\n\
                   Host: 10.255.254.2:8080\r\n\
                   User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0\r\n\
                   Connection: close\r\n\r\n";

    stream.write_all(request.as_bytes()).await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    let re = regex::Regex::new(r#"href="(http://10\.255\.254\.2[^"]*)"#)?;

    if let Some(caps) = re.captures(&response) {
        let url = Url::parse(&caps[1])?;
        Ok(url)
    } else {
        error!("没有找到URL, 请检查网络连接.");
        anyhow::bail!("没有找到URL, 请检查网络连接.")
    }
}

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
    pub fn from_url(url: &Url) -> anyhow::Result<Self> {
        let query: NetAuthorization =
            serde_urlencoded::from_str(url.query().unwrap_or_default())?;
        Ok(query)
    }
}

/// Session info persisted from login, used by logout.
#[derive(Debug)]
pub struct SessionInfo {
    pub device_ip: String,
    pub user_ip: String,
    pub user_mac: String,
    pub cookie: String,
}

impl SessionInfo {
    const SESSION_FILE: &str = ".session";

    /// `raw_cookies` is each raw Set-Cookie header value.
    /// We extract userIndex values and build a Cookie header string.
    pub fn from_login_response(raw_cookies: &[String], mac: &str) -> Option<Self> {
        let mut device_ip = String::new();
        let mut user_ip = String::new();
        let mut cookie_parts: Vec<&str> = Vec::new();

        for val in raw_cookies {
            // Extract name=value (before first ';')
            let nv = val.split(';').next().unwrap_or("");

            if nv.starts_with("userIndex=") {
                cookie_parts.push(nv);
                // Parse the three-part value
                if let Some(start) = nv.find('"') {
                    let rest = &nv[start + 1..];
                    if let Some(end) = rest.find('"') {
                        let parts: Vec<&str> = rest[..end].split(',').collect();
                        if parts.len() >= 3 {
                            device_ip = parts[0].to_string();
                            user_ip = parts[1].to_string();
                        }
                    }
                }
            } else if nv.starts_with("JSESSIONID=") {
                cookie_parts.push(nv);
            }
        }

        if !user_ip.is_empty() {
            Some(SessionInfo {
                device_ip,
                user_ip,
                user_mac: mac.to_string(),
                cookie: cookie_parts.join("; "),
            })
        } else {
            None
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = format!(
            "{}\n{}\n{}\n{}\n",
            self.device_ip, self.user_ip, self.user_mac, self.cookie
        );
        std::fs::write(Self::SESSION_FILE, content)?;
        debug!("Session saved to {}", Self::SESSION_FILE);
        Ok(())
    }

    pub fn load() -> anyhow::Result<Self> {
        if !std::path::Path::new(Self::SESSION_FILE).exists() {
            anyhow::bail!("未找到会话文件 .session，请先执行 login");
        }
        let content = std::fs::read_to_string(Self::SESSION_FILE)?;
        let lines: Vec<&str> = content.lines().collect();

        Ok(match lines.len() {
            4 => SessionInfo {
                device_ip: lines[0].to_string(),
                user_ip: lines[1].to_string(),
                user_mac: lines[2].to_string(),
                cookie: lines[3].to_string(),
            },
            3 => SessionInfo {
                device_ip: lines[0].to_string(),
                user_ip: lines[1].to_string(),
                user_mac: lines[2].to_string(),
                cookie: String::new(),
            },
            _ => anyhow::bail!("会话文件 .session 已损坏"),
        })
    }
}

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
    pub fn new(net_authorization: &NetAuthorization, username: String, pwd: String) -> Self {
        Self {
            qr_code_id: "%E8%AF%B7%E8%BE%93%E5%85%A5%E7%BC%96%E5%8F%B7".to_string(),
            username,
            pwd,
            valid_code: "%E9%AA%8C%E8%AF%81%E7%A0%81".to_string(),
            valid_code_flag: false,
            ssid: net_authorization.ssid.clone(),
            mac: net_authorization.mac.clone(),
            t: net_authorization.t.clone(),
            wlanacname: net_authorization.wlanacname.clone(),
            url: net_authorization.url.clone(),
            nasip: net_authorization.nasip.clone(),
            wlanuserip: net_authorization.wlanuserip.clone(),
        }
    }

    pub fn get_login_payload(net_auth: &NetAuthorization) -> anyhow::Result<String> {
        debug!("{:#?}", net_auth);
        let app_config = config::get().credentials();
        let username = app_config
            .username
            .clone()
            .ok_or_else(|| anyhow::anyhow!("配置文件中缺少 username"))?;
        let pwd = app_config
            .password
            .clone()
            .ok_or_else(|| anyhow::anyhow!("配置文件中缺少 password"))?;

        let b = LoginPostPaylod::new(net_auth, username, pwd);

        let post_body = serde_urlencoded::to_string(&b)?;
        debug!("Login post payload: {}", post_body);
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
    pub fn build(username: String, session: &SessionInfo) -> Self {
        Self {
            username,
            userip: session.user_ip.clone(),
            device_ip: session.device_ip.clone(),
            service_id: String::new(),
            auto_login_flag: false,
            user_mac: session.user_mac.clone(),
            operation_type: String::new(),
            is_mac_fast_auth: false,
        }
    }

    pub fn get_logout_payload(username: String, session: &SessionInfo) -> anyhow::Result<String> {
        let payload = LogoutPostPayload::build(username, session);
        let body = serde_urlencoded::to_string(&payload)?;
        debug!("Logout post payload: {}", body);
        Ok(body)
    }
}
