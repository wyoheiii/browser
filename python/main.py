from browser import Browser


def load(url):
    body = url.request()


if __name__ == "__main__":
    import sys
    import os
    # test if url is empty
    if len(sys.argv) >= 2:
        url = sys.argv[1]
    else:
        base_dir = os.path.dirname(__file__)
        path = os.path.abspath(os.path.join(base_dir, "test.html"))
        url = "file://" + path

    browser = Browser()
    browser.load(url)
    browser.mainloop()
