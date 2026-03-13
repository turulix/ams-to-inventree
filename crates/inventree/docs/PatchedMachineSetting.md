# PatchedMachineSetting

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**key** | Option<**String**> |  | [optional][readonly]
**value** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional][readonly]
**description** | Option<**String**> |  | [optional][readonly]
**r#type** | Option<**String**> |  | [optional][readonly]
**choices** | Option<**Vec<serde_json::Value>**> | Returns the choices available for a given item. | [optional][readonly]
**model_name** | Option<**String**> |  | [optional][readonly]
**model_filters** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional][readonly]
**api_url** | Option<**String**> |  | [optional][readonly]
**typ** | Option<**String**> |  | [optional][readonly]
**units** | Option<**String**> |  | [optional][readonly]
**required** | Option<**bool**> |  | [optional][readonly]
**confirm** | Option<**bool**> | Indicates if changing this setting requires confirmation | [optional][readonly]
**confirm_text** | Option<**String**> |  | [optional][readonly]
**config_type** | Option<[**models::ConfigTypeEnum**](ConfigTypeEnum.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


