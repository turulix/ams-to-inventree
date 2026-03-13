# PartStocktake

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pk** | **i32** |  | [readonly]
**part** | **i32** | Part for stocktake | 
**part_name** | **String** |  | [readonly]
**part_ipn** | Option<**String**> |  | [optional][readonly]
**part_description** | Option<**String**> |  | [optional][readonly]
**date** | [**String**](String.md) | Date stocktake was performed | [readonly]
**item_count** | Option<**i32**> | Number of individual stock entries at time of stocktake | [optional]
**quantity** | **f64** |  | 
**cost_min** | Option<**String**> |  | [optional]
**cost_min_currency** | Option<**String**> | Select currency from available options | [optional]
**cost_max** | Option<**String**> |  | [optional]
**cost_max_currency** | Option<**String**> | Select currency from available options | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


