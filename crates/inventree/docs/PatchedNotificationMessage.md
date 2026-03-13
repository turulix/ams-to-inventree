# PatchedNotificationMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**target** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Function to resolve generic object reference to target. | [optional][readonly]
**source** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Function to resolve generic object reference to source. | [optional][readonly]
**user** | Option<**i32**> |  | [optional][readonly]
**category** | Option<**String**> |  | [optional][readonly]
**name** | Option<**String**> |  | [optional][readonly]
**message** | Option<**String**> |  | [optional][readonly]
**creation** | Option<**String**> |  | [optional][readonly]
**age** | Option<**i32**> | Age of the message in seconds. | [optional][readonly]
**age_human** | Option<**String**> | Humanized age. | [optional][readonly]
**read** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


