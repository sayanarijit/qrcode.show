<h1 align = "center">QRcode.show </h1>

<h2 align = "center">Generate QR code easily for free - QR Code Generation as a Service.</h2>


<br>

### INPUT:

```bash
curl qrcode.show/INPUT
```
```bash
curl qrcode.show -d INPUT
```
```bash
curl qrcode.show -d @/PATH/TO/INPUT
```
```bash
echo INPUT | curl qrcode.show -d @-
```

### INPUT EXAMPLES:

```bash
curl qrcode.show/https://example.com
```
```bash 
curl qrcode.show -d https://example.com
```
```bash
curl qrcode.show -d @/path/to/input.txt
```
```bash
echo https://example.com | curl qrcode.show -d @-
```

### PARAMETERS:

```
    Accept                      Specify the output type
                                Options:
                                    application/octet-stream
                                    text/plain
                                    text/html
                                    image/svg+xml
                                    image/png
                                    image/jpeg
                                Default: application/octet-stream

    X-QR-Width                  Specify the default width

    X-QR-Height                 Specify the default height

    X-QR-Min-Width              Specify the minimum width

    X-QR-Min-Height             Specify the minimun height

    X-QR-Max-Width              Specify the maximum width

    X-QR-Max-Height             Specify the maximum height

    X-QR-Dark-Color             Specify the dark color (hex)
                                Format: rrggbb

    X-QR-Light-Color            Specify the light color (hex)
                                Format: rrggbb

    X-QR-Version-Type           Specify the QR version type
                                Options:
                                    normal
                                    micro
                                Default: auto detect

    X-QR-Version-Number         Specify the QR version number
                                Options:
                                    1..40 for normal
                                    1..4 for micro
                                Default: auto detect

    X-QR-EC-Level               Specify the error checking level
                                Options:
                                    L
                                    M
                                    Q
                                    H
                                Default: L

    X-QR-Quiet-Zone             Specify whether the quiet zone is added
                                Options:
                                    true
                                    false
                                Default: true
```

### PARAMETER EXAMPLES:

```bash
curl qrcode.show/INPUT -H "Accept: image/svg+xml"
```

### SHELL FUNCTIONS:

Shell functions that can be added to `.bashrc` or `.bash_profle` for quickly generating QR codes from the command line. The command takes the argument as input or reads from stdin if none was supplied and outputs the QR code to stdout: `qrcode INPUT` or `echo INPUT | qrcode`

```bash
qrcode () {
  local input="$*"
  [ -z "$input" ] && local input="@/dev/stdin"
  curl -d "$input" https://qrcode.show
}
```

```bash
qrsvg () {
  local input="$*"
  [ -z "$input" ] && local input="@/dev/stdin"
  curl -d "${input}" https://qrcode.show -H "Accept: image/svg+xml"
}
```

```bash
qrserve () {
  local port=${1:-8080}
  local dir=${2:-.}
  local ip="$(ifconfig | grep -Eo '[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}' | fzf --prompt IP:)" \
    && echo http://$ip:$port | qrcode \
    && python -m http.server $port -b $ip -d $dir
}
```

### 🚀 FEATURES :
    
* No data collection or retention
* Fast and simple API that works on both web and terminal
* Supports GET and POST requests
* Supports `Accept` header to control the output format

### 📝 TODO:
    
* Support more parameters
* Get a logo
* Support dynamic QR codes
* Generate premium/branded QR codes

### 💖 SPONSORS:

* Swordscode - https://swordscode.com - $5
* Nolan Rumble - https://nolanrumble.com - $5

> Top 5 sponsors get mentioned here (updated monthly) Visit https://opencollective.com/qrcodeshow

### 📋 CREDITS:

* Main Library                https://github.com/kennytm/qrcode-rust
* Cloudflare Worker           https://github.com/cloudflare/workers-rs
* Alternate Web Server        https://github.com/tokio-rs/axum

> Only the direct dependencies for the main business logic are listed here. Please contact the project maintainer if you are missing from the list.


### 🔗 RELATED LINKS:

* Alternate Domain            https://qrqr.show

* Project Repository          https://github.com/sayanarijit/qrcode.show
* Project Maintainer          https://arijitbasu.in


### 📓 COPYRIGHT:

**© Arijit Basu 2021**
