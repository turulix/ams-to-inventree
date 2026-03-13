# EmailMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **uuid::Uuid** | Unique identifier for this message | [readonly]
**global_id** | **uuid::Uuid** | Unique identifier for this message | [readonly]
**message_id_key** | Option<**String**> | Identifier for this message (might be supplied by external system) | [optional]
**thread_id_key** | Option<**String**> | Identifier for this message thread (might be supplied by external system) | [optional]
**thread** | Option<**uuid::Uuid**> | Linked thread for this message | [optional]
**subject** | **String** |  | 
**body** | **String** |  | 
**to** | **String** |  | 
**sender** | **String** |  | 
**status** | Option<**Status**> |  (enum: A, S, F, D, R, C, , ) | [optional]
**timestamp** | **String** |  | [readonly]
**headers** | Option<**serde_json::Value**> |  | [optional]
**full_message** | Option<**String**> |  | [optional]
**direction** | Option<**Direction**> |  (enum: I, O, , ) | [optional]
**priority** | [**models::PriorityEnum**](PriorityEnum.md) |  | 
**error_code** | Option<**String**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**error_timestamp** | Option<**String**> |  | [optional]
**delivery_options** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


