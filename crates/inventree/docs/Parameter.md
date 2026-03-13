# Parameter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**template** | **i32** | Parameter template | 
**model_type** | Option<[**models::ModelType4f8Enum**](ModelType4f8Enum.md)> |  | [optional][default to ]
**model_id** | **u64** | ID of the target model for this parameter | 
**data** | **String** | Parameter Value | 
**data_numeric** | Option<**f64**> |  | [optional]
**note** | Option<**String**> | Optional note field | [optional]
**updated** | Option<**String**> | Timestamp of last update | [optional][readonly]
**updated_by** | Option<**i32**> | User who last updated this object | [optional][readonly]
**template_detail** | [**models::ParameterTemplate**](ParameterTemplate.md) |  | [readonly]
**updated_by_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


