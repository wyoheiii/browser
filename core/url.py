import socket
import ssl

class URL:
    def __init__(self, url):
        if url.startswith("data:"):
            self.scheme = "data"
            self.host = None
            self.port = None
            self.path = None
            self.data_url = url[len("data:"):]
            return

        if url.startswith("view-source:"):
            url = url[len("view-source:"):]
            self.is_view_source = True
        else:
            self.is_view_source = False

        self.scheme, url = url.split("://", 1)
        assert self.scheme in ["http", "https", "file"]

        if self.scheme == "file":
            self.host = None
            self.port = None
            self.path = url

            assert self.path.startswith("/")
            return

        if self.scheme == "http":
            self.port = 80
        elif self.scheme == "https":
            self.port = 443

        if "/" not in url:
            url = url + "/"

        self.host, url = url.split("/", 1)
        self.path = "/" + url

        if ":" in self.host:
            self.host, port = self.host.split(":", 1)
            self.port = int(port)

    def parse_data_url(self):
        from urllib.parse import unquote
        import base64
        metadata, data = self.data_url.split(",", 1)

        if metadata.endswith(";base64"):
            return base64.b64decode(data).decode("utf-8")
        else:
            return unquote(data)

    def request(self):
        if self.scheme == "data":
            return self.parse_data_url()

        if self.scheme == "file":
            with open(self.path, "r", encoding="utf-8") as f:
                return f.read()

        s = socket.socket(
            family=socket.AF_INET,
            type=socket.SOCK_STREAM,
            proto=socket.IPPROTO_TCP
        )

        s.connect((self.host, self.port))

        if self.scheme == "https":
            ctx = ssl.create_default_context()
            s = ctx.wrap_socket(s, server_hostname=self.host)

        request_headers = {
            "Host": self.host,
            "Connection": "close",
            "User-Agent": "omocha-browser/0.1"
        }

        request = "GET {} HTTP/1.1\r\n".format(self.path)

        for key, value in request_headers.items():
            request += "{}: {}\r\n".format(key, value)

        request += "\r\n"
        s.send(request.encode("utf-8"))

        response = s.makefile("r", encoding="utf8", newline="\r\n")

        statusline = response.readline()
        version, status, explanation = statusline.split(" ", 2)

        response_headers = {}
        while True:
            line = response.readline()
            if line == "\r\n":
                break

            header, value = line.split(":", 1)
            response_headers[header.casefold()] = value.strip()

        #assert "transfer-encoding" not in response_headers
        #assert "content-length" not in response_headers
        #assert "content-length" in response_headers

        content = response.read()
        s.close()

        return content


def show(body):
    in_tag = False

    for c in body:
        if c == "<":
            in_tag = True
        elif c == ">":
            in_tag = False

        elif not in_tag:
            print(c, end="")

def load(url):
    body = url.request()

    if url.is_view_source:
        print(body)
    else:
      show(body)

if __name__ == "__main__":
    import sys
    import os
    # test if url is empty
    if len(sys.argv) >= 2:
        url = sys.argv[1]
    else:
        path = os.path.abspath("test.html")
        url = "file://" + path

    url = URL(url)
    load(url)