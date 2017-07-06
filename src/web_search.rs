use ddg;
use ddg::response::TopicResult;
use ddg::{Query, Response, RelatedTopic};

const APP_NAME: &'static str = "trebuchet-rs";

pub fn get_related_and_print(search_query: &str) {

    println!("Related　topics!");
    let related_result = get_related(search_query);

    match related_result {
        Ok(related) => {
            println!("Results count for query '{}': {}", search_query, related.len());
            for (i, r) in related.iter().enumerate() {
                match r {
                    &ddg::RelatedTopic::TopicResult(ref topic) => {
                        print_topic_result(i, topic);
                    }

                    &ddg::RelatedTopic::Topic(_) => {
                        println!("todo: implement");
                    }
                }
            }
        }
        Err(e) => println!("Error! Details: {:?}", e),
    }

}

pub fn instant_answer_and_print(query: &str) {

    let result: ddg::Response = match get_instant_answer(query) {
        Some(t) => t,
        None => panic!(),
    };
    match &result.abstract_text[..] {
                "" => {
                    println!("Unclear as to what answer you needed");
                    println!("Possible answers include:");
                    let mut counter = 1;
                    for elem in result.related_topics {
                        
                        match elem {
                            RelatedTopic::TopicResult(t) => {
                                let mut counter = &mut counter;
                                println!("{}. {}", counter, t.text);
                                *counter = *counter + 1;
                            },
                            RelatedTopic::Topic(_) => continue
                        };
                        
                    }
                },
                _ => println!("{}", result.abstract_text)
     };

}

fn get_instant_answer(search_query: &str) -> Option<Response> {
    let query: Query = Query::new(search_query, APP_NAME);
    match query.execute() {
        Ok(t) => return Some(t),
        Err(_) => return None,
    };
}

fn get_related(search_query: &str) -> Result<Vec<RelatedTopic>, ddg::query::Error> {
    let query: Query = Query::new(search_query, APP_NAME).no_html();
    let results: Response = query.execute()?;
    let related = results.related_topics;
    Ok(related)

}

fn print_topic_result(index: usize, topic: &TopicResult) {

    println!("{}:", index);
    println!(" title: {}", topic.text);
    println!(" url: {}", topic.first_url);
}
