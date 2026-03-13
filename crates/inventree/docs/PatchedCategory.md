# PatchedCategory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**name** | Option<**String**> | Name | [optional]
**description** | Option<**String**> | Description (optional) | [optional]
**default_location** | Option<**i32**> | Default location for parts in this category | [optional]
**default_keywords** | Option<**String**> | Default keywords for parts in this category | [optional]
**level** | Option<**i32**> |  | [optional][readonly]
**parent** | Option<**i32**> | Parent part category | [optional]
**part_count** | Option<**i32**> |  | [optional][readonly]
**subcategories** | Option<**i32**> |  | [optional][readonly]
**pathstring** | Option<**String**> | Path | [optional][readonly]
**path** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> |  | [optional][readonly]
**starred** | Option<**bool**> | Return True if the category is directly \"starred\" by the current user. | [optional][readonly]
**structural** | Option<**bool**> | Parts may not be directly assigned to a structural category, but may be assigned to child categories. | [optional]
**icon** | Option<**String**> | Icon (optional) | [optional]
**parent_default_location** | Option<**i32**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


