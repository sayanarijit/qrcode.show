QRcode.show

Generate QR code easily for free - QR Code Generation as a Service

INPUT:

    $ curl qrcode.show/INPUT

    $ curl qrcode.show -d INPUT

    $ curl qrcode.show -d @/PATH/TO/INPUT

    $ echo INPUT | curl qrcode.show -d @-

INPUT EXAMPLES:

    $ curl qrcode.show/http://example.com

    $ curl qrcode.show -d http://example.com

    $ curl qrcode.show -d @/path/to/input.txt

    $ echo http://example.com | curl qrcode.show -d @-

PARAMETERS:

    Accept          Specify the output type
                    Options:
                        text/plain
                        image/svg+xml
                        text/html
                    Default: text/plain

PARAMETER EXAMPLES:

    $ curl qrcode.show/INPUT -H "Accept: image/svg+xml"

FEATURES:
    
    * No data collection or retention
    * Fast and simple API that works on both web and terminal
    * Supports GET and POST requests
    * Supports `Accept` header to control the output format

TODO:
    
    * Decide on a license
    * Use HTTPS
    * Download JPEG, PNG support
    * Control height, width, color and other properties using parameters
    * Get a logo
    * Support dynamic QR codes
    * Generate premium/branded QR codes

SPONSORS:

    Top 5 sponsors get mentioned here

CREDITS:

    Main Library                https://github.com/kennytm/qrcode-rust
    Web Server                  https://github.com/tokio-rs/axum

    Please contact the project maintainer to if you are missing from the list

RELATED LINKS:

    Alternate Domain            http://qrqr.show

    Project Repository          https://github.com/sayanarijit/qrcode.show
    Project Maintainer          https://arijitbasu.in

    Donate & Support            https://ko-fi.com/sayanarijit
                                https://liberapay.com/sayanarijit

COPYRIGHT:

    © Arijit Basu 2021
