use crate::{common::SessionInfo, config};
use log::{debug, info};
use reqwest::{
    Client,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_TYPE, HeaderMap, ORIGIN, REFERER,
        UPGRADE_INSECURE_REQUESTS, USER_AGENT,
    },
};

fn headers(cookie: &str) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(reqwest::header::COOKIE, cookie.parse().unwrap());
    h.insert(
        USER_AGENT,
        "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0"
            .parse()
            .unwrap(),
    );
    h.insert(
        ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    h.insert(
        ACCEPT_LANGUAGE,
        "zh-CN,zh;q=0.9,zh-TW;q=0.8,zh-HK;q=0.7,en-US;q=0.6,en;q=0.5"
            .parse()
            .unwrap(),
    );
    h.insert(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    h.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    h.insert(ORIGIN, "http://10.255.254.2:8080".parse().unwrap());
    h.insert(
        REFERER,
        "http://10.255.254.2:8080/zportal/goToAuthResult"
            .parse()
            .unwrap(),
    );
    h.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    h.insert("Priority", "u=0, i".parse().unwrap());
    h
}

const PATH: &str = "http://10.255.254.2:8080/zportal/logout";

pub async fn logout() -> anyhow::Result<()> {
    info!("正在注销下线......");

    let session = SessionInfo::load()?;
    let username = config::get()
        .credentials()
        .username
        .clone()
        .ok_or_else(|| anyhow::anyhow!("配置文件中缺少 username"))?;

    let payload = crate::common::LogoutPostPayload::get_logout_payload(username, &session)?;

    let rsp = Client::builder()
        .build()?
        .post(PATH)
        .headers(headers(&session.cookie))
        .body(payload)
        .send()
        .await?;

    if rsp.status() == 200 {
        debug!("--- RESPONSE HEADERS ---");
        for (k, v) in rsp.headers() {
            debug!("{:?}: {:?}", k, v);
        }
        debug!("------------------------");

        let body = rsp.text().await?;
        debug!("Body length: [{}]", body.len());
        debug!("Response body: {}", body);

        let _ = std::fs::remove_file(".session");
        info!("已下线!")
    }
    Ok(())
}
