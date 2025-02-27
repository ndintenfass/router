//! Canned data for use with MockSugbraph.
//! Eventually we may replace this with a real subgraph.
use crate::utils::test::mock::subgraph::MockSubgraph;
use serde_json::json;

/// Canned responses for accounts_subgraphs.
pub fn accounts_subgraph() -> MockSubgraph {
    let account_mocks = vec![
        (
            json! {{
                    "query": "query TopProducts__accounts__3($representations:[_Any!]!){_entities(representations:$representations){...on User{name}}}",
                    "operationName": "TopProducts__accounts__3",
                    "variables": {
                        "representations": [
                            {
                                "__typename": "User",
                                "id": "1"
                            },
                            {
                                "__typename": "User",
                                "id": "2"
                            },
                            {
                                "__typename": "User",
                                "id": "1"
                            }
                        ]
                    }
                }},
            json! {{
                    "data": {
                        "_entities": [
                            {
                                "name": "Ada Lovelace"
                            },
                            {
                                "name": "Alan Turing"
                            },
                            {
                                "name": "Ada Lovelace"
                            }
                        ]
                    }
                }}
        )
    ].into_iter().map(|(query, response)| (serde_json::from_value(query).unwrap(), serde_json::from_value(response).unwrap())).collect();
    MockSubgraph::new(account_mocks)
}

/// Canned responses for reviews_subgraphs.
pub fn reviews_subgraph() -> MockSubgraph {
    let review_mocks = vec![
        (
            json! {{
                    "query": "query TopProducts__reviews__1($representations:[_Any!]!){_entities(representations:$representations){...on Product{reviews{id product{__typename upc}author{__typename id}}}}}",
                    "operationName": "TopProducts__reviews__1",
                    "variables": {
                        "representations":[
                            {
                                "__typename": "Product",
                                "upc":"1"
                            },
                            {
                                "__typename": "Product",
                                "upc": "2"
                            }
                        ]
                    }
                }},
            json! {{
                    "data": {
                        "_entities": [
                            {
                                "reviews": [
                                    {
                                        "id": "1",
                                        "product": {
                                            "__typename": "Product",
                                            "upc": "1"
                                        },
                                        "author": {
                                            "__typename": "User",
                                            "id": "1"
                                        }
                                    },
                                    {
                                        "id": "4",
                                        "product": {
                                            "__typename": "Product",
                                            "upc": "1"
                                        },
                                        "author": {
                                            "__typename": "User",
                                            "id": "2"
                                        }
                                    }
                                ]
                            },
                            {
                                "reviews": [
                                    {
                                        "id": "2",
                                        "product": {
                                            "__typename": "Product",
                                            "upc": "2"
                                        },
                                        "author": {
                                            "__typename": "User",
                                            "id": "1"
                                        }
                                    }
                                ]
                            }
                        ]
                    }
                }}
        )
    ].into_iter().map(|(query, response)| (serde_json::from_value(query).unwrap(), serde_json::from_value(response).unwrap())).collect();
    MockSubgraph::new(review_mocks)
}

/// Canned responses for products_subgraphs.
pub fn products_subgraph() -> MockSubgraph {
    let product_mocks = vec![
        (
            json!{{
                    "query": "query TopProducts__products__0($first:Int){topProducts(first:$first){__typename upc name}}",
                    "operationName": "TopProducts__products__0",
                    "variables":{
                        "first":2u8
                    },
                }},
            json!{{
                    "data": {
                        "topProducts": [
                            {
                                "__typename": "Product",
                                "upc": "1",
                                "name":"Table"
                            },
                            {
                                "__typename": "Product",
                                "upc": "2",
                                "name": "Couch"
                            }
                        ]
                    }
                }}
        ),
        (
            json!{{
                    "query": "query TopProducts__products__2($representations:[_Any!]!){_entities(representations:$representations){...on Product{name}}}",
                    "operationName": "TopProducts__products__2",
                    "variables": {
                        "representations": [
                            {
                                "__typename": "Product",
                                "upc": "1"
                            },
                            {
                                "__typename": "Product",
                                "upc": "1"
                            },
                            {
                                "__typename": "Product",
                                "upc": "2"
                            }
                        ]
                    }
                }},
            json!{{
                    "data": {
                        "_entities": [
                            {
                                "name": "Table"
                            },
                            {
                                "name": "Table"
                            },
                            {
                                "name": "Couch"
                            }
                        ]
                    }
                }}
        )
    ].into_iter().map(|(query, response)| (serde_json::from_value(query).unwrap(), serde_json::from_value(response).unwrap())).collect();
    MockSubgraph::new(product_mocks)
}
