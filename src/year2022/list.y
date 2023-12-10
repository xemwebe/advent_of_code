%start List
%%
List -> Result<List, ()>:
      '(' Itemlist ')' { Ok( List::Array($2?) ) }
    | '(' ')' { Ok(List::Empty) }
    ;

Item -> Result< Box<List>, ()>:
        Num { Ok(Box::new($1?)) }
    |   List { Ok(Box::new($1?)) }
    ;

Itemlist -> Result< Vec<Box<List>>, ()>:
        Itemlist ',' Item { 
            let mut v = $1?;
            v.push($3?);
            Ok(v) 
        }
    |   Item { Ok(vec![ $1? ]) }
    ;

Num -> Result<List, ()>:
    'INT' {
        let v = $1.map_err(|_| ())?;
        Ok(List::Num(parse_int($lexer.span_str(v.span()))?))
      }
    ;
%%
// Any functions here are in scope for all the grammar actions above.
use crate::year2022::list::List;

fn parse_int(s: &str) -> Result<u8, ()> {
    match s.parse::<u8>() {
        Ok(val) => Ok(val),
        Err(_) => {
            format!("{} cannot be represented as a u8", s);
            Err(())
        }
    }
}
