# MachineSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**key** | **String** |  | [readonly]
**value** | Option<**String**> |  | 
**name** | **String** |  | [readonly]
**description** | **String** |  | [readonly]
**r#type** | **String** |  | [readonly]
**choices** | **Vec<serde_json::Value>** | Returns the choices available for a given item. | [readonly]
**model_name** | Option<**String**> |  | [optional][readonly]
**model_filters** | **std::collections::HashMap<String, serde_json::Value>** |  | [readonly]
**api_url** | Option<**String**> |  | [optional][readonly]
**typ** | **String** |  | [readonly]
**units** | **String** |  | [readonly]
**required** | **bool** |  | [readonly]
**confirm** | **bool** | Indicates if changing this setting requires confirmation | [readonly]
**confirm_text** | **String** |  | [readonly]
**config_type** | [**models::ConfigTypeEnum**](ConfigTypeEnum.md) |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


