import tkinter
from tkinter.ttk import _VsapiStatespec
from core.url import URL
WIDTH, HEIGHT = 800, 600

class Browser:
    def __init__(self):
        self.window = tkinter.Tk()
        self.canvas = tkinter.Canvas(
            self.window,
            width=WIDTH,
            height=HEIGHT,
        )
        self.canvas.pack()

    def load(self, url):
        url = URL(url)
        body = url.request()
        text = lex(body)

        HSTEP, VSTEP = 13, 18
        cursor_x, cursor_y = HSTEP, VSTEP

        for c in text:
            self.canvas.create_text(cursor_x, cursor_y, text=c)
            cursor_x += HSTEP

            if cursor_x >= WIDTH - HSTEP:
                cursor_y += VSTEP
                cursor_x = HSTEP




    def mainloop(self):
        self.window.mainloop()

def layout(text):
    display_list = []

    cursor_x, cursor_y = HSTEP, VSTEP
    for c in text:
        display_list.append((cursor_x, cursor_y, c))

def lex(body):
    in_tag = False
    text = ""

    for c in body:
        if c == "<":
            in_tag = True
        elif c == ">":
            in_tag = False

        elif not in_tag:
            text += c
    return text