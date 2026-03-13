# DataOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**created** | [**String**](String.md) |  | [readonly]
**user** | Option<**i32**> |  | [optional]
**user_detail** | [**models::User**](User.md) |  | [readonly]
**total** | Option<**u32**> |  | [optional]
**progress** | Option<**u32**> |  | [optional]
**complete** | Option<**bool**> |  | [optional]
**output_type** | Option<**String**> |  | [optional]
**template_name** | Option<**String**> |  | [optional]
**plugin** | Option<**String**> |  | [optional]
**output** | Option<**String**> |  | [optional][readonly]
**errors** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


