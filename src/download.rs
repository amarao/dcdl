use crate::config::Config;
use tokio;
use reqwest::Client;
use serde_json::json;

pub fn download(cfg: Config) -> Result<std::string::String, Box<dyn std::error::Error>>{
    let url = cfg.url;
    let token = cfg.token;
    let query = json!({
        "query": "query cardsQuery(\n  $count: Int!\n  $cursor: String\n  $deckId: ID!\n  $search: String\n  $cardState: CardState\n) {\n  node(id: $deckId) {\n    __typename\n    ...cardsQuery_Deck_1yGN6X\n    id\n  }\n}\n\nfragment cardsQuery_Deck_1yGN6X on Deck {\n  cards(first: $count, after: $cursor, search: $search, cardState: $cardState) {\n    edges {\n      node {\n        id\n        sCardId\n        sBackId\n        sourceId\n        front\n        back\n        hint\n        waiting\n        knownCount\n        source {\n          kind\n          course\n          id\n        }\n        sCard {\n          theory {\n            sCardId\n            theory\n            theoryIsChecked\n            theoryNative\n            theoryNativeIsChecked\n            theoryEn\n            lang\n            langNative\n            canEdit\n          }\n          id\n        }\n        svg {\n          flatId\n          url\n          id\n        }\n        __typename\n      }\n      cursor\n    }\n    pageInfo {\n      endCursor\n      hasNextPage\n    }\n  }\n  id\n}\n",
        "variables": {
            "cardState": null,
            "count": 30,
            "cursor": null,
            "deckId": cfg.deckid,
            "search": ""
        }
    });

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(async {
        let client = Client::new();
        let response = client
            .post(url)
            .bearer_auth(token)
            .json(&query)
            .send()
            .await?;
        let body = response.text().await?;
        Ok(body)
    });
    return result
}
