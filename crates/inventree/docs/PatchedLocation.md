# PatchedLocation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**barcode_hash** | Option<**String**> | Unique hash of barcode data | [optional][readonly]
**name** | Option<**String**> | Name | [optional]
**level** | Option<**i32**> |  | [optional][readonly]
**description** | Option<**String**> | Description (optional) | [optional]
**parent** | Option<**i32**> | Parent stock location | [optional]
**pathstring** | Option<**String**> | Path | [optional][readonly]
**path** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> |  | [optional][readonly]
**items** | Option<**i32**> |  | [optional][readonly]
**sublocations** | Option<**i32**> |  | [optional][readonly]
**owner** | Option<**i32**> | Select Owner | [optional]
**icon** | Option<**String**> |  | [optional][readonly]
**custom_icon** | Option<**String**> | Icon (optional) | [optional]
**structural** | Option<**bool**> | Stock items may not be directly located into a structural stock locations, but may be located to child locations. | [optional]
**external** | Option<**bool**> | This is an external stock location | [optional]
**location_type** | Option<**i32**> | Stock location type of this location | [optional]
**location_type_detail** | Option<[**models::StockLocationType**](StockLocationType.md)> |  | [optional][readonly]
**tags** | Option<**Vec<String>**> |  | [optional]
**parameters** | Option<[**Vec<models::Parameter>**](Parameter.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


