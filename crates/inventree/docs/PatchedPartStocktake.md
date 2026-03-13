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
**item_count** | Option<**i64**> | Number of individual stock entries at time of stocktake | [optional]
**quantity** | Option<**f64**> |  | [optional]
**cost_min** | Option<**String**> |  | [optional]
**cost_min_currency** | Option<**String**> | Select currency from available options  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**cost_max** | Option<**String**> |  | [optional]
**cost_max_currency** | Option<**String**> | Select currency from available options  * `AUD` - AUD - Australian Dollar * `CAD` - CAD - Canadian Dollar * `CNY` - CNY - Chinese Yuan * `EUR` - EUR - Euro * `GBP` - GBP - British Pound * `JPY` - JPY - Japanese Yen * `NZD` - NZD - New Zealand Dollar * `USD` - USD - US Dollar  Other valid currencies may be found in the 'CURRENCY_CODES' global setting. | [optional]
**part_detail** | Option<[**models::PartBrief**](PartBrief.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


