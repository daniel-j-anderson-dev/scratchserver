#![allow(unused)]

use color_eyre::{eyre::eyre, Report};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(usize);
impl StatusCode {
    /// The server has received the request headers and the client should proceed to send the request body (in the case of a request for which a body needs to be sent; for example, a POST request). Sending a large request body to a server after a request has been rejected for inappropriate headers would be inefficient. To have a server check the request's headers, a client must send Expect: 100-continue as a header in its initial request and receive a 100 Continue status code in response before sending the body. If the client receives an error code such as 403 (Forbidden) or 405 (Method Not Allowed) then it should not send the request's body. The response 417 Expectation Failed indicates that the request should be repeated without the Expect header as it indicates that the server does not support expectations (this is the case, for example, of HTTP/1.0 servers).[1]: §10.1.1
    pub const CONTINUE: StatusCode = StatusCode(100);

    /// The requester has asked the server to switch protocols and the server has agreed to do so.
    pub const SWITCHING_PROTOCOLS: StatusCode = StatusCode(101);

    /// A WebDAV request may contain many sub-requests involving file operations, requiring a long time to complete the request. This code indicates that the server has received and is processing the request, but no response is available yet.[3] This prevents the client from timing out and assuming the request was lost. The status code is deprecated.[4]
    pub const PROCESSING: StatusCode = StatusCode(102);

    /// Used to return some response headers before final HTTP message.[5]
    pub const EARLY_HINTS: StatusCode = StatusCode(103);

    ///Standard response for successful HTTP requests. The actual response will depend on the request method used. In a GET request, the response will contain an entity corresponding to the requested resource. In a POST request, the response will contain an entity describing or containing the result of the action.
    pub const OK: StatusCode = StatusCode(200);

    /// The request has been fulfilled, resulting in the creation of a new resource.[6]
    pub const CREATED: StatusCode = StatusCode(201);

    /// The request has been accepted for processing, but the processing has not been completed. The request might or might not be eventually acted upon, and may be disallowed when processing occurs.
    pub const ACCEPTED: StatusCode = StatusCode(202);

    /// The server is a transforming proxy (e.g. a Web accelerator) that received a 200 OK from its origin, but is returning a modified version of the origin's response.[1]: §15.3.4 [1]: §7.7
    pub const NON_AUTHORITATIVE_INFORMATION: StatusCode = StatusCode(203);

    /// The server successfully processed the request, and is not returning any content.
    pub const NO_CONTENT: StatusCode = StatusCode(204);

    /// The server successfully processed the request, asks that the requester reset its document view, and is not returning any content.
    pub const RESET_CONTENT: StatusCode = StatusCode(205);

    /// The server is delivering only part of the resource (byte serving) due to a range header sent by the client. The range header is used by HTTP clients to enable resuming of interrupted downloads, or split a download into multiple simultaneous streams.
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(206);

    /// The message body that follows is by default an XML message and can contain a number of separate response codes, depending on how many sub-requests were made.[7]
    pub const MULTI_STATUS: StatusCode = StatusCode(207);

    /// The members of a DAV binding have already been enumerated in a preceding part of the (multi-status) response, and are not being included again.
    pub const ALREADY_REPORTED: StatusCode = StatusCode(208);

    /// The server has fulfilled a request for the resource, and the response is a representation of the result of one or more instance-manipulations applied to the current instance.[8]
    pub const IM_USED: StatusCode = StatusCode(226);

    /// Indicates multiple options for the resource from which the client may choose (via agent-driven content negotiation). For example, this code could be used to present multiple video format options, to list files with different filename extensions, or to suggest word-sense disambiguation.
    pub const MULTIPLE_CHOICES: StatusCode = StatusCode(300);

    /// This and all future requests should be directed to the given URI.
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(301);

    /// Tells the client to look at (browse to) another URL. The HTTP/1.0 specification required the client to perform a temporary redirect with the same method (the original describing phrase was "Moved Temporarily"),[9] but popular browsers implemented 302 redirects by changing the method to GET. Therefore, HTTP/1.1 added status codes 303 and 307 to distinguish between the two behaviors.[1]: §15.4
    pub const FOUND: StatusCode = StatusCode(302);

    /// The response to the request can be found under another URI using the GET method. When received in response to a POST (or PUT/DELETE), the client should presume that the server has received the data and should issue a new GET request to the given URI.
    pub const SEE_OTHER: StatusCode = StatusCode(303);

    /// Indicates that the resource has not been modified since the version specified by the request headers If-Modified-Since or If-None-Match. In such case, there is no need to retransmit the resource since the client still has a previously-downloaded copy.
    pub const NOT_MODIFIED: StatusCode = StatusCode(304);

    /// The requested resource is available only through a proxy, the address for which is provided in the response. For security reasons, many HTTP clients (such as Mozilla Firefox and Internet Explorer) do not obey this status code.[10]
    pub const USE_PROXY: StatusCode = StatusCode(305);

    /// No longer used. Originally meant "Subsequent requests should use the specified proxy."
    pub const SWITCH_PROXY: StatusCode = StatusCode(306);

    /// In this case, the request should be repeated with another URI; however, future requests should still use the original URI. In contrast to how 302 was historically implemented, the request method is not allowed to be changed when reissuing the original request. For example, a POST request should be repeated using another POST request.
    pub const TEMPORARY_REDIRECT: StatusCode = StatusCode(307);

    /// This and all future requests should be directed to the given URI. 308 parallel the behavior of 301, but does not allow the HTTP method to change. So, for example, submitting a form to a permanently redirected resource may continue smoothly.
    pub const PERMANENT_REDIRECT: StatusCode = StatusCode(308);

    /// The server cannot or will not process the request due to an apparent client error (e.g., malformed request syntax, size too large, invalid request message framing, or deceptive request routing).
    pub const BAD_REQUEST: StatusCode = StatusCode(400);

    /// Similar to 403 Forbidden, but specifically for use when authentication is required and has failed or has not yet been provided. The response must include a WWW-Authenticate header field containing a challenge applicable to the requested resource. See Basic access authentication and Digest access authentication. 401 semantically means "unauthorized", the user does not have valid authentication credentials for the target resource. Some sites incorrectly issue HTTP 401 when an IP address is banned from the website (usually the website domain) and that specific address is refused permission to access a website.[citation needed]
    pub const UNAUTHORIZED: StatusCode = StatusCode(401);

    /// Reserved for future use. The original intention was that this code might be used as part of some form of digital cash or micro-payment scheme, as proposed, for example, by GNU Taler,[11] but that has not yet happened, and this code is not widely used. Google Developers API uses this status if a particular developer has exceeded the daily limit on requests.[12] Sipgate uses this code if an account does not have sufficient funds to start a call.[13] Shopify uses this code when the store has not paid their fees and is temporarily disabled.[14] Stripe uses this code for failed payments where parameters were correct, for example blocked fraudulent payments.[15]
    pub const PAYMENT_REQUIRED: StatusCode = StatusCode(402);

    /// The request contained valid data and was understood by the server, but the server is refusing action. This may be due to the user not having the necessary permissions for a resource or needing an account of some sort, or attempting a prohibited action (e.g. creating a duplicate record where only one is allowed). This code is also typically used if the request provided authentication by answering the WWW-Authenticate header field challenge, but the server did not accept that authentication. The request should not be repeated.
    pub const FORBIDDEN: StatusCode = StatusCode(403);

    /// The requested resource could not be found but may be available in the future. Subsequent requests by the client are permissible.
    pub const NOT_FOUND: StatusCode = StatusCode(404);

    /// A request method is not supported for the requested resource; for example, a GET request on a form that requires data to be presented via POST, or a PUT request on a read-only resource.
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(405);

    /// The requested resource is capable of generating only content not acceptable according to the Accept headers sent in the request. See Content negotiation.
    pub const NOT_ACCEPTABLE: StatusCode = StatusCode(406);

    /// The client must first authenticate itself with the proxy.
    pub const PROXY_AUTHENTICATION_REQUIRED: StatusCode = StatusCode(407);

    /// The server timed out waiting for the request. According to HTTP specifications: "The client did not produce a request within the time that the server was prepared to wait. The client MAY repeat the request without modifications at any later time."
    pub const REQUEST_TIMEOUT: StatusCode = StatusCode(408);

    /// Indicates that the request could not be processed because of conflict in the current state of the resource, such as an edit conflict between multiple simultaneous updates.
    pub const CONFLICT: StatusCode = StatusCode(409);

    /// Indicates that the resource requested was previously in use but is no longer available and will not be available again. This should be used when a resource has been intentionally removed and the resource should be purged. Upon receiving a 410 status code, the client should not request the resource in the future. Clients such as search engines should remove the resource from their indices. Most use cases do not require clients and search engines to purge the resource, and a "404 Not Found" may be used instead.
    pub const GONE: StatusCode = StatusCode(410);

    /// The request did not specify the length of its content, which is required by the requested resource.
    pub const LENGTH_REQUIRED: StatusCode = StatusCode(411);

    /// The server does not meet one of the preconditions that the requester put on the request header fields.
    pub const PRECONDITION_FAILED: StatusCode = StatusCode(412);

    /// The request is larger than the server is willing or able to process. Previously called "Request Entity Too Large".[16]: §10.4.14
    pub const PAYLOAD_TOO_LARGE: StatusCode = StatusCode(413);

    /// The URI provided was too long for the server to process. Often the result of too much data being encoded as a query-string of a GET request, in which case it should be converted to a POST request. Called "Request-URI Too Long" previously.[16]: §10.4.15
    pub const URI_TOO_LONG: StatusCode = StatusCode(414);

    /// The request entity has a media type which the server or resource does not support. For example, the client uploads an image as image/svg+xml, but the server requires that images use a different format.
    pub const UNSUPPORTED_MEDIA_TYPE: StatusCode = StatusCode(415);

    /// The client has asked for a portion of the file (byte serving), but the server cannot supply that portion. For example, if the client asked for a part of the file that lies beyond the end of the file. Called "Requested Range Not Satisfiable" previously.[16]: §10.4.17
    pub const RANGE_NOT_SATISFIABLE: StatusCode = StatusCode(416);

    /// The server cannot meet the requirements of the Expect request-header field.[17]
    pub const EXPECTATION_FAILED: StatusCode = StatusCode(417);

    /// This code was defined in 1998 as one of the traditional IETF April Fools' jokes, in RFC 2324, Hyper Text Coffee Pot Control Protocol, and is not expected to be implemented by actual HTTP servers. The RFC specifies this code should be returned by teapots requested to brew coffee.[18] This HTTP status is used as an Easter egg in some websites, such as Google.com's "I'm a teapot" easter egg.[19][20][21] Sometimes, this status code is also used as a response to a blocked request, instead of the more appropriate 403 Forbidden.[22][23]
    pub const IM_A_TEAPOT: StatusCode = StatusCode(418);

    /// The request was directed at a server that is not able to produce a response (for example because of connection reuse).
    pub const MISDIRECTED_REQUEST: StatusCode = StatusCode(421);

    /// The request was well-formed (i.e., syntactically correct) but could not be processed.[24]
    pub const UNPROCESSABLE_CONTENT: StatusCode = StatusCode(422);

    /// The resource that is being accessed is locked.[7]
    pub const LOCKED: StatusCode = StatusCode(423);

    /// The request failed because it depended on another request and that request failed (e.g., a PROPPATCH).[7]
    pub const FAILED_DEPENDENCY: StatusCode = StatusCode(424);

    /// Indicates that the server is unwilling to risk processing a request that might be replayed.
    pub const TOO_EARLY: StatusCode = StatusCode(425);

    /// The client should switch to a different protocol such as TLS/1.3, given in the Upgrade header field.
    pub const UPGRADE_REQUIRED: StatusCode = StatusCode(426);

    /// The origin server requires the request to be conditional. Intended to prevent the 'lost update' problem, where a client GETs a resource's state, modifies it, and PUTs it back to the server, when meanwhile a third party has modified the state on the server, leading to a conflict.[25]
    pub const PRECONDITION_REQUIRED: StatusCode = StatusCode(428);

    /// The user has sent too many requests in a given amount of time. Intended for use with rate-limiting schemes.[25]
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(429);

    /// The server is unwilling to process the request because either an individual header field, or all the header fields collectively, are too large.[25]
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: StatusCode = StatusCode(431);

    /// A server operator has received a legal demand to deny access to a resource or to a set of resources that includes the requested resource.[26] The code 451 was chosen as a reference to the novel Fahrenheit 451 (see the Acknowledgements in the RFC).
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: StatusCode = StatusCode(451);
}
impl StatusCode {
    pub fn as_str(&self) -> &'static str {
        return match *self {
            StatusCode(100) => "CONTINUE",
            StatusCode(101) => "SWITCHING_PROTOCOLS",
            StatusCode(102) => "PROCESSING",
            StatusCode(103) => "EARLY_HINTS",
            StatusCode(200) => "OK",
            StatusCode(201) => "CREATED",
            StatusCode(202) => "ACCEPTED",
            StatusCode(203) => "NON_AUTHORITATIVE_INFORMATION",
            StatusCode(204) => "NO_CONTENT",
            StatusCode(205) => "RESET_CONTENT",
            StatusCode(206) => "PARTIAL_CONTENT",
            StatusCode(207) => "MULTI_STATUS",
            StatusCode(208) => "ALREADY_REPORTED",
            StatusCode(226) => "IM_USED",
            StatusCode(300) => "MULTIPLE_CHOICES",
            StatusCode(301) => "MOVED_PERMANENTLY",
            StatusCode(302) => "FOUND",
            StatusCode(303) => "SEE_OTHER",
            StatusCode(304) => "NOT_MODIFIED",
            StatusCode(305) => "USE_PROXY",
            StatusCode(306) => "SWITCH_PROXY",
            StatusCode(307) => "TEMPORARY_REDIRECT",
            StatusCode(308) => "PERMANENT_REDIRECT",
            StatusCode(400) => "BAD_REQUEST",
            StatusCode(401) => "UNAUTHORIZED",
            StatusCode(402) => "PAYMENT_REQUIRED",
            StatusCode(403) => "FORBIDDEN",
            StatusCode(404) => "NOT_FOUND",
            StatusCode(405) => "METHOD_NOT_ALLOWED",
            StatusCode(406) => "NOT_ACCEPTABLE",
            StatusCode(407) => "PROXY_AUTHENTICATION_REQUIRED",
            StatusCode(408) => "REQUEST_TIMEOUT",
            StatusCode(409) => "CONFLICT",
            StatusCode(410) => "GONE",
            StatusCode(411) => "LENGTH_REQUIRED",
            StatusCode(412) => "PRECONDITION_FAILED",
            StatusCode(413) => "PAYLOAD_TOO_LARGE",
            StatusCode(414) => "URI_TOO_LONG",
            StatusCode(415) => "UNSUPPORTED_MEDIA_TYPE",
            StatusCode(416) => "RANGE_NOT_SATISFIABLE",
            StatusCode(417) => "EXPECTATION_FAILED",
            StatusCode(418) => "IM_A_TEAPOT",
            StatusCode(421) => "MISDIRECTED_REQUEST",
            StatusCode(422) => "UNPROCESSABLE_CONTENT",
            StatusCode(423) => "LOCKED",
            StatusCode(424) => "FAILED_DEPENDENCY",
            StatusCode(425) => "TOO_EARLY",
            StatusCode(426) => "UPGRADE_REQUIRED",
            StatusCode(428) => "PRECONDITION_REQUIRED",
            StatusCode(429) => "TOO_MANY_REQUESTS",
            StatusCode(431) => "REQUEST_HEADER_FIELDS_TOO_LARGE",
            StatusCode(451) => "UNAVAILABLE_FOR_LEGAL_REASONS",
            _ => "Unrecognized Status Code",
        };
    }
}
impl TryFrom<usize> for StatusCode {
    type Error = Report;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = StatusCode(value);
        if StatusCode::as_str(&value) == "Unrecognized Status Code" {
            Err(eyre!("{} is an unrecognized status code", value.0))?;
        }
        return Ok(value);
    }
}
impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{} {}", self.0, self.as_str());
    }
}
