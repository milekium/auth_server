
Authentication server

# Authentication Server
Back-end server for authentication written in rust.

## Basic Authentications
Basic authentication is use with the following components:

### Headers:
Authorization header:
: Authorization: Basic <encoded credentials>   base64-encoding of {username}:{password}

Realms:
: WWW-Authenticate: Basic realm="AuthServer"

## End-Points:
 `/signup`
:   - post: `create_user` , params: *Un-AuthenticatedUser.

`/auth`
:  - post `auth`           , params: *BasicAuth.

`/me`
:  - get     `me`, params:              *AuthenticatedUser.
:  - post    `/update_profile`, params:  *AuthenticatedUser.
:  - delete  `/delete_profile`, params:  *AuthenticatedUser.

`/validate`
:  - post:`validate_email`, params:      *Email.

`/reset password`
:   - */
   - 

# Test end-points:
curl --request GET http://127.0.0.1:3030/hello/sean -H "User-Agent: reqwest/v0.8.6" -H "Host: hyper.rs"

### Session-based authentication