# Nested

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | [readonly]
**metadata** | Option<**serde_json::Value**> | JSON metadata field, for use by external plugins | [optional]
**name** | **String** | Name | 
**description** | Option<**String**> | Description (optional) | [optional]
**pathstring** | Option<**String**> | Path | [optional]
**structural** | Option<**bool**> | Parts may not be directly assigned to a structural category, but may be assigned to child categories. | [optional]
**default_keywords** | Option<**String**> | Default keywords for parts in this category | [optional]
**_icon** | Option<**String**> | Icon (optional) | [optional]
**lft** | **i32** |  | [readonly]
**rght** | **i32** |  | [readonly]
**tree_id** | **i32** |  | [readonly]
**level** | **i32** |  | [readonly]
**parent** | Option<**i32**> |  | [optional]
**default_location** | Option<**i32**> | Default location for parts in this category | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


