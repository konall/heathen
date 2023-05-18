pub(crate) struct Xid<'a>(pub(crate) &'a str);

impl Xid<'_> {
    fn parse(input: &str) -> (Option<Xid>, &str) {
        if !input.chars().next().map(|c| c == '%').unwrap_or_default() {
            return (None, input);
        }
        
        let pos = (&input[1..]).chars().position(|c| !c.is_digit(10));
        
        if let Some(pos) = pos {
            if pos == 0 {
                (None, input)
            } else {
                let xid = &input[1..pos];
                let rest = &input[pos..];
                (Some(Xid(xid)), rest)
            }
        } else {
            (Some(Xid(&input[1..])), "")
        }
    }
}


pub(crate) struct Ident<'a>(pub(crate) &'a str);

impl Ident<'_> {
    fn parse(input: &str) -> (Option<Ident>, &str) {
        if input.chars().next().map(|c| (c == '_') || c.is_alphabetic()).unwrap_or_default() {
            let pos = (&input[1..]).chars().position(|c| !(c.is_alphabetic() || c.is_digit(10) || (c == '_')));
            
            if let Some(pos) = pos {
                if pos == 0 {
                    (Some(Ident(&input[..1])), &input[1..])
                } else {
                    let ident = &input[..(pos+1)];
                    let rest = &input[(pos+1)..];
                    (Some(Ident(ident)), rest)
                }
            } else {
                (Some(Ident(input)), "")
            }
        } else {
            (None, input)
        }
    }
}


pub(crate) enum Op {
    Exists,
    Equals,
    NotEquals,
    StartsWith,
    Contains,
    EndsWith
}

pub(crate) struct Attribute<'a> {
    pub(crate) name: Ident<'a>,
    pub(crate) op: Op,
    pub(crate) value: Option<&'a str>
}

impl Attribute<'_> {
    fn parse(input: &str) -> (Option<Attribute>, &str) {
        if !input.chars().next().map(|c| c == '[').unwrap_or_default() {
            return (None, input);
        }
        
        let (Some(ident), rest) = Ident::parse(&input[1..]) else {
            return (None, input);
        };
        
        if rest.chars().next().map(|c| c == ']').unwrap_or_default() {
            return (
                Some(
                    Attribute {
                        name: ident,
                        op: Op::Exists,
                        value: None
                    }
                ),
                &rest[1..]
            );
        }
        
        let op = match rest.chars().next() {
            Some(c) => match c {
                '=' => Op::Equals,
                eqmod => {
                    if !(&rest[1..]).chars().next().map(|c| c == '=').unwrap_or_default() {
                        return (None, input);
                    }
                    
                    match eqmod {
                        '!' => Op::NotEquals,
                        '^' => Op::StartsWith,
                        '*' => Op::Contains,
                        '$' => Op::EndsWith,
                        _ => return (None, input)
                    }
                }
            },
            None => return (None, input)
        };
        
        let start_pos = match op {
            Op::Equals => 1,
            _ => 2
        };
        let end_pos = (&rest[start_pos..]).chars().position(|c| c == ']');
        if let Some(end_pos) = end_pos {
            if end_pos == 0 {
                (None, input)
            } else {
                let value = &rest[start_pos..end_pos];
                
                (
                    Some(
                        Attribute {
                            name: ident,
                            op,
                            value: Some(value)
                        }
                    ),
                    &rest[(end_pos+1)..]
                )
            }
        } else {
            (None, input)
        }
    }
}


pub(crate) struct Rule<'a> {
    pub(crate) tag: Option<Ident<'a>>,
    pub(crate) xid: Option<Xid<'a>>,
    pub(crate) id: Option<Ident<'a>>,
    pub(crate) classes: Vec<Ident<'a>>,
    pub(crate) attributes: Vec<Attribute<'a>>
}

impl Rule<'_> {
    fn parse(input: &str) -> (Option<Rule>, &str) {
        let (tag, rest) = Ident::parse(input);
        let (xid, rest) = Xid::parse(rest);
        
        let (id, rest) = {
            if rest.chars().next().map(|c| c == '#').unwrap_or_default() {
                Ident::parse(&rest[1..])
            } else {
                (None, rest)
            }
        };
        
        let (classes, rest) = {
            let mut classes = vec![];
            let mut nxt = rest;
            while nxt.chars().next().map(|c| c == '.').unwrap_or_default() {
                let (class, rest) = Ident::parse(&nxt[1..]);
                if let Some(class) = class {
                    classes.push(class);
                    nxt = rest;
                } else {
                    return (None, input);
                }
            }
            (classes, nxt)
        };
        
        let mut attributes = vec![];
        let mut rest = rest;
        while let (Some(attribute), nxt) = Attribute::parse(rest) {
            attributes.push(attribute);
            rest = nxt;
        }
        
        (
            Some(
                Rule { tag, xid, id, classes, attributes }
            ),
            rest
        )
    }
}


pub(crate) enum Link {
    Ancestors,
    Descendants,
    Parent,
    Children,
    NextSibling,
    NextSiblings,
    PrevSibling,
    PrevSiblings,
    Siblings
}

impl Link {
    fn parse(input: &str) -> (Option<Link>, &str) {
        let Some(start_pos) = input.chars().position(|c| !c.is_whitespace()) else {
            return (None, input);
        };
        
        let (link, rest) = match (&input[start_pos..]).chars().next() {
            Some(c) => {
                let rest = &input[(start_pos+1)..];
                let rest2 = &input[(start_pos+2)..];
                let c2 = rest.chars().next();
                match c {
                    '<' => match c2 {
                        Some('<') => (Link::Ancestors, rest2),
                        _ => (Link::Parent, rest)
                    },
                    '>' => match c2 {
                        Some('>') => (Link::Descendants, rest2),
                        _ => (Link::Children, rest)
                    },
                    '+' => match c2 {
                        Some('+') => (Link::NextSiblings, rest2),
                        _ => (Link::NextSibling, rest)
                    },
                    '~' => match c2 {
                        Some('~') => (Link::PrevSiblings, rest2),
                        Some('+') => (Link::Siblings, rest2),
                        _ => (Link::PrevSibling, rest)
                    },
                    _ => return (None, input)
                }
            },
            None => return (None, input)
        };
        
        let Some(end_pos) = rest.chars().position(|c| !c.is_whitespace()) else {
            return (None, input);
        };
        
        (Some(link), &rest[end_pos..])
    }
}


pub(crate) struct Selector<'a> {
    pub(crate) x: Vec<(Rule<'a>, Option<Link>)>,
    pub(crate) src: String
}

impl Selector<'_> {
    pub(crate) fn parse(input: &str) -> Option<Selector> {
        let mut res = vec![];
        let mut rest = input;
        
        loop {
            let (Some(rule), nxt) = Rule::parse(rest) else {
                return None;
            };
            
            if let (Some(link), nxt) = Link::parse(nxt) {
                res.push((rule, Some(link)));
                rest = nxt;
            } else {
                res.push((rule, None));
                return Some(
                    Selector {
                        x: res,
                        src: input.to_string()
                    }
                );
            }
        }
    }
}
