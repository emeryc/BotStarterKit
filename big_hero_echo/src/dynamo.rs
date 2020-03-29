use log::debug;
use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::{
    AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, PutItemInput, ScanError, ScanInput,
    ScanOutput,
};
use std::collections::HashMap;

static TABLE_NAME: &str = "bhp6_echo_v1";
static PRIMARY_KEY: &str = "SlackUserId";

pub(crate) enum DynamoError {
    Unknown,
}

pub(crate) struct EchoTabel {
    ddb_client: DynamoDbClient,
}

impl EchoTabel {
    pub(crate) fn new() -> Self {
        let ddb_client = DynamoDbClient::new(Region::UsWest1);
        EchoTabel { ddb_client }
    }

    pub(crate) async fn get_listeners(&self) -> Vec<String> {
        let result: Result<ScanOutput, RusotoError<ScanError>> = self
            .ddb_client
            .scan(ScanInput {
                attributes_to_get: None,
                conditional_operator: None,
                consistent_read: None,
                exclusive_start_key: None,
                expression_attribute_names: None,
                expression_attribute_values: None,
                filter_expression: None,
                index_name: None,
                limit: None,
                projection_expression: Some(PRIMARY_KEY.to_string()),
                return_consumed_capacity: None,
                scan_filter: None,
                segment: None,
                select: None,
                table_name: TABLE_NAME.to_string(),
                total_segments: None,
            })
            .await;
        debug!("{:?}", result);
        // Need to actually consume everything, but for this test it's fine.
        result.map_or(Vec::new(), |output| {
            output.items.map_or(Vec::new(), |entry| {
                entry
                    .iter()
                    .flat_map(|map| {
                        map.get(PRIMARY_KEY).map_or(Vec::new(), |val| {
                            val.s.clone().map_or(Vec::new(), |s| vec![s])
                        })
                    })
                    .collect()
            })
        })
    }

    pub(crate) async fn add_listener(&self, user: String) -> Result<(), DynamoError> {
        let result = self
            .ddb_client
            .put_item(PutItemInput {
                condition_expression: None,
                conditional_operator: None,
                expected: None,
                expression_attribute_names: None,
                expression_attribute_values: None,
                item: {
                    let mut hm = HashMap::new();
                    hm.insert(
                        PRIMARY_KEY.to_string(),
                        AttributeValue {
                            b: None,
                            bool: None,
                            bs: None,
                            l: None,
                            m: None,
                            n: None,
                            ns: None,
                            null: None,
                            s: Some(user),
                            ss: None,
                        },
                    );
                    hm
                },
                return_consumed_capacity: None,
                return_item_collection_metrics: None,
                return_values: None,
                table_name: TABLE_NAME.to_string(),
            })
            .await;
        debug!("{:?}", result);
        result.map(|_s| ()).map_err(|_err| DynamoError::Unknown)
    }

    pub(crate) async fn remove_listener(&self, user: String) -> Result<(), DynamoError> {
        let result = self
            .ddb_client
            .delete_item(DeleteItemInput {
                condition_expression: None,
                conditional_operator: None,
                expected: None,
                expression_attribute_names: None,
                expression_attribute_values: None,
                key: {
                    let mut hm = HashMap::new();
                    hm.insert(
                        PRIMARY_KEY.to_string(),
                        AttributeValue {
                            b: None,
                            bool: None,
                            bs: None,
                            l: None,
                            m: None,
                            n: None,
                            ns: None,
                            null: None,
                            s: Some(user),
                            ss: None,
                        },
                    );
                    hm
                },
                return_consumed_capacity: None,
                return_item_collection_metrics: None,
                return_values: None,
                table_name: TABLE_NAME.to_string(),
            })
            .await;
        debug!("{:?}", result);
        result.map(|_s| ()).map_err(|_err| DynamoError::Unknown)
    }
}
