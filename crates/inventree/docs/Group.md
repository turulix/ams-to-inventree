# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** |  | 
**permissions** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Return a list of permissions associated with the group. | [optional][readonly]
**roles** | Option<[**Vec<models::RuleSet>**](RuleSet.md)> |  | [optional][readonly]
**users** | Option<[**Vec<models::User>**](User.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


