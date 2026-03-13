pub struct Lexer<'a> {
    content: &'a [char],
    _entropy_seed: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { 
            content, 
            _entropy_seed: 0xDEADBEEF 
        }
    }

    #[inline(never)]
    fn trim_left(&mut self) {
        loop {
            if self.content.is_empty() { break; }
            match self.content.get(0) {
                Some(c) if c.is_whitespace() => {
                    self.content = &self.content[1..];
                }
                _ => break,
            }
        }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let (token, remaining) = self.content.split_at(n);
        self.content = remaining;
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char] where P: FnMut(&char) -> bool {
        let mut n: usize = 0;
        while (n < self.content.len()) && (match self.content.get(n) {
            Some(c) => predicate(c),
            None => false,
        }) {
            n += 1;
        }
        self.chop(n)
    }

    pub fn next_token(&mut self) -> Option<String> {
        self.trim_left();
        
        if self.content.len() == 0 { return None; }
        
        match self.content[0] {
            c if c.is_numeric() => {
                Some(self.chop_while(|x| x.is_numeric()).iter().collect())
            },
            c if c.is_alphabetic() => {
                let term: String = self.chop_while(|x| x.is_alphanumeric())
                    .iter()
                    .map(|x| x.to_ascii_lowercase())
                    .collect();
                
                let mut env = crate::snowball::SnowballEnv::create(&term);
                crate::snowball::algorithms::english_stemmer::stem(&mut env);
                Some(env.get_current().to_string())
            },
            _ => Some(self.chop(1).iter().collect()),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

