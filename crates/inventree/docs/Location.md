# Location

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**barcode_hash** | **String** | Unique hash of barcode data | [readonly]
**name** | **String** | Name | 
**level** | **i32** |  | [readonly]
**description** | Option<**String**> | Description (optional) | [optional]
**parent** | Option<**i32**> | Parent stock location | [optional]
**pathstring** | **String** | Path | [readonly]
**items** | **i32** |  | [readonly]
**sublocations** | **i32** |  | [readonly]
**owner** | Option<**i32**> | Select Owner | [optional]
**icon** | **String** |  | [readonly]
**custom_icon** | Option<**String**> | Icon (optional) | [optional]
**structural** | Option<**bool**> | Stock items may not be directly located into a structural stock locations, but may be located to child locations. | [optional]
**external** | Option<**bool**> | This is an external stock location | [optional]
**location_type** | Option<**i32**> | Stock location type of this location | [optional]
**location_type_detail** | Option<[**models::StockLocationType**](StockLocationType.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


