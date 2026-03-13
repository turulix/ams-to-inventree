# PatchedDataImportSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**timestamp** | Option<**String**> |  | [optional][readonly]
**data_file** | Option<**String**> |  | [optional]
**update_records** | Option<**bool**> | If enabled, existing records will be updated with new data | [optional]
**model_type** | Option<[**models::DataImportSessionModelTypeEnum**](DataImportSessionModelTypeEnum.md)> |  | [optional]
**available_fields** | Option<**serde_json::Value**> |  | [optional][readonly]
**status** | Option<[**models::DataImportSessionStatusEnum**](DataImportSessionStatusEnum.md)> | Import status  * `0` - Initializing * `10` - Mapping Columns * `20` - Importing Data * `30` - Processing Data * `40` - Complete | [optional][readonly]
**user** | Option<**i32**> |  | [optional][readonly]
**user_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]
**columns** | Option<**serde_json::Value**> |  | [optional][readonly]
**column_mappings** | Option<[**Vec<models::DataImportColumnMap>**](DataImportColumnMap.md)> |  | [optional][readonly]
**field_defaults** | Option<**serde_json::Value**> |  | [optional]
**field_overrides** | Option<**serde_json::Value**> |  | [optional]
**field_filters** | Option<**serde_json::Value**> |  | [optional]
**row_count** | Option<**i32**> |  | [optional][readonly]
**completed_row_count** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


