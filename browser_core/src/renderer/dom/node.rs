use alloc::rc::Rc;
use alloc::rc::Weak;
use core::cell::RefCell;
use alloc::string::String;
use crate::renderer::html::attribute::Attribute;
use alloc::vec::Vec;
use core::str::FromStr;
use alloc::format;

// https://dom.spec.whatwg.org/#interface-element
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ElementKind {
  // https://html.spec.whatwg.org/multipage/semantics.html#the-html-element
  Html,
  // https://html.spec.whatwg.org/multipage/semantics.html#the-head-element
  Head,
  // https://html.spec.whatwg.org/multipage/semantics.html#the-style-element
  Style,
  // https://html.spec.whatwg.org/multipage/scripting.html#the-script-element
  Script,
  // https://html.spec.whatwg.org/multipage/sections.html#the-body-element
  Body,
  // https://html.spec.whatwg.org/multipage/grouping-content.html#the-p-element
  P,
  // https://html.spec.whatwg.org/multipage/sections.html#the-h1,-h2,-h3,-h4,-h5,-and-h6-elements
  H1,
  H2,
  // // https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element
  // A,
}

impl FromStr for ElementKind {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "html" => Ok(Self::Html),
      "head" => Ok(Self::Head),
      "style" => Ok(Self::Style),
      "script" => Ok(Self::Script),
      "body" => Ok(Self::Body),
      "p" => Ok(Self::P),
      "h1" => Ok(Self::H1),
      "h2" => Ok(Self::H2),
      _ => Err(format!("unimplemented element name {:?}", s)),
    }
  }
}


// https://html.spec.whatwg.org/multipage/nav-history-apis.html#window
#[derive(Debug, Clone)]
pub struct Window {
    document: Rc<RefCell<Node>>,
}

impl Window {
    pub fn new() -> Self {
      let window = Self {
        document: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
      };

      window
        .document
        .borrow_mut()
        .set_window(Rc::downgrade(&Rc::new(RefCell::new(window.clone()))));

      window
    }

    pub fn document(&self) -> Rc<RefCell<Node>> {
        self.document.clone()
    }
}

// https://dom.spec.whatwg.org/#interface-element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
  kind: ElementKind,
  attributes: Vec<Attribute>,
}

impl Element {
  pub fn new(element_name: &str, attributes: Vec<Attribute>) -> Self {
    Self {
      kind: ElementKind::from_str(element_name)
        .expect("failed to convert string to ElementKind"),
      attributes,
    }
  }

  pub fn kind(&self) -> ElementKind {
    self.kind
  }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    /// https://dom.spec.whatwg.org/#interface-document
    Document,
    /// https://dom.spec.whatwg.org/#interface-element
    Element(Element),
    /// https://dom.spec.whatwg.org/#interface-text
    Text(String),
}

#[derive(Debug, Clone)]
pub struct Node {
  pub kind: NodeKind,
  window: Weak<RefCell<Window>>,
  parent: Weak<RefCell<Node>>,
  first_child: Option<Rc<RefCell<Node>>>,
  last_child: Weak<RefCell<Node>>,
  previous_sibling: Weak<RefCell<Node>>,
  next_sibling: Option<Rc<RefCell<Node>>>,
}

impl Node {
  pub fn new(kind: NodeKind) -> Self {
    Self {
      kind,
      window: Weak::new(),
      parent: Weak::new(),
      first_child: None,
      last_child: Weak::new(),
      previous_sibling: Weak::new(),
      next_sibling: None,
    }
  }

  pub fn set_window(&mut self, window: Weak<RefCell<Window>>) {
    self.window = window;
}

  pub fn set_parent(&mut self, parent: Weak<RefCell<Node>>) {
    self.parent = parent;
  }

  pub fn parent(&self) -> Weak<RefCell<Node>> {
    self.parent.clone()
  }

  pub fn set_first_child(&mut self, first_child: Option<Rc<RefCell<Node>>>) {
    self.first_child = first_child;
  }

  pub fn first_child(&self) -> Option<Rc<RefCell<Node>>> {
    self.first_child.clone()
  }

  pub fn set_last_child(&mut self, last_child: Weak<RefCell<Node>>) {
    self.last_child = last_child;
  }

  pub fn last_child(&self) -> Weak<RefCell<Node>> {
    self.last_child.clone()
  }

  pub fn set_previous_sibling(&mut self, previous_sibling: Weak<RefCell<Node>>) {
    self.previous_sibling = previous_sibling;
  }

  pub fn previous_sibling(&self) -> Weak<RefCell<Node>> {
    self.previous_sibling.clone()
  }

  pub fn set_next_sibling(&mut self, next_sibling: Option<Rc<RefCell<Node>>>) {
    self.next_sibling = next_sibling;
  }

  pub fn next_sibling(&self) -> Option<Rc<RefCell<Node>>> {
    self.next_sibling.clone()
  }

  pub fn kind(&self) -> NodeKind {
    self.kind.clone()
  }

  pub fn get_element(&self) -> Option<Element> {
    match self.kind {
      NodeKind::Document | NodeKind::Text(_) => None,
      NodeKind::Element(ref element) => Some(element.clone()),
    }
  }

  pub fn element_kind(&self) -> Option<ElementKind> {
    match self.kind {
      NodeKind::Document | NodeKind::Text(_) => None,
      NodeKind::Element(ref element) => Some(element.kind()),
    }
  }
}
