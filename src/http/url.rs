use std::str::FromStr;

#[derive(Debug)]
pub struct Url {
    pub protocol: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
}
impl FromStr for Url {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the URL string into parts
        let mut parts = s.split("://");

        // get the protocol
        let protocol = parts.next().ok_or("Missing protocol")?.to_string();
        let rest = parts.next().ok_or("rest of url after protocol missing")?;

        // Split the rest into username, password, host, and path
        let mut url_parts = rest.split('@');
        let (user_info, host_path) = match url_parts.clone().count() {
            1 => (None, url_parts.next().expect("url_parts has length of 1")),
            2 => (
                url_parts.next(),
                url_parts.next().expect("url_parts has length of 2"),
            ),
            _ => Err("Invalid URL format")?,
        };

        let mut user_info_parts = user_info.unwrap_or_default().split(':');
        let username = user_info_parts.next().and_then(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        });
        let password = user_info_parts.next().and_then(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.to_string())
            }
        });

        // Split the host_port_path into host_port and path
        let mut host_path_parts = host_path.splitn(2, '/');
        let host_port = host_path_parts.next().ok_or("Missing host:port")?;
        let path = host_path_parts.next().unwrap_or(&"").to_string();

        // Parse host and port
        let mut host_port_parts = host_port.split(':');
        let host = host_port_parts.next().ok_or("Missing host")?.to_string();
        let port = host_port_parts
            .next()
            .and_then(|port_str| port_str.parse::<u16>().ok());

        // Build the Url struct and return the result
        return Ok(Url {
            protocol,
            username,
            password,
            host,
            path,
            port, // Add your logic for extracting the port
        });
    }
}

#[test]
fn parse_url() {
    let mut output = String::new();

    const URLS: &[&str] = &[
        "http://www.youtube.com",
        "http://www.facebook.com",
        "http://www.baidu.com",
        "http://www.yahoo.com",
        "http://www.amazon.com",
        "http://www.wikipedia.org",
        "http://www.qq.com",
        "http://www.google.co.in",
        "http://www.twitter.com",
        "http://www.live.com",
        "http://www.taobao.com",
        "http://www.bing.com",
        "http://www.instagram.com",
        "http://www.weibo.com",
        "http://www.sina.com.cn",
        "http://www.linkedin.com",
        "http://www.yahoo.co.jp",
        "http://www.msn.com",
        "http://www.vk.com",
        "http://www.google.de",
        "http://www.yandex.ru",
        "http://www.hao123.com",
        "http://www.google.co.uk",
        "http://www.reddit.com",
        "http://www.ebay.com",
        "http://www.google.fr",
        "http://www.t.co",
        "http://www.tmall.com",
        "http://www.google.com.br",
        "http://www.360.cn",
        "http://www.sohu.com",
        "http://www.amazon.co.jp",
        "http://www.pinterest.com",
        "http://www.netflix.com",
        "http://www.google.it",
        "http://www.google.ru",
        "http://www.microsoft.com",
        "http://www.google.es",
        "http://www.wordpress.com",
        "http://www.gmw.cn",
        "http://www.tumblr.com",
        "http://www.paypal.com",
        "http://www.blogspot.com",
        "http://www.imgur.com",
        "http://www.stackoverflow.com",
        "http://www.aliexpress.com",
        "http://www.naver.com",
        "http://www.ok.ru",
        "http://www.apple.com",
        "http://www.github.com",
        "http://www.chinadaily.com.cn",
        "http://www.imdb.com",
        "http://www.google.co.kr",
        "http://www.fc2.com",
        "http://www.jd.com",
        "http://www.blogger.com",
        "http://www.163.com",
        "http://www.google.ca",
        "http://www.whatsapp.com",
        "http://www.amazon.in",
        "http://www.office.com",
        "http://www.tianya.cn",
        "http://www.google.co.id",
        "http://www.youku.com",
        "http://www.rakuten.co.jp",
        "http://www.craigslist.org",
        "http://www.amazon.de",
        "http://www.nicovideo.jp",
        "http://www.google.pl",
        "http://www.soso.com",
        "http://www.bilibili.com",
        "http://www.dropbox.com",
        "http://www.xinhuanet.com",
        "http://www.outbrain.com",
        "http://www.pixnet.net",
        "http://www.alibaba.com",
        "http://www.alipay.com",
        "http://www.microsoftonline.com",
        "http://www.booking.com",
        "http://www.googleusercontent.com",
        "http://www.google.com.au",
        "http://www.popads.net",
        "http://www.cntv.cn",
        "http://www.zhihu.com",
        "http://www.amazon.co.uk",
        "http://www.diply.com",
        "http://www.coccoc.com",
        "http://www.cnn.com",
        "http://www.bbc.co.uk",
        "http://www.twitch.tv",
        "http://www.wikia.com",
        "http://www.google.co.th",
        "http://www.go.com",
        "http://www.google.com.ph",
        "http://www.doubleclick.net",
        "http://www.onet.pl",
        "http://www.googleadservices.com",
        "http://www.accuweather.com",
        "http://www.googleweblight.com",
        "http://www.answers.yahoo.com",
        "https://www.example.com",
        "ftp://user:pass@ftp.example.com",
        "http://localhost:8080/path/to/resource",
        "https://sub.domain.example.org:8443",
        "file:///path/to/file.txt",
        "git+ssh://git@github.com/user/repo.git",
        "https://user:password@www.example.com:8080/path/to/resource",
        "sftp://user@ssh.example.com:2222/home/user",
        "ws://websocket.example.com",
        "wss://secure.websocket.example.org",
        "http://google.com",
        "http://localhost:8080/test/project/",
        "http://mail.yahoo.com",
        "http://www.bing.com",
        "http://www.phpromania.net/forum/viewtopic.php?f=24&t=7549",
        "https://prodgame10.alliances.commandandconquer.com/12/index.aspx",
        "https://prodgame10.alliances.commandandconquer.ro/12/index.aspx",
    ];

    URLS.into_iter().for_each(|url| {
        output.push_str(format!("{}\n{:?}\n\n", url, url.parse::<Url>().unwrap()).as_str())
    });

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./test/parse_url_output.txt")
        .unwrap();
    std::io::Write::write_all(&mut file, output.as_bytes()).unwrap();
}
