use crate::common::LoginPostPaylod;
use crate::common::fetch_portal_url;
use log::info;
use reqwest::{
    Client,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE, HOST, HeaderMap,
        ORIGIN, REFERER, USER_AGENT,
    },
};

pub async fn headers() -> HeaderMap {
    let url = fetch_portal_url().await.unwrap();
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

    h.insert(REFERER, url.to_string().parse().unwrap());
    // h.insert(
    //     COOKIE,
    //     "JSESSIONID=E18205121D5F677E316360238A256BEB; failCounter=0"
    //         .parse()
    //         .unwrap(),
    // );
    h.insert("Priority", "u=0".parse().unwrap());
    h
}

pub const PATH: &str = "http://10.255.254.2:8080/zportal/login/do";

pub async fn login() -> anyhow::Result<()> {
    info!("正在登录中......");

    let rsp = Client::builder()
        .build()?
        .post(PATH)
        .headers(headers().await)
        .body(LoginPostPaylod::get_login_payload().await?)
        .send()
        .await?;

    if rsp.status() == 200 {
        info!(
            "----------------------RESPONSE HEADER--------------------------------------------------------------"
        );

        for (k, v) in rsp.headers() {
            info!("{k:?} : {v:?}")
        }
        info!(
            "----------------------------------------------------------------------------------------------------"
        );

        info!("登录成功啦!");
        info!("{}", rsp.text().await?);
    }
    Ok(())
}
