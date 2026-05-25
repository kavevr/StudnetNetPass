use log::info;
use reqwest::{
    Client,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE, HOST,
        HeaderMap, ORIGIN, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
    },
};

pub fn headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(HOST, "10.255.254.2:8080".parse().unwrap());
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
    h.insert(CONNECTION, "keep-alive".parse().unwrap());
    h.insert(
        REFERER,
        "http://10.255.254.2:8080/zportal/goToAuthResult"
            .parse()
            .unwrap(),
    );
    // h.insert(COOKIE, r#"JSESSIONID=E18205121D5F677E316360238A256BEB; failCounter=0; userIndex="10.255.254.254,10.243.192.157,202300648""#.parse().unwrap());
    h.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
    h.insert("Priority", "u=0, i".parse().unwrap());
    h
}

pub const PATH: &str = "http://10.255.254.2:8080/zportal/logout";

pub async fn logout() -> anyhow::Result<()> {

    // let app_config = config::get().credentials();
    // let username = &app_config.username;

   let payload =  "userName=202300648&userIp=10.243.192.199&deviceIp=10.255.254.254&service.id=&autoLoginFlag=false&userMac=f6901cdeecc7&operationType=&isMacFastAuth=false";


    let rsp = Client::builder()
        .build()?
        .post(PATH)
        .headers(headers())
        .body(payload)
        .send()
        .await?;

    if rsp.status() == 200 {
        info!("----------------------RESPONSE HEADER--------------------------------------------------------------");

        for (k, v) in rsp.headers() {
            info!("{:?}: {:?}", k, v);
        }
        info!("----------------------------------------------------------------------------------------------------");

        info!(
            "Body Length: [{}]",
            rsp.content_length().expect("未能获取到内容.")
        );
        info!("下线啦!")
    }
    Ok(())
}
