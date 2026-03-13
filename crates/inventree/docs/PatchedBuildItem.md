# PatchedBuildItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**build** | Option<**i32**> |  | [optional][readonly]
**build_line** | Option<**i32**> |  | [optional]
**install_into** | Option<**i32**> | Destination stock item | [optional]
**stock_item** | Option<**i32**> | Source stock item | [optional]
**quantity** | Option<**f64**> |  | [optional]
**location** | Option<**i32**> |  | [optional][readonly]
**build_detail** | Option<[**models::Build**](Build.md)> |  | [optional][readonly]
**location_detail** | Option<[**models::LocationBrief**](LocationBrief.md)> |  | [optional][readonly]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]
**stock_item_detail** | Option<[**models::StockItem**](StockItem.md)> |  | [optional][readonly]
**supplier_part_detail** | Option<[**models::SupplierPart**](SupplierPart.md)> |  | [optional][readonly]
**install_into_detail** | Option<[**models::StockItem**](StockItem.md)> |  | [optional][readonly]
**bom_reference** | Option<**String**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


