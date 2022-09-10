
Auth server

# Basic Authentications

## Headers:
  - Authorization header 
    Authorization: Basic <encoded credentials>   base64-encoding of {username}:{password}
  - Realms
    WWW-Authenticate: Basic realm="publish"


# test end point:
curl --request GET http://127.0.0.1:3030/hello/sean -H "User-Agent: reqwest/v0.8.6" -H "Host: hyper.rs"