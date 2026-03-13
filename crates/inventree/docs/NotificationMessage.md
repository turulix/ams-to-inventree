# NotificationMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**target** | **std::collections::HashMap<String, serde_json::Value>** | Function to resolve generic object reference to target. | [readonly]
**source** | **std::collections::HashMap<String, serde_json::Value>** | Function to resolve generic object reference to source. | [readonly]
**user** | **i32** |  | [readonly]
**category** | **String** |  | [readonly]
**name** | **String** |  | [readonly]
**message** | Option<**String**> |  | [optional][readonly]
**creation** | **String** |  | [readonly]
**age** | **i32** | Age of the message in seconds. | [readonly]
**age_human** | **String** | Humanized age. | [readonly]
**read** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


