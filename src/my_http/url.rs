use color_eyre::{eyre::eyre, Report};
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
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the URL string into parts
        let mut parts = s.split("://");

        // get the protocol
        let protocol = parts.next().ok_or(eyre!("Missing protocol"))?.to_string();
        let rest = parts
            .next()
            .ok_or(eyre!("rest of url after protocol missing"))?;

        // Split the rest into username, password, host, and path
        let mut url_parts = rest.split('@');
        let (user_info, host_path) = match url_parts.clone().count() {
            1 => (None, url_parts.next().expect("url_parts has length of 1")),
            2 => (
                url_parts.next(),
                url_parts.next().expect("url_parts has length of 2"),
            ),
            _ => Err(eyre!("Invalid URL format"))?,
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
        let host_port = host_path_parts.next().ok_or(eyre!("Missing host:port"))?;
        let path = host_path_parts
            .next()
            .unwrap_or(&"")
            .parse()
            .expect("Infallible error");

        // Parse host and port
        let mut host_port_parts = host_port.split(':');
        let host = host_port_parts
            .next()
            .ok_or(eyre!("Missing host"))?
            .to_string();
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

/// [W3Schools url encode](https://www.w3schools.com/tags/ref_urlencode.ASP)
pub fn safe_character(character: char) -> Option<&'static str> {
    return match character {
        ' ' => Some("%20"),
        '!' => Some("%21"),
        '"' => Some("%22"),
        '#' => Some("%23"),
        '$' => Some("%24"),
        '%' => Some("%25"),
        '&' => Some("%26"),
        '\'' => Some("%27"),
        '(' => Some("%28"),
        ')' => Some("%29"),
        '*' => Some("%2A"),
        '+' => Some("%2B"),
        ',' => Some("%2C"),
        '-' => Some("%2D"),
        '.' => Some("%2E"),
        '/' => Some("%2F"),
        '0' => Some("%30"),
        '1' => Some("%31"),
        '2' => Some("%32"),
        '3' => Some("%33"),
        '4' => Some("%34"),
        '5' => Some("%35"),
        '6' => Some("%36"),
        '7' => Some("%37"),
        '8' => Some("%38"),
        '9' => Some("%39"),
        ':' => Some("%3A"),
        ';' => Some("%3B"),
        '<' => Some("%3C"),
        '=' => Some("%3D"),
        '>' => Some("%3E"),
        '?' => Some("%3F"),
        '@' => Some("%40"),
        'A' => Some("%41"),
        'B' => Some("%42"),
        'C' => Some("%43"),
        'D' => Some("%44"),
        'E' => Some("%45"),
        'F' => Some("%46"),
        'G' => Some("%47"),
        'H' => Some("%48"),
        'I' => Some("%49"),
        'J' => Some("%4A"),
        'K' => Some("%4B"),
        'L' => Some("%4C"),
        'M' => Some("%4D"),
        'N' => Some("%4E"),
        'O' => Some("%4F"),
        'P' => Some("%50"),
        'Q' => Some("%51"),
        'R' => Some("%52"),
        'S' => Some("%53"),
        'T' => Some("%54"),
        'U' => Some("%55"),
        'V' => Some("%56"),
        'W' => Some("%57"),
        'X' => Some("%58"),
        'Y' => Some("%59"),
        'Z' => Some("%5A"),
        '[' => Some("%5B"),
        '\\' => Some("%5C"),
        ']' => Some("%5D"),
        '^' => Some("%5E"),
        '_' => Some("%5F"),
        '`' => Some("%60"),
        'a' => Some("%61"),
        'b' => Some("%62"),
        'c' => Some("%63"),
        'd' => Some("%64"),
        'e' => Some("%65"),
        'f' => Some("%66"),
        'g' => Some("%67"),
        'h' => Some("%68"),
        'i' => Some("%69"),
        'j' => Some("%6A"),
        'k' => Some("%6B"),
        'l' => Some("%6C"),
        'm' => Some("%6D"),
        'n' => Some("%6E"),
        'o' => Some("%6F"),
        'p' => Some("%70"),
        'q' => Some("%71"),
        'r' => Some("%72"),
        's' => Some("%73"),
        't' => Some("%74"),
        'u' => Some("%75"),
        'v' => Some("%76"),
        'w' => Some("%77"),
        'x' => Some("%78"),
        'y' => Some("%79"),
        'z' => Some("%7A"),
        '{' => Some("%7B"),
        '|' => Some("%7C"),
        '}' => Some("%7D"),
        '~' => Some("%7E"),
        ' ' => Some("%7F"),
        '€' => Some("%E2%82%AC"),
        '' => Some("%81"),
        '‚' => Some("%E2%80%9A"),
        'ƒ' => Some("%C6%92"),
        '„' => Some("%E2%80%9E"),
        '…' => Some("%E2%80%A6"),
        '†' => Some("%E2%80%A0"),
        '‡' => Some("%E2%80%A1"),
        'ˆ' => Some("%CB%86"),
        '‰' => Some("%E2%80%B0"),
        'Š' => Some("%C5%A0"),
        '‹' => Some("%E2%80%B9"),
        'Œ' => Some("%C5%92"),
        '' => Some("%C5%8D"),
        'Ž' => Some("%C5%BD"),
        '' => Some("%8F"),
        '' => Some("%C2%90"),
        '‘' => Some("%E2%80%98"),
        '’' => Some("%E2%80%99"),
        '“' => Some("%E2%80%9C"),
        '”' => Some("%E2%80%9D"),
        '•' => Some("%E2%80%A2"),
        '–' => Some("%E2%80%93"),
        '—' => Some("%E2%80%94"),
        '˜' => Some("%CB%9C"),
        '™' => Some("%E2%84"),
        'š' => Some("%C5%A1"),
        '›' => Some("%E2%80"),
        'œ' => Some("%C5%93"),
        '' => Some("%9D"),
        'ž' => Some("%C5%BE"),
        'Ÿ' => Some("%C5%B8"),
        ' ' => Some("%C2%A0"),
        '¡' => Some("%C2%A1"),
        '¢' => Some("%C2%A2"),
        '£' => Some("%C2%A3"),
        '¤' => Some("%C2%A4"),
        '¥' => Some("%C2%A5"),
        '¦' => Some("%C2%A6"),
        '§' => Some("%C2%A7"),
        '¨' => Some("%C2%A8"),
        '©' => Some("%C2%A9"),
        'ª' => Some("%C2%AA"),
        '«' => Some("%C2%AB"),
        '¬' => Some("%C2%AC"),
        '­' => Some("%C2%AD"),
        '®' => Some("%C2%AE"),
        '¯' => Some("%C2%AF"),
        '°' => Some("%C2%B0"),
        '±' => Some("%C2%B1"),
        '²' => Some("%C2%B2"),
        '³' => Some("%C2%B3"),
        '´' => Some("%C2%B4"),
        'µ' => Some("%C2%B5"),
        '¶' => Some("%C2%B6"),
        '·' => Some("%C2%B7"),
        '¸' => Some("%C2%B8"),
        '¹' => Some("%C2%B9"),
        'º' => Some("%C2%BA"),
        '»' => Some("%C2%BB"),
        '¼' => Some("%C2%BC"),
        '½' => Some("%C2%BD"),
        '¾' => Some("%C2%BE"),
        '¿' => Some("%C2%BF"),
        'À' => Some("%C3%80"),
        'Á' => Some("%C3%81"),
        'Â' => Some("%C3%82"),
        'Ã' => Some("%C3%83"),
        'Ä' => Some("%C3%84"),
        'Å' => Some("%C3%85"),
        'Æ' => Some("%C3%86"),
        'Ç' => Some("%C3%87"),
        'È' => Some("%C3%88"),
        'É' => Some("%C3%89"),
        'Ê' => Some("%C3%8A"),
        'Ë' => Some("%C3%8B"),
        'Ì' => Some("%C3%8C"),
        'Í' => Some("%C3%8D"),
        'Î' => Some("%C3%8E"),
        'Ï' => Some("%C3%8F"),
        'Ð' => Some("%C3%90"),
        'Ñ' => Some("%C3%91"),
        'Ò' => Some("%C3%92"),
        'Ó' => Some("%C3%93"),
        'Ô' => Some("%C3%94"),
        'Õ' => Some("%C3%95"),
        'Ö' => Some("%C3%96"),
        '×' => Some("%C3%97"),
        'Ø' => Some("%C3%98"),
        'Ù' => Some("%C3%99"),
        'Ú' => Some("%C3%9A"),
        'Û' => Some("%C3%9B"),
        'Ü' => Some("%C3%9C"),
        'Ý' => Some("%C3%9D"),
        'Þ' => Some("%C3%9E"),
        'ß' => Some("%C3%9F"),
        'à' => Some("%C3%A0"),
        'á' => Some("%C3%A1"),
        'â' => Some("%C3%A2"),
        'ã' => Some("%C3%A3"),
        'ä' => Some("%C3%A4"),
        'å' => Some("%C3%A5"),
        'æ' => Some("%C3%A6"),
        'ç' => Some("%C3%A7"),
        'è' => Some("%C3%A8"),
        'é' => Some("%C3%A9"),
        'ê' => Some("%C3%AA"),
        'ë' => Some("%C3%AB"),
        'ì' => Some("%C3%AC"),
        'í' => Some("%C3%AD"),
        'î' => Some("%C3%AE"),
        'ï' => Some("%C3%AF"),
        'ð' => Some("%C3%B0"),
        'ñ' => Some("%C3%B1"),
        'ò' => Some("%C3%B2"),
        'ó' => Some("%C3%B3"),
        'ô' => Some("%C3%B4"),
        'õ' => Some("%C3%B5"),
        'ö' => Some("%C3%B6"),
        '÷' => Some("%C3%B7"),
        'ø' => Some("%C3%B8"),
        'ù' => Some("%C3%B9"),
        'ú' => Some("%C3%BA"),
        'û' => Some("%C3%BB"),
        'ü' => Some("%C3%BC"),
        'ý' => Some("%C3%BD"),
        'þ' => Some("%C3%BE"),
        'ÿ' => Some(r##"%C3%BF"#;"##),
        _ => None,
    };
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
