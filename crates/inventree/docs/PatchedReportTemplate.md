# PatchedReportTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**name** | Option<**String**> | Template name | [optional]
**description** | Option<**String**> | Template description | [optional]
**model_type** | Option<[**models::TemplateModelTypeEnum**](TemplateModelTypeEnum.md)> |  | [optional]
**template** | Option<**String**> |  | [optional]
**filters** | Option<**String**> | Template query filters (comma-separated list of key=value pairs) | [optional]
**filename_pattern** | Option<**String**> | Pattern for generating filenames | [optional]
**enabled** | Option<**bool**> | Template is enabled | [optional]
**revision** | Option<**i32**> |  | [optional][readonly]
**attach_to_model** | Option<**bool**> | Save report output as an attachment against linked model instance when printing | [optional]
**page_size** | Option<[**models::PageSizeEnum**](PageSizeEnum.md)> |  | [optional][default to A4]
**landscape** | Option<**bool**> | Render report in landscape orientation | [optional]
**merge** | Option<**bool**> | Render a single report against selected items | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


