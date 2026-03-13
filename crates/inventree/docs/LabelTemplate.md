# LabelTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**name** | **String** | Template name | 
**description** | **String** | Template description | 
**model_type** | [**models::TemplateModelTypeEnum**](TemplateModelTypeEnum.md) |  | 
**template** | **String** |  | 
**filters** | Option<**String**> | Template query filters (comma-separated list of key=value pairs) | [optional]
**filename_pattern** | Option<**String**> | Pattern for generating filenames | [optional]
**enabled** | Option<**bool**> | Template is enabled | [optional]
**revision** | **i32** |  | [readonly]
**attach_to_model** | Option<**bool**> | Save report output as an attachment against linked model instance when printing | [optional]
**width** | Option<**f64**> | Label width, specified in mm | [optional]
**height** | Option<**f64**> | Label height, specified in mm | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


