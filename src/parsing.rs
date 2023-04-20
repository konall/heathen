use nom::{
    character::complete::{one_of, char, multispace0, alpha1, alphanumeric1},
    branch::alt,
    bytes::complete::{tag, take_until1, take_while_m_n},
    combinator::{recognize, opt},
    multi::many0,
    sequence::{delimited, pair, tuple, preceded}
};


#[derive(Debug, Default)]
pub(crate) struct Uuid<'a>(&'a str);

fn is_hex_digit(c: char) -> bool {
    c.is_ascii() && nom::character::is_hex_digit(c as u8)
}

impl Uuid<'_> {
    fn parse(input: &str) -> nom::IResult<&str, Uuid> {
        let (input, uuid) = recognize(
            tuple((
                take_while_m_n(8, 8, is_hex_digit),
                char('-'),
                take_while_m_n(4, 4, is_hex_digit),
                char('-'),
                take_while_m_n(4, 4, is_hex_digit),
                char('-'),
                take_while_m_n(4, 4, is_hex_digit),
                char('-'),
                take_while_m_n(12, 12, is_hex_digit)
            ))
        )(input)?;
        
        nom::IResult::Ok((input, Uuid(uuid)))
    }
}

impl<'a> std::ops::Deref for Uuid<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug, Default)]
pub(crate) struct Ident<'a>(&'a str);

impl Ident<'_> {
    fn parse(input: &str) -> nom::IResult<&str, Ident> {
        let (input, res) = recognize(
            pair(
                alt((alpha1, tag("_"))),
                many0(
                    alt((alphanumeric1, tag("_")))
                )
            )
        )(input)?;
        
        nom::IResult::Ok((input, Ident(res)))
    }
}

impl<'a> std::ops::Deref for Ident<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug, Default)]
pub(crate) enum SelectorAttributeRuleType {
    #[default]
    Exists,
    Equals,
    NotEquals,
    StartsWith,
    Contains,
    EndsWith
}

#[derive(Debug, Default)]
pub(crate) struct SelectorAttributeRule<'a> {
    pub(crate) attr: Ident<'a>,
    pub(crate) value: Option<&'a str>,
    pub(crate) ty: SelectorAttributeRuleType
}

impl SelectorAttributeRule<'_> {
    fn parse(input: &str) -> nom::IResult<&str, SelectorAttributeRule> {
        let (input, attr) = Ident::parse(input)?;
        if input.chars().next().map(|c| c == ']').unwrap_or_default() {
            return nom::IResult::Ok((input, SelectorAttributeRule {
                attr,
                ..Default::default()
            }));
        }
        
        let (input, eqmod) = opt(one_of("!^*$"))(input)?;
        let (input, _) = char('=')(input)?;
        let (input, value) = take_until1("]")(input)?;
        
        nom::IResult::Ok((input, SelectorAttributeRule {
            attr,
            value: Some(value),
            ty: {
                if let Some(eqmod) = eqmod {
                    match eqmod {
                        '!' => SelectorAttributeRuleType::NotEquals,
                        '^' => SelectorAttributeRuleType::StartsWith,
                        '*' => SelectorAttributeRuleType::Contains,
                        '$' => SelectorAttributeRuleType::EndsWith,
                        _ => unreachable!()
                    }
                } else {
                    SelectorAttributeRuleType::Equals
                }
            }
        }))
    }
}


#[derive(Debug, Default)]
pub(crate) struct Selector<'a> {
    pub(crate) tag: Option<Ident<'a>>,
    pub(crate) uuid: Option<Uuid<'a>>,
    pub(crate) id: Option<Ident<'a>>,
    pub(crate) classes: Vec<Ident<'a>>,
    pub(crate) attribute_rules: Vec<SelectorAttributeRule<'a>>
}

impl Selector<'_> {
    fn parse(input: &str) -> nom::IResult<&str, Selector> {
        let (input, tag) = opt(Ident::parse)(input)?;
        let (input, uuid) = opt(preceded(char('%'), Uuid::parse))(input)?;
        let (input, id) = opt(preceded(char('#'), Ident::parse))(input)?;
        let (input, classes) = many0(preceded(char('.'), Ident::parse))(input)?;
        let (input, attribute_rules) = many0(delimited(char('['), SelectorAttributeRule::parse, char(']')))(input)?;
        
        nom::IResult::Ok((input, Selector {
            tag, uuid, id, classes, attribute_rules
        }))
    }
}


#[derive(Debug)]
pub(crate) enum SelectorLink {
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

impl SelectorLink {
    fn parse(input: &str) -> nom::IResult<&str, SelectorLink> {
        let (input, link) = delimited(
            multispace0,
            alt((tag("<<"), tag(">>"), tag("++"), tag("~~"), tag("~+"), recognize(one_of("<>+~")))),
            multispace0
        )(input)?;
        
        let link = match link {
            "<<" => SelectorLink::Ancestors,
            ">>" => SelectorLink::Descendants,
            "<" => SelectorLink::Parent,
            ">" => SelectorLink::Children,
            "+" => SelectorLink::NextSibling,
            "++" => SelectorLink::NextSiblings,
            "~" => SelectorLink::PrevSibling,
            "~~" => SelectorLink::PrevSiblings,
            "~+" => SelectorLink::Siblings,
            _ => unreachable!()
        };
        
        nom::IResult::Ok((input, link))
    }
}


#[derive(Debug, Default)]
pub(crate) struct Selectors<'a>(Vec<(Selector<'a>, Option<SelectorLink>)>);

impl<'a> Selectors<'a> {
    pub(crate) fn new(selector: &'a str) -> Self {
        Self::parse(selector).unwrap().1
    }
    
    fn parse(input: &str) -> nom::IResult<&str, Selectors> {
        let mut selectors = vec![];
        let mut input = input;
        let mut eoi = false;
        
        while !eoi {
            let (input2, (selector_rule, selector_link)) = pair(Selector::parse, opt(SelectorLink::parse))(input)?;
            eoi = selector_link.is_none();
            selectors.push((selector_rule, selector_link));
            input = input2;
        }
        
        nom::IResult::Ok((input, Selectors(selectors)))
    }
}

impl<'a> std::ops::Deref for Selectors<'a> {
    type Target = Vec<(Selector<'a>, Option<SelectorLink>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// #[test]
// pub fn fx() {
//     println!("x: {:?}", Selectors::parse("div%a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8#a.x.y.z[att=123][rrr*=qwerty][ijk] > button ~~ [role]"));
// }
