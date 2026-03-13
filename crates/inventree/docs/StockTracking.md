# StockTracking

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**item** | Option<**i32**> |  | [optional]
**item_detail** | Option<[**models::StockItem**](StockItem.md)> |  | [optional][readonly]
**part** | Option<**i32**> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**date** | **String** |  | [readonly]
**deltas** | Option<**serde_json::Value**> |  | [readonly]
**label** | **String** |  | [readonly]
**notes** | Option<**String**> | Entry notes | [optional]
**tracking_type** | **i32** |  | [readonly]
**user** | Option<**i32**> |  | [optional][readonly]
**user_detail** | Option<[**models::User**](User.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


