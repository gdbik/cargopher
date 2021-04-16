#[derive(Debug)]
struct Item {
    code: char,
    title: String,
    path: String,
    host: String,
    port: u16
}

#[derive(Debug)]
pub struct Parser {
    page: Vec<Item>
}

impl Parser {
    pub fn new(tcp_string: &str)-> Self {
        let mut collect_string = tcp_string.split("\n");
        let mut page: Vec<Item> = vec![];

        for c in collect_string {
            let split_string = c.split("\t");
            let split_collect: Vec<&str> = split_string.collect();

            let get_title = split_collect
                .get(0)
                .unwrap_or(&"")
                .chars();

            let current_attr: Item = Item {
                code: split_collect
                    .get(0)
                    .unwrap_or(&"")
                    .chars()
                    .next()
                    .unwrap_or('e'),
                title: {
                    let mut title = get_title
                        .clone()
                        .skip(1)
                        .take(
                            get_title
                                .count()
                        )
                        .collect();

                    Parser::pop_return(&mut title)
                },
                path: split_collect
                    .get(1)
                    .unwrap_or(&"")
                    .to_string(),
                host: split_collect
                    .get(2)
                    .unwrap_or(&"")
                    .to_string(),
                port: {
                    let mut str = split_collect.get(3).unwrap_or(&"").to_string();
                    let pop = Parser::pop_return(&mut str);

                    pop.parse::<u16>().unwrap_or(0)
                }
            };

            page.push(current_attr);
        }

        Parser { page }
    }

    fn pop_return(to_parse: &mut String) -> String {
        if to_parse.ends_with('\r') {
            to_parse.pop();
        }

        to_parse.to_string()
    }
}