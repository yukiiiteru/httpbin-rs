# httpbin-rs

Reimplemented HTTP Request & Response Service, written in Rust + Volo-HTTP.

## Status

- [x] HTTP Methods
  - [x] `/delete`
  - [x] `/get`
  - [x] `/patch`
  - [x] `/post`
  - [x] `/put`
- [ ] Auth
  - [ ] `/basic-auth/{user}/{passwd}`
  - [ ] `/bearer`
  - [ ] `/digest-auth/{qop}/{user}/{passwd}`
  - [ ] `/digest-auth/{qop}/{user}/{passwd}/{algorithm}`
  - [ ] `/digest-auth/{qop}/{user}/{passwd}/{algorithm}/{stale_after}`
  - [ ] `/hidden-basic-auth/{user}/{passwd}`
- [x] Status Codes
  - [x] `/status/{codes}`
- [x] Request inspection
  - [x] `/headers`
  - [x] `/ip`
  - [x] `/user-agent`
- [ ] Response Headers
  - [ ] `/cache`
  - [ ] `/cache/{value}`
  - [ ] `/etag/{etag}`
  - [x] `/response-headers`
  - [x] `/response-headers`
- [ ] Response formats
  - [ ] `/brotli`
  - [ ] `/deflate`
  - [ ] `/deny`
  - [ ] `/encoding/utf8`
  - [ ] `/gzip`
  - [ ] `/html`
  - [x] `/json`
  - [ ] `/robots.txt`
  - [ ] `/xml`
- [ ] Dynamic data
  - [ ] `/base64/{value}`
  - [x] `/bytes/{n}`
  - [x] `/delay/{delay}`
  - [ ] `/drip`
  - [ ] `/links/{n}/{offset}`
  - [ ] `/range/{numbytes}`
  - [x] `/stream-bytes/{n}`
  - [x] `/stream/{n}`
  - [ ] `/uuid`
- [ ] Cookies
  - [ ] `/cookies`
  - [ ] `/cookies/delete`
  - [ ] `/cookies/set`
  - [ ] `/cookies/set/{name}/{value}`
- [ ] Images
  - [ ] `/image`
  - [ ] `/image/jpeg`
  - [ ] `/image/png`
  - [ ] `/image/svg`
  - [ ] `/image/webp`
- [x] Redirects
  - [x] `/absolute-redirect/{n}`
  - [x] `/redirect-to`
  - [ ] `/redirect/{n}` (unimplemented because it is the same as `relative-redirect`)
  - [x] `/relative-redirect/{n}`
- [ ] Anything (unimplemented because it is the same as HTTP Methods)
  - [ ] `/anything`
  - [ ] `/anything/{anything}`
