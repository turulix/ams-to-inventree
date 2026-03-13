# PatchedParameter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**template** | Option<**i32**> | Parameter template | [optional]
**model_type** | Option<[**models::ModelType4f8Enum**](ModelType4f8Enum.md)> |  | [optional][default to ]
**model_id** | Option<**u32**> | ID of the target model for this parameter | [optional]
**data** | Option<**String**> | Parameter Value | [optional]
**data_numeric** | Option<**f64**> |  | [optional]
**note** | Option<**String**> | Optional note field | [optional]
**updated** | Option<**String**> | Timestamp of last update | [optional][readonly]
**updated_by** | Option<**i32**> | User who last updated this object | [optional][readonly]
**template_detail** | Option<[**models::ParameterTemplate**](ParameterTemplate.md)> |  | [optional][readonly]
**updated_by_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


