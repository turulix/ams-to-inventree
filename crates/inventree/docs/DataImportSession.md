# DataImportSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**timestamp** | **String** |  | [readonly]
**data_file** | **String** |  | 
**update_records** | Option<**bool**> | If enabled, existing records will be updated with new data | [optional]
**model_type** | [**models::DataImportSessionModelTypeEnum**](DataImportSessionModelTypeEnum.md) |  | 
**available_fields** | Option<**serde_json::Value**> |  | [readonly]
**status** | [**models::DataImportSessionStatusEnum**](DataImportSessionStatusEnum.md) | Import status  * `0` - Initializing * `10` - Mapping Columns * `20` - Importing Data * `30` - Processing Data * `40` - Complete | [readonly]
**user** | Option<**i32**> |  | [optional][readonly]
**user_detail** | [**models::User**](User.md) |  | [readonly]
**columns** | Option<**serde_json::Value**> |  | [optional][readonly]
**column_mappings** | [**Vec<models::DataImportColumnMap>**](DataImportColumnMap.md) |  | [readonly]
**field_defaults** | Option<**serde_json::Value**> |  | [optional]
**field_overrides** | Option<**serde_json::Value**> |  | [optional]
**field_filters** | Option<**serde_json::Value**> |  | [optional]
**row_count** | **i32** |  | [readonly]
**completed_row_count** | **i32** |  | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


