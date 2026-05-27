use crate::common::{fetch_portal_url, LoginPostPaylod, NetAuthorization, SessionInfo};
use log::{debug, info};
use reqwest::{
    Client,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE, HOST, HeaderMap,
        ORIGIN, REFERER, USER_AGENT,
    },
};
use url::Url;

fn build_headers(url: &Url) -> HeaderMap {
    let host = format!("{}:{}", url.host().unwrap(), url.port().unwrap());

    let mut h = HeaderMap::new();
    h.insert(HOST, host.parse().unwrap());
    h.insert(
        USER_AGENT,
        "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0"
            .parse()
            .unwrap(),
    );
    h.insert(ACCEPT, "*/*".parse().unwrap());
    h.insert(
        ACCEPT_LANGUAGE,
        "zh-CN,zh;q=0.9,zh-TW;q=0.8,zh-HK;q=0.7,en-US;q=0.6,en;q=0.5"
            .parse()
            .unwrap(),
    );
    h.insert(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    h.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded; charset=UTF-8"
            .parse()
            .unwrap(),
    );
    h.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
    h.insert(ORIGIN, format!("http://{}", host).parse().unwrap());
    h.insert(CONNECTION, "keep-alive".parse().unwrap());
    h.insert(REFERER, url.as_str().parse().unwrap());
    h.insert("Priority", "u=0".parse().unwrap());
    h
}

const PATH: &str = "http://10.255.254.2:8080/zportal/login/do";

pub async fn login() -> anyhow::Result<()> {
    info!("正在登录中......");

    let portal_url = fetch_portal_url().await?;
    let net_auth = NetAuthorization::from_url(&portal_url)?;

    let rsp = Client::builder()
        .build()?
        .post(PATH)
        .headers(build_headers(&portal_url))
        .body(LoginPostPaylod::get_login_payload(&net_auth)?)
        .send()
        .await?;

    if rsp.status() == 200 {
        let cookie_headers = rsp.headers().get_all("set-cookie");
        let cookie_strs: Vec<String> = cookie_headers
            .iter()
            .filter_map(|v| v.to_str().ok().map(String::from))
            .collect();

        if let Some(session) = SessionInfo::from_login_response(&cookie_strs, &net_auth.mac) {
            session.save()?;
            info!("会话信息已保存，可执行 logout 下线");
        } else {
            info!("未检测到 userIndex cookie，不影响登录");
        }

        debug!("--- RESPONSE HEADERS ---");
        for (k, v) in rsp.headers() {
            debug!("{k:?} : {v:?}")
        }
        debug!("------------------------");

        let body = rsp.text().await?;
        info!("登录成功! {}", body);
    }
    Ok(())
}
