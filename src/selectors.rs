pub(crate) struct Xid(pub(crate) (usize, usize));

impl Xid {
    fn parse(src: &str, range: (usize, usize)) -> (Option<Xid>, (usize, usize)) {
        if !(&src[range.0..range.1]).chars().next().map(|c| c == '%').unwrap_or_default() {
            return (None, range);
        }
        
        let pos = (&src[(range.0 + 1)..range.1]).chars().position(|c| !c.is_digit(10));
        
        if let Some(pos) = pos {
            if pos == 0 {
                (None, range)
            } else {
                let xid = ((range.0 + 1), (range.0 + pos));
                let rest = ((range.0 + pos), range.1);
                (Some(Xid(xid)), rest)
            }
        } else {
            (Some(Xid(((range.0 + 1), range.1))), (range.1, src.chars().count()))
        }
    }
}


pub(crate) struct Ident(pub(crate) (usize, usize));

impl Ident {
    fn parse(src: &str, range: (usize, usize)) -> (Option<Ident>, (usize, usize)) {
        if (&src[range.0..range.1]).chars().next().map(|c| (c == '_') || c.is_alphabetic()).unwrap_or_default() {
            let pos = (&src[(range.0 + 1)..range.1]).chars().position(|c| !(c.is_alphabetic() || c.is_digit(10) || (c == '_')));
            
            if let Some(pos) = pos {
                if pos == 0 {
                    (Some(Ident((range.0, (range.0 + 1)))), ((range.0 + 1), range.1))
                } else {
                    let ident = (range.0, (range.0 + pos + 1));
                    let rest = ((range.0 + pos + 1), range.1);
                    (Some(Ident(ident)), rest)
                }
            } else {
                (Some(Ident(range)), (range.1, src.chars().count()))
            }
        } else {
            (None, range)
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

pub(crate) struct Attribute {
    pub(crate) name: Ident,
    pub(crate) op: Op,
    pub(crate) value: Option<(usize, usize)>
}

impl Attribute {
    fn parse(src: &str, range: (usize, usize)) -> (Option<Attribute>, (usize, usize)) {
        if !(&src[range.0..range.1]).chars().next().map(|c| c == '[').unwrap_or_default() {
            return (None, range);
        }
        
        let (Some(ident), rest) = Ident::parse(src, ((range.0 + 1), range.1)) else {
            return (None, range);
        };
        
        if (&src[rest.0..rest.1]).chars().next().map(|c| c == ']').unwrap_or_default() {
            return (
                Some(
                    Attribute {
                        name: ident,
                        op: Op::Exists,
                        value: None
                    }
                ),
                ((rest.0 + 1), rest.1)
            );
        }
        
        let op = match (&src[rest.0..rest.1]).chars().next() {
            Some(c) => match c {
                '=' => Op::Equals,
                eqmod => {
                    if !(&src[(rest.0 + 1)..rest.1]).chars().next().map(|c| c == '=').unwrap_or_default() {
                        return (None, range);
                    }
                    
                    match eqmod {
                        '!' => Op::NotEquals,
                        '^' => Op::StartsWith,
                        '*' => Op::Contains,
                        '$' => Op::EndsWith,
                        _ => return (None, range)
                    }
                }
            },
            None => return (None, range)
        };
        
        let start_pos = match op {
            Op::Equals => 1,
            _ => 2
        };
        let end_pos = (&src[(rest.0 + start_pos)..rest.1]).chars().position(|c| c == ']');
        if let Some(end_pos) = end_pos {
            if end_pos == 0 {
                (None, range)
            } else {
                let value = ((rest.0 + start_pos), (rest.0 + start_pos + end_pos));
                (
                    Some(
                        Attribute {
                            name: ident,
                            op,
                            value: Some(value)
                        }
                    ),
                    ((rest.0 + start_pos + end_pos + 1), rest.1)
                )
            }
        } else {
            (None, range)
        }
    }
}


pub(crate) struct Rule {
    pub(crate) tag: Option<Ident>,
    pub(crate) xid: Option<Xid>,
    pub(crate) id: Option<Ident>,
    pub(crate) classes: Vec<Ident>,
    pub(crate) attributes: Vec<Attribute>
}

impl Rule {
    fn parse(src: &str, range: (usize, usize)) -> (Option<Rule>, (usize, usize)) {
        let (tag, rest) = Ident::parse(src, range);
        let (xid, rest) = Xid::parse(src, rest);
        
        let (id, rest) = {
            if (&src[rest.0..rest.1]).chars().next().map(|c| c == '#').unwrap_or_default() {
                Ident::parse(src, ((rest.0 + 1), rest.1))
            } else {
                (None, rest)
            }
        };
        
        let (classes, rest) = {
            let mut classes = vec![];
            let mut nxt = rest;
            while (&src[nxt.0..nxt.1]).chars().next().map(|c| c == '.').unwrap_or_default() {
                let (class, rest) = Ident::parse(src, ((nxt.0 + 1), nxt.1));
                if let Some(class) = class {
                    classes.push(class);
                    nxt = rest;
                } else {
                    return (None, range);
                }
            }
            (classes, nxt)
        };
        
        let mut attributes = vec![];
        let mut rest = rest;
        while let (Some(attribute), nxt) = Attribute::parse(src, rest) {
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
    fn parse(src: &str, range: (usize, usize)) -> (Option<Link>, (usize, usize)) {
        let Some(start_pos) = (&src[range.0..range.1]).chars().position(|c| !c.is_whitespace()) else {
            return (None, range);
        };
        
        let (link, rest) = match (&src[(range.0 + start_pos)..range.1]).chars().next() {
            Some(c) => {
                let rest = &src[(range.0 + start_pos + 1)..range.1];
                let rest2 = &src[(range.0 + start_pos + 2)..range.1];
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
                    _ => return (None, range)
                }
            },
            None => return (None, range)
        };
        
        let Some(end_pos) = rest.chars().position(|c| !c.is_whitespace()) else {
            return (None, range);
        };
        
        (Some(link), ((range.0 + end_pos), range.1))
    }
}


pub(crate) struct Selector<'a> {
    pub(crate) rules: Vec<(Rule, Option<Link>)>,
    pub(crate) src: std::borrow::Cow<'a, str>
}

impl Selector<'_> {
    pub(crate) fn parse(src: std::borrow::Cow<str>) -> Option<Selector> {
        let mut rules = vec![];
        let mut range = (0, src.chars().count());
        
        loop {
            let (Some(rule), nxt) = Rule::parse(src.as_ref(), range) else {
                return None;
            };
            
            if let (Some(link), nxt) = Link::parse(src.as_ref(), nxt) {
                rules.push((rule, Some(link)));
                range = nxt;
            } else {
                rules.push((rule, None));
                return Some(
                    Selector {
                        rules, src
                    }
                );
            }
        }
    }
    
    pub(crate) fn get(&self, range: (usize, usize)) -> &str {
        &self.src[range.0..range.1]
    }
}
