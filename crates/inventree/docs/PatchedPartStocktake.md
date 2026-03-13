# PatchedPartStocktake

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | Option<**i32**> |  | [optional][readonly]
**part** | Option<**i32**> | Part for stocktake | [optional]
**part_name** | Option<**String**> |  | [optional][readonly]
**part_ipn** | Option<**String**> |  | [optional][readonly]
**part_description** | Option<**String**> |  | [optional][readonly]
**date** | Option<[**String**](String.md)> | Date stocktake was performed | [optional][readonly]
**item_count** | Option<**i32**> | Number of individual stock entries at time of stocktake | [optional]
**quantity** | Option<**f64**> |  | [optional]
**cost_min** | Option<**String**> |  | [optional]
**cost_min_currency** | Option<**String**> | Select currency from available options | [optional]
**cost_max** | Option<**String**> |  | [optional]
**cost_max_currency** | Option<**String**> | Select currency from available options | [optional]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


